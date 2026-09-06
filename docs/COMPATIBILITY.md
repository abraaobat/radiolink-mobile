# RadioLink Platform — Compatibility Matrix

## Purpose

RadioLink classifies radios and interfaces by **actual exposed capabilities**, not by marketing labels such as “Bluetooth radio”.

A Bluetooth connection alone does not imply Packet, APRS, KISS or TNC support.

## Capability checklist

Each supported device should be documented with the following independent flags:

```text
Bluetooth
[ ] CAT / radio control
[ ] Audio RX
[ ] Audio TX
[ ] PTT
[ ] Serial/data transport
[ ] KISS
[ ] Embedded TNC
[ ] USB audio
[ ] USB serial/CAT
```

## Compatibility classes

### Class A — Embedded BLE KISS/TNC

Best RadioLink experience.

```text
Host ↔ BLE KISS ↔ Radio/TNC ↔ RF
```

Typical characteristics:
- no DigiRig required;
- no software TNC required;
- APRS/Packet frames can move directly through the TNC backend.

### Class B — External BLE KISS TNC

For radios without an embedded TNC.

```text
Host ↔ BLE KISS TNC ↔ audio/PTT ↔ Radio ↔ RF
```

Typical characteristics:
- Bluetooth between host and TNC;
- TNC hardware performs modem/AX.25 framing functions;
- radio remains conventional analog RF endpoint.

### Class C — DigiRig / USB audio + software TNC

Desktop compatibility path.

```text
Host ↔ Direwolf ↔ DigiRig ↔ Radio ↔ RF
```

Typical characteristics:
- DigiRig provides audio/PTT/optional serial;
- Direwolf provides the TNC/modem function;
- ideal fallback for radios without integrated TNC.

### Class D — Bluetooth CAT only

```text
Host ↔ Bluetooth CAT ↔ Radio
```

Supports radio control only.

This class is **not** Packet/APRS capable unless a second audio/TNC path exists.

### Class E — Bluetooth audio + PTT

Potential software-TNC path.

```text
Host ↔ Bluetooth audio/PTT ↔ Radio
```

Packet/APRS viability depends on:
- usable bidirectional audio;
- codec/filter behavior;
- PTT access;
- latency;
- AGC/noise-processing behavior.

This must be validated per radio/platform and should not be assumed from the presence of Bluetooth audio alone.

## TNC backend mapping

| Hardware path | TNC location | RadioLink backend |
|---|---|---|
| Embedded BLE KISS radio | inside radio | Hardware TNC / BLE KISS |
| External BLE KISS TNC | external accessory | Hardware TNC / BLE KISS |
| DigiRig + conventional radio | host computer | Software TNC / Direwolf |
| Bluetooth CAT-only radio | none | Control only |
| Bluetooth audio/PTT radio | host computer if viable | Software TNC / experimental |

## Driver requirements

Every device profile must document:

- manufacturer;
- model;
- transport type;
- Bluetooth profile/service identifiers where known;
- CAT availability;
- audio RX/TX availability;
- PTT mechanism;
- KISS availability;
- embedded TNC availability;
- tested platforms;
- supported RadioLink modules;
- limitations/quirks.

## Rule of thumb

> **Bluetooth tells us how we may connect. TNC/KISS tells us whether Packet/APRS frames are available.**

RadioLink must never infer one from the other.
