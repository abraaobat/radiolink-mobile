# RadioLink Platform — Architecture

## Architectural statement

RadioLink is a cross-platform amateur-radio application hub. Android, iOS, Linux and macOS are first-class hosts. The host device owns application logic, user experience, local data and protocol orchestration. Radios/TNCs expose RF and device capabilities through Bluetooth/BLE whenever possible, with USB/audio as a secondary compatibility path.

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
- background/headless execution where supported.

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

`packages/core`

Platform-neutral domain logic:
- station model;
- message model;
- radio capability model;
- connection/session state;
- module orchestration;
- routing;
- local history abstractions.

### 4. Protocols

`packages/protocols`

Protocol codecs and state machines:
- KISS framing;
- AX.25 framing/parsing;
- APRS decoding/encoding;
- Packet session helpers;
- future Winlink transport helpers.

Protocol code must be testable without a real radio.

### 5. Drivers

`packages/drivers`

Drivers translate device-specific Bluetooth/USB/services/commands into a common capability interface.

Conceptual API:

```text
RadioDevice
├── identity
├── connection
├── capabilities
│   ├── kiss
│   ├── radioControl
│   ├── ptt
│   ├── audioData
│   ├── usb
│   └── telemetry
├── receiveFrames()
├── sendFrame()
└── optional radio controls
```

### 6. Services

`packages/services`

Shared higher-level services such as:
- device registry;
- module registry;
- station/message persistence;
- diagnostics/logging;
- optional Internet bridges such as APRS-IS later.

### 7. UI

`packages/ui`

Shared design system/view models where practical. Native platform conventions remain authoritative.

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

### 9. Optional Hardware Bridge

`hardware/bridge`

A future compact accessory for radios without native Bluetooth/data interfaces.

Its role is constrained to transport/TNC/PTT/CAT/audio adaptation. It must not become a second general-purpose computer.

## Data flow — APRS receive

```text
RF
 ↓
Radio/TNC
 ↓ Bluetooth KISS / USB / audio TNC
Device Driver
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
 ↓
Station List / Map / Message UI
```

## Key architectural constraints

1. Android, iOS, Linux and macOS are first-class targets.
2. No Raspberry Pi is required for core operation.
3. DigiPi is a product inspiration/reference, not a runtime dependency.
4. The user sees one hub with modules, not separate unrelated programs.
5. Bluetooth/BLE is the preferred radio transport.
6. USB/audio is a supported compatibility transport.
7. Protocol/core code must be platform-neutral and independently testable.
8. Device-specific behavior belongs in drivers.
9. Module-specific behavior should not leak into device drivers.
10. Linux/macOS may expose GUI and CLI/headless modes.
11. UI presents user concepts before protocol concepts.

## Interoperability strategy

Prefer established protocols/transports:
- Bluetooth LE / Bluetooth;
- KISS;
- AX.25;
- APRS;
- USB serial/audio when necessary.

RadioLink-specific extensions should only be introduced where existing standards cannot represent a required capability cleanly, and must be documented.
