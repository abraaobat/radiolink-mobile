//! AX.25 framing/parsing primitives.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ax25Address {
    pub callsign: String,
    pub ssid: u8,
}
