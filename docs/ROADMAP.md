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
- [x] Record ADR-0005 for Context Providers and layered diagnostics.
- [x] Create nominal device-registry governance and initial P0 lab profiles.
- [x] Select the first owned-hardware compatibility PoC path: macOS CLI + DigiRig + Quansheng UV-K1/UV-K5 + software TNC.
- [ ] Acquire/borrow and validate the first direct BLE KISS reference device; BTECH UV-Pro is the current P1 candidate.
- [ ] Capture Bluetooth services/characteristics for the first BLE reference device.
- [ ] Select the first USB-native reference device/interface path.
- [ ] Define KISS/AX.25/APRS packet fixtures.

### Exit criteria
A developer can clone the repository and understand the platform targets, mission UX, three I/O paths, shared-core strategy, context-provider model, device registry, first owned-hardware PoC and modern reference candidates.

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
- [ ] Multi-transport capability composition for one logical `RadioDevice`.
- [ ] Runtime Device Registry.
- [ ] Capability Registry.
- [ ] Transport Manager interface.
- [ ] TNC/Modem Provider interface.
- [ ] Context Provider Registry.
- [ ] `LocationProvider` abstraction.
- [ ] `TimeProvider` abstraction.
- [ ] Context-provider quality/precedence/selection policy.
- [ ] Operations Engine skeleton.
- [ ] Resource lifecycle model.
- [ ] Connection/session/recovery state machine.
- [ ] Layered diagnostic-state model: physical/power → transport → logical interface → capability → provider → protocol → service readiness.
- [ ] Operational mode/service registry model.
- [ ] Shared fixture/test harness.
- [ ] Host-facing API boundary for future FFI.

### Exit criteria
Core logic runs in tests without Android/iOS/Linux/macOS-specific Bluetooth or USB APIs; services resolve generic transport/TNC/context providers through the Operations Engine; and failures can be represented at the correct diagnostic layer.

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
- [ ] Layered reporting for pairing → service discovery → capability match → provider readiness.
- [ ] Raw RX/TX diagnostics.
- [ ] First reference driver.
- [ ] Desktop/macOS implementation first.
- [ ] Mobile adapters after core validation.

### Exit criteria
RadioLink can connect to the reference BLE device, exchange raw data reliably, recover a dropped session and correctly distinguish control/audio/KISS/TNC capabilities with a specific failure layer when unavailable.

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
- [ ] Context/location-provider state view.
- [ ] KISS monitor.
- [ ] APRS receive view.
- [ ] Basic APRS transmit.
- [ ] Linux/macOS parity baseline.
- [ ] Validate at least one direct BLE KISS path.
- [ ] Validate at least one USB/serial or USB-native digital path.
- [ ] Validate at least one software-TNC/audio path using DigiRig or equivalent.

### Exit criteria
A desktop user can complete APRS RX/TX from one RadioLink service through more than one transport/provider path and inspect the resolved diagnostic/context pipeline.

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
- [ ] `radiolink diagnose`.
- [ ] `radiolink context`.
- [ ] Real BLE device discovery.
- [ ] Real USB device discovery.
- [ ] Real USB audio/serial interface enumeration for DigiRig-class hardware.
- [ ] Display selected/available Location/Time Providers.
- [ ] Display layered diagnostic stage and human-readable failure reason.
- [ ] Real KISS monitor.
- [ ] Operations Engine integration.

### Exit criteria
Core radio diagnostics and Packet/APRS operations can run without a GUI and reveal the resolved device → transport → provider → protocol/service pipeline plus active context providers and the highest verified diagnostic stage.

---

## F7 — APRS Service / Module

**Status:** NOT STARTED

### Deliverables
- [ ] Station list.
- [ ] Map.
- [ ] Last-heard state.
- [ ] Local persistence.
- [ ] Position beacon TX.
- [ ] `LocationProvider` integration for beacon/position workflows.
- [ ] Chat-like APRS messaging.
- [ ] ACK/REJ handling.
- [ ] Mission-first actions: message / position / stations.
- [ ] Provider/transport independence.

### Exit criteria
APRS is a coherent service regardless of whether frames come from an embedded TNC, external BLE/USB KISS TNC or software TNC/audio path, and position workflows can use an appropriate context provider independent of the radio transport.

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
- [ ] Preserve mixed-transport capability composition under one logical device identity.

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
- [ ] Host `LocationProvider` adapter.
- [ ] Context-provider selection/state UX where relevant.
- [ ] Mission/action launcher.
- [ ] APRS/Packet parity baseline.
- [ ] Capability-aware device screen.
- [ ] Recovery/fallback UX.
- [ ] Beta distribution build.

### Exit criteria
Android completes the supported core workflows validated by CLI/desktop, can use at least one direct BLE path and can provide host location through the shared context-provider API.

---

## F11 — iOS App

**Status:** NOT STARTED

### Deliverables
- [ ] iOS shell in Swift.
- [ ] Rust core binding.
- [ ] CoreBluetooth adapter.
- [ ] Supported wired accessory/USB strategy assessment.
- [ ] Background/location behavior.
- [ ] Host `LocationProvider` adapter.
- [ ] Context-provider selection/state UX where relevant.
- [ ] Mission/action launcher.
- [ ] APRS/Packet parity baseline.
- [ ] Capability-aware device screen.
- [ ] Recovery UX.
- [ ] TestFlight build.

### Exit criteria
iPhone completes the same supported core workflows within iOS transport/background constraints and provides host location through the shared context-provider API.

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

**Status:** IN PROGRESS

### Goal
Make hardware support capability-driven and capture known-good radio/interface/cable combinations.

### Deliverables
- [ ] Stable driver interface.
- [ ] Capability schema documentation.
- [ ] Transport schema documentation.
- [ ] Driver test harness.
- [x] Device profile documentation template.
- [x] Initial compatibility matrix document.
- [x] Nominal P0/P1/P2/watchlist device registry.
- [x] Initial P0 lab profiles for UV-K1 2 MB, UV-K5 V3, UV-5 Mini and QYT KT-7900D.
- [x] Document profile fields for firmware/version, interface/cable, required radio-side settings, validation state and evidence.
- [ ] Runtime profile schema/loader.
- [ ] Audio RX/TX calibration tooling/fields in runtime schema.
- [ ] PTT method representation in runtime schema.
- [ ] Known-good platform/mode validation state in runtime schema.
- [ ] Context-provider compatibility/profile fields.
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
  - [ ] Radio GPS/GNSS source exposure.
  - [ ] telemetry.
- [ ] Tested-platform matrix: Android / iOS / Linux / macOS.
- [ ] Supported-service matrix: APRS / Packet / Winlink / Radio Control / diagnostics.
- [ ] Profile promotion tooling/state transitions: RESEARCHED → CANDIDATE → LAB_AVAILABLE → CONNECTED → RX_VERIFIED → TX_VERIFIED → PROFILE_VERIFIED → SUPPORTED.
- [ ] Contribution guide.
- [ ] Future community profile submission workflow design.

### Initial candidate device classes
- [ ] Class A — Embedded BLE KISS/TNC radio.
- [ ] Class B — Embedded USB KISS/TNC radio/interface.
- [ ] Class C — External BLE/USB KISS TNC.
- [ ] Class D — DigiRig/USB audio + software TNC.
- [ ] Class E — Bluetooth CAT/control only.
- [ ] Class F — Bluetooth audio + PTT software-TNC candidate.
- [ ] Class G — Multi-transport/composite device.
- [ ] RadioLink Bridge.

### Nominal reference targets

- **P0 / lab available:** Quansheng UV-K1 2 MB, Quansheng UV-K5 V3, Baofeng UV-5 Mini, QYT KT-7900D, DigiRig Mobile.
- **P1 / modern reference candidates:** BTECH UV-Pro, VGC/Vero VR-N76 family, Kenwood TH-D75.
- **P2 / boundary reference:** Retevis Ailunce HA2.
- **Watchlist:** Radtel RT-660, Baofeng DM-UV32.

See `docs/devices/REGISTRY.md` for current state and support rules.

### Exit criteria
A contributor can describe a radio accurately by capabilities/transports/context requirements and add support without modifying APRS/Packet core logic; support state is traceable to exact bench evidence.

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
- [ ] Layered USB diagnostics: power/visibility → enumeration → interface discovery → capability/provider readiness.

### Legacy / Analog Bridge candidates
- [ ] DigiRig-class USB audio/PTT.
- [ ] Validate the first owned P0 path using DigiRig + UV-K1/UV-K5.
- [ ] Direwolf as a default desktop software TNC provider for AX.25/APRS/Packet where appropriate.
- [ ] External software TNC/modem provider interface.
- [ ] Serial CAT interfaces.
- [ ] Audio/PTT calibration workflow.
- [ ] Capability composition using CAT from one path and TNC/audio from another.

### Guardrail
BLE, USB and audio/PTT are independent transport/provider paths. No service may bind directly to one of them.

### Exit criteria
The same APRS service can operate through a wired digital KISS path and a conventional audio/software-TNC path without service-level changes, with failures reported at the correct transport/provider layer.

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
- [ ] Context-provider capability/source description where relevant.
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
- [x] Complete KM4ACK corpus and deep cross-source validation.
- [x] Record structured findings with source/video/evidence.
- [x] Maintain problem → workaround → opportunity map.
- [x] Create source-neutral synthesis findings through `RL-RS-018`.
- [x] Create Research Promotion Register linking findings to ADR/roadmap/implementation/validation state.
- [ ] Import timestamp/deep-link segment evidence for findings promoted into implementation/conformance requirements.
- [ ] Add manufacturer/protocol documentation as a third evidence class for device-specific claims.

### Labs candidates
- [ ] Winlink/Mercury modem-provider experiments.
- [ ] Reticulum.
- [ ] LoRa.
- [ ] Modern BBS/store-and-forward messaging.
- [ ] Future delivery-independent Messaging Service experiments.
- [ ] Offline radio knowledge assistant / optional Data Services.
- [ ] Field readiness/self-test/exercise tooling.
- [ ] Field power/telemetry integrations.
- [ ] Additional digital modes.

### Guardrail
Labs experiments must not become mandatory dependencies of RadioLink Core or the MVP until product value and architecture fit are demonstrated.

### Exit criteria
Research findings can be promoted deliberately through `PROMOTION-REGISTER.md` into ADRs, requirements, Profiles, Bridge work or future product modules with traceable evidence and validation status.

---

# MVP validation matrix

The MVP should prove architectural portability, not every possible combination.

Minimum validation target:

```text
Host platforms
├── macOS CLI/reference
└── at least one mobile host (Android or iOS)

I/O paths
├── one direct modern digital path (BLE KISS)
├── one wired/USB path
└── one legacy/audio compatibility path

Context/diagnostics
├── at least one host LocationProvider
└── layered transport→service diagnostics

Protocol/service
└── APRS RX/TX + messaging baseline
```

USB-native digital support is an official platform path and should be developed/validated early enough that it does not become an afterthought. The owned-hardware DigiRig path is now the first executable bench slice because the current lab does not yet contain a direct BLE KISS radio.

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

The next engineering step is now an owned-hardware vertical slice on macOS:

1. use **DigiRig Mobile + Quansheng UV-K1 2 MB** as the first P0 radio/interface pair;
2. enumerate the DigiRig USB audio/serial interfaces from `radiolink-cli` and expose layered diagnostics;
3. establish RX audio/PTT path and a software-TNC provider;
4. carry real KISS/AX.25 data into the shared core;
5. capture and decode live APRS RX, then perform a controlled APRS TX validation;
6. update the UV-K1 profile with exact firmware, cable, settings, calibration and evidence;
7. reproduce the path with UV-K5 V3 as the second P0 profile;
8. in parallel, acquire/borrow a **BTECH UV-Pro** or another validated BLE KISS reference for F2;
9. identify one USB-native digital device/interface candidate for the direct Wired Digital path.

Current first PoC:

```text
macOS CLI ↔ USB/DigiRig ↔ audio/PTT ↔ software TNC ↔ UV-K1 ↔ RF
```

Parallel modern BLE target:

```text
macOS CLI ↔ BLE KISS ↔ BTECH UV-Pro (candidate) ↔ RF
```

Planned modern wired validation:

```text
macOS CLI ↔ USB-C/USB KISS or serial/data ↔ Radio/TNC ↔ RF
```
