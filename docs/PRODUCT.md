# RadioLink Platform — Product Definition

## Vision

Create a clean, open-source amateur-radio platform that turns a smartphone or computer into the main computing and interaction layer for digital radio operation.

RadioLink should reduce integration friction rather than add another isolated radio application. The operator should think in terms of **messages, position, e-mail, stations, terminal sessions and radio state** while RadioLink resolves the required device, transport, TNC/modem provider and protocol stack underneath.

## Core premise

- **Android / iOS / Linux / macOS = first-class supported hosts**
- **Radio = RF endpoint**
- **Shared Rust core = platform-neutral protocol/domain engine**
- **Bluetooth/BLE + USB-C/USB = modern first-class interfaces**
- **Audio/PTT/CAT = intentional compatibility path for legacy/conventional radios**
- **External hardware = optional compatibility layer, not the center of the system**
- **One platform = multiple radio services/modules**
- **CLI-first development = validation strategy, not a desktop-only product direction**

## Product family

RadioLink is organized as a platform/ecosystem rather than a single monolithic app.

### RadioLink Applications

User-facing hosts using the same core:

- Android;
- iOS;
- Linux;
- macOS;
- CLI/headless workflows where practical.

### RadioLink Core

Shared domain/protocol/runtime components:

- Operations Engine;
- Capability Registry;
- Device Registry;
- Transport Manager;
- TNC/Modem Provider abstraction;
- KISS / AX.25 / APRS and future protocol services;
- shared session, station and message state.

### RadioLink Profiles

Tested compatibility knowledge for:

- radios;
- interfaces;
- cables;
- transports;
- audio/PTT calibration;
- capabilities;
- supported modes and validated workflows.

### RadioLink Bridge

Future optional accessory for radios without smartphone-friendly digital interfaces. The target concept is a compact adapter exposing radio audio/PTT/CAT/TNC capabilities over **BLE and USB-C**.

### RadioLink Ready

Future documented interoperability profile/specification for radios and accessories that expose capabilities in a RadioLink-friendly, standards-oriented way.

### RadioLink Labs

Research area for ideas that should not expand the MVP until validated, including:

- Winlink/Mercury experiments beyond the core Packet path;
- Reticulum;
- LoRa;
- modern BBS/store-and-forward concepts;
- offline radio knowledge tools;
- additional digital modes and network transports.

## The three official I/O paths

RadioLink formally supports three connection families.

### 1. Wireless Digital

Bluetooth/BLE for cable-free mobile operation.

Typical capabilities may include:

- KISS;
- serial/data transport;
- CAT/radio control;
- telemetry;
- GPS/state;
- embedded TNC access.

Bluetooth is a transport only. Bluetooth presence never implies KISS, TNC, audio or CAT capability.

### 2. Wired Digital

USB-C/USB for reliable wired operation.

A USB device may expose one or more independent interfaces:

- KISS;
- USB CDC/serial;
- CAT;
- USB audio;
- network-style data interfaces;
- power, where applicable.

USB-C is a connector; RadioLink models the actual capabilities/interfaces exposed by the device.

### 3. Legacy / Analog Bridge

Compatibility path for the installed base of conventional radios using:

- audio RX/TX;
- PTT;
- optional CAT;
- DigiRig-class interfaces;
- software TNC/modem providers;
- future RadioLink Bridge hardware.

This path is not deprecated. It is how RadioLink can support large numbers of existing radios while the market transitions toward richer BLE/USB interfaces.

## Product idea

RadioLink preserves the best usability idea behind DigiPi: several radio capabilities available from one coherent environment. The difference is that RadioLink runs directly on the user's existing phone or computer instead of requiring a Raspberry Pi appliance.

The product should move from a protocol-first mental model toward a mission/action-first model.

Instead of requiring the normal user to choose low-level components such as Direwolf, KISS ports or audio devices, the main UX should favor actions such as:

- Send message;
- Share position;
- Send e-mail;
- View nearby stations;
- Open Packet terminal;
- Inspect diagnostics.

Advanced users may still inspect the selected device/provider/transport pipeline.

## Target user experience

1. Open RadioLink.
2. Connect or discover a radio/TNC/interface.
3. RadioLink identifies the actual capabilities and available transports.
4. The user chooses what they want to do.
5. The Operations Engine selects and starts a valid device → transport → provider → protocol pipeline.
6. If a transport drops, RadioLink attempts recovery or offers a compatible fallback without changing the service concept.

Example diagnostic view:

```text
APRS Messaging
via BTECH UV-Pro → BLE → KISS → AX.25
```

or:

```text
APRS Messaging
via conventional radio → DigiRig → USB Audio → software TNC → AX.25
```

The user-facing service remains the same.

## Primary use cases

1. Connect a supported radio/TNC over Bluetooth/BLE.
2. Connect a supported radio/TNC/interface over USB-C/USB.
3. Connect a conventional radio through USB audio/PTT or RadioLink Bridge.
4. Receive APRS traffic and display nearby stations.
5. Send APRS position beacons and messages.
6. Use Packet/AX.25 tools.
7. Use Winlink from the same platform after the core Packet path is stable.
8. Control basic radio parameters when hardware supports it.
9. Run equivalent core functionality on Android, iOS, Linux and macOS.
10. Use Linux/macOS in GUI or CLI/headless modes where practical.
11. Reuse tested RadioLink Profiles for known-good hardware configurations.

## Product principles

### 1. Cross-platform core
Protocol, domain and orchestration logic should be shared whenever technically practical.

### 2. Mission-first UX
Normal users should think in terms of outcomes such as messaging, position and e-mail. Protocol names remain visible for diagnostics and advanced workflows.

### 3. Transport independence
No service should be hard-bound to Bluetooth, USB, audio, one radio model or one TNC implementation.

### 4. Modern dual-interface strategy
BLE is preferred for cable-free mobile workflows when reliable. USB-C/USB is equally first-class for robust wired operation, digital audio, serial/CAT and long-running sessions.

### 5. Legacy compatibility by design
Audio/PTT/CAT remains an official path through DigiRig-class interfaces, software TNCs and the future RadioLink Bridge.

### 6. Capability-driven hardware abstraction
Common capabilities are exposed through device profiles/drivers rather than hard-coding one radio model into APRS/Packet/Winlink logic.

### 7. Operations Engine owns lifecycle
Capability discovery, resource selection, service startup/shutdown, recovery and fallback belong to the runtime orchestration layer.

### 8. Offline-first field operation
Core RF functions and local state should not require an Internet connection.

### 9. Progressive capability
A BLE KISS device may expose Packet/APRS only; richer radios may add CAT/PTT/GPS/telemetry; an audio-only radio may still participate through a software TNC or Bridge.

### 10. Stability over feature count
The MVP should support a small number of reliable, well-tested workflows rather than many partially integrated modes.

## Initial module set

- APRS;
- Packet / AX.25;
- KISS monitor/diagnostics;
- Radio control;
- Winlink after core Packet support.

## Later modules / Labs candidates

- SSTV;
- selected digital modes;
- APRS-IS/iGate experiments;
- Mercury/other modem providers;
- Reticulum / LoRa experiments;
- modern BBS/store-and-forward concepts;
- richer offline field knowledge features.

## Initial supported device classes

- Radios with embedded BLE KISS/TNC;
- Bluetooth KISS TNCs;
- USB KISS/serial TNCs;
- USB audio/PTT interfaces;
- radios with USB audio/CAT;
- conventional radios through DigiRig/software TNC;
- future RadioLink Bridge for analog/legacy radios.

## Candidate first validation hardware

- BTECH UV-PRO-class embedded Bluetooth/TNC radio;
- Mobilinkd-class BLE KISS TNC;
- DigiRig + conventional HT as a compatibility baseline;
- one USB-native radio/interface path when available for validation.

## MVP success criteria

The first platform MVP is successful when a normal user can:

1. install RadioLink on at least one desktop and one mobile platform;
2. connect a compatible radio or TNC;
3. see the device's real capabilities rather than a generic “Bluetooth connected” state;
4. complete APRS RX/TX through at least one digital path;
5. complete APRS RX/TX through at least one compatibility/audio path;
6. receive and decode real RF traffic;
7. transmit a valid APRS beacon;
8. exchange APRS messages;
9. inspect raw KISS/AX.25 diagnostics when needed;
10. reuse the same core protocol/service implementation across host platforms and transport paths.

## Non-goals for initial MVP

- DMR networking;
- LoRa mesh as a core requirement;
- multimode infrastructure gateways;
- replacing every DigiPi application immediately;
- hosting arbitrary Linux services;
- supporting every radio brand from day one;
- building the RadioLink Bridge before the software/device abstractions are validated;
- turning Labs experiments into mandatory runtime dependencies.
