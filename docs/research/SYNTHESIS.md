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

Build-a-Pi, Pat Menu and D75 connection scripts exist for the same reason: preconfigure Direwolf/AX.25/software stacks, start the right components and avoid fragile manual setup.

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

Pat Menu and related wrappers start the components needed for HF or packet connections instead of making the operator manually manage each configuration/service.

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

### Evidence

KM4ACK's TH-D75 workflow uses Bluetooth for KISS/TNC and USB-C for GPS because both are not available over the same Bluetooth path in that configuration. This is consistent with RadioLink's existing multi-path capability model.

### RadioLink conclusion

A `RadioDevice` must not be constrained to one transport per session. Capabilities/providers may resolve through different active transports and be composed into one logical device state.

### Product destination

- F1 Operations/Capability model;
- F9 mixed-transport control/data support;
- F13 Profiles.

---

## RL-RS-008 — Known-good profiles are operational data and configuration recipes

**Status:** CONFIRMED + EXTENDED  
**Confidence:** Very High

### The Tech Prepper

Preserves radio/audio/PTT settings, reference hardware and field cards to reproduce working configurations.

### KM4ACK

Repeated tutorials depend on precise TNC type, Bluetooth/USB connection type, radio menu settings and firmware version. Firmware can materially change capabilities, and radios may require several specific menu values before a capability is actually usable.

### RadioLink conclusion

Profiles should persist:

- radio + firmware/version range;
- interface + cable;
- transport + provider;
- required radio-side settings/preflight recipe;
- calibration;
- verified-service state;
- known incompatibilities/quirks.

Profiles should complement runtime discovery, not replace it.

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

Explicitly criticizes APRS message-entry UI across expensive and inexpensive radios and uses phone/tablet applications to provide the usable interface.

### RadioLink conclusion

Embedded APRS/TNC capability strengthens RadioLink rather than replacing it. RadioLink's value includes human interface, state, workflow and interoperability above the radio firmware.

---

## RL-RS-011 — Hardware interface choice is a trade-off between embedded intelligence and generic signal access

**Status:** CONFIRMED + EXTENDED  
**Confidence:** High

### Evidence

Mobilinkd is a true Bluetooth TNC with excellent phone/packet convenience, while DigiRig exposes audio + serial/CAT and supports broader software-modem workflows such as VARA FM but requires a host/software TNC. The Tech Prepper corpus similarly alternates between embedded/external TNC and raw audio/modem paths depending on mission.

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

**Status:** CONFIRMED + EXTENDED  
**Confidence:** High

### The Tech Prepper

Explores BBS/private messaging/bulletins/files and SITREP-style information over radio networks.

### KM4ACK

Builds a Meshtastic BBS and demonstrates APRS store-and-forward, APRSLink and Winlink-over-APRS mechanisms.

### RadioLink conclusion

Modern resilient messaging is a legitimate Labs opportunity. The architectural lesson is broader than BBS: the user's messaging intent can sit above several delivery mechanisms. APRS/Packet foundations remain the MVP priority.

---

## RL-RS-013 — RadioLink must not depend on a special-purpose SBC/appliance host

**Status:** CONFIRMED  
**Confidence:** High

### The Tech Prepper

After practical field use, including high-temperature failures, the current EmComm Tools reference direction moves away from Raspberry Pi and standardizes on host-class 64-bit hardware.

### KM4ACK

`Why I Quit the Raspberry Pi` describes availability, total system cost and ARM compatibility layers as reasons inexpensive x86 laptops/mini-PCs became more practical for many radio-computing workflows.

### RadioLink conclusion

The durable asset is the shared radio stack, not a particular appliance. Android, iOS, Linux and macOS remain first-class hosts; Raspberry Pi may be interoperable later but is not a core runtime requirement.

### Product destination

Already represented by the shared Rust core, CLI-first validation and no-Pi core premise.

---

## RL-RS-014 — Location/time are context providers, not properties owned by one radio

**Status:** CONFIRMED + EXTENDED  
**Confidence:** High

### The Tech Prepper

APRS positioning is discussed using geo-coordinates from a GPS source such as the phone, and later field work separately treats off-grid time synchronization as an operational input.

### KM4ACK

Uses phone GPS, USB GPS, radio GPS/GNSS and mixed BLE+USB workflows; the phone GPS is explicitly reused as a better source in one setup.

### RadioLink conclusion

Services should request context from providers rather than directly from `RadioDevice.gps`.

Candidate abstractions:

```text
LocationProvider
├── Host OS / smartphone
├── Radio GPS/GNSS
├── USB GPS
├── network/GPSD
└── manual/static

TimeProvider
├── system clock
├── GPS/GNSS
├── network time
└── other validated source
```

The Operations Engine/Context Registry resolves the best available provider for an operation.

---

## RL-RS-015 — Diagnostics must be layered from physical transport to service readiness

**Status:** CONFIRMED + EXTENDED  
**Confidence:** High

### The Tech Prepper

Bluetooth pairing/reconnect complexity and preference for deterministic USB in some field workflows show that connection-state failures must be understood below the application layer.

### KM4ACK

Repeated USB support failures are traced to charge-only/unsuitable cables; the radio never had a usable data path even though the user experienced the problem as “the radio won't connect.”

### RadioLink conclusion

Diagnostics should report the highest verified stage:

```text
physical/power
  → enumeration/pairing
  → logical interfaces
  → capability match
  → provider/TNC handshake
  → protocol traffic
  → service readiness
```

Human-readable error states are a product feature, not just logging.

---

## RL-RS-016 — Messaging intent should be independent from the delivery mechanism

**Status:** EXTENDED  
**Confidence:** Medium-High

### The Tech Prepper

BBS/private messaging/SITREP workflows emphasize resilient user communication over the specific lower-level access method.

### KM4ACK

Shows direct APRS messaging, APRS store-and-forward, APRSLink and Winlink-over-APRS, each solving a different delivery/reachability problem.

### RadioLink conclusion

Preserve a higher-level future `Messaging Service` boundary capable of presenting delivery state while using protocol-specific providers underneath. Do not hide regulatory/protocol constraints and do not expand the APRS MVP around every provider now.

### Product destination

Labs / post-MVP messaging architecture.

---

## RL-RS-017 — Operational reliability includes practice and self-test

**Status:** CONFIRMED  
**Confidence:** Medium-High

### The Tech Prepper

Uses repeated field tests, training exercises and after-action reports to make communication procedures reliable rather than merely theoretical.

### KM4ACK

Explicitly identifies lack of frequent emergency-communications practice as a problem and uses repeatable exercises to build proficiency.

### RadioLink conclusion

A future readiness layer can include guided self-test, known-good test paths, exercise checklists and after-action logs. This is an operational product insight, not a reason to enlarge the protocol MVP.

### Product destination

Labs / future field-readiness tooling.

---

## RL-RS-018 — Off-grid information retrieval can be modeled as independent data services

**Status:** CONFIRMED  
**Confidence:** Medium

### The Tech Prepper

Offline reference data, local BBS information and later local knowledge tooling are recurring themes.

### KM4ACK

Demonstrates RF-based weather retrieval and a fully offline local communications/reference server.

### RadioLink conclusion

Weather/reference/knowledge retrieval should be modeled as optional higher-level services, not embedded in transports or the APRS decoder.

### Product destination

Labs / future Data Services.

---

# Open questions after deep cross-source mining

1. How often do BLE failures occur in normal field use versus configuration/firmware problems?
2. Which USB interfaces are realistically accessible on Android and iOS without platform-specific accessory constraints?
3. How broadly do users want automatic provider/transport selection versus explicit advanced control?
4. Which RadioLink Profile fields can be auto-detected and which require community/manual validation?
5. Does a combined BLE + USB-C + KISS + audio Bridge solve enough pain to justify custom hardware?
6. How much demand exists for network/TCP KISS outside desktop/node workflows?
7. Which location/time providers can be safely auto-selected and what precedence/quality rules are required?
8. How should a future Messaging Service expose delivery semantics without pretending different radio networks provide identical guarantees?
9. Which transport diagnostics can be automated on macOS first and later mapped cleanly to Android/iOS?

These questions remain research/field-test targets and should not be converted into assumptions.
