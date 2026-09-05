# RadioLink Platform — Product Definition

## Vision

Create a clean, open-source amateur-radio platform that turns a smartphone or computer into the main computing and interaction layer for digital radio operation.

## Core premise

- **Android / iOS / Linux / macOS = supported hosts**
- **Radio = RF endpoint**
- **Bluetooth/BLE = preferred bridge**
- **USB/audio = secondary compatibility path**
- **External hardware = optional compatibility layer, not the center of the system**
- **One hub = multiple radio applications/modules**

## Product idea

RadioLink preserves the best usability idea behind DigiPi: several radio applications available from one easy launcher. The difference is that RadioLink runs directly on the user's existing phone or computer instead of requiring a Raspberry Pi appliance.

## Target user experience

The operator opens RadioLink and chooses a module such as APRS, Packet, Winlink, KISS diagnostics or Radio Control. The same connected radio/TNC and shared station/message state are reused across modules.

Protocol details such as AX.25, KISS and AFSK remain available for diagnostics, but do not dominate the normal UX.

## Primary use cases

1. Connect a supported radio/TNC over Bluetooth/BLE.
2. Select an operating module from a central launcher.
3. Receive APRS traffic and display nearby stations.
4. Send APRS position beacons and messages.
5. Use Packet/AX.25 tools.
6. Use Winlink from the same platform.
7. Control basic radio parameters when hardware supports it.
8. Run equivalent core functionality on Android, iOS, Linux and macOS.
9. Use Linux/macOS in GUI or CLI/headless modes where practical.
10. Connect legacy radios through USB/audio or an optional RadioLink Bridge.

## Product principles

### 1. Cross-platform core
Protocol and domain logic should be shared whenever technically practical.

### 2. App Hub Architecture
The product is a launcher plus modules, not a collection of unrelated programs.

### 3. Bluetooth-first
Prefer cable-free operation when the radio/TNC exposes a reliable data/control channel.

### 4. Protocols behind the interface
Users should think in terms of stations, messages, inboxes, maps and radio state.

### 5. Hardware abstraction
Common capabilities are exposed through drivers instead of hard-coding one radio model.

### 6. Progressive capability
A BLE KISS device may expose Packet/APRS only; richer radios may add CAT/PTT/configuration.

### 7. Offline-first field operation
Core RF functions and local state should not require an Internet connection.

### 8. Desktop + mobile parity where useful
The same core behavior should exist across platforms, while each platform can use native UX conventions.

## Initial module set

- APRS
- Packet / AX.25
- KISS monitor/diagnostics
- Radio control
- Winlink (after core Packet path)

## Later modules

- SSTV
- selected digital modes
- APRS-IS/iGate experiments
- additional device transports

## Initial supported device classes

- Radios with embedded BLE KISS/TNC
- Bluetooth KISS TNCs
- USB/audio interfaces
- Future RadioLink Bridge for analog/legacy radios

## Candidate first validation hardware

- BTECH UV-PRO-class embedded Bluetooth/TNC radio
- Mobilinkd-class BLE KISS TNC
- DigiRig + conventional HT as a compatibility baseline

## MVP success criteria

The first platform MVP is successful when a normal user can:

1. install RadioLink on at least one desktop and one mobile platform;
2. connect a compatible radio or TNC;
3. select APRS/Packet from a central hub;
4. receive and decode real RF traffic;
5. transmit a valid APRS beacon;
6. exchange APRS messages;
7. inspect raw KISS/AX.25 diagnostics when needed;
8. reuse the same core protocol implementation across host platforms.

## Non-goals for initial MVP

- DMR networking
- LoRa mesh
- multimode infrastructure gateways
- replacing every DigiPi application immediately
- hosting arbitrary Linux services
- supporting every radio brand from day one
