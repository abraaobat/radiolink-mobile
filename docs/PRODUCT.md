# RadioLink Mobile — Product Definition

## Vision

Create a clean, open-source mobile platform that turns an Android or iPhone into the main computing and interaction layer for amateur radio operation.

## Core premise

- **Smartphone = computer and UI**
- **Radio = RF endpoint**
- **Bluetooth/BLE = preferred bridge**
- **External hardware = optional compatibility layer, not the center of the system**

## Target user experience

The operator should interact with familiar mobile concepts: connected device, map, contacts, messages, radio status and simple controls. Protocol details such as AX.25, KISS and AFSK should remain available for advanced diagnostics, but should not dominate the normal UX.

## Primary use cases

1. Connect a supported radio/TNC over Bluetooth/BLE.
2. Receive APRS traffic and display nearby stations.
3. Send APRS position beacons.
4. Exchange APRS messages in a chat-like interface.
5. Control basic radio parameters when the hardware protocol allows it.
6. Use Packet/Winlink from the same mobile platform in a later phase.
7. Connect legacy radios through an optional RadioLink Bridge.

## Product principles

### 1. Mobile-native
Do not reproduce a desktop/Linux UI on a phone.

### 2. Bluetooth-first
Prefer cable-free operation whenever the radio/TNC can provide a reliable data/control channel.

### 3. Protocols behind the interface
Users should think in terms of stations, messages and radio state — not daemon ports and Linux configuration.

### 4. Hardware abstraction
The app must expose common capabilities through drivers instead of hard-coding one radio model throughout the product.

### 5. Progressive capability
A simple BLE KISS radio may expose APRS only; a richer device may add CAT/PTT/configuration. The app should adapt to capabilities.

### 6. Offline-first field operation
Core RF functions, maps/cache where practical, message history and device configuration should not require Internet access.

## Initial supported device classes

- Radios with embedded BLE KISS/TNC
- Bluetooth KISS TNCs
- Future RadioLink Bridge for analog/legacy radios

## Candidate first validation hardware

- BTECH UV-PRO as a high-value reference device because of its integrated Bluetooth/TNC concept
- Mobilinkd-class BLE KISS TNCs as an interoperability reference

Candidate devices are validation targets, not permanent architectural dependencies.

## MVP success criteria

The MVP is successful when a normal user can:

1. install the app;
2. pair/connect a compatible radio or TNC;
3. see connection state;
4. receive APRS packets;
5. see decoded stations on a list/map;
6. transmit a valid APRS beacon;
7. send and receive APRS messages;
8. do this without Raspberry Pi, DigiPi or an external Linux computer.

## Non-goals for MVP

- DMR networking
- LoRa mesh
- multimode infrastructure gateways
- replacing DigiPi feature-for-feature
- hosting arbitrary Linux services on the phone
- supporting every radio brand from day one
