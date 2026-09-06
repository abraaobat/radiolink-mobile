# RadioLink Platform — Roadmap Master

Status legend: `NOT STARTED` · `IN PROGRESS` · `DONE` · `BLOCKED`

## North Star

Deliver one cross-platform amateur-radio application hub for Android, iOS, Linux and macOS where users connect a compatible radio/TNC, select a module such as APRS, Packet or Winlink, and operate without requiring a Raspberry Pi/DigiPi appliance.

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
- [x] Create initial compatibility capability matrix.
- [x] Create Roadmap Master.
- [x] Record ADR-0001.
- [x] Record ADR-0002 for cross-platform App Hub architecture.
- [x] Define shared-core toolchain/language decision: **Rust stable + Cargo**.
- [x] Record ADR-0003 for Rust Shared Core.
- [x] Bootstrap Cargo workspace and initial Rust crates.
- [ ] Select first reference radio/TNC for PoC.
- [ ] Select first desktop reference environment (macOS or Linux).
- [ ] Capture Bluetooth services/characteristics for first reference device.
- [ ] Define KISS/AX.25/APRS packet fixtures.

### Exit criteria
A developer can clone the repository and understand the platform targets, module model, first hardware, shared-core strategy and first technical spike.

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
- [ ] Connection/session model.
- [ ] Module registry model.
- [ ] Shared fixture/test harness.
- [ ] Host-facing API boundary for future FFI.

### Exit criteria
Core logic runs in tests without Android/iOS/Linux/macOS-specific Bluetooth APIs and APRS/Packet modules can consume a generic TNC backend.

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
- [ ] First reference driver.
- [ ] Desktop implementation first.
- [ ] Mobile adapters after desktop validation.

### Exit criteria
RadioLink can connect to the reference device, exchange raw data reliably and correctly distinguish control/audio/KISS/TNC capabilities.

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
- [ ] Validate at least one direct BLE KISS path.
- [ ] Validate at least one software-TNC path using Direwolf + DigiRig or equivalent.

### Exit criteria
A desktop user can complete an APRS RX/TX workflow from the RadioLink hub through either a hardware BLE KISS TNC or a software TNC backend.

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
APRS is a coherent module available from the RadioLink hub regardless of whether frames come from Direwolf, an external BLE KISS TNC or an embedded radio TNC.

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
- [ ] Beta distribution build.

### Exit criteria
Android completes the same supported core workflows validated on desktop.

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
  - [ ] USB audio.
  - [ ] USB serial/CAT.
- [ ] Tested-platform matrix: Android / iOS / Linux / macOS.
- [ ] Supported-module matrix: APRS / Packet / Winlink / Radio Control / diagnostics.
- [ ] Contribution guide.

### Initial candidate device classes
- [ ] Class A — Embedded BLE KISS/TNC radio.
- [ ] Class B — External BLE KISS TNC.
- [ ] Class C — DigiRig/USB audio + software TNC.
- [ ] Class D — Bluetooth CAT-only radio.
- [ ] Class E — Bluetooth audio/PTT radio requiring validation.
- [ ] RadioLink Bridge.

### Exit criteria
A contributor can describe a radio accurately by capabilities and add support without modifying APRS/Packet core logic.

---

## F14 — USB / Audio + Software TNC Compatibility Layer

**Status:** NOT STARTED

### Goal
Support conventional radios where Bluetooth KISS/TNC is not available.

### Candidate paths
- [ ] DigiRig-class USB audio/PTT.
- [ ] Direwolf as the default desktop software TNC backend for AX.25/APRS/Packet where appropriate.
- [ ] External software TNC/modem adapter interface.
- [ ] Serial CAT interfaces.
- [ ] Platform-specific USB device integration.
- [ ] Capability detection that can combine CAT from one transport and TNC/audio from another.

### Guardrail
Bluetooth remains the preferred path; USB/audio exists for compatibility. Bluetooth alone never implies Packet capability.

---

## F15 — RadioLink Bridge for Legacy Radios

**Status:** NOT STARTED

### Goal
Provide an optional compact Bluetooth accessory for radios without smartphone/desktop-friendly data interfaces.

### Candidate responsibilities
- [ ] BLE transport.
- [ ] KISS/TNC.
- [ ] PTT.
- [ ] Audio/data interface.
- [ ] Optional CAT/control.

### Guardrail
The Bridge is an accessory, not a second general-purpose computer.

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

# Current execution order

```text
F0 → F1 → F2 → F3 → F5 → F6
                    │
                    ├→ F7 APRS
                    ├→ F8 Packet
                    └→ F9 Radio Control

After desktop/core validation:
F10 Android → F11 iOS → F12 Winlink

Then:
F13 Drivers → F14 USB/audio + software TNC → F15 Bridge
```

# Immediate next action

Complete the remaining F0 hardware decisions:

1. select **macOS or Linux** as the first reference desktop host;
2. select the first **BLE KISS radio/TNC**;
3. capture its Bluetooth services/characteristics;
4. add real KISS/AX.25/APRS fixtures;
5. connect that transport to the Rust `TncTransport` abstraction and `radiolink-cli`.

In parallel, define the Class C fallback test using a conventional radio + DigiRig + Direwolf.

Preferred shortest PoC:

```text
Mac/Linux ↔ Bluetooth KISS ↔ Radio/TNC ↔ RF
```

Compatibility fallback:

```text
Mac/Linux ↔ Direwolf ↔ DigiRig ↔ conventional radio ↔ RF
```
