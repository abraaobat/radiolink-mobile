//! RadioLink Operations Engine skeleton.
//!
//! The engine resolves a logical device, a transport that satisfies an
//! operation's required capabilities, and a ready TNC/modem provider. It does
//! not own platform-specific connection code yet; those adapters can feed this
//! deterministic registry/resolution layer later.

use std::collections::BTreeMap;

use radiolink_core::{
    DeviceId, DeviceRegistry, RadioCapability, TransportDescriptor, TransportId,
};
use radiolink_tnc::{TncProviderDescriptor, TncProviderStatus, TncSessionState};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    Aprs,
    Packet,
}

impl OperationKind {
    pub fn required_transport_capability(self) -> RadioCapability {
        match self {
            Self::Aprs | Self::Packet => RadioCapability::Kiss,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderBinding {
    pub provider: TncProviderDescriptor,
    pub status: TncProviderStatus,
    pub device_id: DeviceId,
    pub transport_id: TransportId,
}

impl ProviderBinding {
    pub fn new(
        provider: TncProviderDescriptor,
        state: TncSessionState,
        device_id: DeviceId,
        transport_id: TransportId,
    ) -> Self {
        let status = TncProviderStatus {
            descriptor: provider.clone(),
            state,
            detail: None,
        };
        Self {
            provider,
            status,
            device_id,
            transport_id,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.status.state == TncSessionState::Ready
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderRegistry {
    bindings: BTreeMap<String, ProviderBinding>,
}

impl ProviderRegistry {
    pub fn upsert(&mut self, binding: ProviderBinding) -> Option<ProviderBinding> {
        self.bindings.insert(binding.provider.id.clone(), binding)
    }

    pub fn get(&self, provider_id: &str) -> Option<&ProviderBinding> {
        self.bindings.get(provider_id)
    }

    pub fn for_device(
        &self,
        device_id: &DeviceId,
    ) -> impl Iterator<Item = &ProviderBinding> {
        self.bindings
            .values()
            .filter(move |binding| &binding.device_id == device_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PipelineResolution {
    pub operation: OperationKind,
    pub device_id: DeviceId,
    pub transport: TransportDescriptor,
    pub provider: TncProviderDescriptor,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ResolutionError {
    #[error("logical radio device not found: {0}")]
    DeviceNotFound(String),
    #[error("device has no transport satisfying operation requirements")]
    NoCompatibleTransport,
    #[error("no TNC/modem provider is bound to the compatible transport")]
    NoBoundProvider,
    #[error("compatible TNC/modem providers exist but none are ready")]
    ProviderNotReady,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationsEngine {
    pub devices: DeviceRegistry,
    pub providers: ProviderRegistry,
}

impl OperationsEngine {
    pub fn resolve(
        &self,
        device_id: &DeviceId,
        operation: OperationKind,
    ) -> Result<PipelineResolution, ResolutionError> {
        let device = self
            .devices
            .get(device_id)
            .ok_or_else(|| ResolutionError::DeviceNotFound(device_id.0.clone()))?;
        let required = operation.required_transport_capability();

        let candidate_transports: Vec<_> = device.transports_supporting(required).collect();
        if candidate_transports.is_empty() {
            return Err(ResolutionError::NoCompatibleTransport);
        }

        let mut saw_binding = false;
        for transport in candidate_transports {
            for binding in self.providers.for_device(device_id) {
                if binding.transport_id != transport.id {
                    continue;
                }
                saw_binding = true;
                if binding.is_ready() {
                    return Ok(PipelineResolution {
                        operation,
                        device_id: device.id.clone(),
                        transport: transport.clone(),
                        provider: binding.provider.clone(),
                    });
                }
            }
        }

        if saw_binding {
            Err(ResolutionError::ProviderNotReady)
        } else {
            Err(ResolutionError::NoBoundProvider)
        }
    }
}

#[cfg(test)]
mod tests {
    use radiolink_core::{
        RadioDevice, TransportKind,
    };
    use radiolink_tnc::{TncProviderKind, TncProviderDescriptor};

    use super::*;

    fn reference_engine(provider_state: TncSessionState) -> OperationsEngine {
        let mut device = RadioDevice::new("uv-k1-001", "Quansheng UV-K1 + DigiRig");
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

        let device_id = device.id.clone();
        let mut engine = OperationsEngine::default();
        engine.devices.upsert(device);
        engine.providers.upsert(ProviderBinding::new(
            TncProviderDescriptor::new(
                "direwolf-local",
                "Dire Wolf local",
                TncProviderKind::Software,
            ),
            provider_state,
            device_id,
            TransportId::new("direwolf-kiss"),
        ));
        engine
    }

    #[test]
    fn resolves_reference_software_tnc_pipeline() {
        let engine = reference_engine(TncSessionState::Ready);
        let resolved = engine
            .resolve(&DeviceId::new("uv-k1-001"), OperationKind::Aprs)
            .unwrap();

        assert_eq!(resolved.transport.id, TransportId::new("direwolf-kiss"));
        assert_eq!(resolved.provider.id, "direwolf-local");
        assert_eq!(resolved.operation, OperationKind::Aprs);
    }

    #[test]
    fn distinguishes_missing_transport_from_unready_provider() {
        let engine = reference_engine(TncSessionState::Disconnected);
        assert_eq!(
            engine.resolve(&DeviceId::new("uv-k1-001"), OperationKind::Packet),
            Err(ResolutionError::ProviderNotReady)
        );

        let mut no_kiss = OperationsEngine::default();
        no_kiss
            .devices
            .upsert(RadioDevice::new("plain-radio", "Plain radio"));
        assert_eq!(
            no_kiss.resolve(&DeviceId::new("plain-radio"), OperationKind::Aprs),
            Err(ResolutionError::NoCompatibleTransport)
        );
    }
}
