//! APRS domain codecs and models.
//!
//! This crate intentionally starts with a small, deterministic subset of APRS:
//! uncompressed position reports without timestamps, status reports, and
//! messages/acknowledgements/rejections. Unsupported data type identifiers stay
//! explicit so additional APRS formats can be added without weakening parsing.

use std::str;

use radiolink_ax25::Ax25UiFrame;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AprsPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AprsPositionReport {
    pub position: AprsPosition,
    pub symbol_table: char,
    pub symbol_code: char,
    pub comment: String,
    pub messaging_capable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AprsStatus {
    pub timestamp: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AprsMessageKind {
    Text {
        text: String,
        message_id: Option<String>,
    },
    Acknowledgement {
        message_id: String,
    },
    Rejection {
        message_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AprsMessage {
    pub addressee: String,
    pub kind: AprsMessageKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AprsPacket {
    Position(AprsPositionReport),
    Status(AprsStatus),
    Message(AprsMessage),
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AprsError {
    #[error("APRS information field is empty")]
    EmptyInformation,
    #[error("APRS information field must be valid UTF-8/ASCII for this parser subset")]
    InvalidText,
    #[error("unsupported APRS data type identifier 0x{0:02X}")]
    UnsupportedDataType(u8),
    #[error("malformed APRS status report")]
    MalformedStatus,
    #[error("malformed APRS message")]
    MalformedMessage,
    #[error("malformed APRS uncompressed position report")]
    MalformedPosition,
}

pub fn parse_ax25_ui_frame(frame: &Ax25UiFrame) -> Result<AprsPacket, AprsError> {
    parse_information_field(&frame.information)
}

pub fn parse_information_field(information: &[u8]) -> Result<AprsPacket, AprsError> {
    let (&data_type, _) = information
        .split_first()
        .ok_or(AprsError::EmptyInformation)?;

    match data_type {
        b'!' => parse_position(information, false).map(AprsPacket::Position),
        b'=' => parse_position(information, true).map(AprsPacket::Position),
        b'>' => parse_status(information).map(AprsPacket::Status),
        b':' => parse_message(information).map(AprsPacket::Message),
        other => Err(AprsError::UnsupportedDataType(other)),
    }
}

pub fn encode_packet(packet: &AprsPacket) -> Result<Vec<u8>, AprsError> {
    match packet {
        AprsPacket::Position(report) => encode_position(report),
        AprsPacket::Status(status) => encode_status(status),
        AprsPacket::Message(message) => encode_message(message),
    }
}

pub fn encode_status(status: &AprsStatus) -> Result<Vec<u8>, AprsError> {
    if !is_safe_text(&status.text) {
        return Err(AprsError::MalformedStatus);
    }

    let mut out = String::from(">");
    if let Some(timestamp) = &status.timestamp {
        if !is_dhm_timestamp(timestamp) {
            return Err(AprsError::MalformedStatus);
        }
        out.push_str(timestamp);
    }
    out.push_str(&status.text);
    Ok(out.into_bytes())
}

pub fn encode_message(message: &AprsMessage) -> Result<Vec<u8>, AprsError> {
    if message.addressee.is_empty()
        || message.addressee.len() > 9
        || !message
            .addressee
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        return Err(AprsError::MalformedMessage);
    }

    let body = match &message.kind {
        AprsMessageKind::Text { text, message_id } => {
            if !is_safe_text(text) {
                return Err(AprsError::MalformedMessage);
            }
            if let Some(message_id) = message_id {
                if !is_message_id(message_id) {
                    return Err(AprsError::MalformedMessage);
                }
                format!("{text}{{{message_id}")
            } else {
                text.clone()
            }
        }
        AprsMessageKind::Acknowledgement { message_id } => {
            if !is_message_id(message_id) {
                return Err(AprsError::MalformedMessage);
            }
            format!("ack{message_id}")
        }
        AprsMessageKind::Rejection { message_id } => {
            if !is_message_id(message_id) {
                return Err(AprsError::MalformedMessage);
            }
            format!("rej{message_id}")
        }
    };

    Ok(format!(":{:<9}:{body}", message.addressee).into_bytes())
}

pub fn encode_position(report: &AprsPositionReport) -> Result<Vec<u8>, AprsError> {
    if !report.symbol_table.is_ascii_graphic()
        || !report.symbol_code.is_ascii_graphic()
        || !is_safe_text(&report.comment)
    {
        return Err(AprsError::MalformedPosition);
    }

    let latitude = encode_coordinate(report.position.latitude, 2, 90.0, 'N', 'S')?;
    let longitude = encode_coordinate(report.position.longitude, 3, 180.0, 'E', 'W')?;
    let data_type = if report.messaging_capable { '=' } else { '!' };

    Ok(format!(
        "{data_type}{latitude}{}{longitude}{}{}",
        report.symbol_table, report.symbol_code, report.comment
    )
    .into_bytes())
}

fn encode_coordinate(
    value: f64,
    degree_width: usize,
    maximum: f64,
    positive_hemisphere: char,
    negative_hemisphere: char,
) -> Result<String, AprsError> {
    if !value.is_finite() || value.abs() > maximum {
        return Err(AprsError::MalformedPosition);
    }

    let absolute = value.abs();
    let mut degrees = absolute.floor() as u16;
    let raw_minutes = (absolute - f64::from(degrees)) * 60.0;
    let mut minutes = (raw_minutes * 100.0).round() / 100.0;

    if minutes >= 60.0 {
        degrees += 1;
        minutes = 0.0;
    }
    if f64::from(degrees) > maximum {
        return Err(AprsError::MalformedPosition);
    }

    let hemisphere = if value.is_sign_negative() {
        negative_hemisphere
    } else {
        positive_hemisphere
    };

    Ok(format!(
        "{degrees:0degree_width$}{minutes:05.2}{hemisphere}"
    ))
}

fn parse_status(information: &[u8]) -> Result<AprsStatus, AprsError> {
    let text = decode_text(information)?;
    let payload = text.strip_prefix('>').ok_or(AprsError::MalformedStatus)?;

    let (timestamp, status_text) = if payload.len() >= 7 && is_dhm_timestamp(&payload[..7]) {
        (Some(payload[..7].to_owned()), &payload[7..])
    } else {
        (None, payload)
    };

    Ok(AprsStatus {
        timestamp,
        text: status_text.to_owned(),
    })
}

fn is_dhm_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 7 && bytes[..6].iter().all(u8::is_ascii_digit) && matches!(bytes[6], b'z' | b'Z')
}

fn parse_message(information: &[u8]) -> Result<AprsMessage, AprsError> {
    let text = decode_text(information)?;
    if text.len() < 11 || !text.starts_with(':') || text.as_bytes()[10] != b':' {
        return Err(AprsError::MalformedMessage);
    }

    let addressee = text[1..10].trim_end();
    if addressee.is_empty() {
        return Err(AprsError::MalformedMessage);
    }

    let body = &text[11..];
    let kind = if let Some(message_id) = parse_control_message(body, "ack") {
        AprsMessageKind::Acknowledgement { message_id }
    } else if let Some(message_id) = parse_control_message(body, "rej") {
        AprsMessageKind::Rejection { message_id }
    } else {
        let (text, message_id) = split_message_id(body);
        AprsMessageKind::Text {
            text: text.to_owned(),
            message_id,
        }
    };

    Ok(AprsMessage {
        addressee: addressee.to_owned(),
        kind,
    })
}

fn parse_control_message(body: &str, prefix: &str) -> Option<String> {
    let message_id = body.strip_prefix(prefix)?;
    if is_message_id(message_id) {
        Some(message_id.to_owned())
    } else {
        None
    }
}

fn split_message_id(body: &str) -> (&str, Option<String>) {
    let Some(index) = body.rfind('{') else {
        return (body, None);
    };

    let message_id = &body[index + 1..];
    if !is_message_id(message_id) {
        return (body, None);
    }

    (&body[..index], Some(message_id.to_owned()))
}

fn is_message_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 5 && value.bytes().all(|byte| byte.is_ascii_alphanumeric())
}

fn parse_position(
    information: &[u8],
    messaging_capable: bool,
) -> Result<AprsPositionReport, AprsError> {
    let text = decode_text(information)?;
    if text.len() < 20 {
        return Err(AprsError::MalformedPosition);
    }

    let latitude = parse_latitude(&text[1..9])?;
    let symbol_table = text.as_bytes()[9] as char;
    let longitude = parse_longitude(&text[10..19])?;
    let symbol_code = text.as_bytes()[19] as char;
    let comment = text[20..].to_owned();

    if !symbol_table.is_ascii_graphic() || !symbol_code.is_ascii_graphic() {
        return Err(AprsError::MalformedPosition);
    }

    Ok(AprsPositionReport {
        position: AprsPosition {
            latitude,
            longitude,
        },
        symbol_table,
        symbol_code,
        comment,
        messaging_capable,
    })
}

fn parse_latitude(value: &str) -> Result<f64, AprsError> {
    if value.len() != 8 {
        return Err(AprsError::MalformedPosition);
    }
    let hemisphere = value.as_bytes()[7];
    if !matches!(hemisphere, b'N' | b'S') {
        return Err(AprsError::MalformedPosition);
    }

    let degrees = parse_decimal(&value[..2])?;
    let minutes = parse_decimal(&value[2..7])?;
    if degrees > 90.0 || minutes >= 60.0 || (degrees == 90.0 && minutes != 0.0) {
        return Err(AprsError::MalformedPosition);
    }

    let decimal = degrees + minutes / 60.0;
    Ok(if hemisphere == b'S' {
        -decimal
    } else {
        decimal
    })
}

fn parse_longitude(value: &str) -> Result<f64, AprsError> {
    if value.len() != 9 {
        return Err(AprsError::MalformedPosition);
    }
    let hemisphere = value.as_bytes()[8];
    if !matches!(hemisphere, b'E' | b'W') {
        return Err(AprsError::MalformedPosition);
    }

    let degrees = parse_decimal(&value[..3])?;
    let minutes = parse_decimal(&value[3..8])?;
    if degrees > 180.0 || minutes >= 60.0 || (degrees == 180.0 && minutes != 0.0) {
        return Err(AprsError::MalformedPosition);
    }

    let decimal = degrees + minutes / 60.0;
    Ok(if hemisphere == b'W' {
        -decimal
    } else {
        decimal
    })
}

fn parse_decimal(value: &str) -> Result<f64, AprsError> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
        || value.bytes().filter(|&byte| byte == b'.').count() > 1
    {
        return Err(AprsError::MalformedPosition);
    }

    value
        .parse::<f64>()
        .map_err(|_| AprsError::MalformedPosition)
}

fn decode_text(information: &[u8]) -> Result<&str, AprsError> {
    let text = str::from_utf8(information).map_err(|_| AprsError::InvalidText)?;
    if !text.bytes().all(|byte| {
        byte == b'\r' || byte == b'\n' || byte == b'\t' || byte.is_ascii_graphic() || byte == b' '
    }) {
        return Err(AprsError::InvalidText);
    }
    Ok(text)
}

fn is_safe_text(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| byte == b' ' || byte.is_ascii_graphic())
}

#[cfg(test)]
mod tests {
    use radiolink_ax25::{Ax25Address, Ax25UiFrame};

    use super::*;

    #[test]
    fn parses_uncompressed_position_without_timestamp() {
        let packet = parse_information_field(b"!0249.00N/06040.00W>Boa Vista").unwrap();
        let AprsPacket::Position(report) = packet else {
            panic!("expected position");
        };

        assert!((report.position.latitude - 2.816_666_666_7).abs() < 1e-9);
        assert!((report.position.longitude + 60.666_666_666_7).abs() < 1e-9);
        assert_eq!(report.symbol_table, '/');
        assert_eq!(report.symbol_code, '>');
        assert_eq!(report.comment, "Boa Vista");
        assert!(!report.messaging_capable);
    }

    #[test]
    fn parses_messaging_capable_position() {
        let packet = parse_information_field(b"=4903.50N/07201.75W-Test").unwrap();
        let AprsPacket::Position(report) = packet else {
            panic!("expected position");
        };

        assert!(report.messaging_capable);
        assert!((report.position.latitude - 49.058_333_333_3).abs() < 1e-9);
        assert!((report.position.longitude + 72.029_166_666_7).abs() < 1e-9);
        assert_eq!(report.symbol_code, '-');
        assert_eq!(report.comment, "Test");
    }

    #[test]
    fn round_trips_supported_position_encoding() {
        let original = AprsPacket::Position(AprsPositionReport {
            position: AprsPosition {
                latitude: 2.816_666_666_7,
                longitude: -60.666_666_666_7,
            },
            symbol_table: '/',
            symbol_code: '>',
            comment: "Boa Vista".into(),
            messaging_capable: true,
        });

        let encoded = encode_packet(&original).unwrap();
        assert_eq!(encoded, b"=0249.00N/06040.00W>Boa Vista");
        assert_eq!(parse_information_field(&encoded).unwrap(), original);
    }

    #[test]
    fn rejects_invalid_position_minutes() {
        assert_eq!(
            parse_information_field(b"!0260.00N/06040.00W>bad"),
            Err(AprsError::MalformedPosition)
        );
    }

    #[test]
    fn parses_status_with_and_without_timestamp() {
        assert_eq!(
            parse_information_field(b">Net Control Center"),
            Ok(AprsPacket::Status(AprsStatus {
                timestamp: None,
                text: "Net Control Center".into(),
            }))
        );
        assert_eq!(
            parse_information_field(b">092345zNet Control Center"),
            Ok(AprsPacket::Status(AprsStatus {
                timestamp: Some("092345z".into()),
                text: "Net Control Center".into(),
            }))
        );
    }

    #[test]
    fn round_trips_status_encoding() {
        let original = AprsPacket::Status(AprsStatus {
            timestamp: Some("092345z".into()),
            text: "RadioLink online".into(),
        });

        let encoded = encode_packet(&original).unwrap();
        assert_eq!(encoded, b">092345zRadioLink online");
        assert_eq!(parse_information_field(&encoded).unwrap(), original);
    }

    #[test]
    fn parses_message_with_number() {
        assert_eq!(
            parse_information_field(b":PV8ABC-7 :Cheguei bem{42"),
            Ok(AprsPacket::Message(AprsMessage {
                addressee: "PV8ABC-7".into(),
                kind: AprsMessageKind::Text {
                    text: "Cheguei bem".into(),
                    message_id: Some("42".into()),
                },
            }))
        );
    }

    #[test]
    fn round_trips_message_encoding() {
        let original = AprsPacket::Message(AprsMessage {
            addressee: "PV8ABC-7".into(),
            kind: AprsMessageKind::Text {
                text: "Cheguei bem".into(),
                message_id: Some("42".into()),
            },
        });

        let encoded = encode_packet(&original).unwrap();
        assert_eq!(encoded, b":PV8ABC-7 :Cheguei bem{42");
        assert_eq!(parse_information_field(&encoded).unwrap(), original);
    }

    #[test]
    fn parses_acknowledgement_and_rejection() {
        assert_eq!(
            parse_information_field(b":PV8ABC-7 :ack003"),
            Ok(AprsPacket::Message(AprsMessage {
                addressee: "PV8ABC-7".into(),
                kind: AprsMessageKind::Acknowledgement {
                    message_id: "003".into(),
                },
            }))
        );
        assert_eq!(
            parse_information_field(b":PV8ABC-7 :rej003"),
            Ok(AprsPacket::Message(AprsMessage {
                addressee: "PV8ABC-7".into(),
                kind: AprsMessageKind::Rejection {
                    message_id: "003".into(),
                },
            }))
        );
    }

    #[test]
    fn round_trips_acknowledgement_encoding() {
        let original = AprsPacket::Message(AprsMessage {
            addressee: "PV8ABC-7".into(),
            kind: AprsMessageKind::Acknowledgement {
                message_id: "003".into(),
            },
        });

        let encoded = encode_packet(&original).unwrap();
        assert_eq!(encoded, b":PV8ABC-7 :ack003");
        assert_eq!(parse_information_field(&encoded).unwrap(), original);
    }

    #[test]
    fn parses_directly_from_ax25_ui_information() {
        let frame = Ax25UiFrame {
            destination: Ax25Address::new("APRS", 0).unwrap(),
            source: Ax25Address::new("PV8ABC", 7).unwrap(),
            digipeaters: Vec::new(),
            information: b">RadioLink online".to_vec(),
        };

        assert_eq!(
            parse_ax25_ui_frame(&frame),
            Ok(AprsPacket::Status(AprsStatus {
                timestamp: None,
                text: "RadioLink online".into(),
            }))
        );
    }

    #[test]
    fn rejects_bad_encoding_inputs() {
        assert_eq!(
            encode_message(&AprsMessage {
                addressee: "TOO-LONG-10".into(),
                kind: AprsMessageKind::Text {
                    text: "test".into(),
                    message_id: None,
                },
            }),
            Err(AprsError::MalformedMessage)
        );
        assert_eq!(
            encode_position(&AprsPositionReport {
                position: AprsPosition {
                    latitude: 91.0,
                    longitude: 0.0,
                },
                symbol_table: '/',
                symbol_code: '>',
                comment: String::new(),
                messaging_capable: false,
            }),
            Err(AprsError::MalformedPosition)
        );
    }

    #[test]
    fn keeps_unsupported_data_type_explicit() {
        assert_eq!(
            parse_information_field(b"/092345z4903.50N/07201.75W-Test"),
            Err(AprsError::UnsupportedDataType(b'/'))
        );
    }
}
