# RadioLink Research Synthesis

**Status:** Active  
**Sources currently compared:** The Tech Prepper + KM4ACK  
**Started:** 2026-09-06

This document contains **source-neutral synthesis findings**. A synthesis finding is created only when a problem or architectural lesson has evidence from more than one independent research source, or when a second source materially qualifies the first.

## Validation labels

- `CONFIRMED` — independently supported by both sources.
- `QUALIFIED` — second source supports the core idea but changes its scope or interpretation.
- `EXTENDED` — second source adds a new architectural/product consequence.
- `OPEN` — insufficient independent evidence.

---

## RL-RS-001 — Integration complexity is a systemic digital-radio problem

**Status:** CONFIRMED  
**Confidence:** Very High

### The Tech Prepper

Shows repeated friction across radio, interface, cable, sound card, PTT/CAT, software and mode configuration, then builds EmComm Tools to hide that complexity.

### KM4ACK

Build-a-Pi and Pat Menu exist for the same reason: preconfigure Direwolf/AX.25/software stacks, start the right components and avoid fragile manual config editing.

### RadioLink conclusion

The core product opportunity is the integration/orchestration layer, not another isolated protocol client.

### Product destination

- Operations Engine;
- Capability Registry;
- Transport Manager;
- Profiles;
- mission-first UX.

---

## RL-RS-002 — User intent should sit above protocols and daemons

**Status:** CONFIRMED  
**Confidence:** Very High

### The Tech Prepper

Frames the desired experience around actions such as send e-mail, send message and find a station.

### KM4ACK

Pat Menu starts the components needed for HF or packet connections instead of making the operator manually manage each configuration/service.

### RadioLink conclusion

Normal UX should expose messaging, position, e-mail, station discovery and terminal actions. Protocols/providers remain visible in diagnostics/advanced views.

---

## RL-RS-003 — Smartphone-first field operation is independently validated

**Status:** CONFIRMED  
**Confidence:** Very High

### The Tech Prepper

Explores full emergency communications on a phone and later validates phone + BLE/KISS radio workflows, while rejecting desktop-Linux-port UX as the product direction.

### KM4ACK

Builds compact Winlink kits explicitly around the cell phone as the primary communications device and repeatedly uses WoAD/APRSDroid with small radios/TNCs.

### RadioLink conclusion

The smartphone is a credible primary radio computer. Shared core + native mobile UX remains the correct direction.

---

## RL-RS-004 — KISS/TNC must be decoupled from physical transport

**Status:** CONFIRMED  
**Confidence:** Very High

### The Tech Prepper

Uses embedded/external KISS TNCs and software TNC/provider paths behind multiple workflows.

### KM4ACK

WoAD keeps KISS as the TNC type while Bluetooth, USB or TCP/IP can be selected as connection types; APRSDroid also demonstrates KISS over TCP/IP to Direwolf.

### RadioLink conclusion

`KISS/TNC Provider` and `Transport` are independent architectural layers.

---

## RL-RS-005 — BLE is a first-class convenience path, not a universal capability

**Status:** QUALIFIED  
**Confidence:** Very High

### The Tech Prepper

Shows excellent BLE/KISS portability but later questions Bluetooth reliability in emergency-oriented operation.

### KM4ACK

Shows many successful Bluetooth TNC workflows but also demonstrates that Bluetooth capabilities vary widely by radio and firmware: Bluetooth may expose programming/headset/PTT but no TNC, or support APRS before a firmware update adds Winlink/KISS behavior.

### RadioLink conclusion

BLE remains first-class, but:

- Bluetooth presence never implies KISS/TNC;
- firmware/profile context matters;
- reconnect/health/fallback remain required;
- USB remains an equal architectural peer for robust wired operation.

---

## RL-RS-006 — USB is a strategic modern transport, not merely legacy fallback

**Status:** CONFIRMED + EXTENDED  
**Confidence:** Very High

### The Tech Prepper

Later field workflows prefer deterministic wired USB in several contexts.

### KM4ACK

Shows USB audio/serial interfaces, direct USB device workflows and a TH-D75 case where USB-C is used for GPS while Bluetooth simultaneously carries KISS/TNC.

### RadioLink conclusion

USB must be first-class and modeled by the actual logical interfaces it exposes: CDC/serial, KISS, CAT, audio, GPS/data or power-only.

---

## RL-RS-007 — One logical radio may aggregate capabilities from multiple transports

**Status:** EXTENDED  
**Confidence:** High

### New independent evidence

KM4ACK's TH-D75 workflow uses Bluetooth for KISS/TNC and USB-C for GPS because both are not available over the same Bluetooth path in that configuration.

### RadioLink conclusion

A `RadioDevice` must not be constrained to one transport per session. Capabilities/providers may resolve through different active transports and be composed into one logical device state.

### Product destination

- F1 Operations/Capability model;
- F9 mixed-transport control/data support;
- F13 Profiles.

---

## RL-RS-008 — Known-good profiles are operational data

**Status:** CONFIRMED  
**Confidence:** Very High

### The Tech Prepper

Preserves radio/audio/PTT settings and uses reference cards to reproduce working field configurations.

### KM4ACK

Repeated tutorials depend on precise TNC type, Bluetooth/USB connection type, menu settings and radio configuration. Firmware can materially change capabilities.

### RadioLink conclusion

Profiles should persist radio + firmware + interface + cable + transport + provider + calibration + verified-service state.

---

## RL-RS-009 — Offline-first is a repeated operational requirement

**Status:** CONFIRMED  
**Confidence:** Very High

### The Tech Prepper

Offline field appliances and data are a recurring EmComm Tools goal.

### KM4ACK

`MY OFF-GRID COMMS SERVER` is explicitly designed to run completely without Internet while serving APRS and local tools/reference data.

### RadioLink conclusion

Core RF, profiles, diagnostics and local history must not depend on cloud services.

---

## RL-RS-010 — Companion software can outperform the radio's native UX

**Status:** CONFIRMED + EXTENDED  
**Confidence:** High

### The Tech Prepper

Repeatedly moves operator interaction to a better computing surface while leaving the radio as RF/TNC hardware.

### KM4ACK

Explicitly criticizes APRS message-entry UI across expensive and inexpensive radios and uses phone applications to provide the usable interface.

### RadioLink conclusion

Embedded APRS/TNC capability strengthens RadioLink rather than replacing it. RadioLink's value includes human interface, state, workflow and interoperability above the radio firmware.

---

## RL-RS-011 — Hardware interface choice is a trade-off between embedded intelligence and generic signal access

**Status:** CONFIRMED + EXTENDED  
**Confidence:** High

### KM4ACK evidence

Mobilinkd is a true Bluetooth TNC with excellent phone/packet convenience, while DigiRig exposes audio + serial/CAT and supports broader software-modem workflows such as VARA FM but requires a host/software TNC and has no built-in Bluetooth in the compared device.

### RadioLink conclusion

The future RadioLink Bridge should not merely clone Mobilinkd or DigiRig. Its strongest design hypothesis is to combine:

- BLE + USB-C;
- KISS/TNC capability;
- raw/audio modem path;
- PTT;
- optional CAT/control;
- capability discovery.

This remains a product hypothesis until prototype/field validation.

---

## RL-RS-012 — Store-and-forward messaging remains relevant but is not MVP core

**Status:** CONFIRMED  
**Confidence:** Medium-High

### The Tech Prepper

Explores BBS/private messaging/bulletins/files over radio networks.

### KM4ACK

Builds a Meshtastic BBS and explores APRS bulletins/Winlink messaging patterns.

### RadioLink conclusion

Modern resilient local messaging remains a legitimate Labs opportunity, but APRS/Packet foundations should be completed first.

---

## Open questions after first cross-source pass

1. How often do BLE failures occur in normal field use versus configuration/firmware problems?
2. Which USB interfaces are realistically accessible on Android and iOS without platform-specific accessory constraints?
3. How broadly do users want automatic provider/transport selection versus explicit advanced control?
4. Which RadioLink Profile fields can be auto-detected and which require community/manual validation?
5. Does a combined BLE + USB-C + KISS + audio Bridge solve enough pain to justify custom hardware?
6. How much demand exists for network/TCP KISS outside desktop/node workflows?

These questions remain research/field-test targets and should not be converted into assumptions.
