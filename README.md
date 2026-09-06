# RadioLink Platform

**Cross-platform, Bluetooth-first amateur radio application hub.**

RadioLink is an open-source platform for Android, iOS, Linux and macOS that connects computers and smartphones directly to amateur radios/TNCs, preferably through Bluetooth/BLE.

> **Device = computer. Radio = RF. Bluetooth = preferred bridge.**

A key architectural rule is now explicit: **the host device should perform as much protocol/modem processing as practical**. The optional RadioLink Bridge exists first to solve the physical radio interface — audio RX/TX, PTT and transport — while embedded AFSK/AX.25/KISS remains an additional capability for cable-free mobile operation.

## Why

The value of DigiPi is not only running individual radio applications; it is the convenience of gathering several functions in one place and switching between them easily. RadioLink keeps that idea, but removes the requirement for a Raspberry Pi/Linux appliance when the user's phone or computer can run the radio stack directly.

RadioLink aims to provide one consistent hub for:

- APRS
- Packet / AX.25
- Winlink
- KISS diagnostics
- radio control
- SSTV and other future digital-mode modules

The normal user experience should look like one product with selectable modules, not a collection of daemons and configuration files.

## TNC strategy

RadioLink supports more than one place for the TNC/modem to live:

1. **Host software TNC** — preferred for development and conventional radios. The phone/computer owns AFSK/AX.25 processing when a clean audio/PTT path is available.
2. **Embedded radio TNC** — use the radio's native KISS/TNC when exposed.
3. **External BLE KISS TNC** — Mobilinkd-class path.
4. **RadioLink Bridge embedded TNC mode** — the bridge can later perform AFSK/AX.25 locally and expose KISS over BLE for fully cable-free mobile use.

The bridge therefore has two conceptual modes:

```text
HOST-TNC MODE
Radio ↔ audio/PTT ↔ RadioLink Bridge ↔ USB ↔ RadioLink/Direwolf on host

EMBEDDED-TNC MODE
Radio ↔ audio/PTT ↔ RadioLink Bridge [AFSK/AX.25] ↔ BLE KISS ↔ RadioLink
```

Raw Bluetooth audio is not the primary packet-radio transport because codec processing, buffering, latency and platform behavior can reduce modem reliability.

## DigiPi functional benchmark

DigiPi 2.2-1 is used as a **functional coverage benchmark**, not as an architectural template. RadioLink tracks which useful DigiPi operations should become native modules, integrations, later-phase features or intentionally omitted appliance-management functions.

See [DigiPi 2.2-1 → RadioLink Functional Benchmark](docs/DIGIPI-BENCHMARK.md).

## Target platforms

- Android
- iOS
- Linux desktop/headless
- macOS desktop/headless

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

Platform-specific Bluetooth, UI, location, permissions and background behavior remain outside the Rust core.

## Transport strategy

1. Bluetooth/BLE radio or embedded KISS TNC — preferred when reliable KISS/data exists.
2. Bluetooth KISS TNC attached to a conventional radio.
3. USB audio/PTT + software TNC — reference validation path for conventional radios.
4. Optional RadioLink Bridge for legacy radios, first as an audio/PTT interface and later with embedded KISS/TNC mode.

## Architecture

```text
Android / iOS / Linux / macOS
              │
        RadioLink Shell
              │
 ┌────────────┼──────────────┐
 │            │              │
APRS        Packet         Winlink
 │            │              │
 └────── Rust Shared Core ────┘
              │
      TNC Backend Abstraction
              │
 ┌────────────┼─────────────────────┐
 │            │                     │
BLE KISS   Software TNC       RadioLink Bridge
 │            │              audio/PTT or KISS
 └────────────┴─────────────────────┘
              │
            Radio
              │
             RF
```

## App Hub principle

The home screen should make each function easy to select:

```text
RADIOLINK

● Radio connected

[ APRS ]    [ WINLINK ]
[ PACKET ]  [ SSTV ]
[ KISS ]    [ RADIO ]
```

Protocol details remain available for diagnostics, but they should not dominate the normal workflow.

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
│   └── radiolink-cli/
├── hardware/
│   └── bridge/
├── docs/
└── tests/
```

The repository keeps its current GitHub name for continuity; the product name is now **RadioLink Platform**.

## Current phase

**F0 — Platform Foundation**

The shared-core language/toolchain decision is complete: **Rust**. The next hardware proof should validate the conventional-radio path before embedding a modem in the ESP32:

```text
radio → audio/PTT interface → host software TNC → AX.25/APRS
```

After that baseline is proven, the same bridge hardware can gain embedded AFSK/AX.25 + BLE KISS mode.

See [Roadmap](docs/ROADMAP.md), [Product](docs/PRODUCT.md), [Architecture](docs/ARCHITECTURE.md), [Compatibility Matrix](docs/COMPATIBILITY.md), [DigiPi Benchmark](docs/DIGIPI-BENCHMARK.md), and the ADRs under `docs/adr/`.

## Status

Early architecture / proof-of-concept stage. The Rust workspace, TNC abstraction, capability model, initial KISS encoder and CLI bootstrap are in place. The development strategy now uses a host software TNC as the reference modem during hardware bring-up, while preserving BLE KISS as the target cable-free mobile mode.
