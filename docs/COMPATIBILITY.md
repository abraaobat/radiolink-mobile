# RadioLink Platform — Compatibility Matrix

## Purpose

RadioLink classifies radios and interfaces by **actual exposed transports and capabilities**, not by marketing labels such as “Bluetooth radio” or “USB radio”.

A connection type alone does not imply Packet, APRS, KISS, TNC, CAT, audio or satellite suitability.

Specific radios and interfaces are tracked separately in the nominal device registry:

- [`devices/REGISTRY.md`](devices/REGISTRY.md)
- [`devices/README.md`](devices/README.md)
- [`devices/profiles/`](devices/profiles/)

Being listed in the nominal registry is not a support claim.

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
[ ] Frequency read/control
[ ] Independent RX/TX frequency control / split
[ ] Mode / bandwidth control
[ ] Tone / signaling control
[ ] TX power control
[ ] Audio RX
[ ] Audio TX
[ ] PTT
[ ] Serial/data transport
[ ] KISS
[ ] Embedded TNC
[ ] Radio GPS/GNSS source exposure
[ ] Telemetry
[ ] USB audio
[ ] USB serial/CAT
[ ] Dual watch / Main-Sub monitoring
[ ] Full duplex / simultaneous RX while TX
```

Location/time used by an application are modeled separately through Context Providers. A radio may expose GPS/GNSS data, but `location` is not assumed to be owned by the radio.

For Satellite Operations, frequency control, independent RX/TX control and full-duplex behavior are separate capability facts. **Dual watch never implies full duplex.**

## Compatibility classes

### Class A — Embedded BLE KISS/TNC radio

Cable-free direct digital path.

```text
Host ↔ BLE KISS ↔ Radio/TNC ↔ RF
```

Typical characteristics:
- no DigiRig required;
- no software TNC required;
- APRS/Packet frames can move directly through the provider abstraction;
- radio-control capabilities may exist on the same or a separate Bluetooth service and must be modeled independently.

The BTECH UV-PRO is the first owned P0 reference for this class and also the first Satellite Operations radio-control reference.

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

This class is **not** Packet/APRS capable unless a second TNC/audio path exists. It may still be useful for Satellite Operations when frequency control is available and a separate receiver/TNC path supplies data/audio.

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
               ┌─ BLE → KISS / CAT / telemetry
Host / RadioLink
               └─ USB-C → GPS / KISS / Audio / CAT
```

RadioLink should compose these capabilities through the Device Registry, Capability Registry, Context Provider Registry and Transport Manager rather than forcing the device into a single transport class.

This composition is especially useful for satellite workflows, where RF/TNC, radio control, location and even downlink monitoring may come from different devices/providers.

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

## Context Provider mapping

Operational context is separate from radio transport/capability.

```text
LocationProvider
├── Host/smartphone location
├── Radio GPS/GNSS
├── USB GPS
├── Network/GPSD
└── Manual/static

TimeProvider
├── System clock
├── GPS/GNSS time
├── Network time when available
└── other validated provider
```

A workflow may therefore combine:

```text
BLE → KISS/TNC
BLE → radio control
USB → GPS
Host OS → location fallback / time
SDR → satellite downlink RX
```

without treating those sources as one physical interface.

## Satellite capability mapping

Satellite Operations adds an orchestration layer on top of existing capabilities rather than a new physical compatibility class.

A device/profile may be usable in one of several satellite roles:

| Role | Minimum relevant capabilities |
|---|---|
| Pass planning only | LocationProvider + TimeProvider; no radio required |
| Receive-only satellite radio | frequency control recommended; RX path required |
| Single-radio FM/Packet satellite | frequency control + PTT/TNC or voice path as required by profile |
| Satellite APRS/Packet | KISS/TNC + frequency control where Doppler correction is required |
| Independent uplink/downlink | split or independent RX/TX frequency control |
| Full-duplex station | explicit simultaneous RX/TX evidence, or separate RX/TX devices |
| Radio + SDR | TX radio-control path + independent SDR receive provider |
| Rotor-assisted station | Satellite Engine + future Rotator Provider |

A profile must never claim full duplex from `dualWatch=true` alone.

See [`SATELLITE-OPERATIONS.md`](SATELLITE-OPERATIONS.md).

## RadioLink Profile requirements

Every device/profile should document, where applicable:

- manufacturer;
- model and hardware revision;
- firmware version/range;
- validation state;
- tested RadioLink/host version;
- transport(s);
- Bluetooth profile/service identifiers where known;
- USB interface classes where known;
- CAT availability;
- frequency read/control;
- independent RX/TX or split capability;
- mode/bandwidth/tone/power control where applicable;
- audio RX/TX availability;
- PTT mechanism;
- KISS availability;
- embedded TNC availability;
- radio GPS/GNSS/telemetry exposure;
- dual-watch state separately from full-duplex evidence;
- cable/interface requirements;
- required radio-side settings/preflight recipe;
- audio RX/TX calibration;
- tested host platforms;
- supported/verified RadioLink services;
- Satellite Operations role/limits where applicable;
- layered diagnostic result;
- test evidence;
- limitations/quirks.

The canonical template is [`devices/PROFILE-TEMPLATE.md`](devices/PROFILE-TEMPLATE.md).

## Device validation states

Nominal devices advance through:

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

Success in one device + firmware + host + transport + provider + service combination does not automatically validate another.

Satellite validation should record the exact satellite profile/TLE epoch and whether the test was pass prediction, receive-only, Doppler control, Packet/APRS or TX-capable operation.

## Runtime rules

1. Bluetooth presence never implies KISS/TNC.
2. USB presence never implies a specific logical interface.
3. A device may expose capabilities on multiple transports.
4. Capabilities may be composed, for example CAT over BLE and KISS/audio over USB.
5. Context may come from a different source/transport than RF/TNC capability.
6. Services such as APRS/Packet/Winlink/Satellite consume provider abstractions and must not bind directly to a compatibility class.
7. Known-good Profiles supplement, but do not replace, runtime capability discovery when discovery is technically possible.
8. Firmware and required radio-side configuration are part of the capability-validation context.
9. RadioLink should report the highest verified diagnostic layer rather than collapsing every problem into a service-level error.
10. Dual watch/Main-Sub monitoring never implies full duplex.
11. Automatic Doppler control must be enabled only when the exact radio-control capability/path has been validated.
12. Satellite TX configuration must remain profile- and operator-gated; RadioLink must not infer that a technically tunable frequency is legally or operationally valid for transmission.

## Rule of thumb

> **Transport tells RadioLink how it may connect. Capabilities tell RadioLink what it can actually do. Context Providers tell services where operational data such as location/time comes from. Satellite Profiles tell the Satellite Engine how to orchestrate those capabilities for a specific spacecraft/service.**

RadioLink must never infer one from the other.
