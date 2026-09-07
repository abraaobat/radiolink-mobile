# RadioLink Platform — Compatibility Matrix

## Purpose

RadioLink classifies radios and interfaces by **actual exposed transports and capabilities**, not by marketing labels such as “Bluetooth radio” or “USB radio”.

A connection type alone does not imply Packet, APRS, KISS, TNC, CAT or audio support.

## Official I/O families

```text
Wireless Digital
└── Bluetooth / BLE

Wired Digital
└── USB-C / USB

Legacy / Analog Bridge
└── Audio + PTT + optional CAT
```

A single device may expose more than one family at the same time.

## Capability checklist

Each supported device/profile should document independent transport and capability flags.

```text
Transports
[ ] Bluetooth / BLE
[ ] USB
[ ] Serial
[ ] Audio
[ ] Network/IP where applicable

Capabilities
[ ] CAT / radio control
[ ] Audio RX
[ ] Audio TX
[ ] PTT
[ ] Serial/data transport
[ ] KISS
[ ] Embedded TNC
[ ] GPS
[ ] Telemetry
[ ] USB audio
[ ] USB serial/CAT
```

## Compatibility classes

### Class A — Embedded BLE KISS/TNC radio

Cable-free direct digital path.

```text
Host ↔ BLE KISS ↔ Radio/TNC ↔ RF
```

Typical characteristics:
- no DigiRig required;
- no software TNC required;
- APRS/Packet frames can move directly through the provider abstraction.

### Class B — Embedded USB KISS/TNC radio or interface

Direct wired digital path.

```text
Host ↔ USB KISS/serial ↔ Radio/TNC ↔ RF
```

Typical characteristics:
- reliable wired session;
- may combine KISS, CAT and/or USB Audio on one physical USB-C/USB connection;
- actual logical interfaces must be detected independently.

### Class C — External BLE/USB KISS TNC

For radios without an embedded TNC.

```text
Host ↔ BLE/USB KISS TNC ↔ audio/PTT ↔ Radio ↔ RF
```

Typical characteristics:
- TNC hardware performs modem/AX.25 framing functions;
- radio remains a conventional RF endpoint;
- the same logical TNC Provider can be reached over BLE or USB depending on hardware.

### Class D — DigiRig / USB Audio + software TNC

Legacy/analog compatibility path.

```text
Host ↔ software TNC ↔ DigiRig/USB audio/PTT ↔ Radio ↔ RF
```

Typical characteristics:
- DigiRig-class hardware provides audio/PTT/optional serial;
- Direwolf or another software TNC/modem provider performs modem/TNC functions;
- audio/PTT calibration may be device/profile-specific.

### Class E — Bluetooth CAT/control only

```text
Host ↔ Bluetooth CAT ↔ Radio
```

Supports control only.

This class is **not** Packet/APRS capable unless a second TNC/audio path exists.

### Class F — Bluetooth audio + PTT

Potential software-TNC path.

```text
Host ↔ Bluetooth audio/PTT ↔ Radio
```

Packet/APRS viability depends on:
- usable bidirectional audio;
- codec/filter behavior;
- PTT access;
- latency;
- AGC/noise-processing behavior;
- platform restrictions.

This must be validated per radio/platform and should not be assumed from the presence of Bluetooth audio alone.

### Class G — Multi-transport / composite device

Modern radios may expose different capabilities over different transports.

```text
               ┌─ BLE → CAT / telemetry
Host / RadioLink
               └─ USB-C → KISS / Audio / CAT
```

RadioLink should compose these capabilities through the Device Registry, Capability Registry and Transport Manager rather than forcing the device into a single transport class.

## TNC/Modem Provider mapping

| Hardware path | TNC/modem location | RadioLink provider |
|---|---|---|
| Embedded BLE KISS radio | inside radio | Hardware TNC / BLE KISS |
| Embedded USB KISS radio/interface | inside radio/interface | Hardware TNC / USB KISS |
| External BLE/USB KISS TNC | external accessory | Hardware TNC / KISS |
| DigiRig + conventional radio | host computer | Software TNC / Direwolf-class provider |
| Bluetooth CAT-only radio | none | Control only |
| Bluetooth audio/PTT radio | host if viable | Software TNC / experimental |
| RadioLink Bridge + conventional radio | Bridge | Hardware/embedded provider over BLE or USB-C |

## RadioLink Profile requirements

Every device/profile should document, where applicable:

- manufacturer;
- model;
- firmware;
- transport(s);
- Bluetooth profile/service identifiers where known;
- USB interface classes where known;
- CAT availability;
- audio RX/TX availability;
- PTT mechanism;
- KISS availability;
- embedded TNC availability;
- GPS/telemetry availability;
- cable/interface requirements;
- audio RX/TX calibration;
- tested host platforms;
- supported RadioLink services;
- verified workflows;
- limitations/quirks.

## Runtime rules

1. Bluetooth presence never implies KISS/TNC.
2. USB presence never implies a specific logical interface.
3. A device may expose capabilities on multiple transports.
4. Capabilities may be composed, for example CAT over BLE and KISS/audio over USB.
5. Services such as APRS/Packet/Winlink consume provider abstractions and must not bind directly to a compatibility class.
6. Known-good Profiles supplement, but do not replace, runtime capability discovery when discovery is technically possible.

## Rule of thumb

> **Transport tells RadioLink how it may connect. Capabilities tell RadioLink what it can actually do.**

RadioLink must never infer one from the other.
