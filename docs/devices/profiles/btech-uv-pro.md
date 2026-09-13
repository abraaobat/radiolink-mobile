# RadioLink Device Profile — BTECH UV-PRO

## Identity

- **Manufacturer:** BTECH
- **Model:** UV-PRO
- **Hardware revision:** `UNVERIFIED`
- **Firmware:** `UNVERIFIED` on the lab unit
- **Registry priority:** P0
- **Validation state:** `LAB_AVAILABLE`

## Intended RadioLink role

First owned direct modern-radio reference for:

- BLE/Bluetooth discovery and connection;
- embedded KISS/TNC interoperability;
- Radio Control / device-driver work;
- firmware-aware capability profiling;
- Satellite Operations reference validation.

Manufacturer documentation describes Bluetooth KISS TNC, APRS/Packet, app-driven radio control and satellite-mode functionality. These are research/documentation facts only until the exact lab unit/firmware is bench-verified by RadioLink.

## Host/test context

- **Host:** macOS reference host
- **OS/version:** `UNVERIFIED`
- **RadioLink commit/version:** `UNVERIFIED`
- **Test date:** `UNVERIFIED`

## Physical interface

- **Interface/adapter:** native radio Bluetooth/BLE path; exact services `UNVERIFIED`
- **Cable:** none expected for the host-to-radio wireless digital path
- **Connector:** N/A for BLE validation
- **Power considerations:** radio battery / USB-C charging per manufacturer; bench behavior `UNVERIFIED`

## Transport observations

| Transport | Present | Connected | Notes |
|---|---|---|---|
| Bluetooth/BLE | DOCUMENTED | UNVERIFIED | Manufacturer documents Bluetooth KISS and app connectivity; exact services/characteristics still need capture. |
| USB | DOCUMENTED FOR CHARGING / OTHER DATA UNVERIFIED | UNVERIFIED | Do not infer KISS/serial over USB without bench evidence. |
| Serial | UNVERIFIED | UNVERIFIED | |
| Audio | DOCUMENTED BLUETOOTH HFP | UNVERIFIED | Not automatically a software-TNC path. |
| Network/IP | UNVERIFIED | UNVERIFIED | |

## Capability observations

| Capability | State | Provider/path | Notes |
|---|---|---|---|
| CAT/radio control | DOCUMENTED / NOT BENCH-VERIFIED | official app Bluetooth path | Exact protocol/commands to be mapped; HTCommander is a reference. |
| Audio RX | DOCUMENTED HFP / NOT BENCH-VERIFIED | Bluetooth audio | Do not infer modem suitability. |
| Audio TX | DOCUMENTED HFP / NOT BENCH-VERIFIED | Bluetooth audio | Do not infer modem suitability. |
| PTT | DOCUMENTED IN ACCESSORY/APP ECOSYSTEM / NOT BENCH-VERIFIED | Bluetooth/control | Exact RadioLink-accessible method unknown. |
| Serial/data | DOCUMENTED DATA MODES / NOT BENCH-VERIFIED | Bluetooth | Exact GATT/service representation to capture. |
| KISS | DOCUMENTED | Bluetooth KISS | Official firmware supports KISS TNC; RadioLink bench test pending. |
| Embedded TNC | DOCUMENTED | radio | Official BTECH documentation describes built-in Bluetooth KISS TNC. |
| GPS/GNSS | DOCUMENTED | radio | Exposure to RadioLink requires validation. |
| Telemetry | DOCUMENTED/LIKELY VIA APP | UNVERIFIED | Battery/radio state exposure must be mapped explicitly. |

## Satellite capability observations

| Capability | State | Notes |
|---|---|---|
| Satellite mode | DOCUMENTED | Official firmware history includes Satellite Mode. |
| Pass data display | DOCUMENTED | Official firmware history includes satellite name, azimuth, elevation, orbital altitude, range and pass countdown. |
| Frequency control | DOCUMENTED / NOT BENCH-VERIFIED | App/radio control exists; exact RadioLink command path must be mapped. |
| Independent RX/TX frequency control | DOCUMENTED FOR KISS PROFILE / NOT BENCH-VERIFIED | Firmware changelog documents independent TX/RX frequency configuration in KISS mode. |
| Automatic Doppler control path | RESEARCH TARGET | Official app is the behavioral reference; RadioLink implementation and device-control interface need bench validation. |
| Dual Watch | DOCUMENTED ON RADIO FAMILY / NOT A FULL-DUPLEX CLAIM | Must remain separate from simultaneous RX/TX capability. |
| Full duplex / simultaneous RX while TX | `UNVERIFIED` | Never infer from dual watch or Main/Sub display. |

## Context providers

| Context | Provider | State | Notes |
|---|---|---|---|
| Location | host or radio GPS/GNSS | DOCUMENTED SOURCES / RADIO EXPOSURE UNVERIFIED | Satellite engine should resolve through `LocationProvider`. |
| Time | system / GNSS | UNVERIFIED | Satellite prediction uses shared `TimeProvider`. |

## Required radio-side settings / preflight recipe

- exact firmware version: `UNVERIFIED`
- KISS TNC enablement/settings: `UNVERIFIED` on lab unit
- pairing procedure: `UNVERIFIED` in RadioLink
- satellite/control preparation: `UNVERIFIED`

## Audio/PTT calibration

Not required for the preferred embedded BLE KISS path.

- **RX level:** N/A for KISS path
- **TX level:** N/A for KISS path
- **PTT method:** embedded TNC/control path `UNVERIFIED`
- **Observed clipping/noise/AGC behavior:** `UNVERIFIED`

## Service validation

| Service/workflow | RX | TX | Test evidence | Notes |
|---|---|---|---|---|
| APRS | UNVERIFIED | UNVERIFIED | | KISS path first target. |
| Packet/AX.25 | UNVERIFIED | UNVERIFIED | | |
| Winlink Packet | UNVERIFIED | UNVERIFIED | | Depends on connected AX.25 support in RadioLink. |
| Radio control | UNVERIFIED | N/A | | F2/F9 reference. |
| Satellite pass planning | SOFTWARE PENDING | N/A | | F18.1. |
| Satellite Doppler tuning | UNVERIFIED | UNVERIFIED | | F18.2 reference target. |
| Satellite Packet/APRS | UNVERIFIED | UNVERIFIED | | F18.4 target. |
| Diagnostics | UNVERIFIED | N/A | | First milestone is service/characteristic capture. |

## Layered diagnostics result

```text
physical/power
  ↓
transport enumeration / pairing
  ↓
logical interfaces
  ↓
capability match
  ↓
provider/TNC/context handshake
  ↓
protocol traffic
  ↓
service readiness
```

- **Highest verified stage:** physical possession / `LAB_AVAILABLE`
- **Failure reason:** none; bench integration not started

## Immediate validation plan

1. record exact radio firmware/version;
2. run real macOS Bluetooth discovery from `radiolink-cli`;
3. capture advertised services and characteristics;
4. compare control/protocol observations with HTCommander and official documentation;
5. enable and validate KISS mode;
6. receive raw KISS/AX.25 data;
7. validate controlled TX;
8. map frequency-control capabilities required by Satellite Operations;
9. run receive-oriented satellite pass validation before any advanced TX workflow.

## Evidence / research sources

- BTECH UV-PRO firmware changelog: https://baofengtech.com/uv-pro-firmware-changelog/
- BTECH UV Programmer app manual: https://baofengtech.com/wp-content/uploads/2025/11/BTECH-APP-Updated-Manual-2025-9-9.pdf
- BTECH UV-PRO product/support page: https://baofengtech.com/product/uv-pro/
- HTCommander: https://github.com/Ylianst/HTCommander

## Known limitations / quirks

- exact firmware and hardware revision of the lab unit are not yet recorded;
- official app behavior does not automatically define an open RadioLink control protocol;
- KISS mode and official app connectivity may be mutually exclusive in some firmware/configurations; validate current firmware behavior;
- Bluetooth audio must not be treated as a Packet/TNC path unless separately validated;
- dual watch is not evidence of full duplex.

## Support statement

This profile is **not supported** yet. Current state is `LAB_AVAILABLE`; all RadioLink capability claims require evidence for the exact device + firmware + host + transport + provider + service combination.