# RadioLink Platform — Product Definition

## Vision

Create a clean, open-source amateur-radio platform that turns a smartphone or computer into the main computing and interaction layer for digital radio operation.

## Core premise

- **Android / iOS / Linux / macOS = supported hosts**
- **Host device = main computer and preferred protocol/modem engine**
- **Radio = RF endpoint**
- **Bluetooth/BLE = preferred bridge when reliable data/KISS is available**
- **USB/audio = reference compatibility and development path**
- **External hardware = physical radio interface first, embedded TNC second**
- **One hub = multiple radio applications/modules**

## Product idea

RadioLink preserves the best usability idea behind DigiPi: several radio applications available from one easy launcher. The difference is that RadioLink runs directly on the user's existing phone or computer instead of requiring a Raspberry Pi appliance.

The system should exploit the processing power already present on the host. A dedicated ESP32 accessory is not required because a smartphone lacks computing power; it is useful because conventional radios still need a clean physical interface for audio, PTT, control and a cable-free transport option.

## Host-first processing principle

Whenever practical, protocol and modem processing should run on the smartphone or computer:

```text
Radio
  │
audio RX/TX + PTT
  │
RadioLink Bridge / DigiRig-class interface
  │
USB or another reliable transport
  │
Host
├── software modem / TNC
├── AX.25
├── APRS / Packet / Winlink
├── maps / messages
├── GPS
└── UI
```

For fully cable-free operation, the RadioLink Bridge may additionally run an embedded modem/TNC and expose KISS over BLE:

```text
Radio
  │
audio RX/TX + PTT
  │
RadioLink Bridge
├── AFSK
├── AX.25
└── KISS
  │
BLE
  │
Host / RadioLink App
```

These are complementary modes, not competing products.

## Target user experience

The operator opens RadioLink and chooses a module such as APRS, Packet, Winlink, KISS diagnostics or Radio Control. The same connected radio/TNC and shared station/message state are reused across modules.

Protocol details such as AX.25, KISS and AFSK remain available for diagnostics, but do not dominate the normal UX.

## Primary use cases

1. Connect a supported radio/TNC over Bluetooth/BLE.
2. Connect a conventional radio through audio/PTT and run a software TNC on the host.
3. Select an operating module from a central launcher.
4. Receive APRS traffic and display nearby stations.
5. Send APRS position beacons and messages.
6. Use Packet/AX.25 tools.
7. Use Winlink from the same platform.
8. Control basic radio parameters when hardware supports it.
9. Run equivalent core functionality on Android, iOS, Linux and macOS.
10. Use Linux/macOS in GUI or CLI/headless modes where practical.
11. Use a RadioLink Bridge with legacy radios either as a simple radio interface or, later, as an embedded BLE KISS TNC.

## Product principles

### 1. Host-first computing
Use the smartphone/computer for protocol, modem, application and UX work whenever the transport permits it reliably.

### 2. Cross-platform core
Protocol and domain logic should be shared whenever technically practical.

### 3. App Hub Architecture
The product is a launcher plus modules, not a collection of unrelated programs.

### 4. Bluetooth-first UX, reliability-first transport
Prefer cable-free operation when the radio/TNC exposes a reliable data/control channel. Do not force raw Bluetooth audio into packet-radio workflows when USB audio or KISS is more deterministic.

### 5. Protocols behind the interface
Users should think in terms of stations, messages, inboxes, maps and radio state.

### 6. Hardware abstraction
Common capabilities are exposed through drivers instead of hard-coding one radio model.

### 7. Progressive capability
A BLE KISS device may expose Packet/APRS only; richer radios may add CAT/PTT/configuration. A RadioLink Bridge can begin as audio/PTT hardware and later add embedded TNC mode.

### 8. Offline-first field operation
Core RF functions and local state should not require an Internet connection.

### 9. Desktop + mobile parity where useful
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
- USB/audio interfaces with software TNC
- RadioLink Bridge in host-TNC interface mode
- Future RadioLink Bridge embedded-TNC/BLE-KISS mode

## Candidate first validation hardware

- Conventional HT + audio/PTT interface + Direwolf on macOS/Linux as the reference modem path
- BTECH UV-PRO-class embedded Bluetooth/TNC radio
- Mobilinkd-class BLE KISS TNC
- DigiRig + conventional HT as a compatibility baseline

## RadioLink Bridge product boundary

The Bridge is not a second general-purpose computer. Its job is to solve the radio edge:

- audio RX;
- audio TX;
- PTT;
- optional CAT/control;
- transport to the host;
- optional embedded AFSK/AX.25/KISS for mobile cable-free operation.

The first hardware validation should prove audio/PTT and host software-TNC operation before embedded modem work is treated as required.

## MVP success criteria

The first platform MVP is successful when a normal user can:

1. install RadioLink on at least one desktop and one mobile platform;
2. connect a compatible radio or TNC;
3. select APRS/Packet from a central hub;
4. receive and decode real RF traffic;
5. transmit a valid APRS beacon;
6. exchange APRS messages;
7. inspect raw KISS/AX.25 diagnostics when needed;
8. reuse the same core protocol implementation across host platforms;
9. validate at least one conventional-radio path where the host performs the TNC/modem work;
10. validate at least one cable-free KISS path.

## Non-goals for initial MVP

- DMR networking
- LoRa mesh
- multimode infrastructure gateways
- replacing every DigiPi application immediately
- hosting arbitrary Linux services
- supporting every radio brand from day one
- streaming raw packet-radio audio over Bluetooth as the primary architecture
