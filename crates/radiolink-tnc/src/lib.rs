//! Shared TNC abstraction for hardware and software TNC backends.

#[derive(Debug, thiserror::Error)]
pub enum TncError {
    #[error("TNC is not connected")]
    NotConnected,
    #[error("transport error: {0}")]
    Transport(String),
}

pub trait TncTransport {
    fn name(&self) -> &str;
    fn is_connected(&self) -> bool;
    fn send_frame(&mut self, frame: &[u8]) -> Result<(), TncError>;
    fn receive_frame(&mut self) -> Result<Option<Vec<u8>>, TncError>;
}
