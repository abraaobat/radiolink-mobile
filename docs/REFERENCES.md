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

The Tech Prepper is one strong longitudinal source, not a proxy for the entire amateur-radio market. Product hypotheses should be cross-validated with independent sources such as KM4ACK, project issues/discussions, manufacturer documentation, community reports and RadioLink field testing.

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

RadioLink Platform
  -> combines evidence behind one capability-driven architecture
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
