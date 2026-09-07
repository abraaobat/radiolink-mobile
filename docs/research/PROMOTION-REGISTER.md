# RadioLink Research Promotion Register

This register prevents research insights from being lost between source analysis and implementation.

It tracks whether a synthesis finding has been:

```text
SYNTHESIZED
   ↓
ADOPTED or DEFERRED
   ↓
ADR-LINKED
   ↓
ROADMAP-LINKED
   ↓
IMPLEMENTED
   ↓
BENCH-VALIDATED
   ↓
FIELD-VALIDATED / SUPPORTED
```

Not every finding should reach implementation. Deferred/Labs findings are intentionally preserved so they do not reappear later as undocumented “new ideas.”

## Current register

| Finding | Decision | Architecture / product destination | Roadmap destination | Implementation | Validation |
|---|---|---|---|---|---|
| RL-RS-001 — integration complexity | ADOPTED | Operations Engine, Capability Registry, Transport Manager, Profiles | F1, F6, F13, F14 | pending | pending |
| RL-RS-002 — intent above protocols | ADOPTED | mission-first UX / application shell | F1, F5, F7, F10, F11 | pending | pending |
| RL-RS-003 — smartphone-first | ADOPTED | shared core + native mobile hosts | F10, F11; CLI/desktop as validation path | partial foundation | pending |
| RL-RS-004 — KISS/TNC independent from transport | ADOPTED | ADR-0004; TNC/Modem Provider abstraction | F1, F3, F14 | initial abstractions only | pending |
| RL-RS-005 — BLE convenience, not universal capability | ADOPTED | ADR-0004; capability-driven BLE | F2, F13, F14 | pending | pending |
| RL-RS-006 — USB first-class | ADOPTED | ADR-0004; Wired Digital path | F6, F14 | pending | pending |
| RL-RS-007 — multi-transport capability composition | ADOPTED | ADR-0004 + ADR-0005; logical `RadioDevice` composition | F1, F9, F13 | pending | pending |
| RL-RS-008 — profiles as operational recipes | ADOPTED | RadioLink Profiles / device registry | F13, F14 | registry/profile structure created | bench validation pending |
| RL-RS-009 — offline-first | ADOPTED | local-first core/services | F7, F10, F11 | pending | pending |
| RL-RS-010 — companion software can outperform radio UI | ADOPTED | mobile/desktop application UX | F5, F7, F10, F11 | pending | pending |
| RL-RS-011 — embedded intelligence vs raw signal access | VALIDATE | RadioLink Bridge hypothesis | F15 | not started | prototype/field validation required |
| RL-RS-012 — store-and-forward relevance | DEFERRED / LABS | modern resilient messaging | F17 | not started | research/prototype required |
| RL-RS-013 — no mandatory SBC appliance | ADOPTED | cross-platform Rust core; no-Pi runtime requirement | F0/F1 baseline | foundation adopted | ongoing host validation |
| RL-RS-014 — location/time as context providers | ADOPTED | ADR-0005; `LocationProvider` / `TimeProvider` | F1, F6, F7, F10, F11 | pending | pending |
| RL-RS-015 — layered diagnostics | ADOPTED | ADR-0005; diagnostic state model | F1, F2, F6, F14 | pending | pending |
| RL-RS-016 — messaging independent from delivery mechanism | DEFERRED / LABS | future Messaging Service boundary | F17 / post-MVP | not started | research/prototype required |
| RL-RS-017 — practice and self-test | DEFERRED / LABS | future readiness/self-test tooling | F17 | not started | research required |
| RL-RS-018 — off-grid information/data services | DEFERRED / LABS | future Data Services | F17 | not started | research required |

## Promotion checklist

Before changing an item from `ADOPTED` to implementation work, verify:

1. source evidence remains linked in `EVIDENCE.md`;
2. contradictory/qualifying evidence is represented in `CONTRADICTIONS.md`;
3. the synthesis finding remains accurate in `SYNTHESIS.md`;
4. the architectural destination is explicit;
5. a roadmap task exists;
6. the implementation has a concrete validation plan;
7. bench/field evidence is captured when completed.

## Negative-decision rule

A finding marked `DEFERRED`, `LABS` or `REJECTED` must not be silently promoted later. Promotion requires an explicit update to this register with new evidence/reasoning.

## Device-related validation

Device support follows the separate evidence ladder in `docs/devices/README.md`.

Research claims about a radio do not count as RadioLink support. Promotion to support requires profile and bench evidence for the exact device + firmware + host + transport + provider + service combination.
