# RadioLink Research

This directory contains traceable research artifacts used to inform RadioLink architecture, roadmap and product hypotheses.

Raw third-party corpora are not committed here. This directory stores only derived findings, source references and decisions.

## Current sources

### The Tech Prepper

- [`THE-TECH-PREPPER.md`](THE-TECH-PREPPER.md) — corpus/source record and methodology.
- [`FINDINGS.md`](FINDINGS.md) — structured findings and RadioLink consequences.
- [`MARKET-OPPORTUNITIES.md`](MARKET-OPPORTUNITIES.md) — product/ecosystem hypothesis map.

### KM4ACK

**Status:** corpus collection / pending ingestion and cross-source validation.

When available, KM4ACK findings should not be placed in a disconnected report. They should be compared against the existing finding taxonomy and used to:

- confirm an existing finding;
- contradict or qualify an existing finding;
- add new evidence;
- create a genuinely new finding.

## Research pipeline

```text
External source
      ↓
Local corpus / source capture
      ↓
Problem / workaround extraction
      ↓
Structured finding
      ↓
Cross-source confidence
      ↓
Market/product implication
      ↓
ADR / roadmap / profile / Labs
      ↓
Implementation + field validation
```

## Finding ID policy

Source-specific finding IDs use a source prefix where useful:

- `RL-TP-*` — The Tech Prepper-derived findings;
- future `RL-KM-*` — KM4ACK-specific findings when they do not map cleanly to existing findings.

As evidence matures across multiple sources, important concepts may later receive source-neutral RadioLink requirement or ADR identifiers.

## Confidence rule

Confidence should increase when:

- the same problem appears independently across multiple sources;
- the problem appears repeatedly over time;
- a workaround requires substantial custom engineering;
- RadioLink field tests reproduce the same issue;
- manufacturer or protocol documentation supports the technical constraint.

Confidence should decrease or be qualified when:

- evidence reflects only one operator's preference;
- later source material reverses an earlier conclusion;
- the problem is specific to one platform/device version;
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
