//! RadioLink platform-neutral domain core.
//!
//! The core models a logical radio independently from any platform-specific
//! Bluetooth, USB, serial, audio/PTT or network implementation. A single
//! `RadioDevice` may expose capabilities through multiple transports.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(pub String);

impl DeviceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransportId(pub String);

impl TransportId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Ble,
    BluetoothClassic,
    Usb,
    Serial,
    AudioPtt,
    Network,
    Other(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RadioCapability {
    Kiss,
    IntegratedTnc,
    Cat,
    Ptt,
    AudioRx,
    AudioTx,
    UsbAudio,
    UsbSerial,
    Location,
    Telemetry,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilitySet {
    values: BTreeSet<RadioCapability>,
}

impl CapabilitySet {
    pub fn new(capabilities: impl IntoIterator<Item = RadioCapability>) -> Self {
        Self {
            values: capabilities.into_iter().collect(),
        }
    }

    pub fn contains(&self, capability: RadioCapability) -> bool {
        self.values.contains(&capability)
    }

    pub fn insert(&mut self, capability: RadioCapability) -> bool {
        self.values.insert(capability)
    }

    pub fn extend(&mut self, capabilities: &CapabilitySet) {
        self.values.extend(capabilities.values.iter().copied());
    }

    pub fn iter(&self) -> impl Iterator<Item = RadioCapability> + '_ {
        self.values.iter().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransportDescriptor {
    pub id: TransportId,
    pub kind: TransportKind,
    pub display_name: String,
    pub capabilities: CapabilitySet,
}

impl TransportDescriptor {
    pub fn new(
        id: impl Into<String>,
        kind: TransportKind,
        display_name: impl Into<String>,
        capabilities: impl IntoIterator<Item = RadioCapability>,
    ) -> Self {
        Self {
            id: TransportId::new(id),
            kind,
            display_name: display_name.into(),
            capabilities: CapabilitySet::new(capabilities),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RadioDevice {
    pub id: DeviceId,
    pub display_name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub native_capabilities: CapabilitySet,
    pub transports: Vec<TransportDescriptor>,
}

impl RadioDevice {
    pub fn new(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            id: DeviceId::new(id),
            display_name: display_name.into(),
            manufacturer: None,
            model: None,
            native_capabilities: CapabilitySet::default(),
            transports: Vec::new(),
        }
    }

    pub fn effective_capabilities(&self) -> CapabilitySet {
        let mut capabilities = self.native_capabilities.clone();
        for transport in &self.transports {
            capabilities.extend(&transport.capabilities);
        }
        capabilities
    }

    pub fn supports(&self, capability: RadioCapability) -> bool {
        self.effective_capabilities().contains(capability)
    }

    pub fn transports_supporting(
        &self,
        capability: RadioCapability,
    ) -> impl Iterator<Item = &TransportDescriptor> {
        self.transports
            .iter()
            .filter(move |transport| transport.capabilities.contains(capability))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceRegistry {
    devices: BTreeMap<DeviceId, RadioDevice>,
}

impl DeviceRegistry {
    pub fn upsert(&mut self, device: RadioDevice) -> Option<RadioDevice> {
        self.devices.insert(device.id.clone(), device)
    }

    pub fn remove(&mut self, id: &DeviceId) -> Option<RadioDevice> {
        self.devices.remove(id)
    }

    pub fn get(&self, id: &DeviceId) -> Option<&RadioDevice> {
        self.devices.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &RadioDevice> {
        self.devices.values()
    }

    pub fn devices_supporting(
        &self,
        capability: RadioCapability,
    ) -> impl Iterator<Item = &RadioDevice> {
        self.devices
            .values()
            .filter(move |device| device.supports(capability))
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }
}

/// Legacy boolean capability view kept for compatibility with early callers.
/// New code should prefer `CapabilitySet` and `RadioCapability`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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

impl From<&CapabilitySet> for RadioCapabilities {
    fn from(capabilities: &CapabilitySet) -> Self {
        Self {
            kiss: capabilities.contains(RadioCapability::Kiss),
            tnc_integrated: capabilities.contains(RadioCapability::IntegratedTnc),
            cat: capabilities.contains(RadioCapability::Cat),
            ptt: capabilities.contains(RadioCapability::Ptt),
            audio_rx: capabilities.contains(RadioCapability::AudioRx),
            audio_tx: capabilities.contains(RadioCapability::AudioTx),
            usb_audio: capabilities.contains(RadioCapability::UsbAudio),
            usb_serial: capabilities.contains(RadioCapability::UsbSerial),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_device_composes_capabilities_across_transports() {
        let mut device = RadioDevice::new("uv-k1-001", "Quansheng UV-K1");
        device.transports.push(TransportDescriptor::new(
            "digirig-audio",
            TransportKind::AudioPtt,
            "DigiRig audio/PTT",
            [
                RadioCapability::AudioRx,
                RadioCapability::AudioTx,
                RadioCapability::Ptt,
            ],
        ));
        device.transports.push(TransportDescriptor::new(
            "direwolf-kiss",
            TransportKind::Network,
            "Dire Wolf TCP KISS",
            [RadioCapability::Kiss],
        ));

        assert!(device.supports(RadioCapability::AudioRx));
        assert!(device.supports(RadioCapability::Ptt));
        assert!(device.supports(RadioCapability::Kiss));
        assert!(!device.supports(RadioCapability::Cat));
        assert_eq!(device.effective_capabilities().iter().count(), 4);
    }

    #[test]
    fn capability_query_returns_matching_logical_devices() {
        let mut registry = DeviceRegistry::default();

        let mut native_kiss = RadioDevice::new("native-kiss", "Native KISS radio");
        native_kiss
            .native_capabilities
            .insert(RadioCapability::IntegratedTnc);
        native_kiss.transports.push(TransportDescriptor::new(
            "ble-kiss",
            TransportKind::Ble,
            "BLE KISS",
            [RadioCapability::Kiss],
        ));
        registry.upsert(native_kiss);

        let audio_only = RadioDevice::new("audio-only", "Audio-only radio");
        registry.upsert(audio_only);

        let matches: Vec<_> = registry
            .devices_supporting(RadioCapability::Kiss)
            .map(|device| device.id.0.as_str())
            .collect();

        assert_eq!(matches, vec!["native-kiss"]);
    }

    #[test]
    fn legacy_boolean_view_is_derived_from_capability_set() {
        let capabilities = CapabilitySet::new([
            RadioCapability::Kiss,
            RadioCapability::UsbSerial,
            RadioCapability::Telemetry,
        ]);
        let legacy = RadioCapabilities::from(&capabilities);

        assert!(legacy.kiss);
        assert!(legacy.usb_serial);
        assert!(!legacy.cat);
    }
}
