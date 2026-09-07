# RadioLink Platform

**Cross-platform amateur-radio platform for smartphones and computers, with BLE, USB and legacy audio/PTT support.**

RadioLink is an open-source platform for Android, iOS, Linux and macOS that turns the user's existing phone or computer into the main computing and interaction layer for digital radio operation.

> **Host = computer. Radio = RF. RadioLink resolves the path in between.**

## Why

Amateur-radio digital modes are powerful, but real-world operation is fragmented across radios, TNCs, Bluetooth, USB, sound cards, PTT, CAT, software modems and protocol-specific applications.

RadioLink's goal is not to add another isolated radio application. It provides one capability-driven platform where the user thinks in terms of actions such as:

- send a message;
- share position;
- send e-mail;
- view stations;
- open a Packet terminal;
- inspect diagnostics;
- control the radio where supported.

The platform decides which device, transport, TNC/modem provider and protocol stack can satisfy that operation.

## Target platforms

- Android
- iOS
- Linux desktop/headless
- macOS desktop/headless

Development is **CLI-first on macOS/Linux** for fast validation of transports, protocols and the Operations Engine, but the product architecture remains cross-platform.

## The three official I/O paths

### 1. Wireless Digital — Bluetooth/BLE

For cable-free mobile operation, KISS/data, control and telemetry where the radio/TNC exposes documented usable capabilities.

```text
Host ↔ BLE ↔ Radio/TNC ↔ RF
```

### 2. Wired Digital — USB-C/USB

For robust wired operation. A device may expose KISS, serial/CAT, USB Audio or other logical interfaces over USB.

```text
Host ↔ USB-C/USB ↔ Radio/TNC ↔ RF
```

### 3. Legacy / Analog Bridge — audio + PTT + optional CAT

For the large installed base of conventional radios using DigiRig-class interfaces, software TNC/modem providers or the future RadioLink Bridge.

```text
Host ↔ software TNC/modem ↔ USB audio/PTT ↔ Radio ↔ RF
```

Bluetooth and USB are **transports**, not automatic Packet/APRS capabilities. RadioLink always models what a device actually exposes.

## Platform family

```text
RadioLink Platform
│
├── RadioLink Applications
│   ├── Android
│   ├── iOS
│   ├── Linux/macOS
│   └── CLI/headless
│
├── RadioLink Core
│   ├── Operations Engine
│   ├── Capability Registry
│   ├── Device Registry
│   ├── Transport Manager
│   ├── TNC/Modem Providers
│   └── Protocol/Service Core
│
├── RadioLink Profiles
│   └── tested radios / cables / interfaces / settings
│
├── RadioLink Bridge
│   └── future BLE + USB-C adapter/TNC for conventional radios
│
├── RadioLink Ready
│   └── future interoperability profile/specification
│
└── RadioLink Labs
    └── Winlink/Mercury, Reticulum, LoRa, BBS and other research
```

## Architecture

```text
User action / Application UX
            ↓
     Operational Service
            ↓
          Protocol
            ↓
    TNC / Modem Provider
            ↓
     Transport Manager
       /      |       \
     BLE     USB    Audio/PTT
       \      |       /
          Device
            ↓
            RF
```

No APRS, Packet or Winlink service should depend directly on Bluetooth, USB, Direwolf, one radio model or one TNC implementation.

## Shared Core toolchain

The shared protocol/domain core is implemented in **Rust**.

- Rust stable + Cargo workspace
- `tokio` for async work where required
- `serde` for serialization
- `thiserror` / `anyhow` for errors
- `tracing` for structured diagnostics
- `clap` for the native CLI
- Kotlin host layer on Android
- Swift host layer on iOS
- native Linux/macOS adapters for Bluetooth/USB and desktop integration

Platform-specific Bluetooth, USB, UI, location, permissions and background behavior remain outside the Rust core.

## Initial service set

- APRS
- Packet / AX.25
- KISS diagnostics
- Radio Control
- Winlink after the Packet path is stable

Future research/modules stay in Labs until they justify promotion into the product core.

## Reference projects

RadioLink uses external projects as engineering references with clearly separated roles:

- **DigiPi** — functional coverage and integrated-operation benchmark.
- **Mobilinkd TNC4** — BLE KISS TNC / portable radio-interface benchmark.
- **HTCommander** — Bluetooth radio integration, device-control and driver/protocol reference.
- **EmComm Tools / ETC** — operational-mode orchestration, lifecycle and zero-configuration reference.
- **Mercury** — example of a modem/provider implementation that benefits from a compatible, decoupled interface.

These are references, not runtime dependencies or wholesale architectural templates.

See [Technical References](docs/REFERENCES.md) and [DigiPi 2.2-1 → RadioLink Functional Benchmark](docs/DIGIPI-BENCHMARK.md).

## Repository

```text
radiolink-mobile/
├── Cargo.toml
├── apps/
│   ├── android/
│   ├── ios/
│   ├── linux/
│   └── macos/
├── crates/
│   ├── radiolink-core/
│   ├── radiolink-kiss/
│   ├── radiolink-ax25/
│   ├── radiolink-aprs/
│   ├── radiolink-tnc/
│   └── radiolink-drivers/
├── tools/
│   ├── radiolink-cli/
│   └── research/
├── hardware/
│   └── bridge/
├── docs/
└── tests/
```

The repository keeps its current GitHub name for continuity; the product umbrella is **RadioLink Platform**.

## Current phase

**F0/F1 — Platform Foundation + Shared Core/Operations abstractions**

The Rust shared core and CLI bootstrap are in place. The immediate engineering path is to prove a real hardware vertical slice through the common abstractions:

```text
macOS CLI ↔ BLE KISS ↔ Radio/TNC ↔ RF
```

with a parallel compatibility baseline:

```text
macOS CLI ↔ software TNC ↔ DigiRig/USB audio/PTT ↔ conventional radio ↔ RF
```

and early validation of a USB-native digital path.

See [Roadmap](docs/ROADMAP.md), [Product](docs/PRODUCT.md), [Architecture](docs/ARCHITECTURE.md), [Compatibility Matrix](docs/COMPATIBILITY.md), [Technical References](docs/REFERENCES.md), [ADR-0001](docs/adr/ADR-0001-smartphone-first-bluetooth-first.md), [ADR-0002](docs/adr/ADR-0002-cross-platform-app-hub.md), [ADR-0003](docs/adr/ADR-0003-rust-shared-core.md) and [ADR-0004](docs/adr/ADR-0004-three-path-io-and-platform-ecosystem.md).
