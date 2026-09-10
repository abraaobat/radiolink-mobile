use radiolink_aprs::{parse_ax25_ui_frame, AprsMessage, AprsMessageKind, AprsPacket};
use radiolink_ax25::{Ax25Address, Ax25UiFrame};
use radiolink_kiss::decode_frame;

fn decode_hex(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .map(|value| u8::from_str_radix(value, 16).expect("valid fixture hex"))
        .collect()
}

#[test]
fn decodes_versioned_golden_kiss_ax25_aprs_message_fixture() {
    let bytes = decode_hex(include_str!(
        "../../../fixtures/protocol/aprs-message-via-wide1.kiss.hex"
    ));
    let kiss = decode_frame(&bytes).expect("valid KISS fixture");
    assert!(kiss.is_data());

    let ax25 = Ax25UiFrame::decode(&kiss.payload).expect("valid AX.25 fixture");
    assert_eq!(ax25.destination, Ax25Address::new("APRS", 0).unwrap());
    assert_eq!(ax25.source, Ax25Address::new("PV8XYZ", 9).unwrap());
    assert_eq!(
        ax25.digipeaters,
        vec![Ax25Address::new("WIDE1", 1).unwrap()]
    );

    assert_eq!(
        parse_ax25_ui_frame(&ax25).expect("valid APRS fixture"),
        AprsPacket::Message(AprsMessage {
            addressee: "PV8ABC-7".into(),
            kind: AprsMessageKind::Text {
                text: "RadioLink pipeline".into(),
                message_id: Some("42".into()),
            },
        })
    );
}
