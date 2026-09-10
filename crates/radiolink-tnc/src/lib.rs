//! Shared TNC abstraction for hardware and software TNC backends.

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TncError {
    #[error("TNC is not connected")]
    NotConnected,
    #[error("transport error: {0}")]
    Transport(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TncProviderKind {
    Embedded,
    External,
    Software,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TncSessionState {
    Disconnected,
    Connecting,
    Ready,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TncProviderDescriptor {
    pub id: String,
    pub display_name: String,
    pub kind: TncProviderKind,
}

impl TncProviderDescriptor {
    pub fn new(
        id: impl Into<String>,
        display_name: impl Into<String>,
        kind: TncProviderKind,
    ) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TncProviderStatus {
    pub descriptor: TncProviderDescriptor,
    pub state: TncSessionState,
    pub detail: Option<String>,
}

pub trait TncTransport {
    fn name(&self) -> &str;
    fn is_connected(&self) -> bool;
    fn send_frame(&mut self, frame: &[u8]) -> Result<(), TncError>;
    fn receive_frame(&mut self) -> Result<Option<Vec<u8>>, TncError>;
}

/// A logical TNC/modem provider exposed to RadioLink services.
///
/// Implementations can wrap an embedded radio TNC, an external KISS TNC or a
/// host software modem such as Dire Wolf. APRS/Packet callers consume this
/// interface rather than branching on provider placement.
pub trait TncProvider: TncTransport {
    fn descriptor(&self) -> &TncProviderDescriptor;
    fn session_state(&self) -> TncSessionState;

    fn status(&self) -> TncProviderStatus {
        TncProviderStatus {
            descriptor: self.descriptor().clone(),
            state: self.session_state(),
            detail: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeSoftwareTnc {
        descriptor: TncProviderDescriptor,
        connected: bool,
        sent: Vec<Vec<u8>>,
    }

    impl FakeSoftwareTnc {
        fn new() -> Self {
            Self {
                descriptor: TncProviderDescriptor::new(
                    "direwolf-local",
                    "Dire Wolf local TCP KISS",
                    TncProviderKind::Software,
                ),
                connected: true,
                sent: Vec::new(),
            }
        }
    }

    impl TncTransport for FakeSoftwareTnc {
        fn name(&self) -> &str {
            &self.descriptor.display_name
        }

        fn is_connected(&self) -> bool {
            self.connected
        }

        fn send_frame(&mut self, frame: &[u8]) -> Result<(), TncError> {
            if !self.connected {
                return Err(TncError::NotConnected);
            }
            self.sent.push(frame.to_vec());
            Ok(())
        }

        fn receive_frame(&mut self) -> Result<Option<Vec<u8>>, TncError> {
            if !self.connected {
                return Err(TncError::NotConnected);
            }
            Ok(None)
        }
    }

    impl TncProvider for FakeSoftwareTnc {
        fn descriptor(&self) -> &TncProviderDescriptor {
            &self.descriptor
        }

        fn session_state(&self) -> TncSessionState {
            if self.connected {
                TncSessionState::Ready
            } else {
                TncSessionState::Disconnected
            }
        }
    }

    #[test]
    fn software_tnc_uses_same_provider_contract_as_hardware_tnc() {
        let mut provider = FakeSoftwareTnc::new();
        assert_eq!(provider.descriptor().kind, TncProviderKind::Software);
        assert_eq!(provider.status().state, TncSessionState::Ready);

        provider.send_frame(b"frame").unwrap();
        assert_eq!(provider.sent, vec![b"frame".to_vec()]);
    }
}
