# RadioLink Mobile

**Smartphone-first, Bluetooth-first amateur radio platform.**

RadioLink Mobile is an open-source project for integrating Android/iOS smartphones with amateur radios through Bluetooth/BLE, with the phone acting as the main computer and user interface while the radio remains the RF endpoint.

> **Phone = computer. Radio = RF. Bluetooth = bridge.**

## Why

Traditional mobile radio-computing stacks often depend on Raspberry Pi/Linux boxes, external displays, GPS modules, cables and multiple daemons. RadioLink Mobile starts from a cleaner premise: modern smartphones already provide the display, CPU, storage, GPS, connectivity, battery and polished UX.

## Initial scope

- Bluetooth/BLE device discovery and connection
- KISS transport
- AX.25 packet handling
- APRS receive/transmit
- APRS station map
- APRS messaging
- Basic radio control where supported
- Android and iOS apps
- Winlink Packet in a later MVP phase
- Driver model for multiple radios/TNCs
- Optional RadioLink Bridge for legacy radios

## Explicitly out of MVP

- Raspberry Pi / DigiPi dependency
- DMR
- LoRa / mesh
- General-purpose Linux services
- Large gateway/server deployments

These may integrate later, but they must not complicate the core mobile experience.

## Architecture

```text
Android / iOS
     │
     ├── UI: map, messages, radio, settings
     │
     ├── Radio Core: APRS, AX.25, messaging
     │
     ├── Driver Layer
     │
     └── Bluetooth / BLE
              │
              ▼
      Radio / TNC / Bridge
              │
              ▼
             RF
```

## Repository

```text
radiolink-mobile/
├── apps/
│   ├── android/
│   └── ios/
├── packages/
│   ├── core/
│   ├── protocols/
│   ├── drivers/
│   └── ui/
├── hardware/
│   └── bridge/
├── docs/
│   ├── PRODUCT.md
│   ├── ARCHITECTURE.md
│   ├── ROADMAP.md
│   └── adr/
└── tests/
```

## Current phase

**F0 — Project Foundation**

See [Roadmap](docs/ROADMAP.md), [Product](docs/PRODUCT.md), [Architecture](docs/ARCHITECTURE.md) and [ADR-0001](docs/adr/ADR-0001-smartphone-first-bluetooth-first.md).

## Status

Early architecture / proof-of-concept stage. The first technical target is a real **Bluetooth/BLE + KISS + APRS** path with a supported radio/TNC.
