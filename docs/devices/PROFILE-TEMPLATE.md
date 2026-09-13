# RadioLink Device Profile Template

> Copy this file into `docs/devices/profiles/<vendor>-<model>.md` and keep unknown facts explicitly `UNVERIFIED` rather than guessing.

## Identity

- **Manufacturer:**
- **Model:**
- **Hardware revision:** `UNVERIFIED`
- **Firmware:** `UNVERIFIED`
- **Registry priority:** P0 / P1 / P2 / WATCH
- **Validation state:** RESEARCHED / CANDIDATE / LAB_AVAILABLE / CONNECTED / RX_VERIFIED / TX_VERIFIED / PROFILE_VERIFIED / SUPPORTED

## Intended RadioLink role

Describe why this device is in the registry and which architectural path it should validate.

## Host/test context

- **Host:**
- **OS/version:**
- **RadioLink commit/version:**
- **Test date:**

## Physical interface

- **Interface/adapter:** `UNVERIFIED`
- **Cable:** `UNVERIFIED`
- **Connector:** `UNVERIFIED`
- **Power considerations:** `UNVERIFIED`

## Transport observations

| Transport | Present | Connected | Notes |
|---|---|---|---|
| Bluetooth/BLE | UNVERIFIED | UNVERIFIED | |
| USB | UNVERIFIED | UNVERIFIED | |
| Serial | UNVERIFIED | UNVERIFIED | |
| Audio | UNVERIFIED | UNVERIFIED | |
| Network/IP | UNVERIFIED | UNVERIFIED | |

## Capability observations

| Capability | State | Provider/path | Notes |
|---|---|---|---|
| CAT/radio control | UNVERIFIED | | |
| Frequency read/control | UNVERIFIED | | |
| Independent RX/TX / split | UNVERIFIED | | |
| Mode/bandwidth control | UNVERIFIED | | |
| Tone/signaling control | UNVERIFIED | | |
| TX power control | UNVERIFIED | | |
| Audio RX | UNVERIFIED | | |
| Audio TX | UNVERIFIED | | |
| PTT | UNVERIFIED | | |
| Serial/data | UNVERIFIED | | |
| KISS | UNVERIFIED | | |
| Embedded TNC | UNVERIFIED | | |
| GPS/GNSS | UNVERIFIED | | |
| Telemetry | UNVERIFIED | | |
| Dual watch / Main-Sub | UNVERIFIED | | Do not infer full duplex. |
| Full duplex / simultaneous RX while TX | UNVERIFIED | | Requires explicit bench evidence. |

Do not infer capabilities from the presence of Bluetooth or USB. Do not infer full duplex from dual watch, Main/Sub display or two programmable channels.

## Context providers

Record context separately from radio capabilities.

| Context | Provider | State | Notes |
|---|---|---|---|
| Location | Host / radio / USB GPS / GPSD / manual / other | UNVERIFIED | |
| Time | system / GNSS / network / other | UNVERIFIED | |

## Satellite Operations observations

Complete this section only when the device is being evaluated for satellite workflows.

| Satellite capability / role | State | Notes |
|---|---|---|
| Satellite mode/profile support | UNVERIFIED | |
| Pass-data display/control | UNVERIFIED | |
| Doppler tuning control path | UNVERIFIED | |
| Independent uplink/downlink correction | UNVERIFIED | |
| Satellite Packet/APRS path | UNVERIFIED | |
| Receive-only pass validation | UNVERIFIED | |
| Full-duplex satellite operation | UNVERIFIED | Must not be inferred from dual watch. |
| Two-radio / SDR composition | UNVERIFIED | Usually an Operations Engine composition rather than one-device capability. |

When recording a satellite test, capture the satellite profile, TLE epoch/source, observer context provider, radio firmware, host, transport, control path and whether TX was used.

## Required radio-side settings / preflight recipe

Record every setting that materially affects reproducibility. Examples may include mode, squelch, volume, VOX/PTT, data mode or menu settings, but only include values actually validated for this device.

- `UNVERIFIED`

## Audio/PTT calibration

When applicable:

- **RX level:** `UNVERIFIED`
- **TX level:** `UNVERIFIED`
- **PTT method:** `UNVERIFIED`
- **Observed clipping/noise/AGC behavior:** `UNVERIFIED`

## Service validation

| Service/workflow | RX | TX | Test evidence | Notes |
|---|---|---|---|---|
| APRS | UNVERIFIED | UNVERIFIED | | |
| Packet/AX.25 | UNVERIFIED | UNVERIFIED | | |
| Winlink | UNVERIFIED | UNVERIFIED | | |
| Radio control | UNVERIFIED | N/A | | |
| Satellite pass planning | UNVERIFIED | N/A | | |
| Satellite Doppler control | UNVERIFIED | UNVERIFIED | | |
| Satellite Packet/APRS | UNVERIFIED | UNVERIFIED | | |
| Diagnostics | UNVERIFIED | N/A | | |

## Layered diagnostics result

Record the highest verified stage:

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

- **Highest verified stage:** `UNVERIFIED`
- **Failure reason:** none / `UNVERIFIED`

## Evidence

- captured logs:
- packet/frame fixtures:
- screenshots/photos if useful:
- research source links:
- related evidence IDs (`EV-*`):
- satellite profile/TLE epoch where applicable:

## Known limitations / quirks

- `UNVERIFIED`

## Support statement

This profile is **not supported** until its state reaches `SUPPORTED` for an explicitly stated combination of device + firmware + host + transport + provider + service.
