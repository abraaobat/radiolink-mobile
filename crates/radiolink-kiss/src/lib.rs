//! KISS framing and streaming parser.

pub const FEND: u8 = 0xC0;
pub const FESC: u8 = 0xDB;
pub const TFEND: u8 = 0xDC;
pub const TFESC: u8 = 0xDD;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KissFrame {
    pub port: u8,
    pub command: u8,
    pub payload: Vec<u8>,
}

impl KissFrame {
    pub fn is_data(&self) -> bool {
        self.command == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    MissingBoundary,
    EmptyFrame,
    InvalidEscape(u8),
}

pub fn encode_data_frame(payload: &[u8]) -> Vec<u8> {
    encode_data_frame_for_port(0, payload)
}

pub fn encode_data_frame_for_port(port: u8, payload: &[u8]) -> Vec<u8> {
    let command = (port & 0x0f) << 4;
    let mut out = vec![FEND, command];
    append_escaped(&mut out, payload);
    out.push(FEND);
    out
}

fn append_escaped(out: &mut Vec<u8>, bytes: &[u8]) {
    for &byte in bytes {
        match byte {
            FEND => out.extend_from_slice(&[FESC, TFEND]),
            FESC => out.extend_from_slice(&[FESC, TFESC]),
            _ => out.push(byte),
        }
    }
}

pub fn decode_frame(frame: &[u8]) -> Result<KissFrame, DecodeError> {
    if frame.len() < 2 || frame.first() != Some(&FEND) || frame.last() != Some(&FEND) {
        return Err(DecodeError::MissingBoundary);
    }
    decode_body(&frame[1..frame.len() - 1])
}

fn decode_body(body: &[u8]) -> Result<KissFrame, DecodeError> {
    let (&command_byte, escaped_payload) = body.split_first().ok_or(DecodeError::EmptyFrame)?;
    let mut payload = Vec::with_capacity(escaped_payload.len());
    let mut index = 0;

    while index < escaped_payload.len() {
        let byte = escaped_payload[index];
        if byte != FESC {
            payload.push(byte);
            index += 1;
            continue;
        }

        let escaped = escaped_payload
            .get(index + 1)
            .copied()
            .ok_or(DecodeError::InvalidEscape(FESC))?;
        match escaped {
            TFEND => payload.push(FEND),
            TFESC => payload.push(FESC),
            other => return Err(DecodeError::InvalidEscape(other)),
        }
        index += 2;
    }

    Ok(KissFrame {
        port: command_byte >> 4,
        command: command_byte & 0x0f,
        payload,
    })
}

#[derive(Debug, Default)]
pub struct StreamingDecoder {
    synchronized: bool,
    body: Vec<u8>,
}

impl StreamingDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.synchronized = false;
        self.body.clear();
    }

    pub fn push(&mut self, bytes: &[u8]) -> Vec<Result<KissFrame, DecodeError>> {
        let mut frames = Vec::new();

        for &byte in bytes {
            if byte == FEND {
                if !self.synchronized {
                    self.synchronized = true;
                    self.body.clear();
                    continue;
                }

                if self.body.is_empty() {
                    continue;
                }

                frames.push(decode_body(&self.body));
                self.body.clear();
                continue;
            }

            if self.synchronized {
                self.body.push(byte);
            }
        }

        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_reserved_bytes() {
        let frame = encode_data_frame(&[0x01, FEND, FESC, 0x02]);
        assert_eq!(
            frame,
            vec![FEND, 0x00, 0x01, FESC, TFEND, FESC, TFESC, 0x02, FEND]
        );
    }

    #[test]
    fn round_trips_reserved_bytes() {
        let encoded = encode_data_frame_for_port(3, &[0x01, FEND, FESC, 0x02]);
        let decoded = decode_frame(&encoded).expect("valid KISS frame");

        assert_eq!(decoded.port, 3);
        assert_eq!(decoded.command, 0);
        assert!(decoded.is_data());
        assert_eq!(decoded.payload, vec![0x01, FEND, FESC, 0x02]);
    }

    #[test]
    fn rejects_invalid_escape_sequence() {
        let frame = [FEND, 0x00, FESC, 0x01, FEND];
        assert_eq!(decode_frame(&frame), Err(DecodeError::InvalidEscape(0x01)));
    }

    #[test]
    fn streaming_decoder_handles_fragmented_transport_chunks() {
        let encoded = encode_data_frame(&[0x41, FEND, 0x42]);
        let split = encoded.len() - 2;
        let mut decoder = StreamingDecoder::new();

        assert!(decoder.push(&encoded[..split]).is_empty());
        let frames = decoder.push(&encoded[split..]);

        assert_eq!(frames.len(), 1);
        assert_eq!(
            frames[0],
            Ok(KissFrame {
                port: 0,
                command: 0,
                payload: vec![0x41, FEND, 0x42],
            })
        );
    }

    #[test]
    fn streaming_decoder_emits_multiple_frames_and_ignores_noise_before_sync() {
        let mut stream = vec![0x01, 0x02, 0x03];
        stream.extend_from_slice(&encode_data_frame(b"one"));
        stream.extend_from_slice(&encode_data_frame(b"two"));

        let mut decoder = StreamingDecoder::new();
        let frames = decoder.push(&stream);

        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].as_ref().unwrap().payload, b"one");
        assert_eq!(frames[1].as_ref().unwrap().payload, b"two");
    }
}
