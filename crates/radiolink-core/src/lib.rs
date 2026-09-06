//! RadioLink platform-neutral domain core.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RadioCapabilities {
    pub kiss: bool,
    pub tnc_integrated: bool,
    pub cat: bool,
    pub ptt: bool,
    pub audio_rx: bool,
    pub audio_tx: bool,
    pub usb_audio: bool,
    pub usb_serial: bool,
}

impl Default for RadioCapabilities {
    fn default() -> Self {
        Self {
            kiss: false,
            tnc_integrated: false,
            cat: false,
            ptt: false,
            audio_rx: false,
            audio_tx: false,
            usb_audio: false,
            usb_serial: false,
        }
    }
}
