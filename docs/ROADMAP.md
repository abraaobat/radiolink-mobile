# RadioLink Platform — Roadmap Master

Status legend: `NOT STARTED` · `IN PROGRESS` · `DONE` · `BLOCKED`

## North Star

Deliver one cross-platform amateur-radio application hub for Android, iOS, Linux and macOS where users connect a compatible radio/TNC, select a module such as APRS, Packet or Winlink, and operate without requiring a Raspberry Pi/DigiPi appliance.

The smartphone/computer is the preferred computing layer. For conventional radios, the first hardware goal is to expose clean audio RX/TX + PTT to the host and validate RF with a software TNC. Embedded AFSK/AX.25 + BLE KISS is an additive cable-free mode, not a prerequisite for proving the radio interface.

---

## F0 — Platform Foundation

**Status:** IN PROGRESS

### Goal
Establish product boundaries, cross-platform architecture, repository layout and first validation path.

### Deliverables
- [x] Create public GitHub repository.
- [x] Define product vision and guardrails.
- [x] Define Bluetooth-first architecture.
- [x] Expand official targets to Android, iOS, Linux and macOS.
- [x] Adopt App Hub Architecture inspired by DigiPi's convenience.
- [x] Define rule: Bluetooth capability must not be treated as synonymous with KISS/TNC capability.
- [x] Define TNC backend abstraction: embedded TNC, external BLE KISS TNC or software TNC.
- [x] Define host-first TNC/modem principle.
- [x] Define RadioLink Bridge as physical radio interface first and embedded TNC second.
- [x] Reject raw Bluetooth audio as the default AFSK transport.
- [x] Create initial compatibility capability matrix.
- [x] Create Roadmap Master.
- [x] Record ADR-0001.
- [x] Record ADR-0002 for cross-platform App Hub architecture.
- [x] Define shared-core toolchain/language decision: **Rust stable + Cargo**.
- [x] Record ADR-0003 for Rust Shared Core.
- [x] Record ADR-0004 for host-first TNC and dual-mode Bridge.
- [x] Bootstrap Cargo workspace and initial Rust crates.
- [ ] Select first desktop reference environment (macOS or Linux).
- [ ] Freeze first conventional-radio hardware PoC: radio + codec/interface + PTT.
- [ ] Validate first software-TNC reference path with Direwolf or equivalent.
- [ ] Select first BLE KISS device for the cable-free comparison path.
- [ ] Capture Bluetooth services/characteristics for first BLE KISS reference device.
- [ ] Define KISS/AX.25/APRS packet fixtures.

### Exit criteria
A developer can clone the repository and understand the platform targets, module model, host-first TNC strategy, Bridge modes, shared-core strategy and first technical validation path.

---

## F1 — Shared Core + Device/TNC Abstraction

**Status:** IN PROGRESS

### Goal
Create the platform-neutral core and capability model used by all hosts, including a TNC backend abstraction so modules do not depend directly on one hardware path.

### Deliverables
- [x] Core project/toolchain bootstrap.
- [x] Initial `RadioCapabilities` model.
- [x] Initial `TncTransport` abstraction.
- [x] Initial crate boundaries for KISS, AX.25, APRS, TNC and drivers.
- [ ] Stable `RadioDevice` abstraction.
- [ ] Complete capability schema.
- [ ] Add explicit `hostTncCompatible` / audio/PTT capability representation.
- [ ] Connection/session model.
- [ ] Module registry model.
- [ ] Shared fixture/test harness.
- [ ] Host-facing API boundary for future FFI.
- [ ] Software-TNC backend adapter boundary.

### Exit criteria
Core logic runs in tests without Android/iOS/Linux/macOS-specific Bluetooth APIs and APRS/Packet modules can consume a generic TNC backend whether frames originate in Direwolf, a native host modem, an external BLE KISS TNC or an embedded radio TNC.

---

## F2 — Bluetooth Device Layer

**Status:** NOT STARTED

### Goal
Discover, connect and maintain reliable sessions with Bluetooth/BLE radios/TNCs while identifying the actual services/capabilities exposed by each device.

### Deliverables
- [ ] Device scan.
- [ ] Pair/connect flow.
- [ ] Reconnect behavior.
- [ ] BLE service/characteristic discovery.
- [ ] Capability detection/profile mapping.
- [ ] Raw RX/TX diagnostics.
- [ ] First BLE KISS reference driver.
- [ ] Desktop implementation first.
- [ ] Mobile adapters after desktop validation.

### Exit criteria
RadioLink can connect to the reference BLE device, exchange raw data reliably and correctly distinguish control/audio/KISS/TNC capabilities.

---

## F3 — KISS Transport Core

**Status:** IN PROGRESS

### Deliverables
- [x] Initial KISS data-frame encoder.
- [x] Reserved-byte escaping test.
- [ ] KISS frame decoder.
- [ ] Streaming parser.
- [ ] Fragmented BLE frame handling.
- [ ] Captured-frame fixtures.
- [ ] Driver-to-KISS adapter.
- [ ] TNC-backend-to-KISS adapter.
- [ ] Direwolf KISS reference fixtures.

### Exit criteria
Real KISS frames from reference hardware or a software TNC backend reach the shared protocol core deterministically.

---

## F4 — AX.25 + APRS Core

**Status:** IN PROGRESS

### Deliverables
- [x] Initial AX.25 address model.
- [x] Initial APRS position model.
- [ ] AX.25 address binary parsing/encoding.
- [ ] AX.25 UI frame parsing.
- [ ] APRS position decode/encode.
- [ ] APRS message decode/encode.
- [ ] Basic status/telemetry/text handling.
- [ ] Diagnostics preservation for unsupported packets.

### Exit criteria
Captured and live APRS RF packets decode correctly through the shared core independently of where the TNC lives.

---

## F5 — Desktop Reference App (macOS/Linux)

**Status:** NOT STARTED

### Goal
Use desktop as the first full validation environment before mobile hardening.

### Deliverables
- [ ] RadioLink desktop shell.
- [ ] Module launcher.
- [ ] Radio connection screen.
- [ ] KISS monitor.
- [ ] APRS receive view.
- [ ] Basic APRS transmit.
- [ ] Linux/macOS parity baseline.
- [ ] Validate host software-TNC path first using Direwolf + RadioLink Bridge/DigiRig-class interface.
- [ ] Validate at least one direct BLE KISS path after the host-TNC reference path is stable.

### Exit criteria
A desktop user can complete an APRS RX/TX workflow from the RadioLink hub through both a software-TNC backend and at least one KISS hardware backend.

---

## F6 — RadioLink CLI / Headless

**Status:** IN PROGRESS

### Goal
Expose the shared core on Linux/macOS through terminal workflows.

### Candidate commands
- [x] CLI crate/bootstrap.
- [x] `radiolink scan` placeholder.
- [x] `radiolink monitor` placeholder.
- [x] `radiolink aprs` placeholder.
- [x] `radiolink packet` placeholder.
- [ ] `radiolink connect`.
- [ ] `radiolink serve`.
- [ ] `radiolink capabilities`.
- [ ] Real device discovery.
- [ ] Real KISS monitor.
- [ ] Software-TNC backend selection/status.

### Exit criteria
Core radio diagnostics and Packet/APRS operations can run without the GUI.

---

## F7 — APRS Module

**Status:** NOT STARTED

### Deliverables
- [ ] Station list.
- [ ] Map.
- [ ] Last-heard state.
- [ ] Local persistence.
- [ ] Position beacon TX.
- [ ] Chat-like APRS messaging.
- [ ] ACK/REJ handling.

### Exit criteria
APRS is a coherent module available from the RadioLink hub regardless of whether frames come from a host software TNC, external BLE KISS TNC, embedded Bridge TNC or embedded radio TNC.

---

## F8 — Packet Module

**Status:** NOT STARTED

### Deliverables
- [ ] Connected AX.25 transport spike.
- [ ] Packet terminal.
- [ ] Connection/session state.
- [ ] BBS/RMS interoperability tests.
- [ ] Shared TNC backend usage.

### Exit criteria
A user can open Packet from the launcher and conduct a practical connected-mode session through a supported TNC backend.

---

## F9 — Radio Control Module

**Status:** NOT STARTED

### Deliverables
- [ ] Capability-gated frequency/channel state.
- [ ] Mode/power/PTT hooks where safely exposed.
- [ ] Unsupported-state UX.
- [ ] Shared state with APRS/Packet modules.
- [ ] Support control-only Bluetooth devices without falsely marking them as Packet-capable.

### Exit criteria
Richer radios expose control without making control mandatory for simpler KISS TNCs, and CAT-only devices are represented accurately.

---

## F10 — Android App

**Status:** NOT STARTED

### Goal
Bring the proven hub/core/device architecture to Android.

### Deliverables
- [ ] Android shell in Kotlin.
- [ ] Rust core binding.
- [ ] Bluetooth lifecycle.
- [ ] Permissions.
- [ ] Location/background policy.
- [ ] Module launcher.
- [ ] APRS/Packet parity baseline.
- [ ] Capability-aware device screen.
- [ ] Evaluate native host software-modem audio path on Android.
- [ ] Beta distribution build.

### Exit criteria
Android completes the same supported core workflows validated on desktop within mobile platform constraints.

---

## F11 — iOS App

**Status:** NOT STARTED

### Deliverables
- [ ] iOS shell in Swift.
- [ ] Rust core binding.
- [ ] CoreBluetooth adapter.
- [ ] Background/location behavior.
- [ ] Module launcher.
- [ ] APRS/Packet parity baseline.
- [ ] Capability-aware device screen.
- [ ] Evaluate native host software-modem audio path on iOS.
- [ ] TestFlight build.

### Exit criteria
iPhone completes the same supported core workflows within iOS constraints.

---

## F12 — Winlink Module

**Status:** NOT STARTED

### Deliverables
- [ ] Feasibility/architecture decision.
- [ ] Packet connection transport integration.
- [ ] Inbox/outbox/compose model.
- [ ] RMS configuration.
- [ ] TNC backend integration.
- [ ] Interoperability testing.

### Exit criteria
A supported host/radio combination completes a practical Winlink Packet exchange from the same RadioLink hub.

---

## F13 — Driver SDK + Compatibility Matrix

**Status:** NOT STARTED

### Goal
Make hardware support capability-driven and prevent “Bluetooth” from being treated as a proxy for Packet/TNC compatibility.

### Deliverables
- [ ] Stable driver interface.
- [ ] Capability schema documentation.
- [ ] Driver test harness.
- [ ] Device profile template.
- [x] Initial compatibility matrix document.
- [ ] Per-device capability flags:
  - [ ] Bluetooth.
  - [ ] CAT / radio control.
  - [ ] Audio RX.
  - [ ] Audio TX.
  - [ ] PTT.
  - [ ] Serial/data transport.
  - [ ] KISS.
  - [ ] Embedded TNC.
  - [ ] Host software-TNC compatible.
  - [ ] USB audio.
  - [ ] USB serial/CAT.
- [ ] Tested-platform matrix: Android / iOS / Linux / macOS.
- [ ] Supported-module matrix: APRS / Packet / Winlink / Radio Control / diagnostics.
- [ ] Contribution guide.

### Initial candidate device classes
- [ ] Class A — Embedded BLE KISS/TNC radio.
- [ ] Class B — External BLE KISS TNC.
- [ ] Class C — USB audio/PTT + host software TNC.
- [ ] Class D — Bluetooth CAT-only radio.
- [ ] Class E — Bluetooth audio/PTT radio requiring validation.
- [ ] Class F — RadioLink Bridge interface mode.
- [ ] Class G — RadioLink Bridge embedded-TNC/BLE-KISS mode.

### Exit criteria
A contributor can describe a radio accurately by capabilities and add support without modifying APRS/Packet core logic.

---

## F14 — USB / Audio + Host Software TNC Compatibility Layer

**Status:** IN PROGRESS

### Goal
Make the host software TNC the reference modem path for conventional radios and use it to validate hardware before implementing an embedded modem.

### Candidate paths
- [ ] RadioLink Bridge V0.x USB audio/PTT mode.
- [ ] DigiRig-class USB audio/PTT.
- [ ] Direwolf as the first desktop reference TNC for AX.25/APRS/Packet.
- [ ] External software TNC/modem adapter interface.
- [ ] Future native RadioLink host modem backend.
- [ ] Serial CAT interfaces.
- [ ] Platform-specific USB device integration.
- [ ] Capability detection that can combine CAT from one transport and TNC/audio from another.

### Validation sequence

```text
Radio
 ↓
radio interface / Bridge
 ↓ USB audio + PTT
Direwolf on macOS/Linux
 ↓ KISS/AX.25
RadioLink TncBackend
 ↓
APRS / Packet
```

### Guardrail
Bluetooth remains the preferred cable-free user experience, but host software-TNC operation is the preferred **engineering reference path** for conventional-radio bring-up. Bluetooth alone never implies Packet capability.

### Exit criteria
A conventional radio completes valid APRS RX/TX using host software-TNC processing through the generic TNC backend, with audio/PTT behavior characterized and reproducible.

---

## F15 — RadioLink Bridge for Legacy Radios

**Status:** IN PROGRESS

### Goal
Provide an optional compact radio-edge accessory for radios without smartphone/desktop-friendly data interfaces.

### Phase A — Interface mode (first)
- [ ] ESP32-S3 hardware bring-up.
- [ ] Codec/audio RX path.
- [ ] Codec/audio TX path.
- [ ] Safe radio-level conditioning.
- [ ] PTT control through transistor/opto/MOSFET interface as appropriate.
- [ ] K-plug reference interface for Quansheng-class HT.
- [ ] USB host transport.
- [ ] Validate with Direwolf/software TNC.
- [ ] Real APRS RX test.
- [ ] Real APRS TX test.

### Phase B — Embedded TNC mode (after Phase A)
- [ ] AFSK 1200 demodulator.
- [ ] AFSK 1200 modulator.
- [ ] AX.25 framing integration as needed.
- [ ] KISS interface.
- [ ] BLE KISS transport.
- [ ] Mobile cable-free interoperability test.
- [ ] Compare results against the host software-TNC reference path.

### Optional later responsibilities
- [ ] CAT/control where useful.
- [ ] Configuration service.
- [ ] Device telemetry/diagnostics.

### Guardrail
The Bridge is an accessory, not a second general-purpose computer. It must be useful as an audio/PTT radio interface even before embedded TNC firmware exists.

### Exit criteria
The same Bridge hardware can support at least one host software-TNC workflow, and the embedded-TNC path can be added without changing APRS/Packet application logic.

---

# Post-MVP modules

Only after the core hub is stable:

- SSTV.
- Selected additional digital modes.
- APRS-IS / iGate experiments.
- Richer offline maps.
- Additional packet speeds/modems.
- Integration with RadioNode-BR.
- Possible LoRa/DMR capability modules kept separate from the core MVP.
- Experimental Wi-Fi/raw-audio transports if they show practical value.

# Current execution order

Two tracks run together.

### Platform/core track

```text
F0 → F1 → F3 → F4 → F5 → F6
                       │
                       ├→ F7 APRS
                       ├→ F8 Packet
                       └→ F9 Radio Control
```

### Conventional-radio hardware/reference track

```text
F15 Phase A
    ↓
F14 host software TNC
    ↓
real APRS RX/TX reference
    ↓
F15 Phase B embedded AFSK/KISS
    ↓
F2 BLE device layer / BLE KISS
```

After desktop/core validation:

```text
F10 Android → F11 iOS → F12 Winlink
```

Then mature hardware support through F13 driver/compatibility work.

# Immediate next action

Complete the first hardware/reference decision set:

1. select **macOS or Linux** as the first desktop reference host;
2. freeze the **RadioLink Bridge V0.1** electrical path: ESP32-S3 + codec + PTT + K-plug/radio conditioning;
3. expose clean RX/TX audio and PTT to the host;
4. validate the radio path with **Direwolf as the reference software TNC**;
5. capture real KISS/AX.25/APRS fixtures from that working path;
6. connect that backend to the Rust `TncTransport` abstraction and `radiolink-cli`;
7. only then implement embedded AFSK/AX.25 and BLE KISS for the cable-free mobile mode.

Preferred first PoC:

```text
Mac/Linux ↔ software TNC ↔ USB audio/PTT ↔ RadioLink Bridge ↔ conventional radio ↔ RF
```

Target cable-free mobile mode after validation:

```text
Android/iOS ↔ BLE KISS ↔ RadioLink Bridge embedded TNC ↔ conventional radio ↔ RF
```
