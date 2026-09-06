//! APRS domain codecs and models.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AprsPosition {
    pub latitude: f64,
    pub longitude: f64,
}
