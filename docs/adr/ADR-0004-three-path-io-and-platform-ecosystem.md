# ADR-0004 — Three-path I/O and RadioLink Platform Ecosystem

**Status:** Accepted  
**Date:** 2026-09-06

## Context

RadioLink is already defined as a cross-platform amateur-radio hub for Android, iOS, Linux and macOS, with a shared Rust core and platform-specific host adapters.

Research and field-oriented analysis reinforced a recurring problem in amateur-radio digital operation: the protocols already exist, but hardware integration is fragmented across Bluetooth, USB/serial, USB audio, PTT, CAT, embedded TNCs, external TNCs and software modems. Binding a service such as APRS directly to one of these paths would make RadioLink brittle and hardware-specific.

At the same time, two interface families are increasingly important for modern radios and accessories:

- Bluetooth/BLE for cable-free mobile operation, control, telemetry and KISS/data transport;
- USB-C/USB for reliable wired data, serial/CAT, KISS, digital audio and, where applicable, power.

A third path remains necessary to serve the large installed base of conventional radios that expose only analog audio/PTT and optional CAT.

## Decision

RadioLink will formally support three first-class I/O paths:

1. **Wireless Digital** — Bluetooth/BLE, preferably exposing documented KISS, serial/data, control and telemetry capabilities.
2. **Wired Digital** — USB-C/USB, exposing one or more of KISS, serial, CAT, digital audio, network-style interfaces or power as supported by the device and host platform.
3. **Legacy / Analog Bridge** — audio + PTT + optional CAT through DigiRig-class interfaces, software TNC/modem providers or the future RadioLink Bridge.

These paths are transports/adapters, not application features. APRS, Packet, Winlink and other services must not depend directly on Bluetooth, USB or audio.

RadioLink will use a capability-driven architecture with explicit separation between:

```text
Application / UX
       ↓
Operational Mode / Service
       ↓
Protocol
       ↓
TNC / Modem Provider
       ↓
Transport Manager
       ↓
Device / Adapter
       ↓
RF
```

The Operations Engine is responsible for resolving available capabilities, selecting a valid provider/transport path, managing resource lifecycle and recovering sessions when a transport changes or reconnects.

Bluetooth/BLE is preferred for cable-free mobile workflows when reliable. USB is equally first-class for wired operation and may be preferred for long-running or high-reliability sessions. Audio/PTT remains an intentional compatibility path, not a deprecated afterthought.

The product family will be organized under the **RadioLink Platform** umbrella:

- **RadioLink Mobile/Desktop/CLI** — user-facing applications using the shared core;
- **RadioLink Core** — protocols, domain models, capability resolution and operations engine;
- **RadioLink Profiles** — tested device/interface/cable/capability profiles;
- **RadioLink Bridge** — optional BLE + USB-C accessory for conventional radios;
- **RadioLink Ready** — future documented interoperability profile/specification for radios and accessories;
- **RadioLink Labs** — research area for Winlink/Mercury, Reticulum, LoRa, BBS, offline knowledge and other experiments that must not inflate the MVP.

## Consequences

### Positive

- APRS/Packet/Winlink can reuse the same service logic across radically different hardware paths.
- Modern BLE/KISS radios and conventional analog radios can participate in the same platform.
- USB provides a robust fallback when Bluetooth is unavailable or unreliable.
- A future RadioLink Bridge can expose the same logical capabilities over BLE and USB-C.
- Device support becomes capability-driven instead of brand/model hard-coded in application modules.
- RadioLink can evolve into an ecosystem without turning the main application into a monolith.

### Trade-offs

- Transport and capability resolution become core architectural responsibilities.
- Host platforms have different Bluetooth/USB permissions and lifecycle constraints.
- Some devices may expose capabilities through more than one transport and require preference/recovery policies.
- Audio/PTT paths require calibration and device profiles that pure KISS paths do not.

## Guardrails

- No Protocol/Service implementation may depend directly on Bluetooth, USB, a specific radio model or a specific TNC implementation.
- `Bluetooth = true` must never imply `KISS`, `TNC`, `audio` or `CAT` capability.
- USB must be modeled by the actual interfaces it exposes, not treated as a single capability.
- RadioLink Bridge must remain an adapter/TNC accessory, not become a second general-purpose computer.
- RadioLink Ready extensions must be documented and should prefer established standards such as KISS, USB CDC/audio and standard Bluetooth services where practical.
- Experimental network/mode features belong in Labs until their product value and architectural fit are demonstrated.
