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
│   ├── bluetooth
│   ├── cat
│   ├── audioRx
│   ├── audioTx
│   ├── ptt
│   ├── serial
│   ├── kiss
│   ├── tnc
│   ├── radioControl
│   ├── usb
│   └── telemetry
├── receiveFrames()
├── sendFrame()
└── optional radio controls
```

### Bluetooth is not a capability shortcut

A radio advertising Bluetooth is **not automatically Packet/APRS-ready**. Bluetooth is only a transport. RadioLink must identify what the radio actually exposes.

Every device profile should classify these capabilities independently:

```text
Bluetooth
[ ] CAT / radio control
[ ] Audio RX
[ ] Audio TX
[ ] PTT
[ ] Serial/data transport
[ ] KISS
[ ] Embedded TNC
```

Rules:

1. `Bluetooth = true` does not imply `KISS = true`.
2. `Bluetooth = true` does not imply `TNC = true`.
3. CAT-only Bluetooth can control the radio but cannot by itself carry AX.25/APRS frames.
4. Bluetooth audio may permit a software TNC only if usable bidirectional audio and PTT are exposed reliably.
5. A radio with BLE KISS / embedded TNC can connect directly to the RadioLink KISS backend.
6. A conventional radio can use DigiRig-class USB audio/PTT plus a software TNC such as Direwolf on desktop hosts.
7. An external BLE KISS TNC can provide the same logical TNC service for a radio without an embedded TNC.

### TNC backend abstraction

APRS/Packet modules must not care where the TNC lives.

```text
                 RadioLink TNC API
                        │
          ┌─────────────┼─────────────┐
          │             │             │
     Embedded TNC   External TNC   Software TNC
       BLE KISS       BLE KISS       Direwolf
          │             │             │
        Radio         Radio      DigiRig + Radio
```

Conceptually, modules consume a shared interface:

```text
TncBackend
├── connect()
├── disconnect()
├── receiveFrame()
├── sendFrame()
├── status()
└── capabilities()
```

This allows the same APRS, Packet and future Winlink modules to work with different hardware paths.

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
 ↓
Station List / Map / Message UI
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

### C. Radio + DigiRig + software TNC

```text
RadioLink desktop ↔ Direwolf ↔ USB audio/PTT ↔ DigiRig ↔ Radio ↔ RF
```

### D. Bluetooth CAT-only radio

```text
RadioLink host ↔ Bluetooth CAT ↔ Radio
```

This supports control only unless another TNC/audio path is also present.

## Key architectural constraints

1. Android, iOS, Linux and macOS are first-class targets.
2. No Raspberry Pi is required for core operation.
3. DigiPi is a product inspiration/reference, not a runtime dependency.
4. The user sees one hub with modules, not separate unrelated programs.
5. Bluetooth/BLE is the preferred radio transport, but Bluetooth alone must never be assumed to provide Packet/TNC capability.
6. USB/audio is a supported compatibility transport.
7. Protocol/core code must be platform-neutral and independently testable.
8. Device-specific behavior belongs in drivers.
9. Module-specific behavior should not leak into device drivers.
10. APRS/Packet modules consume a TNC backend abstraction rather than binding directly to Direwolf or a specific BLE radio.
11. Linux/macOS may expose GUI and CLI/headless modes.
12. UI presents user concepts before protocol concepts.

## Interoperability strategy

Prefer established protocols/transports:
- Bluetooth LE / Bluetooth;
- KISS;
- AX.25;
- APRS;
- USB serial/audio when necessary.

RadioLink-specific extensions should only be introduced where existing standards cannot represent a required capability cleanly, and must be documented.
