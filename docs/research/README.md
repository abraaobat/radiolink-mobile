# RadioLink Research

This directory contains traceable research artifacts used to inform RadioLink architecture, roadmap and product hypotheses.

Raw third-party corpora are not committed here. This directory stores derived source records, evidence, findings, contradictions, synthesis and decisions.

## Current sources

### The Tech Prepper

- [`THE-TECH-PREPPER.md`](THE-TECH-PREPPER.md) — corpus/source record and methodology.
- [`FINDINGS.md`](FINDINGS.md) — original source-specific findings and RadioLink consequences.

### KM4ACK

- [`KM4ACK.md`](KM4ACK.md) — corpus snapshot, A/B/C/D screening summary, first-pass validation and KM4ACK-specific hypotheses.

Corpus status: **828 items, 0 extraction failures**. Because the channel is broader than The Tech Prepper, mining uses a screening stage before deep review.

Initial screening:

- A — direct core relevance: **154**;
- B — adjacent integration/off-grid/software: **209**;
- C — radio/field context: **330**;
- D — low immediate relevance: **135**.

Primary deep-mining universe: **363 A+B videos**. Screening is provisional; C/D videos may be promoted when evidence searches reveal hidden relevance.

## Cross-source artifacts

- [`EVIDENCE.md`](EVIDENCE.md) — source-level evidence ledger (`EV-TP-*`, `EV-KM-*`).
- [`CONTRADICTIONS.md`](CONTRADICTIONS.md) — disagreements, reversals and qualifications that must remain visible.
- [`SYNTHESIS.md`](SYNTHESIS.md) — source-neutral `RL-RS-*` findings supported or qualified across multiple sources.
- [`MARKET-OPPORTUNITIES.md`](MARKET-OPPORTUNITIES.md) — product/ecosystem hypothesis map.

## Research pipeline

```text
External source
      ↓
Local corpus / source capture
      ↓
Screening / relevance classification
      ↓
Evidence extraction
      ↓
Source-specific finding
      ↓
Cross-source comparison
      ├── confirms
      ├── qualifies
      ├── contradicts
      └── extends
      ↓
Source-neutral synthesis finding
      ↓
Market/product implication
      ↓
ADR / roadmap / profile / Labs
      ↓
Implementation + field validation
```

## What must be recorded

Research must not depend on conversation memory. Preserve at least:

1. **Source snapshot** — channel/project, collection date, corpus size, transcript source and date range.
2. **Screening decision** — what was selected/excluded and whether the classification is provisional.
3. **Evidence** — source, video, date, observation and later timestamp/deep-link where required.
4. **Finding** — interpretation separated from raw evidence.
5. **Contradictions/qualifications** — later or independent evidence that changes scope/confidence.
6. **Decision linkage** — which ADR, roadmap phase, profile, experiment or rejected hypothesis resulted.
7. **Confidence** — why confidence increased or decreased.
8. **Negative decisions** — rejected/deferred ideas and the reason they were not promoted.
9. **Version context** — firmware, platform and device version when capability depends on them.
10. **Validation status** — research-only, prototype-confirmed, field-tested or supported.

## Finding ID policy

- `RL-TP-*` — The Tech Prepper-derived findings.
- `RL-KM-*` — KM4ACK-specific findings that do not map cleanly to an existing item.
- `RL-RS-*` — source-neutral RadioLink Research Synthesis findings supported/qualified across multiple independent sources.

Evidence IDs use:

- `EV-TP-*` — The Tech Prepper evidence;
- `EV-KM-*` — KM4ACK evidence.

## Confidence rule

Confidence should increase when:

- the same problem appears independently across multiple sources;
- the problem appears repeatedly over time;
- a workaround requires substantial custom engineering;
- RadioLink field tests reproduce the same issue;
- manufacturer/protocol documentation supports the technical constraint.

Confidence should decrease or be qualified when:

- evidence reflects only one operator's preference;
- later source material reverses an earlier conclusion;
- the problem is specific to one platform/device/firmware version;
- the proposed solution conflicts with RadioLink's architectural boundaries.

## Promotion rule

A research idea can be promoted into the core product only when it has:

1. a clearly stated problem;
2. source evidence;
3. a RadioLink-specific implication;
4. architectural fit;
5. an explicit roadmap/ADR/profile destination;
6. a validation plan.

Experimental ideas that fail this test remain in Labs rather than expanding the MVP.

## Current next research action

Deep mine KM4ACK classes A and B, then use targeted searches to catch false negatives in C/D. Promote only findings that survive evidence review into `SYNTHESIS.md` and the product roadmap.
