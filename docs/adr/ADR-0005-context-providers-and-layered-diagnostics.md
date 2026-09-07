# ADR-0005 — Context Providers and Layered Diagnostics

**Status:** Accepted  
**Date:** 2026-09-06

## Context

Cross-source research from The Tech Prepper and KM4ACK shows that digital-radio services depend on operational context that may originate outside the radio itself.

Location is a clear example. Practical workflows use:

- host/smartphone location;
- radio-integrated GPS/GNSS;
- USB GPS receivers;
- network/GPSD sources;
- multiple transports simultaneously, such as KISS/TNC over Bluetooth while GPS data arrives over USB.

Time synchronization has a similar shape for modes that require accurate time.

The same research also shows that users frequently experience a low-level transport failure as an application failure. Examples include Bluetooth pairing/reconnect problems and USB charge-only cables that provide power but no data. Reporting only `connection failed` or `APRS unavailable` loses the actual failure layer.

## Decision

### 1. Context is resolved through providers

RadioLink will not treat GPS/location as a capability owned exclusively by `RadioDevice`.

The shared architecture will support provider-style context interfaces, initially conceptualized as:

```text
LocationProvider
├── HostLocationProvider
├── RadioGpsProvider
├── UsbGpsProvider
├── NetworkGpsProvider / GPSD
└── ManualLocationProvider

TimeProvider
├── SystemClockProvider
├── GpsTimeProvider
├── NetworkTimeProvider when available
└── other validated providers
```

Operational services request the context they require. The Operations Engine / provider registry resolves an available source according to capability, quality and operator policy.

A radio may therefore contribute RF/TNC capability while the smartphone contributes location, or one radio may expose KISS through BLE and GPS through USB.

### 2. Context providers are independent from transport providers

Location/time data may arrive through BLE, USB, host OS APIs or network sources, but higher-level services must not bind directly to those physical paths.

Dependency direction remains:

```text
Service
  ↓
Context Provider API
  ↓
Provider implementation
  ↓
Host/device/transport-specific source
```

### 3. Diagnostics are layered

RadioLink diagnostics will track and expose connection progress in layers:

```text
physical/power
  ↓
transport enumeration / Bluetooth pairing
  ↓
logical interfaces discovered
  ↓
capability match
  ↓
TNC/modem/context-provider handshake
  ↓
protocol traffic
  ↓
service readiness
```

The system should report the highest verified stage and the most specific human-readable failure reason available.

Examples:

- `USB device has power but no data interface detected`;
- `Bluetooth connected; KISS service not exposed`;
- `KISS provider ready; no AX.25 frames received yet`;
- `APRS ready; location source unavailable for beacon TX`.

### 4. Profiles may provide preflight knowledge

RadioLink Profiles may record firmware ranges, required radio-side settings, expected interfaces and known cable/configuration requirements. Runtime discovery remains authoritative where detection is possible.

## Consequences

### Positive

- APRS/Winlink position can use the best available source without coupling to one radio.
- Multi-transport radios are represented naturally.
- Host location can reduce external GPS hardware requirements on mobile.
- Future time-sensitive modes can reuse the same provider pattern.
- Diagnostics distinguish physical transport failures from protocol/application failures.
- Profiles can explain known device setup prerequisites before the user begins troubleshooting the wrong layer.

### Trade-offs

- Operations Engine/provider resolution becomes slightly richer.
- Provider quality/precedence policy must eventually be defined.
- Mobile operating systems have different location/background permissions.
- USB power-vs-data detection may be platform-dependent and cannot always diagnose the cable itself with certainty.

## Guardrails

- Do not assume `gps=true` means location must come from the radio.
- Do not make Internet/network time mandatory for core RF operation.
- Do not silently switch context providers when doing so could materially change operator intent; expose selection/state when relevant.
- Do not claim a specific physical fault when the platform can only establish that a higher layer is unavailable.
- Profiles are hints/known-good knowledge; runtime observations remain authoritative.

## Research basis

Primary derived research references:

- `docs/research/KM4ACK-DEEP-MINING.md`
- `docs/research/EVIDENCE.md`
- `docs/research/SYNTHESIS.md`

Representative evidence includes KM4ACK phone-GPS streaming, TH-D75 BLE KISS + USB GPS operation, radio-integrated GPS/GNSS workflows, USB cable failure diagnostics, and The Tech Prepper field/context-provider and transport-reliability observations.
