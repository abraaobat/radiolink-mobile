# RadioLink Device Registry

This directory is the nominal registry of radios, TNCs and interfaces used to validate RadioLink against real hardware.

The registry is intentionally separate from `docs/COMPATIBILITY.md`:

- `COMPATIBILITY.md` defines **capability/transport classes**;
- this directory records **specific devices**, their priority, evidence and validation state.

Being listed here does **not** mean a device is supported.

## Validation-state ladder

Devices move through an explicit evidence ladder:

```text
RESEARCHED
   ↓
CANDIDATE
   ↓
LAB_AVAILABLE
   ↓
CONNECTED
   ↓
RX_VERIFIED
   ↓
TX_VERIFIED
   ↓
PROFILE_VERIFIED
   ↓
SUPPORTED
```

Definitions:

- `RESEARCHED` — appears in source evidence/documentation relevant to RadioLink.
- `CANDIDATE` — selected as a useful architectural/reference target.
- `LAB_AVAILABLE` — physically available to the RadioLink test lab, but not yet validated.
- `CONNECTED` — RadioLink/host can establish the intended transport path.
- `RX_VERIFIED` — intended receive workflow is reproduced with captured evidence.
- `TX_VERIFIED` — intended transmit workflow is reproduced with captured evidence.
- `PROFILE_VERIFIED` — firmware, interface, cable/settings, capabilities and limitations are recorded in a reproducible profile.
- `SUPPORTED` — the documented workflow has passed the project support criteria on the stated host/platform/version combination.

A device may legitimately remain at different states for different transports or services.

## Priority levels

- **P0 — Current lab:** hardware already available and used for immediate CLI-first validation.
- **P1 — Reference target:** high-value device that proves a distinct modern architecture and should be acquired, borrowed or otherwise tested when practical.
- **P2 — Boundary/negative-control target:** useful for proving that the capability model handles limitations correctly.
- **WATCH — Research watchlist:** interesting device, but insufficient evidence or priority for a formal reference target.

## Support rule

RadioLink never promotes a device to `SUPPORTED` from research evidence alone.

A supported profile should preserve, where applicable:

- manufacturer/model and hardware revision;
- firmware version/range;
- host platform and OS version;
- physical connection/interface;
- transport(s);
- logical interfaces/capabilities;
- TNC/modem provider;
- required radio-side settings/preflight recipe;
- cable/adapter identity;
- audio calibration when relevant;
- receive/transmit test evidence;
- limitations/quirks;
- protocol/service validation state.

Runtime discovery remains authoritative where technically possible; profiles capture known-good context and constraints.

## Files

- [`REGISTRY.md`](REGISTRY.md) — prioritized nominal device list.
- [`PROFILE-TEMPLATE.md`](PROFILE-TEMPLATE.md) — profile schema for manual/device validation.
- `profiles/` — per-device profile records.

## Research linkage

Nominal candidates derived from research must link back to:

- `docs/research/EVIDENCE.md`;
- `docs/research/SYNTHESIS.md`;
- relevant ADRs/roadmap phases.

This preserves the chain:

```text
source → evidence → finding → architecture/roadmap → device profile → bench test → support state
```
