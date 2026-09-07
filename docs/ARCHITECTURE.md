# RadioLink Platform — Architecture

## Architectural statement

RadioLink is a cross-platform amateur-radio platform. Android, iOS, Linux and macOS are first-class hosts. The host device owns application logic, user experience, local data and protocol orchestration. Radios/TNCs expose RF and device capabilities through one or more transport paths.

RadioLink formally supports three I/O families:

1. **Wireless Digital** — Bluetooth/BLE;
2. **Wired Digital** — USB-C/USB;
3. **Legacy / Analog Bridge** — audio + PTT + optional CAT through interfaces such as DigiRig, software TNC/modem providers or the future RadioLink Bridge.

Bluetooth and USB are transports, not application capabilities. The architecture must model what a device actually exposes and keep APRS/Packet/Winlink independent from the physical connection path.

Operational context such as **location and time** is also independent from the radio. A service may use RF/TNC capability from one device while location/time comes from the host, the radio, a USB GPS/GNSS source or another validated provider.

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
│ Context Registry • Provider Resolution      │
│ Transport Manager • Lifecycle • Recovery    │
│ Layered Diagnostics • Fallback              │
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

Context providers may enter independently from the host/device side:

```text
Host location ───────┐
Radio GPS/GNSS ──────┼→ Context Provider API → Service
USB GPS / GPSD ──────┤
Manual/static ───────┘
```

## Architectural layering

The preferred RF/data dependency direction is:

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

Context uses a parallel provider boundary:

```text
Operational Mode / Service
       ↓
Context Provider API
       ↓
Provider implementation
       ↓
Host / device / transport-specific source
```

No higher-level service should know whether a frame ultimately travelled over BLE, USB serial, USB audio or another supported transport. Likewise, a position service should not require location to be owned by the radio itself.

## Repository layers

### 1. Host Applications

`apps/android`, `apps/ios`, `apps/linux`, `apps/macos`

Responsibilities:
- platform UI shell;
- permissions;
- Bluetooth lifecycle;
- USB/device access where applicable;
- host location/context adapters;
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
- resolve required Context Providers;
- acquire/release radio/audio/serial/context resources;
- start/stop operational services;
- restore known-good state between mode changes;
- monitor health;
- reconnect and recover sessions;
- provide a compatible fallback path when possible;
- maintain layered diagnostic state;
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

For a position workflow, context resolution may happen in parallel:

```text
APRS beacon request
       │
       ├→ TNC/transport resolution
       │
       └→ LocationProvider resolution
              ├→ host phone
              ├→ radio GPS/GNSS
              ├→ USB GPS
              └→ other validated source
```

### 4. Core

`packages/core`

Platform-neutral domain logic:
- station model;
- message model;
- radio capability model;
- connection/session state;
- operational mode model;
- context-provider interfaces/state;
- layered diagnostic-state concepts;
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
│   ├── manufacturer/model
│   ├── hardware revision where known
│   └── firmware/version where known
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
│   ├── gpsGnssSourceExposure
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

A different modern device may expose KISS over BLE while GPS/GNSS data is consumed through USB. The Transport Manager and provider registries must represent these independently.

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
- radio GPS/GNSS/state exposure where available;
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
├── GPS/GNSS/data interface
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
10. Firmware/version and required radio-side configuration may materially change whether a capability is actually usable.

## Context Provider abstraction

Location/time are operational inputs, not capabilities owned exclusively by `RadioDevice`.

Initial conceptual providers:

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

Services request context through the provider API. The Operations Engine / Context Registry resolves an available source according to capability, quality, platform permissions and operator policy.

Guardrails:
- do not assume `gps=true` means the radio must provide position;
- do not make Internet/network time mandatory for core RF operation;
- do not silently switch providers when a switch could materially change operator intent;
- expose provider selection/state in diagnostics when relevant.

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

## Layered diagnostics

RadioLink must preserve the highest verified stage of a connection rather than collapsing every failure into an application error.

```text
physical / power
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

Examples of useful states:

- `USB device has power but no data interface detected`;
- `Bluetooth connected; KISS service not exposed`;
- `KISS provider ready; no AX.25 frames received yet`;
- `APRS ready; location source unavailable for beacon TX`.

Diagnostics must not claim a specific physical cause if the host can only prove that a higher layer is unavailable.

## Device Profiles / RadioLink Profiles

Profiles capture known-good combinations instead of forcing users to rediscover hardware-specific settings.

Canonical governance and nominal devices:

- `docs/devices/README.md`;
- `docs/devices/REGISTRY.md`;
- `docs/devices/PROFILE-TEMPLATE.md`;
- `docs/devices/profiles/`.

Potential fields:
- manufacturer/model/hardware revision;
- firmware version/range;
- validation state;
- interface/adapter;
- cable;
- transports;
- PTT method;
- CAT support;
- audio RX/TX calibration;
- radio GPS/GNSS exposure;
- required radio-side settings/preflight recipe;
- verified services/modes;
- platform compatibility;
- layered diagnostic result;
- captured evidence;
- known limitations.

Profiles are metadata/configuration and must not replace runtime capability detection where detection is possible.

Device support advances explicitly through:

```text
RESEARCHED
   ↓
CANDIDATE
   ↓
LAB_AVAILABLE
   ↓
CONNECTED
   ↓
RX_VERIFIED
   ↓
TX_VERIFIED
   ↓
PROFILE_VERIFIED
   ↓
SUPPORTED
```

## Services

`packages/services`

Shared higher-level services such as:
- runtime device registry;
- capability registry;
- transport/provider registry;
- context provider registry;
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
radiolink transports
radiolink pipeline
radiolink diagnose
radiolink context
radiolink monitor
radiolink aprs
radiolink packet
radiolink serve
```

The CLI uses the same Core, Protocol, Driver, Transport, TNC/Modem Provider and Context Provider layers as GUI applications.

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
- GPS/GNSS/context-source availability;
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
- delivery-independent Messaging Service experiments;
- offline radio knowledge/data services;
- field readiness/self-test tooling;
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

### APRS position/beacon context

```text
Location source
  ↓
LocationProvider
  ↓
APRS Service
  ↓
AX.25 / TNC Provider / Transport
  ↓
RF
```

The location source may be the host, radio or another provider independently from the RF transport.

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

### F. Bluetooth audio/PTT radio

```text
RadioLink host ↔ Bluetooth audio/PTT ↔ software TNC ↔ Radio ↔ RF
```

This path remains per-device/platform experimental until audio/PTT behavior is validated.

### G. Multi-transport radio

```text
                ┌─ BLE → KISS/CAT/telemetry
RadioLink host ─┤
                └─ USB-C → GPS/KISS/audio/CAT
                         ↓
                        Radio
                         ↓
                         RF
```

RadioLink may compose capabilities and context sources from multiple paths.

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
14. Location/time are resolved through Context Providers rather than assumed to belong to the radio.
15. The Operations Engine owns capability/context resolution, resource lifecycle, recovery and fallback.
16. Diagnostics preserve the highest verified layer from physical transport through service readiness.
17. Firmware and required device configuration are part of capability-validation context.
18. Device research evidence does not equal RadioLink support; support requires explicit profile/bench validation.
19. Linux/macOS may expose GUI and CLI/headless modes.
20. UI presents user intent before protocol concepts.
21. RadioLink Bridge remains an accessory, not a general-purpose computer.
22. RadioLink Ready and Labs must not expand the MVP until validated.

## Interoperability strategy

Prefer established protocols/transports:
- Bluetooth LE / Bluetooth;
- USB CDC / serial;
- USB Audio;
- KISS;
- AX.25;
- APRS;
- documented CAT/control interfaces;
- standard GPS/GNSS/NMEA/GPSD-style interfaces where appropriate;
- IP/network transports where they naturally apply.

RadioLink-specific extensions should only be introduced where existing standards cannot represent a required capability cleanly, and must be documented.

## Decision references

- `docs/adr/ADR-0004-three-path-io-and-platform-ecosystem.md` — transport/provider/platform ecosystem.
- `docs/adr/ADR-0005-context-providers-and-layered-diagnostics.md` — Context Providers, multi-source operational context and layered diagnostics.
- `docs/research/PROMOTION-REGISTER.md` — traceability from research findings to architecture, roadmap, implementation and validation.
