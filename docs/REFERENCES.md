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

**Role:** hardware/TNC reference for the RadioNode-BR and for external BLE KISS TNC interoperability.

Key concepts to preserve as reference points:

- wireless host-to-TNC link;
- KISS TNC abstraction;
- radio-side audio/PTT interface;
- APRS and general Packet operation;
- 1200/9600 baud capability as a longer-term benchmark;
- compact portable implementation.

For RadioLink, Mobilinkd-class hardware should be represented as a capability-driven `TncTransport`, not as a special case inside APRS or Packet modules.

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
- F13 — Driver SDK + Compatibility Matrix.

#### Guardrail

Bluetooth control support discovered through HTCommander or similar projects must not be interpreted as proof of KISS/TNC capability. RadioLink must continue to represent control, audio, serial/data, KISS and embedded-TNC capabilities independently.

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
- experimental Reticulum/LoRa/BBS concepts should remain in Labs until independently validated.

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
