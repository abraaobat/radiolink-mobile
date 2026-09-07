# RadioLink Platform — Roadmap Master

Status legend: `NOT STARTED` · `IN PROGRESS` · `DONE` · `BLOCKED`

## North Star

Deliver one cross-platform amateur-radio platform for Android, iOS, Linux and macOS where users connect a compatible radio/TNC/interface, choose an operational goal such as messaging, position, Packet or Winlink, and operate without requiring a Raspberry Pi/DigiPi appliance.

RadioLink must support three official I/O paths without coupling services to any one of them:

1. **Wireless Digital** — Bluetooth/BLE;
2. **Wired Digital** — USB-C/USB;
3. **Legacy / Analog Bridge** — audio + PTT + optional CAT through DigiRig-class interfaces, software TNC/modem providers or RadioLink Bridge.

---

# Platform tracks

The existing F0–F15 phase numbering is preserved for continuity. The broader platform is organized into five tracks.

## Track A — Core + Applications

Shared core, Operations Engine, protocols, CLI and user-facing applications.

Primary phases: `F0–F12`.

## Track B — Compatibility + Profiles

Driver SDK, device/cable/interface profiles, capability registry and all three I/O paths.

Primary phases: `F1`, `F2`, `F13`, `F14`.

## Track C — RadioLink Bridge

Optional BLE + USB-C accessory for conventional radios.

Primary phase: `F15`.

## Track D — RadioLink Ready

Future interoperability profile/specification for radios and accessories.

Primary phase: `F16`.

## Track E — Research / Labs

Experimental features and market/architecture research that must not inflate the MVP.

Primary phase: `F17`.

---

## F0 — Platform Foundation

**Status:** IN PROGRESS

### Goal
Establish product boundaries, cross-platform architecture, repository layout and first validation path.

### Deliverables
- [x] Create public GitHub repository.
- [x] Define product vision and guardrails.
- [x] Expand official targets to Android, iOS, Linux and macOS.
- [x] Adopt App Hub / unified platform architecture.
- [x] Define rule: Bluetooth capability must not be treated as synonymous with KISS/TNC capability.
- [x] Define TNC backend abstraction: embedded TNC, external KISS TNC or software TNC.
- [x] Create initial compatibility capability matrix.
- [x] Create Roadmap Master.
- [x] Record ADR-0001.
- [x] Record ADR-0002 for cross-platform App Hub architecture.
- [x] Define shared-core toolchain/language decision: **Rust stable + Cargo**.
- [x] Record ADR-0003 for Rust Shared Core.
- [x] Bootstrap Cargo workspace and initial Rust crates.
- [x] Adopt CLI-first development on macOS as the first low-friction validation workflow.
- [x] Consolidate the **RadioLink Platform** product model.
- [x] Adopt the three official I/O paths: BLE, USB and audio/PTT compatibility.
- [x] Define BLE and USB as first-class modern transports rather than application capabilities.
- [x] Define RadioLink Bridge, RadioLink Profiles, RadioLink Ready and RadioLink Labs as platform tracks/components.
- [x] Record ADR-0004 for three-path I/O and platform ecosystem.
- [ ] Select first reference radio/TNC for PoC.
- [ ] Capture Bluetooth services/characteristics for first BLE reference device.
- [ ] Select first USB-native reference device/interface path.
- [ ] Define KISS/AX.25/APRS packet fixtures.

### Exit criteria
A developer can clone the repository and understand the platform targets, mission UX, three I/O paths, shared-core strategy, first hardware candidates and first technical spike.

---

## F1 — Shared Core + Operations/Capability Abstractions

**Status:** IN PROGRESS

### Goal
Create the platform-neutral core and capability model used by all hosts so services do not depend directly on one hardware or transport path.

### Deliverables
- [x] Core project/toolchain bootstrap.
- [x] Initial `RadioCapabilities` model.
- [x] Initial `TncTransport` abstraction.
- [x] Initial crate boundaries for KISS, AX.25, APRS, TNC and drivers.
- [ ] Stable `RadioDevice` abstraction.
- [ ] Complete capability schema.
- [ ] Explicit `Transport` model: BLE / USB / serial / audio / network where applicable.
- [ ] Device Registry.
- [ ] Capability Registry.
- [ ] Transport Manager interface.
- [ ] TNC/Modem Provider interface.
- [ ] Operations Engine skeleton.
- [ ] Resource lifecycle model.
- [ ] Connection/session/recovery state machine.
- [ ] Operational mode/service registry model.
- [ ] Shared fixture/test harness.
- [ ] Host-facing API boundary for future FFI.

### Exit criteria
Core logic runs in tests without Android/iOS/Linux/macOS-specific Bluetooth or USB APIs and services can resolve a generic provider/transport path through the Operations Engine.

---

## F2 — Wireless Digital: Bluetooth Device Layer

**Status:** NOT STARTED

### Goal
Discover, connect and maintain reliable sessions with Bluetooth/BLE radios/TNCs while identifying the actual services/capabilities exposed by each device.

### Deliverables
- [ ] Device scan.
- [ ] Pair/connect flow.
- [ ] BLE service/characteristic discovery.
- [ ] Capability detection/profile mapping.
- [ ] Connection health model.
- [ ] Automatic reconnect behavior.
- [ ] Session restoration after reconnect.
- [ ] Human-readable error states.
- [ ] Raw RX/TX diagnostics.
- [ ] First reference driver.
- [ ] Desktop/macOS implementation first.
- [ ] Mobile adapters after core validation.

### Exit criteria
RadioLink can connect to the reference BLE device, exchange raw data reliably, recover a dropped session and correctly distinguish control/audio/KISS/TNC capabilities.

---

## F3 — KISS Transport Core

**Status:** IN PROGRESS

### Deliverables
- [x] Initial KISS data-frame encoder.
- [x] Reserved-byte escaping test.
- [ ] KISS frame decoder.
- [ ] Streaming parser.
- [ ] Fragmented transport frame handling.
- [ ] Captured-frame fixtures.
- [ ] BLE-to-KISS adapter.
- [ ] USB/serial-to-KISS adapter.
- [ ] TNC-provider-to-KISS adapter.
- [ ] Session recovery behavior independent of physical transport.

### Exit criteria
Real KISS frames from BLE, USB/serial reference hardware or a software TNC backend reach the shared protocol core deterministically.

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
Captured and live APRS RF packets decode correctly through the shared core independently of where the TNC lives or which transport carries it.

---

## F5 — Desktop Reference App (macOS/Linux)

**Status:** NOT STARTED

### Goal
Use desktop as a full validation environment after CLI/core spikes, without changing the mobile-first product intent.

### Deliverables
- [ ] RadioLink desktop shell.
- [ ] Mission/action launcher.
- [ ] Device/capability connection screen.
- [ ] Pipeline diagnostic view.
- [ ] KISS monitor.
- [ ] APRS receive view.
- [ ] Basic APRS transmit.
- [ ] Linux/macOS parity baseline.
- [ ] Validate at least one direct BLE KISS path.
- [ ] Validate at least one USB/serial or USB-native digital path.
- [ ] Validate at least one software-TNC/audio path using DigiRig or equivalent.

### Exit criteria
A desktop user can complete APRS RX/TX from one RadioLink service through more than one transport/provider path.

---

## F6 — RadioLink CLI / Headless

**Status:** IN PROGRESS

### Goal
Expose the shared core on macOS/Linux through terminal workflows and use the CLI as the first integration harness.

### Candidate commands
- [x] CLI crate/bootstrap.
- [x] `radiolink scan` placeholder.
- [x] `radiolink monitor` placeholder.
- [x] `radiolink aprs` placeholder.
- [x] `radiolink packet` placeholder.
- [ ] `radiolink connect`.
- [ ] `radiolink serve`.
- [ ] `radiolink capabilities`.
- [ ] `radiolink transports`.
- [ ] `radiolink pipeline`.
- [ ] Real BLE device discovery.
- [ ] Real USB device discovery.
- [ ] Real KISS monitor.
- [ ] Operations Engine integration.

### Exit criteria
Core radio diagnostics and Packet/APRS operations can run without a GUI and reveal the resolved device → transport → provider → protocol pipeline.

---

## F7 — APRS Service / Module

**Status:** NOT STARTED

### Deliverables
- [ ] Station list.
- [ ] Map.
- [ ] Last-heard state.
- [ ] Local persistence.
- [ ] Position beacon TX.
- [ ] Chat-like APRS messaging.
- [ ] ACK/REJ handling.
- [ ] Mission-first actions: message / position / stations.
- [ ] Provider/transport independence.

### Exit criteria
APRS is a coherent service regardless of whether frames come from an embedded TNC, external BLE/USB KISS TNC or software TNC/audio path.

---

## F8 — Packet Module

**Status:** NOT STARTED

### Deliverables
- [ ] Connected AX.25 transport spike.
- [ ] Packet terminal.
- [ ] Connection/session state.
- [ ] BBS/RMS interoperability tests.
- [ ] Shared TNC Provider usage.
- [ ] Recovery when underlying transport reconnects where technically possible.

### Exit criteria
A user can conduct a practical connected-mode session through a supported provider without the Packet service knowing the physical transport implementation.

---

## F9 — Radio Control Module

**Status:** NOT STARTED

### Deliverables
- [ ] Capability-gated frequency/channel state.
- [ ] Mode/power/PTT hooks where safely exposed.
- [ ] Unsupported-state UX.
- [ ] Shared state with APRS/Packet services.
- [ ] Support control-only Bluetooth devices without falsely marking them as Packet-capable.
- [ ] Support CAT from one transport while data/TNC uses another where possible.

### Exit criteria
Richer radios expose control without making control mandatory for simpler TNCs, and mixed-transport capability composition is represented accurately.

---

## F10 — Android App

**Status:** NOT STARTED

### Goal
Bring the proven core/operations/device architecture to Android.

### Deliverables
- [ ] Android shell in Kotlin.
- [ ] Rust core binding.
- [ ] Bluetooth lifecycle.
- [ ] USB host/device access where practical.
- [ ] Permissions.
- [ ] Location/background policy.
- [ ] Mission/action launcher.
- [ ] APRS/Packet parity baseline.
- [ ] Capability-aware device screen.
- [ ] Recovery/fallback UX.
- [ ] Beta distribution build.

### Exit criteria
Android completes the supported core workflows validated by CLI/desktop and can use at least one direct BLE path.

---

## F11 — iOS App

**Status:** NOT STARTED

### Deliverables
- [ ] iOS shell in Swift.
- [ ] Rust core binding.
- [ ] CoreBluetooth adapter.
- [ ] Supported wired accessory/USB strategy assessment.
- [ ] Background/location behavior.
- [ ] Mission/action launcher.
- [ ] APRS/Packet parity baseline.
- [ ] Capability-aware device screen.
- [ ] Recovery UX.
- [ ] TestFlight build.

### Exit criteria
iPhone completes the same supported core workflows within iOS transport/background constraints.

---

## F12 — Winlink Module

**Status:** NOT STARTED

### Deliverables
- [ ] Feasibility/architecture decision.
- [ ] Packet connection transport integration.
- [ ] Inbox/outbox/compose model.
- [ ] RMS configuration.
- [ ] TNC/Modem Provider integration.
- [ ] Interoperability testing.
- [ ] Evaluate provider interchangeability patterns for future Mercury/other modem support without coupling the application to one modem.

### Exit criteria
A supported host/radio combination completes a practical Winlink Packet exchange from the same RadioLink platform.

---

## F13 — Driver SDK + RadioLink Profiles / Compatibility Registry

**Status:** NOT STARTED

### Goal
Make hardware support capability-driven and capture known-good radio/interface/cable combinations.

### Deliverables
- [ ] Stable driver interface.
- [ ] Capability schema documentation.
- [ ] Transport schema documentation.
- [ ] Driver test harness.
- [ ] Device profile template.
- [x] Initial compatibility matrix document.
- [ ] Profile fields for radio firmware, interface and cable.
- [ ] Audio RX/TX calibration fields.
- [ ] PTT method fields.
- [ ] Known-good platform/mode validation state.
- [ ] Per-device capability flags:
  - [ ] Bluetooth.
  - [ ] USB.
  - [ ] CAT / radio control.
  - [ ] Audio RX.
  - [ ] Audio TX.
  - [ ] PTT.
  - [ ] Serial/data transport.
  - [ ] KISS.
  - [ ] Embedded TNC.
  - [ ] GPS.
  - [ ] telemetry.
- [ ] Tested-platform matrix: Android / iOS / Linux / macOS.
- [ ] Supported-service matrix: APRS / Packet / Winlink / Radio Control / diagnostics.
- [ ] Contribution guide.
- [ ] Future community profile submission workflow design.

### Initial candidate device classes
- [ ] Class A — Embedded BLE KISS/TNC radio.
- [ ] Class B — Embedded USB KISS/TNC radio/interface.
- [ ] Class C — External BLE/USB KISS TNC.
- [ ] Class D — DigiRig/USB audio + software TNC.
- [ ] Class E — Bluetooth CAT-only radio.
- [ ] Class F — Multi-transport radio with different capabilities per transport.
- [ ] RadioLink Bridge.

### Exit criteria
A contributor can describe a radio accurately by capabilities/transports and add support without modifying APRS/Packet core logic.

---

## F14 — Wired Digital + Legacy Compatibility Layer

**Status:** NOT STARTED

### Goal
Implement the USB-C/USB path and support conventional radios where native digital KISS/TNC is not available.

### Wired Digital candidates
- [ ] USB CDC/serial discovery.
- [ ] USB KISS transport.
- [ ] USB CAT.
- [ ] USB Audio.
- [ ] Multi-interface USB devices.
- [ ] Platform-specific USB integration.

### Legacy / Analog Bridge candidates
- [ ] DigiRig-class USB audio/PTT.
- [ ] Direwolf as a default desktop software TNC provider for AX.25/APRS/Packet where appropriate.
- [ ] External software TNC/modem provider interface.
- [ ] Serial CAT interfaces.
- [ ] Audio/PTT calibration workflow.
- [ ] Capability composition using CAT from one path and TNC/audio from another.

### Guardrail
BLE, USB and audio/PTT are independent transport/provider paths. No service may bind directly to one of them.

### Exit criteria
The same APRS service can operate through a wired digital KISS path and a conventional audio/software-TNC path without service-level changes.

---

## F15 — RadioLink Bridge

**Status:** NOT STARTED

### Goal
Provide an optional compact accessory for radios without smartphone/desktop-friendly data interfaces.

### Candidate responsibilities
- [ ] ESP32-S3-class feasibility spike.
- [ ] BLE transport.
- [ ] USB-C/USB transport.
- [ ] KISS/TNC.
- [ ] Audio RX/TX codec/interface.
- [ ] PTT.
- [ ] Optional CAT/control.
- [ ] Local configuration/profile storage.
- [ ] Health/telemetry.
- [ ] BLE ↔ USB fallback behavior.
- [ ] Reference cable/interface family.
- [ ] Field enclosure/power study.

### Guardrail
The Bridge is an adapter/TNC accessory, not a second general-purpose computer or embedded Linux replacement.

### Exit criteria
A conventional radio can appear to RadioLink as a capability-described device over either BLE or USB-C using the same application/service stack.

---

## F16 — RadioLink Ready Interoperability Profile

**Status:** NOT STARTED

### Goal
Define a future open, documented interoperability profile for radios and accessories that want low-friction RadioLink integration.

### Candidate scope
- [ ] Device identity/discovery model.
- [ ] Capability schema.
- [ ] Transport advertisement/discovery.
- [ ] KISS over BLE profile guidance.
- [ ] KISS/serial over USB guidance.
- [ ] CAT/control capability description.
- [ ] PTT/GPS/battery/telemetry capability description.
- [ ] Reference implementation using RadioLink Bridge.
- [ ] Conformance test concept.
- [ ] Manufacturer/community documentation.

### Guardrail
Prefer established standards such as KISS, USB CDC/audio and documented Bluetooth services. Introduce RadioLink-specific extensions only when necessary and document them openly.

### Exit criteria
A third party can implement a RadioLink-friendly device interface without requiring RadioLink application code changes for each low-level transport detail.

---

## F17 — RadioLink Labs / Research

**Status:** IN PROGRESS

### Goal
Continuously mine real-world workflows, open-source projects and technical creator experiences for problems, architecture lessons and product opportunities without forcing experimental ideas into the core product.

### Current research sources / methods
- [x] Create YouTube corpus builder with timestamped transcripts.
- [x] Build complete The Tech Prepper corpus for research.
- [ ] Complete KM4ACK corpus and cross-source validation.
- [ ] Record structured findings with source/video/timestamp/evidence.
- [ ] Maintain problem → workaround → opportunity map.

### Labs candidates
- [ ] Winlink/Mercury modem-provider experiments.
- [ ] Reticulum.
- [ ] LoRa.
- [ ] Modern BBS/store-and-forward messaging.
- [ ] Offline radio knowledge assistant.
- [ ] Field power/telemetry integrations.
- [ ] Additional digital modes.

### Guardrail
Labs experiments must not become mandatory dependencies of RadioLink Core or the MVP until product value and architecture fit are demonstrated.

### Exit criteria
Research findings can be promoted deliberately into ADRs, requirements, Profiles, Bridge work or future product modules with traceable evidence.

---

# MVP validation matrix

The MVP should prove architectural portability, not every possible combination.

Minimum validation target:

```text
Host platforms
├── macOS CLI/reference
└── at least one mobile host (Android or iOS)

I/O paths
├── one direct modern digital path (BLE KISS preferred for first PoC)
└── one legacy/audio compatibility path

Protocol/service
└── APRS RX/TX + messaging baseline
```

USB-native digital support is an official platform path and should be developed/validated early enough that it does not become an afterthought, even if BLE/KISS remains the shortest first hardware PoC.

---

# Current execution order

Core/protocol work and transport validation can proceed in parallel:

```text
F0 Foundation
  ↓
F1 Core / Operations abstractions
  ├───────────────┬─────────────────┐
  ↓               ↓                 ↓
F2 BLE         F14 USB/audio      F3 KISS
  └───────────────┴────────┬────────┘
                           ↓
                        F4 AX.25/APRS
                           ↓
                        F6 CLI
                           ↓
                        F5 Desktop
                           ↓
                        F7 APRS
                           │
                 ┌─────────┼─────────┐
                 ↓         ↓         ↓
               F8 Packet F9 Control F10 Android
                                     ↓
                                   F11 iOS
                                     ↓
                                   F12 Winlink
```

Platform expansion tracks proceed after the core abstractions are proven, with selective spikes allowed earlier:

```text
F13 Profiles / Compatibility
        ↓
F15 RadioLink Bridge
        ↓
F16 RadioLink Ready

F17 Labs / Research runs continuously in parallel.
```

---

# Immediate next action

The next engineering step remains a small, real hardware vertical slice:

1. select the first **BLE KISS radio/TNC** reference device;
2. capture its Bluetooth services/characteristics;
3. define real KISS/AX.25/APRS fixtures;
4. connect the real transport to the Rust transport/provider abstraction and `radiolink-cli`;
5. define the parallel compatibility baseline with a conventional radio + DigiRig/software TNC;
6. identify one USB-native device/interface candidate for the Wired Digital path.

Preferred shortest first PoC:

```text
macOS CLI ↔ BLE KISS ↔ Radio/TNC ↔ RF
```

Compatibility baseline:

```text
macOS CLI ↔ software TNC ↔ USB audio/PTT ↔ DigiRig ↔ conventional radio ↔ RF
```

Planned modern wired validation:

```text
macOS CLI ↔ USB-C/USB KISS or serial/data ↔ Radio/TNC ↔ RF
```
