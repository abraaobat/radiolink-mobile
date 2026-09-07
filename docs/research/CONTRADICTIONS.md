# RadioLink Research — Contradictions and Qualifications

**Status:** Active

This document preserves disagreements, reversals and source-specific limits so the project does not flatten nuanced evidence into false certainty.

---

## CQ-001 — Bluetooth: excellent convenience, variable operational confidence

### The Tech Prepper

- Demonstrates a very attractive phone + radio BLE/KISS workflow.
- Later describes Bluetooth as hit-or-miss/brittle in personal emergency-communications use and prefers wired USB in some Reticulum workflows.

### KM4ACK

- Repeatedly demonstrates successful Bluetooth TNC workflows with Mobilinkd, Kenwood TH-D75, VGC-class radios and BTECH UV-Pro-class devices.
- Current first-pass evidence does **not** independently support a blanket conclusion that Bluetooth is inherently unreliable.
- KM4ACK instead strongly demonstrates that Bluetooth capability is fragmented by radio/firmware/profile: APRS may work when Winlink does not; programming/headset/PTT Bluetooth may exist without TNC access.

### RadioLink synthesis

Bluetooth remains a first-class, high-convenience path, but the architecture must not depend on it exclusively.

**Decision remains:** BLE + USB are first-class modern transports; health/reconnect/fallback are required; capability discovery must be explicit.

---

## CQ-002 — Embedded APRS does not eliminate the need for a phone application

### Possible assumption

A radio with native APRS could be considered complete enough to make a companion application unnecessary.

### KM4ACK evidence

Several APRS radios are criticized for poor text-entry/message UX, and external applications such as APRSDroid are used specifically to improve the operator experience.

### RadioLink synthesis

The radio may own RF/TNC capability while the smartphone owns the human interface. Native APRS in the radio is therefore a capability provider, not a replacement for RadioLink.

---

## CQ-003 — One device does not necessarily equal one transport

### Simplifying assumption

A connected radio could be modeled as using one transport for the whole session.

### KM4ACK evidence

The TH-D75 workflow uses Bluetooth for KISS/TNC and USB-C for GPS data because the required capabilities are not simultaneously available over Bluetooth.

### RadioLink synthesis

A logical `RadioDevice` may aggregate capabilities across multiple active transports. Transport selection can occur per capability/provider, not only per device.

---

## CQ-004 — USB-C connector presence does not imply a useful RadioLink data path

### Common marketing assumption

A radio advertising USB-C may appear to be a modern digital-data radio.

### Evidence pattern

Across radio products, USB-C may represent charging only, programming/serial, audio, CAT, KISS or some combination. KM4ACK also documents common data-vs-charge cable confusion at the host side.

### RadioLink synthesis

`USB = true` is insufficient. RadioLink must identify actual USB interfaces/capabilities: enumeration, CDC/serial, audio, KISS, CAT, power-only or other exposed functions.

---

## CQ-005 — TNC hardware versus sound-card interface

### Mobilinkd-style path

- compact true TNC;
- Bluetooth/mobile friendly;
- very suitable for APRS/packet;
- may not support broader sound-card modem workflows.

### DigiRig-style path

- USB audio + serial/CAT/PTT interface;
- no embedded TNC;
- requires host/software modem;
- supports broader modem families such as VARA FM;
- no built-in Bluetooth in the compared device.

### RadioLink synthesis

Do not collapse these into one hardware class. Preserve `TNC/Modem Provider` separately from `Transport/Interface`. The future RadioLink Bridge may intentionally combine capabilities from both categories.

---

## Policy

When later research changes a conclusion:

1. append the newer evidence;
2. preserve the older evidence;
3. state whether the conflict is version-, platform-, radio- or operator-specific;
4. update confidence rather than deleting the contradiction;
5. link any resulting architecture/roadmap change.
