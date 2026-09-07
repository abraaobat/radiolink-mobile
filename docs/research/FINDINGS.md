# RadioLink Research Findings

**Status:** Active  
**Initial source:** The Tech Prepper YouTube corpus  
**Started:** 2026-09-06

This document records research findings that may influence RadioLink. A finding is not automatically a feature request. Each item separates observed evidence from the product decision or hypothesis derived from it.

## Status legend

- `ADOPTED` — already represented in product/architecture/roadmap.
- `VALIDATE` — promising but needs more independent evidence or implementation testing.
- `LABS` — deliberately outside the MVP.
- `REJECTED` — researched and intentionally not adopted.

---

## RL-TP-001 — Digital-radio workflows are integration-heavy

**Status:** ADOPTED  
**Confidence:** High

### Evidence

`Why are digital data modes so difficult?` (2022-04-25) demonstrates that a simple APRS/packet goal can require a radio, an interface, radio-specific cabling, operating-system setup, sound-card configuration, serial/PTT configuration and application-specific settings.

Source: https://www.youtube.com/watch?v=hL-TrpGzEkg

### Interpretation

The principal user problem is often not lack of protocol capability; it is integration complexity across hardware and software boundaries.

### RadioLink consequence

RadioLink must own the integration layer through device, capability, transport and provider abstractions rather than exposing raw setup complexity as the normal workflow.

### Implemented in

- `docs/ARCHITECTURE.md`
- `docs/adr/ADR-0004-three-path-io-and-platform-ecosystem.md`
- F1 / F13 / F14 roadmap work

---

## RL-TP-002 — User intent should be above protocol/application selection

**Status:** ADOPTED  
**Confidence:** High

### Evidence

`The EmComm Tools Story` (2022-03-04) describes the desired workflow in terms of outcomes: send an e-mail, send a text message or find another station, without remembering the separate software, startup procedures and message formats required by each mode.

Source: https://www.youtube.com/watch?v=HFdXTk8Zp5c

### Interpretation

A mission-oriented UX is more durable than an app/protocol-oriented launcher.

### RadioLink consequence

Normal UX should prioritize actions such as messaging, position, e-mail, station discovery and terminal access. Protocol details remain available for diagnostics and advanced use.

### Implemented in

- `README.md`
- `docs/PRODUCT.md`
- Operations Engine direction in F1

---

## RL-TP-003 — Offline-first operation is a core requirement

**Status:** ADOPTED  
**Confidence:** High

### Evidence

The EmComm Tools evolution repeatedly treats operation without infrastructure as a design goal, including offline call-sign/location data and rapid field deployment.

Primary source: https://www.youtube.com/watch?v=HFdXTk8Zp5c

### Interpretation

Internet connectivity may enhance RadioLink but cannot be required for core RF workflows.

### RadioLink consequence

Device state, profiles, message history and primary RF workflows must function locally whenever the underlying radio mode permits it.

---

## RL-TP-004 — The smartphone can be the radio computer, but mobile must be native

**Status:** ADOPTED  
**Confidence:** High

### Evidence

`A breakthrough! Full Emergency Comms on a Phone` (2025-03-01) proves that much of a Linux radio stack can run on a PinePhone. The `2025 TTP Recap` later reports that the path did not work well in practice because desktop radio applications were poorly suited to a small display, terminal interaction was difficult, the keyboard was problematic and tested runtime was under roughly two hours.

Sources:

- https://www.youtube.com/watch?v=RDgd4QLNL8M
- https://www.youtube.com/watch?v=bnsv_gGBzjM

### Interpretation

The smartphone-first thesis is valid; the desktop-porting strategy is not.

### RadioLink consequence

Use a shared core with native host layers and mobile-native UX. Do not attempt to ship desktop Linux applications inside the phone as the product architecture.

### Implemented in

- ADR-0002 cross-platform App Hub architecture
- ADR-0003 Rust shared core
- Android/iOS host roadmap

---

## RL-TP-005 — BLE/KISS enables the highest-convenience mobile path

**Status:** ADOPTED  
**Confidence:** High

### Evidence

`Radio Packet Terminal on your Phone using WoAD` (2025-03-15) demonstrates Android + BTECH UV-Pro using the radio's embedded KISS TNC over Bluetooth, allowing the field workflow to collapse toward everyday phone + HT with no separate computer and no host-to-radio cable.

Source: https://www.youtube.com/watch?v=pcdh4R6kdvo

### Interpretation

A documented BLE KISS interface is one of the strongest modern radio-to-phone integration patterns.

### RadioLink consequence

Wireless Digital / BLE remains a first-class path and an important reference target for RadioLink, while KISS is modeled independently from Bluetooth itself.

---

## RL-TP-006 — Bluetooth convenience must not be confused with reliability

**Status:** ADOPTED  
**Confidence:** High

### Evidence

The corpus contains both positive BLE mobile demonstrations and later explicit preference for wired links. In `I am DONE with Reticulum` (2026-08-05), the creator describes Bluetooth as hit-or-miss and brittle in personal emergency-communications use. In `Reticulum - Plug-and-Play Configuration Demo` (2026-06-17), the preferred RNode workflow is wired USB rather than Bluetooth.

Sources:

- https://www.youtube.com/watch?v=E8yarbRFtb4
- https://www.youtube.com/watch?v=UsP5BGs2uuA

### Interpretation

BLE is strategically valuable but cannot be the architecture itself.

### RadioLink consequence

- explicit connection state machine;
- health monitoring;
- reconnect/session restoration;
- USB fallback;
- transport preference policy by operational context.

### Implemented in

- ADR-0004 three-path I/O
- F1 session/recovery model
- F2 BLE reconnect requirements

---

## RL-TP-007 — USB deserves first-class status alongside BLE

**Status:** ADOPTED  
**Confidence:** High

### Evidence

Later field-oriented workflows repeatedly prefer known wired USB devices for predictable detection and operation. The Reticulum plug-and-play demo uses USB device discovery to bring up a known interface automatically.

Source: https://www.youtube.com/watch?v=UsP5BGs2uuA

### Interpretation

USB is not merely a legacy fallback. It is a modern high-reliability transport capable of carrying serial/data, audio, control and other logical interfaces.

### RadioLink consequence

The official I/O model treats **Wireless Digital (BLE)** and **Wired Digital (USB)** as parallel first-class modern paths, plus the legacy/analog audio/PTT compatibility path.

---

## RL-TP-008 — Mode switching requires lifecycle/resource orchestration

**Status:** ADOPTED  
**Confidence:** High

### Evidence

`Packet Radio Field Test - EmComm Tools Mode Switcher` (2024-11-24) demonstrates switching from APRS to Winlink by stopping existing services and restarting the required stack automatically. The same workflow detects CAT and audio before operation.

Source: https://www.youtube.com/watch?v=feVLHX2ypYw

### Interpretation

An operational mode is not just a launcher entry. It owns resources and requires deterministic setup/teardown.

### RadioLink consequence

The Operations Engine is responsible for:

- capability resolution;
- resource ownership;
- lifecycle management;
- provider/transport selection;
- recovery to a known state.

---

## RL-TP-009 — Device/interface profiles can eliminate repeated field calibration

**Status:** ADOPTED  
**Confidence:** High

### Evidence

`How to send email with the DigiRig Lite on Android` (2024-07-03) requires specific TNC/audio settings, radio volume, Android media volume and PTT configuration. The creator notes that WoAD and APRSdroid require different practical levels and uses reference cards to preserve working settings.

Source: https://www.youtube.com/watch?v=zQ3Og2r1yj8

### Interpretation

Known-good configuration is valuable data and should be persisted, shared and validated instead of rediscovered each session.

### RadioLink consequence

RadioLink Profiles should store, where relevant:

- radio model;
- interface/cable;
- transport;
- audio RX/TX levels;
- PTT method;
- CAT support;
- TNC/modem provider;
- verified modes;
- platform-specific quirks.

---

## RL-TP-010 — Modem implementations should sit behind compatible provider interfaces

**Status:** ADOPTED  
**Confidence:** High

### Evidence

The 2026 Mercury HF experiments integrate Mercury into existing Winlink, BBS and EmComm Tools workflows while comparing it with VARA HF. The creator emphasizes the value of an open protocol that can be independently implemented and integrated.

Sources:

- https://www.youtube.com/watch?v=GQ9odnuUcQE
- https://www.youtube.com/watch?v=4LaFSDcLHus

### Interpretation

Applications should consume stable service/provider boundaries rather than bind to one modem implementation.

### RadioLink consequence

Preserve a `TNC/Modem Provider` layer so future providers such as hardware KISS TNCs, Direwolf, Mercury, ARDOP or other modem implementations can be substituted without rewriting application/service logic.

---

## RL-TP-011 — Compatibility and known-good hardware can matter more than maximum theoretical performance

**Status:** ADOPTED  
**Confidence:** Medium-High

### Evidence

Across the corpus the creator repeatedly standardizes hardware and favors field-proven configurations. Experiments with higher-speed packet and alternative stacks are evaluated against practical reach, compatibility and user availability rather than raw throughput alone.

### Interpretation

RadioLink should optimize for interoperable, recoverable and documented operation before chasing every mode or maximum data rate.

### RadioLink consequence

- conservative MVP scope;
- compatibility matrix;
- reference devices;
- verified profiles;
- explicit experimental status for unproven paths.

---

## RL-TP-012 — BBS/store-and-forward remains useful when modernized around workflow

**Status:** LABS  
**Confidence:** Medium

### Evidence

The 2025–2026 BBS work demonstrates private messages, bulletins, files, local information and multiple radio transports in a radio-only community hub. Modern client/plug-and-play layers are repeatedly added to make older packet infrastructure easier to use.

Representative source: https://www.youtube.com/watch?v=i-YBHj1_Df0

### Interpretation

The value is not nostalgia for BBS command syntax; it is resilient local store-and-forward messaging and information distribution.

### RadioLink consequence

Keep a future modern off-grid messaging/BBS concept in Labs. Do not add it to the MVP until APRS/Packet foundations are stable.

---

## RL-TP-013 — Multi-transport networking is strategically interesting but should not inflate the MVP

**Status:** LABS  
**Confidence:** Medium

### Evidence

The Reticulum series explores the same identity/application layer across LoRa, Wi-Fi, AX.25 and other transports, but also exposes substantial configuration and topology complexity.

Representative source: https://www.youtube.com/watch?v=UsP5BGs2uuA

### Interpretation

Transport-neutral networking aligns conceptually with RadioLink architecture, but Reticulum/LoRa is a separate product problem from the initial APRS/Packet mobile integration goal.

### RadioLink consequence

Reticulum and LoRa remain in RadioLink Labs until independently justified.

---

## RL-TP-014 — Research findings must remain traceable to source evidence

**Status:** ADOPTED  
**Confidence:** High

### Evidence

The Tech Prepper corpus spans multiple years and contains evolving or even contradictory preferences. A single snapshot can therefore misrepresent the later operational conclusion.

### Interpretation

RadioLink needs provenance for why architectural decisions were adopted.

### RadioLink consequence

Use the chain:

```text
source → finding → product implication → ADR/roadmap → implementation/test
```

This document is the first implementation of that rule.

---

# Cross-source validation queue

When the KM4ACK corpus becomes available, explicitly test these hypotheses independently:

1. digital-mode setup complexity is a recurring user problem;
2. radio/interface/cable fragmentation creates repeated work;
3. field users value zero-configuration and known-good profiles;
4. smartphones are credible primary field computers;
5. BLE convenience is offset by reliability/lifecycle concerns;
6. USB remains strategically important for robust operation;
7. KISS/TNC abstraction improves interoperability;
8. offline-first operation is a recurring requirement;
9. users benefit from outcome-oriented workflows rather than protocol-first UI;
10. modernized store-and-forward messaging has continued operational value.

A finding that appears independently in multiple technical sources should receive higher confidence and may justify roadmap reprioritization.
