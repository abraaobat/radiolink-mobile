# RadioLink Platform — Satellite Operations

## Purpose

Satellite Operations is a planned RadioLink service that combines orbital prediction, radio control, context providers, Packet/APRS and operational logging into one workflow.

The first hardware reference is the **BTECH UV-PRO**, because it is available in the project lab and its manufacturer-documented app/firmware already demonstrates useful satellite concepts such as pass data, azimuth/elevation, satellite range/countdown, satellite mode and automatic radio-side frequency behavior.

The Satellite service must **not** depend on the UV-PRO. The UV-PRO is the first validation target; the architecture remains capability-driven so other BLE, USB, CAT, SDR and RadioLink Bridge devices can be used later.

## Product goal

The operator should be able to choose a satellite/pass and let RadioLink prepare the operational context:

```text
Satellite / pass
      ↓
Satellite Engine
      ├── Orbit/TLE Provider
      ├── LocationProvider
      ├── TimeProvider
      ├── Pass Predictor
      ├── Doppler Engine
      ├── Satellite Profile
      └── Operations Engine
                 │
        ┌────────┼────────┐
        ↓        ↓        ↓
   Radio Control KISS   Logging
        │        │
        └────┬───┘
             ↓
          Radio/TNC
             ↓
             RF
```

## Scope

### Core satellite data

- satellite identity/name;
- NORAD catalog identifier where applicable;
- TLE import/storage;
- TLE epoch and freshness state;
- orbit propagation using a standards-based TLE model such as SGP4;
- observer position from the RadioLink Context Provider system;
- AOS/LOS prediction;
- azimuth/elevation;
- maximum elevation;
- range/distance;
- radial velocity;
- pass countdown/state;
- optional illumination/visibility metadata later.

### Pass Planner

The planner should support:

- upcoming-pass list;
- AOS/LOS and duration;
- maximum elevation;
- filtering by satellite/profile/mode;
- filtering by minimum elevation;
- favorites;
- local/offline cached TLE data;
- explicit stale-TLE warning;
- optional online TLE update when Internet is available.

Core pass prediction must remain usable offline once current orbital data is available.

### Live Pass view

Candidate live fields:

```text
Satellite: <name>
AOS:       <time>
LOS:       <time>
AZ:        <degrees>
EL:        <degrees>
Range:     <distance>
RX:        <corrected frequency>
TX:        <corrected frequency>
Doppler:   AUTO / HOLD / MANUAL
Profile:   <satellite operating profile>
```

The UI should add a sky plot and a simple antenna-pointing view for portable operation.

## Satellite Profile

Satellite-specific operating data belongs in a data/profile model, not hard-coded application logic.

Candidate fields:

```text
SatelliteProfile
├── identity / NORAD id
├── TLE source / epoch
├── service type
│   ├── FM voice
│   ├── APRS / Packet
│   ├── telemetry
│   └── other supported mode
├── uplink frequency/frequencies
├── downlink frequency/frequencies
├── tone / signaling where required
├── channel bandwidth / mode
├── TX power policy/default
├── Doppler strategy
├── Packet/APRS path/preset where applicable
├── radio capability requirements
└── notes / validation evidence
```

ARISS/ISS workflows may be supplied as presets, but frequencies, paths and operational parameters must remain updateable data rather than permanent constants in code.

## Doppler Engine

The Doppler Engine is a shared RadioLink component, independent from one radio model.

```text
TLE + observer location + time
             ↓
       orbit propagation
             ↓
       radial velocity
             ↓
       Doppler correction
             ↓
      Radio Control API
             ↓
         Radio Driver
```

The engine must support separate uplink and downlink correction when the device/profile exposes independent RX/TX frequency control.

Candidate control states:

- `AUTO` — RadioLink applies corrections continuously at an appropriate cadence;
- `HOLD` — freeze current correction;
- `MANUAL` — operator controls tuning;
- `UNAVAILABLE` — device cannot expose the required control capability.

The correction cadence must respect device/transport rate limits and avoid unnecessary write traffic.

## Radio capability requirements

Satellite support requires capabilities to be modeled independently.

Relevant capabilities include:

- frequency read/control;
- independent RX/TX frequency control or split operation;
- mode/bandwidth control where exposed;
- tone/signaling control where exposed;
- TX power control where exposed;
- PTT where appropriate;
- KISS/TNC for Packet/APRS satellite workflows;
- GPS/GNSS/context exposure where available;
- telemetry/state;
- full-duplex capability as a separate explicit fact.

### Guardrail: Dual Watch is not Full Duplex

A device advertising two channels, dual watch or Main/Sub monitoring must **not** be marked full duplex unless bench evidence proves that it can receive the satellite downlink while transmitting the uplink in the required configuration.

RadioLink should model at least:

```text
satelliteFrequencyControl
independentRxTx
simultaneousRxTx / fullDuplex
kissTnc
radioControl
```

as separate capability facts.

## Operating configurations

### Mode A — Single radio

```text
RadioLink ↔ Radio/TNC ↔ RF
```

Suitable for half-duplex or alternating RX/TX workflows. The first UV-PRO validation belongs here unless bench testing proves stronger capability.

### Mode B — Two radios

```text
             ┌→ RX radio
RadioLink ───┤
             └→ TX radio
```

Allows independent uplink/downlink control and is a candidate path for true full-duplex satellite operation.

### Mode C — Radio + SDR

```text
SDR → downlink RX → RadioLink
Radio → uplink TX → RF
```

Candidate future mode for monitoring the downlink while transmitting through a separate radio.

### Mode D — Rotor-assisted station

Future extension:

```text
Pass Predictor → pointing target → Rotator Provider
```

Rotator support is not part of the initial satellite milestone.

## Packet / APRS satellite operation

Satellite Packet/APRS reuses existing RadioLink abstractions:

```text
Satellite Service
      ↓
APRS / Packet
      ↓
AX.25
      ↓
TNC Provider / KISS
      ↓
Transport
      ↓
Radio/TNC
```

Candidate functions:

- satellite-specific APRS/Packet preset;
- KISS TNC selection;
- automatic radio preparation from the satellite profile;
- live packet monitor during the pass;
- APRS messaging where supported by the satellite service;
- received/transmitted packet counters;
- pass-linked Packet/APRS log.

The Satellite service orchestrates existing APRS/Packet components; it must not duplicate the AX.25/KISS core.

## Logging

A pass log may include:

- satellite/profile;
- TLE epoch used;
- observer location/provider;
- AOS/LOS;
- maximum elevation;
- radio/device/profile;
- RX/TX frequency history or correction summary;
- packets RX/TX;
- QSO/contact records;
- operator notes;
- optional audio recording reference;
- optional telemetry captures.

Export formats can be defined later. Logging should preserve enough context to reproduce or diagnose a pass.

## UV-PRO reference role

The BTECH UV-PRO is the first lab target for the combined F2/F9/Satellite path because official BTECH documentation currently describes:

- Bluetooth KISS TNC operation;
- APRS/Packet workflows;
- satellite mode;
- satellite display data including azimuth/elevation/range/pass countdown in current firmware history;
- independent KISS TX/RX frequency configuration in current firmware history;
- app-driven radio configuration/control.

RadioLink still needs bench validation of the exact unit, firmware, Bluetooth services/characteristics, command/control path and operational limits before these become RadioLink support claims.

## Dependencies

Satellite Operations builds on existing RadioLink work rather than bypassing it:

```text
F1  Core / Operations / Context Providers
F2  Bluetooth Device Layer
F3  KISS
F4  AX.25 / APRS
F7  APRS service
F9  Radio Control
F13 Profiles / Compatibility
            ↓
F18 Satellite Operations
```

Basic pass prediction can be implemented/tested before every radio dependency is complete, but automatic on-air operation requires validated Radio Control capabilities.

## Roadmap — F18 Satellite Operations

### F18.1 — Orbit Core + Pass Prediction

- [ ] TLE model/import/persistence.
- [ ] TLE epoch/freshness tracking.
- [ ] SGP4-class orbit propagation boundary.
- [ ] observer position through `LocationProvider`.
- [ ] time through `TimeProvider`.
- [ ] AOS/LOS/max-elevation/range prediction.
- [ ] deterministic tests using known TLE/pass fixtures.

### F18.2 — Doppler + Radio Control

- [ ] Doppler calculation from radial velocity.
- [ ] separate uplink/downlink correction.
- [ ] radio-control capability contract for satellite tuning.
- [ ] rate-limited tuning scheduler.
- [ ] AUTO/HOLD/MANUAL states.
- [ ] UV-PRO reference-driver validation.

### F18.3 — Satellite UX

- [ ] satellite list/favorites.
- [ ] pass planner.
- [ ] live pass screen.
- [ ] sky plot.
- [ ] antenna-pointing aid.
- [ ] stale-TLE warning/update UX.

### F18.4 — Satellite Packet/APRS

- [ ] satellite profile integration with KISS/AX.25/APRS.
- [ ] ARISS/ISS-style configurable preset support.
- [ ] live Packet/APRS monitor tied to a pass.
- [ ] messaging workflow where supported.
- [ ] pass-linked RX/TX statistics.

### F18.5 — Pass/QSO Logging

- [ ] pass history.
- [ ] QSO/contact records.
- [ ] radio/profile/firmware evidence.
- [ ] optional audio reference.
- [ ] export model study.

### F18.6 — Full-Duplex Expansion

- [ ] explicit full-duplex capability model.
- [ ] two-radio orchestration.
- [ ] radio + SDR architecture.
- [ ] downlink-monitoring UX.

### F18.7 — Advanced Ground-Station Integration

- [ ] rotator-provider abstraction.
- [ ] telemetry-decoder plugin/provider concept.
- [ ] automated pointing integration.
- [ ] richer satellite telemetry/logging.

## Initial exit criterion

The first Satellite milestone is successful when RadioLink can, on macOS with the lab UV-PRO:

1. load a current TLE;
2. calculate a pass from a selected `LocationProvider`;
3. display AOS/LOS/azimuth/elevation/range in real time;
4. calculate uplink/downlink Doppler correction;
5. control the validated UV-PRO frequency path at an appropriate cadence;
6. complete at least one receive-oriented pass test with a reproducible log;
7. demonstrate Packet/APRS satellite orchestration if the selected satellite/profile supports it.

Transmit validation must remain capability-, licensing- and profile-gated; RadioLink must not silently infer a legal or technically valid TX configuration.

## References

Primary manufacturer references for the initial UV-PRO benchmark:

- BTECH UV-PRO firmware changelog: https://baofengtech.com/uv-pro-firmware-changelog/
- BTECH UV Programmer app manual: https://baofengtech.com/wp-content/uploads/2025/11/BTECH-APP-Updated-Manual-2025-9-9.pdf
- BTECH UV-PRO product/support page: https://baofengtech.com/product/uv-pro/

These sources establish research targets; RadioLink support remains dependent on project bench evidence.