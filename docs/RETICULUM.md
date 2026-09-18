# Reticulum — RadioLink Labs Integration Plan

**Status:** LABS / DEFERRED IMPLEMENTATION

**Roadmap home:** F17 — RadioLink Labs / Research

**Purpose:** preserve Reticulum as a deliberate future experiment without changing the current APRS/Packet priorities or coupling RadioLink Core to one network stack.

## Why it fits RadioLink

Reticulum is relevant because RadioLink is already moving toward transport-neutral, offline-first communication with explicit separation between device, transport, provider, protocol and service layers.

The research corpus also shows a recurring product problem around Reticulum: multi-transport networks can become configuration-heavy, and plug-and-play interface discovery/configuration can materially improve field usability.

Reticulum therefore fits best as an optional network/provider module, not as a replacement for APRS, Packet, Meshtastic, KISS or the Operations Engine.

## Why implementation is deferred

The current priority is still to prove the base RadioLink pipeline with real hardware and real packets.

Adding Reticulum before that point would mix two different validation problems:
- stabilizing RadioLink transport/provider/core boundaries;
- validating a second networking stack, topology model and configuration workflow.

That would make failures harder to isolate and could distort the MVP architecture around an experimental dependency.
## Entry gate for the first Reticulum PoC

The Reticulum PoC may begin only after all of the following are true:

1. **F3 KISS Transport Core** is stable enough for real streaming use.
2. **F4 AX.25 + APRS Core** has a validated bidirectional subset with captured/live fixtures.
3. At least one **real end-to-end RadioLink path** is bench-validated through the Operations Engine. The preferred first path remains DigiRig + Dire Wolf TCP KISS on macOS/Linux.
4. Transport, TNC/Modem Provider and service boundaries are stable enough that a Reticulum experiment can be added without modifying APRS/Packet internals.
5. The experiment has a concrete use case that is not already better served by APRS or the Meshtastic-first F19 roadmap.

This means Reticulum is documented now, but implementation does **not** interrupt the current F1/F3/F4 integration work.

## PoC sequence

### R0 — Desktop sandbox
- Run Reticulum outside RadioLink Core.
- Record supported upstream interfaces/transports and configuration requirements.
- Confirm licensing, packaging and host-platform constraints.
- Produce repeatable local test notes.

### R1 — Single-link provider experiment
- Validate one predictable physical/network path.
- Prefer a wired path in the first bench test where practical.
- Measure startup, reconnect, identity/session behavior and failure modes.
- Keep all Reticulum-specific state behind an experimental provider boundary.
### R2 — Configuration and discovery experiment
- Test device/interface discovery.
- Evaluate generated or profile-driven interface configuration.
- Map failures into RadioLink layered diagnostics.
- Determine which configuration can be represented by existing RadioLink Profiles.

### R3 — Multi-transport experiment
- Evaluate whether RadioLink can present multiple Reticulum interfaces without making the Operations Engine Reticulum-specific.
- Compare the operator workflow with the Meshtastic-first provider model.
- Evaluate AX.25/Dire Wolf interoperation only if upstream support and the exact architecture are independently verified.

### R4 — Promotion decision
Reticulum moves beyond Labs only if the PoC demonstrates clear product value and clean architectural fit.

Possible outcomes:
- **PROMOTE** — define a dedicated provider/module roadmap.
- **HOLD IN LABS** — useful experiment, insufficient product value or maturity.
- **REJECT FOR PRODUCT** — preserve research notes but do not integrate runtime support.

## Promotion criteria

Promotion requires:
- a clear RadioLink user workflow;
- no mandatory Reticulum dependency in Core;
- compatibility with the existing Operations Engine/provider model;
- repeatable bench evidence;
- acceptable reconnect and field-recovery behavior;
- configuration complexity that RadioLink can materially reduce;
- a documented relationship to APRS/Packet and Meshtastic so responsibilities do not overlap ambiguously;
- host/platform packaging that is supportable on the intended targets.

## Guardrails

- Reticulum remains optional.
- APRS/Packet remains independently usable.
- F19 remains Meshtastic-first for the planned mesh/situational track unless a later ADR changes that decision.
- Do not hard-code Reticulum assumptions into generic Transport, TNC/Modem Provider, Messaging or Situational interfaces.
- RF operation must be evaluated against the rules applicable to the band, service and jurisdiction used in a given test.

## Research basis

Current internal research references:
- `docs/research/THE-TECH-PREPPER.md` — Reticulum plug-and-play and wired-reliability observations.
- `docs/research/FINDINGS.md` — transport-neutral networking fit and reasons to keep Reticulum/LoRa in Labs.
- `docs/research/MARKET-OPPORTUNITIES.md` — OPP-008, Multi-transport / Reticulum Network Manager.
- `docs/research/CONTRADICTIONS.md` — Bluetooth convenience versus field-reliability concerns.
