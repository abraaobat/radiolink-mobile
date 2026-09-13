# RadioLink Platform — Technical References

RadioLink uses external projects and research corpora as engineering references, not as runtime dependencies or architectural templates to be copied wholesale.

## Reference roles

### DigiPi

**Role:** functional coverage and App Hub reference.

Use DigiPi to benchmark which amateur-radio workflows should be available from one place, including APRS, Packet/AX.25, Winlink, digital modes, logs, GPS and supporting tools.

RadioLink does **not** adopt DigiPi's Raspberry Pi appliance architecture as its own runtime model.

See also: `DIGIPI-BENCHMARK.md`.

---

### Mobilinkd TNC4

**Role:** hardware/TNC reference for the RadioNode-BR/RadioLink Bridge path and for external BLE KISS TNC interoperability.

Key concepts to preserve as reference points:

- wireless host-to-TNC link;
- KISS TNC abstraction;
- radio-side audio/PTT interface;
- APRS and general Packet operation;
- 1200/9600 baud capability as a longer-term benchmark;
- compact portable implementation.

For RadioLink, Mobilinkd-class hardware should be represented as a capability-driven TNC provider/transport path, not as a special case inside APRS or Packet modules.

---

### HTCommander

Repository: https://github.com/Ylianst/HTCommander

**Role:** Bluetooth radio-integration, device-control and driver/protocol reference.

HTCommander is a multi-platform tool focused on control of radios including UV-Pro, GA-5WB, VR-N76 and VR-N7500-class devices. RadioLink should study it specifically for the Bluetooth Device Layer and radio-driver work.

#### What RadioLink should learn from HTCommander

- Bluetooth device discovery and connection lifecycle;
- device identification and profile selection;
- radio-specific protocol handling;
- separation between generic application behavior and per-radio behavior;
- capability-driven controls;
- connection/configuration UX patterns;
- multi-platform handling of supported radio families.

#### What HTCommander is not

HTCommander is **not** the architectural template for the whole RadioLink product and is not treated as the APRS/Packet/Winlink core.

Its main reference value is concentrated in:

- F2 — Bluetooth Device Layer;
- F9 — Radio Control Module;
- F13 — Driver SDK + Compatibility Matrix;
- F18 — Satellite Operations radio-control path, where applicable.

#### Guardrail

Bluetooth control support discovered through HTCommander or similar projects must not be interpreted as proof of KISS/TNC capability. RadioLink must continue to represent control, audio, serial/data, KISS and embedded-TNC capabilities independently.

---

### BTECH UV-PRO official app / firmware

**Role:** first owned modern-radio behavioral and manufacturer-documentation reference for BLE/KISS, Radio Control and Satellite Operations.

Primary sources:

- UV-PRO firmware changelog: https://baofengtech.com/uv-pro-firmware-changelog/
- BTECH UV Programmer app manual: https://baofengtech.com/wp-content/uploads/2025/11/BTECH-APP-Updated-Manual-2025-9-9.pdf
- UV-PRO product/support page: https://baofengtech.com/product/uv-pro/

Manufacturer documentation currently provides reference evidence for:

- Bluetooth KISS TNC operation;
- APRS/Packet workflows;
- app-driven radio control/configuration;
- satellite mode;
- satellite display data such as azimuth, elevation, range and pass countdown in firmware history;
- independent TX/RX frequency configuration for the KISS profile in current firmware history.

These sources define **research and validation targets**, not RadioLink support claims. The exact lab unit, firmware, Bluetooth services/characteristics and control commands must be captured and tested before capabilities are promoted.

For Satellite Operations, the official app is a behavioral benchmark for what a user can accomplish, while RadioLink's implementation remains generic and capability-driven.

See:

- `SATELLITE-OPERATIONS.md`;
- `devices/profiles/btech-uv-pro.md`.

#### Guardrails

- Official satellite mode does not prove full duplex.
- Dual watch/Main-Sub behavior does not prove simultaneous RX while TX.
- Official-app control behavior does not automatically define an open or stable third-party protocol.
- Frequencies/paths/tones must be stored as updateable satellite-profile data, not permanently copied from one app/manual snapshot.
### Meshtastic

Project/documentation: https://meshtastic.org/

Repository: https://github.com/meshtastic/firmware

TAK integration: https://github.com/meshtastic/meshtastic/blob/master/docs/software/android/user/tak.md

**Role:** decentralized, low-power LoRa mesh and host-to-node interoperability reference.

RadioLink should study and interoperate with Meshtastic for:

- external-node mesh routing/rebroadcasting;
- BLE/USB PhoneAPI-style client connections;
- compact text, position and telemetry payloads;
- channel/node identity and delivery state;
- constrained airtime/queue behavior;
- supported TAK/application payloads;
- operation without cellular or Internet infrastructure.

Meshtastic is the first planned `MeshProvider`, not a replacement for RadioLink's KISS/AX.25/APRS stack. A Meshtastic-compatible node is not automatically a TNC, and LoRa must not be treated as a general-purpose path for maps, video, real-time audio or unrestricted large files.

---

### TAK / ATAK-CIV

Product center: https://tak.gov/

ATAK-CIV repository: https://github.com/TAK-Product-Center/atak-civ

TAK protocol reference: https://github.com/TAK-Product-Center/atak-civ/blob/main/takproto/README.txt

**Role:** moving-map, situational-event, CoT and client/server interoperability reference.

RadioLink should study the TAK ecosystem for:

- participant/position presentation;
- chat and operational events;
- markers, routes, shapes and alerts;
- event timestamps, stale/expiry behavior and identity;
- CoT encoding/interoperability;
- optional TAK Server transport;
- plugin/provider boundaries.

ATAK is not itself the LoRa mesh routing layer. RadioLink will build a cross-platform native Situational module and a documented CoT subset rather than embedding ATAK or claiming full compatibility from partial event support.

---

### The Tech Prepper research corpus

Channel: https://www.youtube.com/@TheTechPrepper

**Role:** longitudinal field-workflow, integration-friction and product-research source.

The local research corpus contains 548 videos/streams/shorts and is used to identify recurring problems, workarounds, architecture lessons and market opportunities across APRS, Packet, Winlink, DigiRig, Bluetooth/KISS, EmComm Tools, BBS, Mercury HF, Reticulum/LoRa and offline communications.

The raw transcripts are **not** committed to Git. RadioLink stores only derived research artifacts and traceable source references.

Key lessons currently adopted from the corpus include:

- digital-radio pain is frequently an integration/configuration problem rather than a missing-protocol problem;
- user UX should prioritize operational intent over protocol/application names;
- offline-first and near-zero-configuration operation are high-value field properties;
- a smartphone can be the primary radio computer, but mobile UX should be native rather than a desktop Linux port;
- BLE/KISS can provide an excellent cable-free mobile path;
- Bluetooth reliability must not be assumed, so reconnect/recovery and wired fallback matter;
- USB is a first-class modern transport rather than only a legacy fallback;
- mode switching requires lifecycle/resource orchestration;
- known-good radio/interface/audio/PTT settings should become reusable device profiles;
- TNC/modem implementations should sit behind stable provider interfaces;
- experimental Reticulum/alternative-mesh/BBS concepts should remain in Labs until independently validated; Meshtastic interoperability follows the bounded ADR-0006 roadmap.

Research documentation:

- [`research/THE-TECH-PREPPER.md`](research/THE-TECH-PREPPER.md) — source record, methodology and high-value source sequence;
- [`research/FINDINGS.md`](research/FINDINGS.md) — traceable findings and RadioLink consequences;
- [`research/MARKET-OPPORTUNITIES.md`](research/MARKET-OPPORTUNITIES.md) — product/ecosystem opportunity map.

#### Guardrail

The Tech Prepper is one strong longitudinal source, not a proxy for the entire amateur-radio market.

---

### KM4ACK research corpus

Channel: https://www.youtube.com/@KM4ACK

**Role:** independent cross-source validation and broader amateur-radio software/integration research source.

The local corpus contains **828** items with **0 extraction failures** and spans a broader range of subjects than The Tech Prepper. RadioLink therefore screens the corpus before deep mining.

Initial screening produced:

- A — direct RadioLink core relevance: 154 videos;
- B — adjacent integration/off-grid/software relevance: 209 videos;
- C — radio/field context: 330 videos;
- D — low immediate relevance: 135 videos.

The 363 A+B videos form the primary deep-mining set. C/D are retained for selective review and false-negative recovery.

Early independent validation from KM4ACK strongly supports:

- integration/configuration complexity as a systemic pain point;
- smartphone-centered Winlink/APRS field operation;
- KISS/TNC decoupled from transport;
- BLE and USB as separate transport choices;
- explicit capability discovery instead of treating Bluetooth as synonymous with TNC access;
- firmware-aware device profiles;
- capability composition across simultaneous transports;
- offline-first field services;
- transport diagnostics, including USB data-cable failures;
- the value of companion software when native APRS radio UX is poor.

Research documentation:

- [`research/KM4ACK.md`](research/KM4ACK.md) — corpus snapshot, screening summary and first-pass validation;
- [`research/EVIDENCE.md`](research/EVIDENCE.md) — source-level evidence ledger;
- [`research/CONTRADICTIONS.md`](research/CONTRADICTIONS.md) — disagreements and qualifications;
- [`research/SYNTHESIS.md`](research/SYNTHESIS.md) — source-neutral findings derived from The Tech Prepper + KM4ACK.

#### Guardrail

KM4ACK is a broad technical creator source, not a representative sample of all radio amateurs. Findings should still be checked against protocol/manufacturer documentation, community reports and RadioLink field testing before strong compatibility claims are made.

---

## Consolidated reference model

```text
DigiPi
  -> App Hub / functional coverage

Mobilinkd TNC4
  -> BLE KISS TNC / portable radio interface

HTCommander
  -> Bluetooth radio integration / control / drivers

BTECH UV-PRO official app/firmware
  -> owned BLE/KISS + satellite/radio-control behavior reference
Meshtastic
  -> low-power LoRa mesh / external-node routing / client interoperability

TAK / ATAK-CIV
  -> moving map / situational events / CoT interoperability

The Tech Prepper corpus
  -> field workflow / integration friction / product discovery

KM4ACK corpus
  -> independent validation / broader software-interface workflows

RadioLink Research Synthesis
  -> cross-source evidence and confidence

RadioLink Platform
  -> converts validated evidence into architecture, roadmap and tests
```

## Reference policy

When studying external projects and sources:

1. Prefer documented protocols and public interfaces over implementation-specific assumptions.
2. Preserve RadioLink's platform-neutral Rust core boundaries.
3. Keep platform Bluetooth/USB APIs in host/adapter layers.
4. Keep radio-specific behavior inside drivers/profiles.
5. Never equate Bluetooth presence with Packet/KISS/TNC support.
6. Record interoperability observations and test fixtures before promoting a device to a supported profile.
7. Preserve source provenance for research-derived architectural or product decisions.
8. Keep contradictory observations visible rather than silently reconciling them.
9. Raise confidence when the same problem appears independently across multiple sources.
10. Record firmware/platform/device-version context when capability depends on it.
11. For Satellite Operations, keep orbit/pass logic generic and isolate device-specific tuning/control inside the Radio Driver.
12. Treat full duplex as an independently validated capability; never infer it from dual watch.
11. Treat LoRa payload size, airtime and congestion as first-class delivery constraints.
12. Declare TAK/CoT interoperability by tested event subset rather than a generic compatibility claim.
13. Keep mesh routing on the external node unless a future architecture decision explicitly changes that boundary.
