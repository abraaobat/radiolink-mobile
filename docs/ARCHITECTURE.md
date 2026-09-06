# RadioLink Platform — Architecture

## Architectural statement

RadioLink is a cross-platform amateur-radio application hub. Android, iOS, Linux and macOS are first-class hosts. The host device owns application logic, user experience, local data and protocol orchestration, and should also own modem/TNC processing whenever a reliable audio/PTT transport makes that practical.

Radios and adapters expose RF-side capabilities through Bluetooth/BLE whenever possible, with USB/audio as the deterministic reference path for conventional radios.

```text
┌──────────────────────────────────────┐
│ Android / iOS / Linux / macOS       │
│                                      │
│ RadioLink Shell / App Hub            │
│ APRS • Packet • Winlink • KISS       │
│ Radio Control • future modules       │
│                                      │
│ RadioLink Core                       │
│ Protocol Core                        │
│ Software Modem/TNC where practical   │
│ Driver Layer                         │
└──────────────────┬───────────────────┘
                   │ Bluetooth / USB
┌──────────────────▼───────────────────┐
│ Radio / TNC / DigiRig / Bridge       │
└──────────────────┬───────────────────┘
                   │
                   ▼
                  RF
```

## Core architectural rule: host-first TNC

A smartphone or computer is already a powerful computing platform. RadioLink should not move AFSK/AX.25 processing into an ESP32 merely to save host CPU.

The embedded controller exists primarily to solve physical radio integration and cable-free transport.

The preferred development model is therefore:

```text
Radio
  │
audio RX/TX + PTT
  │
Radio interface / RadioLink Bridge
  │
USB audio/control
  │
Host
  │
Software TNC / RadioLink modem
  │
AX.25 / APRS / Packet
```

After the radio interface is proven, the same bridge may add a second operating mode:

```text
Radio
  │
audio RX/TX + PTT
  │
ESP32 bridge
├── AFSK
├── AX.25
└── KISS
  │
BLE
  │
RadioLink host
```

This embedded-TNC mode exists to enable a compact Mobilinkd-like cable-free workflow, not because the host lacks processing power.

## Why raw Bluetooth audio is not the default packet path

Bluetooth audio stacks may introduce codec compression, resampling, AGC, noise suppression, buffering and variable latency. Those behaviors can be acceptable for voice but are undesirable for deterministic AFSK modem operation.

Therefore:

1. BLE KISS/data is preferred for wireless Packet/APRS operation.
2. USB audio/PTT is preferred for reference software-TNC validation.
3. Raw Bluetooth audio modem operation is experimental and must not be assumed reliable across platforms.
4. Wi-Fi audio streaming may be explored later, but is not part of the initial MVP.

## Repository layers

### 1. Host Applications

`apps/android`, `apps/ios`, `apps/linux`, `apps/macos`

Responsibilities:
- platform UI shell;
- permissions;
- Bluetooth lifecycle;
- USB/device access where applicable;
- location services;
- notifications;
- platform-specific storage/adapters;
- background/headless execution where supported;
- host audio capture/playback integration where software-TNC mode is supported.

### 2. App Hub / Module Shell

The shell is the main user experience. It exposes modules from one launcher and shares device/session state between them.

Initial modules:
- APRS;
- Packet / AX.25;
- KISS monitor/diagnostics;
- Radio Control;
- Winlink later.

Future modules can include SSTV and selected digital modes.

### 3. Core

`crates/radiolink-core`

Platform-neutral domain logic:
- station model;
- message model;
- radio capability model;
- connection/session state;
- module orchestration;
- routing;
- local history abstractions.

### 4. Protocols

Protocol crates include:
- KISS framing;
- AX.25 framing/parsing;
- APRS decoding/encoding;
- Packet session helpers;
- future Winlink transport helpers.

Protocol code must be testable without a real radio.

### 5. TNC / modem layer

The TNC layer must support multiple backends without changing APRS/Packet modules.

```text
                 RadioLink TNC API
                        │
        ┌───────────────┼────────────────┐
        │               │                │
 Embedded TNC      External TNC     Host Software TNC
  BLE KISS           BLE KISS       audio/PTT backend
        │               │                │
      Radio           Radio       Bridge/DigiRig + Radio
```

Conceptually:

```text
TncBackend
├── connect()
├── disconnect()
├── receiveFrame()
├── sendFrame()
├── status()
└── capabilities()
```

A future native RadioLink modem can implement this same backend interface. During early validation, Direwolf serves as a trusted reference software TNC on desktop.

### 6. Drivers

`crates/radiolink-drivers`

Drivers translate device-specific Bluetooth/USB/services/commands into common capabilities.

Conceptual capability model:

```text
RadioDevice
├── identity
├── connection
├── capabilities
│   ├── bluetooth
│   ├── cat
│   ├── audioRx
│   ├── audioTx
│   ├── ptt
│   ├── serial
│   ├── kiss
│   ├── embeddedTnc
│   ├── hostTncCompatible
│   ├── radioControl
│   ├── usb
│   └── telemetry
└── optional radio controls
```

### Bluetooth is not a capability shortcut

A radio advertising Bluetooth is **not automatically Packet/APRS-ready**. Bluetooth is only a transport. RadioLink must identify what the radio actually exposes.

Rules:

1. `Bluetooth = true` does not imply `KISS = true`.
2. `Bluetooth = true` does not imply `TNC = true`.
3. CAT-only Bluetooth can control the radio but cannot by itself carry AX.25/APRS frames.
4. Bluetooth audio permits software-TNC operation only if bidirectional audio and PTT are exposed reliably enough for modem use.
5. A radio with BLE KISS / embedded TNC can connect directly to the RadioLink KISS backend.
6. A conventional radio can use USB audio/PTT plus a host software TNC.
7. An external BLE KISS TNC can provide the same logical TNC service for a radio without an embedded TNC.
8. RadioLink Bridge must advertise whether it is operating as audio/PTT interface, embedded KISS TNC, or both.

### 7. Services

Shared higher-level services such as:
- device registry;
- module registry;
- station/message persistence;
- diagnostics/logging;
- optional Internet bridges such as APRS-IS later.

### 8. CLI / Headless

`tools/radiolink-cli`

Linux and macOS should also support terminal/headless workflows where practical:

```text
radiolink scan
radiolink connect
radiolink monitor
radiolink aprs
radiolink packet
radiolink serve
```

The CLI uses the same Core, Protocol and Driver layers as GUI applications.

## RadioLink Bridge

`hardware/bridge`

The Bridge is an optional compact accessory for radios without native smartphone/desktop-friendly interfaces.

### Bridge V0.x — radio interface mode

First responsibility:
- audio RX;
- audio TX;
- PTT;
- safe level conditioning;
- host transport;
- optional CAT/control.

Reference proof path:

```text
Quansheng / conventional radio
        │
       K-plug
        │
conditioning + codec + PTT
        │
     ESP32-S3
        │
       USB
        │
Mac/Linux host
        │
Direwolf / software TNC
```

### Bridge later — embedded TNC mode

After audio/PTT is validated:
- AFSK 1200 modem;
- AX.25 frame handling as needed;
- KISS transport;
- BLE KISS;
- mobile cable-free workflow.

The Bridge must not become a second general-purpose computer.

## Data flow — APRS receive

### Host software-TNC path

```text
RF
 ↓
Radio
 ↓ audio
RadioLink Bridge / DigiRig
 ↓ USB audio
Software TNC / modem on host
 ↓ AX.25 frames
TncBackend
 ↓
APRS decoder
 ↓
RadioLink Core
 ↓
APRS Module
 ↓
Station List / Map / Message UI
```

### BLE KISS path

```text
RF
 ↓
Radio / embedded or external TNC
 ↓ BLE KISS
Device Driver / TNC Backend
 ↓
KISS decoder
 ↓
AX.25 parser
 ↓
APRS decoder
 ↓
RadioLink Core
 ↓
APRS Module
```

## Supported connection patterns

### A. Radio with embedded BLE KISS/TNC

```text
RadioLink host ↔ BLE KISS ↔ Radio/TNC ↔ RF
```

### B. Radio + external BLE KISS TNC

```text
RadioLink host ↔ BLE KISS TNC ↔ audio/PTT ↔ Radio ↔ RF
```

### C. Radio + USB audio/PTT + host software TNC

```text
RadioLink desktop ↔ software TNC ↔ USB audio/PTT ↔ Bridge/DigiRig ↔ Radio ↔ RF
```

This is the preferred reference path for bringing up conventional-radio hardware.

### D. RadioLink Bridge embedded-TNC mode

```text
RadioLink mobile ↔ BLE KISS ↔ RadioLink Bridge [AFSK/AX.25] ↔ Radio ↔ RF
```

### E. Bluetooth CAT-only radio

```text
RadioLink host ↔ Bluetooth CAT ↔ Radio
```

This supports control only unless another TNC/audio path is also present.

## Development sequence for conventional-radio support

```text
1. electrical radio interface
2. safe RX/TX audio levels
3. PTT control
4. USB audio/control path
5. software-TNC validation on desktop
6. real APRS RX/TX validation
7. embedded AFSK implementation
8. BLE KISS transport
9. mobile cable-free validation
```

This prevents modem firmware bugs from being confused with hardware/audio-level problems.

## Key architectural constraints

1. Android, iOS, Linux and macOS are first-class targets.
2. No Raspberry Pi is required for core operation.
3. DigiPi is a product inspiration/reference, not a runtime dependency.
4. The host is the preferred computing layer, including modem/TNC processing where practical.
5. The user sees one hub with modules, not separate unrelated programs.
6. Bluetooth/BLE is the preferred cable-free transport, but Bluetooth alone must never be assumed to provide Packet/TNC capability.
7. USB/audio is a supported and preferred reference transport for software-TNC validation.
8. Raw Bluetooth audio is not the default AFSK transport.
9. Protocol/core code must be platform-neutral and independently testable.
10. Device-specific behavior belongs in drivers.
11. Module-specific behavior should not leak into device drivers.
12. APRS/Packet modules consume a TNC backend abstraction rather than binding directly to Direwolf or a specific BLE radio.
13. The RadioLink Bridge must first be useful as a radio interface; embedded TNC functionality is additive.
14. Linux/macOS may expose GUI and CLI/headless modes.
15. UI presents user concepts before protocol concepts.

## Interoperability strategy

Prefer established protocols/transports:
- Bluetooth LE / Bluetooth;
- KISS;
- AX.25;
- APRS;
- USB audio/serial when necessary.

RadioLink-specific extensions should only be introduced where existing standards cannot represent a required capability cleanly, and must be documented.
