//! KISS framing and streaming parser.

pub const FEND: u8 = 0xC0;
pub const FESC: u8 = 0xDB;
pub const TFEND: u8 = 0xDC;
pub const TFESC: u8 = 0xDD;

pub fn encode_data_frame(payload: &[u8]) -> Vec<u8> {
    let mut out = vec![FEND, 0x00];
    for &b in payload {
        match b {
            FEND => out.extend_from_slice(&[FESC, TFEND]),
            FESC => out.extend_from_slice(&[FESC, TFESC]),
            _ => out.push(b),
        }
    }
    out.push(FEND);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_reserved_bytes() {
        let frame = encode_data_frame(&[0x01, FEND, FESC, 0x02]);
        assert_eq!(frame, vec![FEND, 0x00, 0x01, FESC, TFEND, FESC, TFESC, 0x02, FEND]);
    }
}
