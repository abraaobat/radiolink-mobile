use radiolink_aprs::{
    AprsMessage, AprsMessageKind, AprsPacket, encode_packet, parse_ax25_ui_frame,
};
use radiolink_ax25::{Ax25Address, Ax25UiFrame};
use radiolink_kiss::{StreamingDecoder, encode_data_frame};

fn address(callsign: &str, ssid: u8) -> Ax25Address {
    Ax25Address::new(callsign, ssid).expect("valid test address")
}

#[test]
fn aprs_message_round_trips_through_ax25_and_fragmented_kiss_stream() {
    let original_packet = AprsPacket::Message(AprsMessage {
        addressee: "PV8ABC-7".into(),
        kind: AprsMessageKind::Text {
            text: "RadioLink pipeline".into(),
            message_id: Some("42".into()),
        },
    });

    let information = encode_packet(&original_packet).expect("APRS encodes");
    let original_ax25 = Ax25UiFrame {
        destination: address("APRS", 0),
        source: address("PV8XYZ", 9),
        digipeaters: vec![address("WIDE1", 1)],
        information,
    };

    let kiss_bytes = encode_data_frame(&original_ax25.encode().expect("AX.25 encodes"));
    let split_at = kiss_bytes.len() / 2;
    let mut stream = StreamingDecoder::new();

    assert!(stream.push(&kiss_bytes[..split_at]).is_empty());
    let decoded_kiss = stream
        .push(&kiss_bytes[split_at..])
        .into_iter()
        .next()
        .expect("one KISS frame")
        .expect("valid KISS frame");

    assert!(decoded_kiss.is_data());

    let decoded_ax25 = Ax25UiFrame::decode(&decoded_kiss.payload).expect("AX.25 decodes");
    assert_eq!(decoded_ax25.destination, original_ax25.destination);
    assert_eq!(decoded_ax25.source, original_ax25.source);
    assert_eq!(decoded_ax25.digipeaters, original_ax25.digipeaters);

    let decoded_packet = parse_ax25_ui_frame(&decoded_ax25).expect("APRS decodes");
    assert_eq!(decoded_packet, original_packet);
}
