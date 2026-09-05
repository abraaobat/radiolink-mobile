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
- [x] Create Roadmap Master.
- [x] Record ADR-0001.
- [x] Record ADR-0002 for cross-platform App Hub architecture.
- [ ] Define shared-core toolchain/language decision.
- [ ] Select first reference radio/TNC for PoC.
- [ ] Select first desktop reference environment (macOS or Linux).
- [ ] Capture Bluetooth services/characteristics for first reference device.
- [ ] Define KISS/AX.25/APRS packet fixtures.

### Exit criteria
A developer can clone the repository and understand the platform targets, module model, first hardware, shared-core strategy and first technical spike.

---

## F1 — Shared Core + Device Abstraction

**Status:** NOT STARTED

### Goal
Create the platform-neutral core and device capability model used by all hosts.

### Deliverables
- [ ] Core project/toolchain bootstrap.
- [ ] RadioDevice abstraction.
- [ ] capability schema.
- [ ] connection/session model.
- [ ] module registry model.
- [ ] test harness.

### Exit criteria
Core logic runs in tests without Android/iOS/Linux/macOS-specific Bluetooth APIs.

---

## F2 — Bluetooth Device Layer

**Status:** NOT STARTED

### Goal
Discover, connect and maintain reliable sessions with Bluetooth/BLE radios/TNCs.

### Deliverables
- [ ] Device scan.
- [ ] Pair/connect flow.
- [ ] reconnect behavior.
- [ ] BLE service/characteristic discovery.
- [ ] raw RX/TX diagnostics.
- [ ] first reference driver.
- [ ] desktop implementation first.
- [ ] mobile adapters after desktop validation.

### Exit criteria
RadioLink can connect to the reference device and exchange raw data reliably.

---

## F3 — KISS Transport Core

**Status:** NOT STARTED

### Deliverables
- [ ] KISS encoder/decoder.
- [ ] escaping/unescaping.
- [ ] streaming parser.
- [ ] fragmented BLE frame handling.
- [ ] captured-frame fixtures.
- [ ] driver-to-KISS adapter.

### Exit criteria
Real KISS frames from reference hardware reach the shared protocol core deterministically.

---

## F4 — AX.25 + APRS Core

**Status:** NOT STARTED

### Deliverables
- [ ] AX.25 address parsing.
- [ ] AX.25 UI frame parsing.
- [ ] APRS position decode/encode.
- [ ] APRS message decode/encode.
- [ ] basic status/telemetry/text handling.
- [ ] diagnostics preservation for unsupported packets.

### Exit criteria
Captured and live APRS RF packets decode correctly through the shared core.

---

## F5 — Desktop Reference App (macOS/Linux)

**Status:** NOT STARTED

### Goal
Use desktop as the first full validation environment before mobile hardening.

### Deliverables
- [ ] RadioLink desktop shell.
- [ ] module launcher.
- [ ] radio connection screen.
- [ ] KISS monitor.
- [ ] APRS receive view.
- [ ] basic APRS transmit.
- [ ] Linux/macOS parity baseline.

### Exit criteria
A desktop user can connect a Bluetooth KISS radio/TNC and complete an APRS RX/TX workflow from the RadioLink hub.

---

## F6 — RadioLink CLI / Headless

**Status:** NOT STARTED

### Goal
Expose the shared core on Linux/macOS through terminal workflows.

### Candidate commands
- [ ] `radiolink scan`
- [ ] `radiolink connect`
- [ ] `radiolink monitor`
- [ ] `radiolink aprs`
- [ ] `radiolink packet`
- [ ] `radiolink serve`

### Exit criteria
Core radio diagnostics and Packet/APRS operations can run without the GUI.

---

## F7 — APRS Module

**Status:** NOT STARTED

### Deliverables
- [ ] station list.
- [ ] map.
- [ ] last-heard state.
- [ ] local persistence.
- [ ] position beacon TX.
- [ ] chat-like APRS messaging.
- [ ] ACK/REJ handling.

### Exit criteria
APRS is a coherent module available from the RadioLink hub.

---

## F8 — Packet Module

**Status:** NOT STARTED

### Deliverables
- [ ] connected AX.25 transport spike.
- [ ] packet terminal.
- [ ] connection/session state.
- [ ] BBS/RMS interoperability tests.

### Exit criteria
A user can open Packet from the launcher and conduct a practical connected-mode session.

---

## F9 — Radio Control Module

**Status:** NOT STARTED

### Deliverables
- [ ] capability-gated frequency/channel state.
- [ ] mode/power/PTT hooks where safely exposed.
- [ ] unsupported-state UX.
- [ ] shared state with APRS/Packet modules.

### Exit criteria
Richer radios expose control without making control mandatory for simpler KISS TNCs.

---

## F10 — Android App

**Status:** NOT STARTED

### Goal
Bring the proven hub/core/device architecture to Android.

### Deliverables
- [ ] Android shell.
- [ ] Bluetooth lifecycle.
- [ ] permissions.
- [ ] location/background policy.
- [ ] module launcher.
- [ ] APRS/Packet parity baseline.
- [ ] beta distribution build.

### Exit criteria
Android completes the same core workflows validated on desktop.

---

## F11 — iOS App

**Status:** NOT STARTED

### Deliverables
- [ ] iOS shell.
- [ ] CoreBluetooth adapter.
- [ ] background/location behavior.
- [ ] module launcher.
- [ ] APRS/Packet parity baseline.
- [ ] TestFlight build.

### Exit criteria
iPhone completes the same supported core workflows within iOS constraints.

---

## F12 — Winlink Module

**Status:** NOT STARTED

### Deliverables
- [ ] feasibility/architecture decision.
- [ ] Packet connection transport integration.
- [ ] inbox/outbox/compose model.
- [ ] RMS configuration.
- [ ] interoperability testing.

### Exit criteria
A supported host/radio combination completes a practical Winlink Packet exchange from the same RadioLink hub.

---

## F13 — Driver SDK + Compatibility Matrix

**Status:** NOT STARTED

### Deliverables
- [ ] stable driver interface.
- [ ] capability schema documentation.
- [ ] driver test harness.
- [ ] device profile template.
- [ ] compatibility matrix.
- [ ] contribution guide.

### Initial candidate device classes
- [ ] embedded BLE KISS radio.
- [ ] BLE KISS TNC.
- [ ] USB/audio interface.
- [ ] RadioLink Bridge.

---

## F14 — USB / Audio Compatibility Layer

**Status:** NOT STARTED

### Goal
Support conventional radios where Bluetooth KISS is not available.

### Candidate paths
- [ ] DigiRig-class USB audio/PTT.
- [ ] external software TNC/modem adapter where needed.
- [ ] serial CAT interfaces.
- [ ] platform-specific USB device integration.

### Guardrail
Bluetooth remains the preferred path; USB/audio exists for compatibility.

---

## F15 — RadioLink Bridge for Legacy Radios

**Status:** NOT STARTED

### Goal
Provide an optional compact Bluetooth accessory for radios without smartphone/desktop-friendly data interfaces.

### Candidate responsibilities
- [ ] BLE transport.
- [ ] KISS/TNC.
- [ ] PTT.
- [ ] audio/data interface.
- [ ] optional CAT/control.

### Guardrail
The Bridge is an accessory, not a second general-purpose computer.

---

# Post-MVP modules

Only after the core hub is stable:

- SSTV.
- selected additional digital modes.
- APRS-IS / iGate experiments.
- richer offline maps.
- additional packet speeds/modems.
- integration with RadioNode-BR.
- possible LoRa/DMR capability modules kept separate from the core MVP.

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
F13 Drivers → F14 USB/audio → F15 Bridge
```

# Immediate next action

Complete F0 by selecting the shared-core toolchain, first reference desktop host and first Bluetooth KISS radio/TNC. The preferred PoC should prove the shortest architecture possible:

```text
Mac/Linux ↔ Bluetooth KISS ↔ Radio ↔ RF
```
