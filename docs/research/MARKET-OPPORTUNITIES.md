# RadioLink — Market Opportunities from Research

**Status:** Active hypothesis map  
**Initial source:** The Tech Prepper corpus  
**Started:** 2026-09-06

This document separates **market/product opportunities** from the RadioLink implementation roadmap.

A research signal does not automatically belong in RadioLink Mobile. Opportunities are ranked by fit, evidence strength and architectural coherence.

## Opportunity ranking

| Rank | Opportunity | Current disposition | Evidence strength |
|---|---|---|---|
| 1 | RadioLink Platform / Mobile | Core product | High |
| 2 | RadioLink Bridge | Platform hardware track | High |
| 3 | RadioLink Profiles / Compatibility Registry | Platform data/service track | High |
| 4 | RadioLink Ready interoperability profile | Strategic ecosystem track | Medium-High |
| 5 | Modern off-grid messaging / BBS | Labs | Medium |
| 6 | Offline radio knowledge assistant | Labs / possible separate product | Medium |
| 7 | Multi-transport / Reticulum network manager | Labs | Medium |
| 8 | Field power / device telemetry | Adjacent feature opportunity | Low-Medium |

---

## OPP-001 — RadioLink Platform

### Problem

Amateur-radio digital operation is fragmented across radios, interfaces, cables, PTT/CAT methods, TNCs/modems, operating-system configuration and protocol-specific applications.

### Evidence pattern

The Tech Prepper repeatedly builds integration and orchestration layers rather than new RF protocols:

- DigiRig-based integration;
- EmComm Tools mode switching;
- automatic hardware detection;
- zero-configuration workflows;
- mobile experiments;
- provider/modem integration;
- plug-and-play Reticulum interfaces.

### Product thesis

> RadioLink is the capability-driven layer between user intent and radio infrastructure.

```text
User goal
   ↓
Operations Engine
   ↓
Service / Protocol
   ↓
TNC / Modem Provider
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

### Evidence pattern

DigiRig and Mobilinkd-class devices repeatedly solve a valuable boundary: converting conventional radio I/O into something a phone/computer can use. The corpus also shows strong value in standardizing interfaces across multiple radios.

### Product thesis

A small accessory can make legacy/conventional radios look like a modern RadioLink-capable device:

```text
Radio
 ├─ audio RX/TX
 ├─ PTT
 └─ optional CAT
       ↓
RadioLink Bridge
 ├─ modem/TNC
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
- profile-driven calibration.

### Strategic value

This expands the addressable hardware base without embedding radio-specific hacks in the application.

### Current disposition

**PLATFORM HARDWARE TRACK — F15.**

---

## OPP-003 — RadioLink Profiles / Compatibility Registry

### Problem

Known-good settings are repeatedly rediscovered: cable choice, audio level, PTT method, CAT behavior, TNC type and platform-specific quirks.

### Evidence pattern

The 2024 Android + DigiRig experiments require different practical audio levels between applications and use reference cards to preserve working settings. The broader corpus repeatedly standardizes known equipment to reduce field uncertainty.

### Product thesis

Create a structured compatibility registry that can be consumed by both users and the Operations Engine.

Example:

```text
Device profile
├── manufacturer/model
├── interface/cable
├── transport(s)
├── capabilities
├── audio RX/TX calibration
├── PTT method
├── CAT behavior
├── TNC/modem provider
├── verified services
├── tested platforms
└── known issues
```

### Strategic value

The profile database can become a durable community asset and reduce support cost while improving first-run success.

### Current disposition

**PLATFORM DATA/SERVICE TRACK — F13/F14.**

---

## OPP-004 — RadioLink Ready

### Problem

Manufacturers expose Bluetooth, USB and radio control in incompatible ways. A marketing label such as “Bluetooth radio” does not indicate whether a device supports KISS, audio, PTT, CAT or telemetry.

### Evidence pattern

The corpus demonstrates the value of compatible interfaces and the cost of hardware-specific integration. Mercury HF also reinforces the strategic value of interfaces that allow independent provider implementations.

### Product thesis

Define a documented interoperability profile for radios/TNCs/accessories.

Possible scope:

- identity/discovery;
- capability declaration;
- BLE KISS profile;
- USB KISS/CDC profile;
- CAT/control capability metadata;
- battery/GPS/frequency/PTT telemetry where available;
- versioned extension mechanism.

### Strategic value

RadioLink could eventually support manufacturer/community devices without bespoke application logic for each model.

### Current disposition

**STRATEGIC — F16; specification work after core interfaces stabilize.**

---

## OPP-005 — Modern off-grid messaging / BBS

### Problem

Store-and-forward, local bulletins and resilient community messaging remain useful, but traditional BBS command-line UX is inaccessible to many operators.

### Evidence pattern

The Tech Prepper repeatedly develops BBS systems, modern terminal clients, packet/VARA access and plug-and-play mode selection. The operational value centers on local resilient communication rather than nostalgia for historical interfaces.

### Product thesis

A modern messaging layer could expose:

- inbox/private messages;
- group/community bulletins;
- SITREP templates;
- local files/information;
- store-and-forward;
- transport-independent routing where technically appropriate.

### Current disposition

**LABS.** Do not expand the APRS/Packet MVP around this until the core is stable.

---

## OPP-006 — Offline Radio Knowledge Assistant

### Problem

Field operators need manuals, procedures, frequencies, configuration notes and troubleshooting information precisely when Internet access may be unavailable.

### Evidence pattern

The corpus repeatedly emphasizes offline documents, reference cards, known configurations and local knowledge. Later experiments also explore local AI/knowledge workflows.

### Product thesis

An offline assistant could answer questions using a curated local knowledge base:

- radio manuals;
- RadioLink Profiles;
- operating procedures;
- local frequency plans;
- field checklists;
- known troubleshooting trees.

### Guardrail

This should not be mixed into the core protocol stack. It can consume RadioLink state/profile data through a clean interface later.

### Current disposition

**LABS / POSSIBLE SEPARATE PRODUCT.**

---

## OPP-007 — Multi-transport / Reticulum Network Manager

### Problem

Reticulum and similar systems can use multiple transport types, but real deployment requires topology, interface and device configuration that remains technical.

### Evidence pattern

The Reticulum series repeatedly adds blueprints, plug-and-play discovery and automated interface generation to reduce setup friction.

### Product thesis

A future RadioLink Network layer could orchestrate selected multi-transport/off-grid networks while hiding low-level device configuration.

### Guardrail

Reticulum/LoRa is not a requirement for the initial RadioLink product and should not distort APRS/Packet architecture.

### Current disposition

**LABS.**

---

## OPP-008 — Field telemetry / hardware health

### Problem

Field communication failures can come from power, temperature, connection state, audio/PTT state or peripheral loss rather than protocol errors.

### Evidence pattern

The corpus includes repeated measurement and monitoring of batteries, temperatures, radios, interfaces and field-system health.

### Product thesis

A future status surface could expose:

```text
Radio       connected
Transport   USB
TNC         ready
GPS         locked
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
3. deliver mobile-native workflows;
4. build the compatibility/profile asset;
5. prototype Bridge only after the abstractions are proven against multiple paths;
6. stabilize interfaces before publishing RadioLink Ready;
7. graduate Labs ideas only after independent validation.

# Cross-source rule

The Tech Prepper corpus is one strong longitudinal source, not the market itself.

Before changing core scope based on a market hypothesis, compare it against:

- KM4ACK;
- additional technical creators;
- open-source project issues/discussions;
- manufacturer documentation;
- amateur-radio community reports;
- RadioLink field testing.

Repeated independent evidence should raise confidence; conflicting evidence should remain documented rather than silently reconciled.
