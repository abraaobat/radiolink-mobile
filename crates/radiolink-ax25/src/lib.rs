//! AX.25 framing/parsing primitives.
//!
//! RadioLink's KISS path carries AX.25 frames without the HDLC flags or FCS, so
//! this crate models the address/control/PID/information portion seen by a TNC.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const UI_CONTROL: u8 = 0x03;
pub const NO_LAYER_3_PID: u8 = 0xF0;
pub const MAX_DIGIPEATERS: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ax25Address {
    pub callsign: String,
    pub ssid: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ax25UiFrame {
    pub destination: Ax25Address,
    pub source: Ax25Address,
    pub digipeaters: Vec<Ax25Address>,
    pub information: Vec<u8>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Ax25Error {
    #[error("AX.25 callsign must contain 1-6 ASCII letters or digits")]
    InvalidCallsign,
    #[error("AX.25 SSID must be in the range 0-15")]
    InvalidSsid,
    #[error("AX.25 address field is truncated")]
    TruncatedAddress,
    #[error("AX.25 frame must contain destination and source addresses")]
    MissingRequiredAddresses,
    #[error("AX.25 frame contains more than {MAX_DIGIPEATERS} digipeaters")]
    TooManyDigipeaters,
    #[error("unsupported AX.25 control/PID combination: control=0x{control:02X}, pid=0x{pid:02X}")]
    UnsupportedControlPid { control: u8, pid: u8 },
    #[error("AX.25 frame is missing control/PID bytes")]
    MissingControlPid,
}

impl Ax25Address {
    pub fn new(callsign: impl Into<String>, ssid: u8) -> Result<Self, Ax25Error> {
        let callsign = callsign.into().trim().to_ascii_uppercase();
        validate_callsign(&callsign)?;
        if ssid > 15 {
            return Err(Ax25Error::InvalidSsid);
        }
        Ok(Self { callsign, ssid })
    }

    pub fn encode(&self, last: bool) -> Result<[u8; 7], Ax25Error> {
        validate_callsign(&self.callsign)?;
        if self.ssid > 15 {
            return Err(Ax25Error::InvalidSsid);
        }

        let mut out = [b' ' << 1; 7];
        for (index, byte) in self.callsign.bytes().enumerate() {
            out[index] = byte.to_ascii_uppercase() << 1;
        }

        // Bits 5 and 6 are the AX.25 reserved bits and are transmitted as 1.
        // Bit 0 marks the final address in the address field.
        out[6] = 0x60 | (self.ssid << 1) | u8::from(last);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<(Self, bool), Ax25Error> {
        if bytes.len() < 7 {
            return Err(Ax25Error::TruncatedAddress);
        }

        let mut callsign = String::with_capacity(6);
        for encoded in &bytes[..6] {
            let decoded = encoded >> 1;
            if decoded == b' ' {
                callsign.push(' ');
            } else if decoded.is_ascii_alphanumeric() {
                callsign.push(char::from(decoded.to_ascii_uppercase()));
            } else {
                return Err(Ax25Error::InvalidCallsign);
            }
        }

        let callsign = callsign.trim_end().to_owned();
        validate_callsign(&callsign)?;
        let ssid = (bytes[6] >> 1) & 0x0F;
        let last = bytes[6] & 0x01 != 0;

        Ok((Self { callsign, ssid }, last))
    }
}

impl Ax25UiFrame {
    pub fn encode(&self) -> Result<Vec<u8>, Ax25Error> {
        if self.digipeaters.len() > MAX_DIGIPEATERS {
            return Err(Ax25Error::TooManyDigipeaters);
        }

        let address_count = 2 + self.digipeaters.len();
        let mut out = Vec::with_capacity(address_count * 7 + 2 + self.information.len());

        out.extend_from_slice(&self.destination.encode(false)?);
        let source_is_last = self.digipeaters.is_empty();
        out.extend_from_slice(&self.source.encode(source_is_last)?);

        for (index, digipeater) in self.digipeaters.iter().enumerate() {
            let is_last = index + 1 == self.digipeaters.len();
            out.extend_from_slice(&digipeater.encode(is_last)?);
        }

        out.push(UI_CONTROL);
        out.push(NO_LAYER_3_PID);
        out.extend_from_slice(&self.information);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, Ax25Error> {
        let mut offset = 0;
        let mut addresses = Vec::new();

        loop {
            if bytes.len().saturating_sub(offset) < 7 {
                return Err(Ax25Error::TruncatedAddress);
            }

            let (address, last) = Ax25Address::decode(&bytes[offset..offset + 7])?;
            addresses.push(address);
            offset += 7;

            if addresses.len() > 2 + MAX_DIGIPEATERS {
                return Err(Ax25Error::TooManyDigipeaters);
            }
            if last {
                break;
            }
        }

        if addresses.len() < 2 {
            return Err(Ax25Error::MissingRequiredAddresses);
        }
        if bytes.len().saturating_sub(offset) < 2 {
            return Err(Ax25Error::MissingControlPid);
        }

        let control = bytes[offset];
        let pid = bytes[offset + 1];
        if control != UI_CONTROL || pid != NO_LAYER_3_PID {
            return Err(Ax25Error::UnsupportedControlPid { control, pid });
        }

        let destination = addresses.remove(0);
        let source = addresses.remove(0);
        Ok(Self {
            destination,
            source,
            digipeaters: addresses,
            information: bytes[offset + 2..].to_vec(),
        })
    }
}

fn validate_callsign(callsign: &str) -> Result<(), Ax25Error> {
    if callsign.is_empty()
        || callsign.len() > 6
        || !callsign.bytes().all(|byte| byte.is_ascii_alphanumeric())
    {
        return Err(Ax25Error::InvalidCallsign);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn address(callsign: &str, ssid: u8) -> Ax25Address {
        Ax25Address::new(callsign, ssid).expect("valid test address")
    }

    #[test]
    fn address_round_trip_preserves_callsign_ssid_and_extension_bit() {
        let original = address("pv8abc", 7);
        let encoded = original.encode(true).expect("address encodes");
        let (decoded, last) = Ax25Address::decode(&encoded).expect("address decodes");

        assert_eq!(decoded, address("PV8ABC", 7));
        assert!(last);
        assert_eq!(encoded[6] & 0x60, 0x60);
    }

    #[test]
    fn ui_frame_round_trip_without_digipeaters() {
        let frame = Ax25UiFrame {
            destination: address("APRS", 0),
            source: address("PV8ABC", 7),
            digipeaters: Vec::new(),
            information: b"!0249.00N/06040.00W>RadioLink".to_vec(),
        };

        let encoded = frame.encode().expect("UI frame encodes");
        let decoded = Ax25UiFrame::decode(&encoded).expect("UI frame decodes");

        assert_eq!(decoded, frame);
        assert_eq!(encoded[13] & 0x01, 1);
        assert_eq!(encoded[14], UI_CONTROL);
        assert_eq!(encoded[15], NO_LAYER_3_PID);
    }

    #[test]
    fn ui_frame_round_trip_with_digipeater_path() {
        let frame = Ax25UiFrame {
            destination: address("APRS", 0),
            source: address("PV8ABC", 0),
            digipeaters: vec![address("WIDE1", 1), address("WIDE2", 1)],
            information: b">RadioLink test".to_vec(),
        };

        let encoded = frame.encode().expect("UI frame encodes");
        let decoded = Ax25UiFrame::decode(&encoded).expect("UI frame decodes");

        assert_eq!(decoded, frame);
        assert_eq!(encoded[27] & 0x01, 1);
    }

    #[test]
    fn rejects_non_ui_frames_without_losing_control_pid_diagnostics() {
        let frame = Ax25UiFrame {
            destination: address("APRS", 0),
            source: address("PV8ABC", 0),
            digipeaters: Vec::new(),
            information: Vec::new(),
        };
        let mut encoded = frame.encode().expect("UI frame encodes");
        encoded[14] = 0x00;

        assert_eq!(
            Ax25UiFrame::decode(&encoded),
            Err(Ax25Error::UnsupportedControlPid {
                control: 0x00,
                pid: NO_LAYER_3_PID,
            })
        );
    }

    #[test]
    fn validates_callsign_and_ssid() {
        assert_eq!(Ax25Address::new("TOOLONG", 0), Err(Ax25Error::InvalidCallsign));
        assert_eq!(Ax25Address::new("PV8ABC", 16), Err(Ax25Error::InvalidSsid));
    }
}
