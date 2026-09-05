# RadioLink Mobile — Architecture

## Architectural statement

RadioLink Mobile is a mobile-native amateur radio platform. The smartphone owns application logic, user experience, local data and protocol orchestration. Radios/TNCs expose RF and device capabilities through Bluetooth/BLE whenever possible.

```text
┌─────────────────────────────┐
│       Android / iOS         │
│                             │
│  UI / Maps / Messages       │
│  Radio Core                 │
│  Protocol Core              │
│  Driver Layer               │
└──────────────┬──────────────┘
               │ BLE / BT
┌──────────────▼──────────────┐
│ Radio / TNC / RadioLink     │
│ Bridge                      │
└──────────────┬──────────────┘
               │
               ▼
              RF
```

## Layers

### 1. Mobile Applications

`apps/android` and `apps/ios` contain platform-specific application shells and hardware integration that cannot be shared safely.

Responsibilities:
- permissions;
- Bluetooth lifecycle;
- background execution policies;
- location services;
- notifications;
- native UI integration;
- platform-specific storage/adapters.

### 2. Core

`packages/core`

Platform-neutral domain logic:
- station model;
- message model;
- radio capability model;
- connection state;
- routing/orchestration;
- local history abstractions.

### 3. Protocols

`packages/protocols`

Protocol codecs and state machines:
- KISS framing;
- AX.25 framing/parsing;
- APRS decoding/encoding;
- future Packet/Winlink transport helpers.

Protocol code should be testable without a real radio.

### 4. Drivers

`packages/drivers`

Drivers translate device-specific Bluetooth services/commands into a common capability interface.

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
│   └── telemetry
├── receiveFrames()
├── sendFrame()
└── optional radio controls
```

The application must query capabilities instead of assuming every radio supports the same operations.

### 5. UI Components

`packages/ui`

Shared design language and view models where practical. Native platform conventions remain authoritative.

### 6. Optional Hardware Bridge

`hardware/bridge`

A future small accessory for radios without native Bluetooth data interfaces.

Its role is intentionally constrained:
- Bluetooth/BLE transport;
- KISS/TNC function where needed;
- PTT;
- optional CAT/control;
- optional audio/data interface.

The bridge must not become a second general-purpose computer.

## Data flow — APRS receive

```text
RF
 ↓
Radio/TNC
 ↓ BLE KISS
Device Driver
 ↓
KISS decoder
 ↓
AX.25 parser
 ↓
APRS decoder
 ↓
Radio Core
 ↓
Station List / Map / Message UI
```

## Data flow — APRS transmit

```text
User / GPS
 ↓
APRS encoder
 ↓
AX.25 frame
 ↓
KISS frame
 ↓
Device Driver
 ↓ BLE
Radio/TNC
 ↓
RF
```

## Key architectural constraints

1. No Raspberry Pi is required for core operation.
2. DigiPi is not a runtime dependency.
3. Linux daemons are not part of the primary architecture.
4. Bluetooth/BLE is the preferred device transport.
5. USB may be added later as an alternative transport, not as the initial center of the design.
6. The protocol core must remain testable independently of Android/iOS Bluetooth APIs.
7. Device-specific behavior belongs in drivers.
8. UI must present user concepts before protocol concepts.

## Security and privacy baseline

- Do not require cloud accounts for basic RF operation.
- Keep station/message history local by default.
- Request only required mobile permissions.
- Make Internet-dependent features clearly distinguishable from RF/offline features.
- Treat any future remote-control capability as privileged and explicitly authorized.

## Interoperability strategy

The project should prefer established transports and protocols where possible:
- Bluetooth LE / Bluetooth;
- KISS;
- AX.25;
- APRS.

RadioLink-specific extensions should only be introduced when a capability cannot be represented cleanly using existing standards, and must be documented.
