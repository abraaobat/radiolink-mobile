# RadioLink — Market Opportunities from Research

**Status:** Active hypothesis map  
**Sources:** The Tech Prepper + KM4ACK  
**Started:** 2026-09-06

This document separates **market/product opportunities** from the RadioLink implementation roadmap.

A research signal does not automatically belong in RadioLink Mobile. Opportunities are ranked by fit, evidence strength and architectural coherence.

## Opportunity ranking

| Rank | Opportunity | Current disposition | Evidence strength |
|---|---|---|---|
| 1 | RadioLink Platform / Mobile | Core product | Very High |
| 2 | RadioLink Bridge | Platform hardware track | High |
| 3 | RadioLink Profiles / Compatibility Registry | Platform data/service track | Very High |
| 4 | RadioLink Ready interoperability profile | Strategic ecosystem track | High |
| 5 | Resilient Messaging / store-and-forward | Labs / post-MVP service layer | High |
| 6 | Offline Radio Knowledge Assistant | Labs / possible separate product | Medium |
| 7 | Off-grid Data Services | Labs / service plugins | Medium |
| 8 | Multi-transport / Reticulum Network Manager | Labs | Medium |
| 9 | Field Readiness / Self-Test | Labs / operations feature | Medium |
| 10 | Field power / device telemetry | Adjacent feature opportunity | Low-Medium |

---

## OPP-001 — RadioLink Platform

### Problem

Amateur-radio digital operation is fragmented across radios, interfaces, cables, PTT/CAT methods, TNCs/modems, operating-system configuration and protocol-specific applications.

### Cross-source evidence pattern

The Tech Prepper repeatedly builds EmComm Tools integration/orchestration rather than new RF protocols. KM4ACK independently builds Build-a-Pi, Pat Menu, connection scripts and preconfigured workflows to hide the same setup burden.

### Product thesis

> RadioLink is the capability-driven layer between user intent and radio infrastructure.

```text
User goal
   ↓
Operations Engine
   ↓
Service / Protocol
   ↓
TNC / Modem / Context Provider
   ↓
Transport
   ↓
Device
   ↓
RF
```

### Why this belongs in the core

It addresses the broadest and most repeated research problem while preserving cross-platform reuse.

### Current disposition

**CORE — already adopted.**

---

## OPP-002 — RadioLink Bridge

### Problem

Many useful radios have no modern documented KISS/data interface. Users therefore need radio-specific cables, audio/PTT interfaces and sometimes external TNC hardware.

### Cross-source evidence pattern

DigiRig and Mobilinkd solve different layers. KM4ACK's direct comparison is especially useful: Mobilinkd provides an embedded Bluetooth TNC and excellent phone/packet convenience, while DigiRig exposes generic audio + serial/CAT and supports broader software-modem workflows such as VARA but requires a host/software modem.

### Product thesis

A compact accessory can combine the strongest properties of both interface families:

```text
Radio
 ├─ audio RX/TX
 ├─ PTT
 └─ optional CAT
       ↓
RadioLink Bridge
 ├─ KISS/TNC
 ├─ raw audio/modem path
 ├─ BLE
 └─ USB-C
       ↓
RadioLink host
```

### Candidate capabilities

- ESP32-S3-class controller;
- codec/audio interface;
- AFSK/packet modem path;
- KISS;
- BLE;
- USB-C;
- PTT;
- optional CAT;
- firmware update;
- capability discovery;
- profile-driven calibration;
- BLE ↔ USB fallback.

### Current disposition

**PLATFORM HARDWARE TRACK — F15.** Prototype only after shared abstractions are proven with existing hardware.

---

## OPP-003 — RadioLink Profiles / Compatibility Registry

### Problem

Known-good settings are repeatedly rediscovered: cable choice, firmware, radio menu values, audio level, PTT method, CAT behavior, TNC type and platform-specific quirks.

### Cross-source evidence pattern

The Tech Prepper preserves field cards/reference hardware. KM4ACK repeatedly requires precise TNC/connection/menu configuration, and the VGC example shows firmware directly changing Bluetooth/Winlink capability.

### Product thesis

Create a structured compatibility registry consumed by both users and the Operations Engine.

```text
Device profile
├── manufacturer/model
├── firmware/range
├── required radio-side settings
├── interface/cable
├── transport(s)
├── capabilities by transport
├── context sources (GPS/GNSS etc.)
├── audio RX/TX calibration
├── PTT method
├── CAT behavior
├── TNC/modem provider
├── verified services
├── tested platforms
└── known issues
```

### Strategic value

The profile database can become a durable community asset, reduce support cost and improve first-run success.

### Current disposition

**PLATFORM DATA/SERVICE TRACK — F13/F14.**

---

## OPP-004 — RadioLink Ready

### Problem

Manufacturers expose Bluetooth, USB and radio control in incompatible ways. Marketing labels such as “Bluetooth” or “USB-C” do not reveal KISS, TNC, audio, PTT, CAT, GPS or telemetry capabilities.

### Evidence pattern

KM4ACK demonstrates radios where Bluetooth exposes programming/headset/PTT but no TNC, and radios whose firmware later adds a missing TNC/Winlink workflow. The Tech Prepper/Mercury research reinforces the value of documented compatible interfaces.

### Product thesis

Define a documented interoperability profile for radios/TNCs/accessories.

Possible scope:

- identity/discovery;
- firmware/version metadata;
- capability declaration by transport;
- BLE KISS profile;
- USB KISS/CDC profile;
- CAT/control capability metadata;
- PTT/GPS/battery/telemetry where available;
- versioned extension mechanism;
- conformance test concept.

### Current disposition

**STRATEGIC — F16; publish only after core interfaces stabilize.**

---

## OPP-005 — Resilient Messaging / Store-and-forward

### Problem

The operator wants a message delivered; direct APRS requires reachability at the right moment and traditional BBS/bridge mechanisms expose inconsistent user experiences.

### Cross-source evidence pattern

The Tech Prepper explores BBS/private messages/bulletins/files/SITREP workflows. KM4ACK demonstrates:

- APRS store-and-forward for missed recipients;
- APRSLink;
- Winlink-over-APRS;
- APRS message alerts;
- Meshtastic BBS experiments.

### Product thesis

A future RadioLink `Messaging Service` can expose user-level state while protocol-specific delivery providers remain explicit underneath:

```text
Message intent
     ↓
Messaging Service
 ├── APRS direct
 ├── APRS store-and-forward where available
 ├── Winlink / APRSLink integration
 ├── Packet/BBS provider
 └── future local/off-grid providers
```

Candidate UX concepts:

- inbox/private messages;
- delivery/ACK state;
- group/community bulletins;
- SITREP templates;
- local files/information;
- store-and-forward where the underlying service provides it.

### Guardrail

Do not pretend different networks provide identical delivery guarantees or hide amateur-radio rules/limitations.

### Current disposition

**LABS / POST-MVP SERVICE LAYER.** APRS/Packet foundations first.

---

## OPP-006 — Offline Radio Knowledge Assistant

### Problem

Field operators need manuals, procedures, frequencies, configuration notes and troubleshooting information precisely when Internet access may be unavailable.

### Evidence pattern

Both corpora emphasize offline documents, reference cards, known configurations and local knowledge. The Tech Prepper later experiments directly with local AI/knowledge workflows.

### Product thesis

An offline assistant could answer questions using a curated local knowledge base:

- radio manuals;
- RadioLink Profiles;
- operating procedures;
- local frequency plans;
- field checklists;
- known troubleshooting trees.

### Guardrail

Do not mix this into the core protocol stack. It can consume RadioLink state/profile data through a clean interface later.

### Current disposition

**LABS / POSSIBLE SEPARATE PRODUCT.**

---

## OPP-007 — Off-grid Data Services

### Problem

Operators may need useful data such as weather or local reference information precisely when cellular/Internet access is unavailable.

### Evidence pattern

KM4ACK explores multiple RF-based weather retrieval methods and an offline communication/reference server. The Tech Prepper repeatedly builds offline local information/BBS/knowledge systems.

### Product thesis

Optional higher-level services could expose data retrieval independently from transport:

```text
Data request
   ↓
Data Service
 ├── RF/weather provider
 ├── packet/BBS provider
 ├── local offline database
 └── Internet provider when available
```

### Current disposition

**LABS / SERVICE PLUGINS.** Not an APRS decoder responsibility.

---

## OPP-008 — Multi-transport / Reticulum Network Manager

### Problem

Reticulum and similar systems can use multiple transport types, but real deployment requires topology, interface and device configuration that remains technical.

### Evidence pattern

The Tech Prepper Reticulum series repeatedly adds blueprints, plug-and-play discovery and automated interface generation. KM4ACK's mesh/Meshtastic material independently shows a broader demand for resilient local networks, though not necessarily the same architecture.

### Product thesis

A future RadioLink Network layer could orchestrate selected multi-transport/off-grid networks while hiding low-level device configuration.

### Guardrail

Reticulum/LoRa is not a requirement for the initial RadioLink product and should not distort APRS/Packet architecture.

### Current disposition

**LABS.**

---

## OPP-009 — Field Readiness / Self-Test

### Problem

A technically functional radio stack may still fail operationally if the operator rarely practices it or discovers configuration faults only during an emergency.

### Cross-source evidence pattern

The Tech Prepper uses frequent field tests, training and after-action reports. KM4ACK explicitly describes lack of regular emergency-communications practice as a problem and builds repeatable practice workflows.

### Product thesis

A future readiness layer could provide:

- guided radio/transport self-test;
- known-good APRS/Packet test flows;
- connection and context-source verification;
- exercise checklists;
- after-action logs;
- profile validation reminders.

### Current disposition

**LABS / OPERATIONS FEATURE.** Core diagnostics first.

---

## OPP-010 — Field telemetry / hardware health

### Problem

Field communication failures can come from power, temperature, connection state, audio/PTT state or peripheral loss rather than protocol errors.

### Evidence pattern

Both corpora include repeated measurement and monitoring of batteries, temperatures, radios, interfaces and field-system health. The new diagnostics synthesis also shows value in distinguishing power/link failure from protocol failure.

### Product thesis

A future status surface could expose:

```text
Radio       connected
Transport   USB
TNC         ready
Location    phone GPS / locked
Battery     76%
Voltage     13.2 V
Audio/PTT   verified
Temperature 43 °C
```

### Current disposition

**ADJACENT / LOW PRIORITY until supporting hardware exposes useful telemetry.**

---

# Strategic synthesis

The strongest opportunity is not one isolated APRS application. It is a small ecosystem with clean boundaries:

```text
                 RadioLink Platform
                        │
       ┌────────────────┼─────────────────┐
       │                │                 │
  Applications       Bridge          Profiles/Ready
       │                │                 │
       └────────────────┼─────────────────┘
                        │
                 Radio ecosystem
```

The research suggests a defensible sequence:

1. prove the shared Core and CLI;
2. prove real RF paths over existing hardware;
3. prove layered diagnostics and context/provider resolution;
4. deliver mobile-native workflows;
5. build the compatibility/profile asset;
6. prototype Bridge only after abstractions are proven against multiple paths;
7. stabilize interfaces before publishing RadioLink Ready;
8. graduate Messaging/Data/Readiness Labs ideas only after the core is reliable.

# Cross-source rule

The Tech Prepper and KM4ACK provide strong independent evidence, but two channels are still not the entire amateur-radio market.

Before changing core scope based on a broader market hypothesis, compare it against:

- additional technical creators;
- open-source project issues/discussions;
- manufacturer documentation;
- amateur-radio community reports;
- RadioLink field testing.

Repeated independent evidence should raise confidence; conflicting evidence should remain documented rather than silently reconciled.
