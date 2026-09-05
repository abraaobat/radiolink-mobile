# RadioLink Platform

**Cross-platform, Bluetooth-first amateur radio application hub.**

RadioLink is an open-source platform for Android, iOS, Linux and macOS that connects computers and smartphones directly to amateur radios/TNCs, preferably through Bluetooth/BLE.

> **Device = computer. Radio = RF. Bluetooth = preferred bridge.**

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

## Target platforms

- Android
- iOS
- Linux desktop/headless
- macOS desktop/headless

## Transport strategy

1. Bluetooth/BLE radio or embedded KISS TNC — preferred.
2. Bluetooth KISS TNC attached to a conventional radio.
3. USB/audio interfaces where needed.
4. Optional RadioLink Bridge for legacy radios.

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
 └────── RadioLink Core ──────┘
              │
      Protocols + Drivers
              │
      Bluetooth / USB
              │
        Radio / TNC
              │
             RF
```

## App Hub principle

The home screen should make each function easy to select:

```text
RAD IOLINK

● Radio connected

[ APRS ]    [ WINLINK ]
[ PACKET ]  [ SSTV ]
[ KISS ]    [ RADIO ]
```

Protocol details remain available for diagnostics, but they should not dominate the normal workflow.

## Repository

```text
radiolink-mobile/
├── apps/
│   ├── android/
│   ├── ios/
│   ├── linux/
│   └── macos/
├── packages/
│   ├── core/
│   ├── protocols/
│   ├── drivers/
│   ├── services/
│   └── ui/
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

See [Roadmap](docs/ROADMAP.md), [Product](docs/PRODUCT.md), [Architecture](docs/ARCHITECTURE.md), [ADR-0001](docs/adr/ADR-0001-smartphone-first-bluetooth-first.md) and the new App Hub/platform decision in ADR-0002.

## Status

Early architecture / proof-of-concept stage. The first technical path should validate a real Bluetooth/BLE + KISS + APRS workflow on desktop first, while keeping the protocol core portable to mobile.