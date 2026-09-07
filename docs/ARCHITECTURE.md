# RadioLink Platform — Architecture

## Architectural statement

RadioLink is a cross-platform amateur-radio platform. Android, iOS, Linux and macOS are first-class hosts. The host device owns application logic, user experience, local data and protocol orchestration. Radios/TNCs expose RF and device capabilities through one or more transport paths.

RadioLink formally supports three I/O families:

1. **Wireless Digital** — Bluetooth/BLE;
2. **Wired Digital** — USB-C/USB;
3. **Legacy / Analog Bridge** — audio + PTT + optional CAT through interfaces such as DigiRig, software TNC/modem providers or the future RadioLink Bridge.

Bluetooth and USB are transports, not application capabilities. The architecture must model what a device actually exposes and keep APRS/Packet/Winlink independent from the physical connection path.

```text
┌──────────────────────────────────────────────┐
│ Android / iOS / Linux / macOS               │
│                                              │
│ RadioLink Application Shell                 │
│ Message • Position • Packet • Winlink       │
│ Diagnostics • Radio Control                 │
│                                              │
│ RadioLink Operations Engine                 │
│ Capability Registry • Device Registry       │
│ Transport Manager • Provider Resolution     │
│ Lifecycle • Recovery • Fallback             │
│                                              │
│ Protocol / Service Core                     │
└──────────────────────┬───────────────────────┘
                       │
       ┌───────────────┼────────────────┐
       │               │                │
       ▼               ▼                ▼
  BLE / BT          USB-C/USB       Audio/PTT/CAT
 Wireless Digital   Wired Digital    Legacy Bridge
       │               │                │
       └───────────────┬────────────────┘
                       ▼
              Radio / TNC / Bridge
                       │
                       ▼
                      RF
```

## Architectural layering

The preferred dependency direction is:

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

No higher-level service should know whether a frame ultimately travelled over BLE, USB serial, USB audio or another supported transport.

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

The product is cross-platform even though development is **CLI-first on macOS/Linux** for lower-friction validation of the shared core and transports.

### 2. Application Shell / Mission UX

The shell is the main user experience and should expose user intent before low-level protocol machinery.

Preferred normal-user concepts:
- Send message;
- Share position;
- Send e-mail;
- View stations;
- Open Packet terminal;
- Diagnostics;
- Radio control.

Protocol names such as APRS, AX.25 and KISS remain available for advanced users and diagnostics.

### 3. RadioLink Operations Engine

The Operations Engine is the runtime orchestrator. It is not only a module launcher.

Responsibilities:
- query the Capability Registry;
- resolve devices and adapters;
- select an appropriate TNC/Modem Provider;
- select/prefer an appropriate transport;
- acquire/release radio/audio/serial resources;
- start/stop operational services;
- restore known-good state between mode changes;
- monitor health;
- reconnect and recover sessions;
- provide a compatible fallback path when possible;
- expose human-readable failure reasons.

Conceptually:

```text
User action: "Send APRS message"
             │
             ▼
      Operations Engine
             │
      capability query
             │
       ┌─────┼───────────────┐
       ▼     ▼               ▼
    BLE KISS USB KISS    USB Audio
       │     │          + software TNC
       └─────┼───────────────┘
             ▼
           AX.25
             ▼
            APRS
```

### 4. Core

`packages/core`

Platform-neutral domain logic:
- station model;
- message model;
- radio capability model;
- connection/session state;
- operational mode model;
- module/service orchestration interfaces;
- routing;
- local history abstractions;
- recovery/state concepts shared across hosts.

### 5. Protocols

`packages/protocols`

Protocol codecs and state machines:
- KISS framing;
- AX.25 framing/parsing;
- APRS decoding/encoding;
- Packet session helpers;
- future Winlink transport helpers.

Protocol code must be testable without a real radio and must not import platform Bluetooth/USB implementations.

### 6. Device / Driver Layer

`packages/drivers`

Drivers translate device-specific services/commands into a common capability model.

Conceptual API:

```text
RadioDevice
├── identity
├── connection state
├── transports[]
│   ├── ble
│   ├── usb
│   ├── serial
│   ├── audio
│   └── tcp/network where appropriate
├── capabilities
│   ├── cat
│   ├── audioRx
│   ├── audioTx
│   ├── ptt
│   ├── serialData
│   ├── kiss
│   ├── tnc
│   ├── radioControl
│   ├── gps
│   └── telemetry
├── receiveFrames()
├── sendFrame()
└── optional radio controls
```

A device may expose more than one transport and different capabilities on each transport.

Example:

```text
Radio X
├── BLE
│   ├── CAT
│   └── telemetry
└── USB-C
    ├── USB Audio
    ├── CAT
    └── KISS
```

The Transport Manager must represent these independently.

## The three official I/O paths

### Path A — Wireless Digital

```text
RadioLink host ↔ Bluetooth/BLE ↔ Radio/TNC/Bridge ↔ RF
```

Target use:
- cable-free mobile operation;
- KISS/data;
- control;
- telemetry;
- GPS/state;
- lightweight diagnostics.

Bluetooth presence is never treated as shorthand for Packet capability.

### Path B — Wired Digital

```text
RadioLink host ↔ USB-C/USB ↔ Radio/TNC/Bridge ↔ RF
```

USB may expose multiple logical interfaces:

```text
USB-C
├── CDC serial → CAT
├── CDC serial → KISS/data
├── USB Audio → RX/TX
├── network-style interface
└── power where supported
```

USB-C is only the connector. Capability detection must inspect the actual exposed interfaces.

This path may be preferred for long-running, high-reliability or higher-bandwidth workflows.

### Path C — Legacy / Analog Bridge

```text
RadioLink host
      ↓
software TNC/modem
      ↓
USB audio/PTT / RadioLink Bridge / DigiRig
      ↓
conventional radio
      ↓
RF
```

This path intentionally preserves support for existing analog radios that do not expose modern data interfaces.

## Transport Manager

The Transport Manager normalizes connection/session behavior across physical paths.

Conceptual responsibilities:

```text
TransportManager
├── discover()
├── connect()
├── disconnect()
├── health()
├── reconnect()
├── capabilities()
├── read()/write()
└── preference / fallback policy
```

Transport choice is operation-sensitive, not globally fixed.

Examples:
- a fast field APRS session may prefer BLE KISS;
- a long Winlink/modem session may prefer USB;
- a conventional HT may require USB audio/PTT plus a software TNC;
- if BLE drops and the same device is available over USB, the Operations Engine may offer/perform a compatible recovery path.

## Capability Registry

Capabilities are explicit facts about a device/provider path.

Rules:

1. `Bluetooth = true` does not imply `KISS = true`.
2. `Bluetooth = true` does not imply `TNC = true`.
3. `USB = true` does not imply serial, audio or KISS; those are separate interfaces.
4. CAT-only Bluetooth can control the radio but cannot by itself carry AX.25/APRS frames.
5. Bluetooth audio permits a software TNC only if usable bidirectional audio and PTT are exposed reliably.
6. A radio with BLE KISS / embedded TNC can connect directly to the RadioLink KISS provider.
7. A conventional radio can use DigiRig-class USB audio/PTT plus a software TNC on hosts where supported.
8. An external BLE/USB KISS TNC can provide the same logical TNC service for a radio without an embedded TNC.
9. Capabilities may be composed from multiple paths, for example CAT over Bluetooth plus TNC/audio over USB.

## TNC / Modem Provider abstraction

APRS/Packet/Winlink modules must not care where the TNC/modem lives.

```text
                     RadioLink Provider API
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
     Embedded TNC        External TNC       Software TNC
      BLE/USB KISS        BLE/USB KISS      Direwolf/etc.
          │                   │                   │
        Radio               Radio         Audio/PTT + Radio
```

Conceptual interface:

```text
TncBackend / ModemProvider
├── connect()
├── disconnect()
├── receiveFrame()/receiveData()
├── sendFrame()/sendData()
├── status()
└── capabilities()
```

This abstraction also enables future modem-provider interoperability such as Mercury/other implementations without coupling the application directly to a proprietary modem.

## Device Profiles / RadioLink Profiles

Profiles capture known-good combinations instead of forcing users to rediscover hardware-specific settings.

Potential fields:
- radio model/firmware;
- interface/adapter;
- cable;
- transports;
- PTT method;
- CAT support;
- audio RX/TX calibration;
- verified modes;
- platform compatibility;
- known limitations.

Profiles are metadata/configuration and must not replace runtime capability detection where detection is possible.

## Services

`packages/services`

Shared higher-level services such as:
- device registry;
- capability registry;
- transport/provider registry;
- module registry;
- station/message persistence;
- diagnostics/logging;
- optional Internet bridges such as APRS-IS later.

## UI

`packages/ui`

Shared design system/view models where practical. Native platform conventions remain authoritative.

The normal UX should expose user concepts first and technical pipeline detail second.

## CLI / Headless

`tools/radiolink-cli`

Linux and macOS support terminal/headless workflows where practical:

```text
radiolink scan
radiolink connect
radiolink capabilities
radiolink monitor
radiolink aprs
radiolink packet
radiolink serve
```

The CLI uses the same Core, Protocol, Driver, Transport and Provider layers as GUI applications.

## Optional RadioLink Bridge

`hardware/bridge`

A future compact accessory for radios without native smartphone/desktop-friendly data interfaces.

Target architecture:

```text
Radio audio/PTT/CAT
        │
        ▼
  RadioLink Bridge
        │
   ┌────┴────┐
   ▼         ▼
  BLE      USB-C
   │         │
   └────┬────┘
        ▼
 RadioLink host
```

Candidate responsibilities:
- BLE transport;
- USB-C/USB transport;
- KISS/TNC;
- audio RX/TX;
- PTT;
- optional CAT/control;
- device telemetry/configuration.

The Bridge must not become a second general-purpose computer.

## RadioLink Ready

RadioLink Ready is a future interoperability profile/specification, not an MVP dependency.

It may define a documented way for radios/accessories to expose:
- identity;
- capability discovery;
- KISS/data;
- CAT/control;
- PTT;
- GPS;
- battery/telemetry;
- transport availability.

Prefer established standards and documented interfaces before inventing RadioLink-specific protocols.

## RadioLink Labs

Experimental work belongs in Labs until product fit and architectural maturity are demonstrated.

Candidates:
- Winlink/Mercury modem experiments;
- Reticulum;
- LoRa;
- modern BBS/store-and-forward;
- offline radio knowledge tooling;
- additional digital modes.

Labs must not become runtime dependencies of the core MVP.

## Data flow — APRS receive

### Embedded/direct KISS

```text
RF
 ↓
Radio/TNC
 ↓ BLE KISS or USB KISS
Transport / Device Adapter
 ↓
TNC Provider
 ↓
KISS decoder
 ↓
AX.25 parser
 ↓
APRS decoder
 ↓
RadioLink Core
 ↓
APRS Service / UX
```

### Conventional radio via audio

```text
RF
 ↓
Conventional Radio
 ↓ audio/PTT
DigiRig / Bridge / USB audio
 ↓
Software TNC Provider
 ↓
AX.25
 ↓
APRS
 ↓
RadioLink Core
 ↓
APRS Service / UX
```

## Supported connection patterns

### A. Radio with embedded BLE KISS/TNC

```text
RadioLink host ↔ BLE KISS ↔ Radio/TNC ↔ RF
```

### B. Radio with embedded USB KISS/TNC

```text
RadioLink host ↔ USB KISS ↔ Radio/TNC ↔ RF
```

### C. Radio + external BLE/USB KISS TNC

```text
RadioLink host ↔ KISS TNC ↔ audio/PTT ↔ Radio ↔ RF
```

### D. Radio + DigiRig + software TNC

```text
RadioLink host ↔ software TNC ↔ USB audio/PTT ↔ DigiRig ↔ Radio ↔ RF
```

### E. Bluetooth CAT-only radio

```text
RadioLink host ↔ Bluetooth CAT ↔ Radio
```

Control only unless another TNC/audio path is also present.

### F. Multi-transport radio

```text
                ┌─ BLE → CAT/telemetry
RadioLink host ─┤
                └─ USB-C → KISS/audio
                         ↓
                        Radio
                         ↓
                         RF
```

RadioLink may compose capabilities from both paths.

## Key architectural constraints

1. Android, iOS, Linux and macOS are first-class targets.
2. CLI-first desktop development is a validation strategy, not a product-platform restriction.
3. No Raspberry Pi is required for core operation.
4. DigiPi is a product inspiration/reference, not a runtime dependency.
5. The user sees one coherent platform, not separate unrelated programs.
6. BLE and USB are first-class modern transports; audio/PTT is a first-class compatibility path.
7. Bluetooth alone must never be assumed to provide Packet/TNC capability.
8. USB alone must never be assumed to provide a specific logical interface.
9. Protocol/core code must be platform-neutral and independently testable.
10. Device-specific behavior belongs in drivers/adapters.
11. Transport behavior belongs in the Transport Manager/platform adapters.
12. Module/service-specific behavior must not leak into device drivers.
13. APRS/Packet/Winlink consume provider abstractions rather than binding directly to Direwolf, a specific BLE radio or a physical transport.
14. The Operations Engine owns capability resolution, resource lifecycle, recovery and fallback.
15. Linux/macOS may expose GUI and CLI/headless modes.
16. UI presents user intent before protocol concepts.
17. RadioLink Bridge remains an accessory, not a general-purpose computer.
18. RadioLink Ready and Labs must not expand the MVP until validated.

## Interoperability strategy

Prefer established protocols/transports:
- Bluetooth LE / Bluetooth;
- USB CDC / serial;
- USB Audio;
- KISS;
- AX.25;
- APRS;
- documented CAT/control interfaces;
- IP/network transports where they naturally apply.

RadioLink-specific extensions should only be introduced where existing standards cannot represent a required capability cleanly, and must be documented.
