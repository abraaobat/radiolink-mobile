# The Tech Prepper — Research Source Record

**Status:** Active research source  
**Source type:** YouTube technical corpus  
**Research date:** 2026-09-06  
**Project:** RadioLink Platform

## Purpose

The Tech Prepper corpus is used as a longitudinal engineering and product-research source for RadioLink.

The goal is **not** to copy the creator's projects or architecture. The goal is to identify recurring operational problems, workarounds, integration patterns, reliability lessons and product opportunities across years of practical amateur-radio experimentation.

The corpus is especially useful because it captures the evolution of real field workflows across:

- APRS and Packet/AX.25;
- Winlink;
- DigiRig and audio/PTT interfaces;
- embedded and external TNCs;
- Bluetooth/BLE and KISS;
- EmComm Tools / ETC mode orchestration;
- Linux field computers and mobile experiments;
- BBS/store-and-forward systems;
- Mercury HF / VARA comparisons;
- Reticulum / LoRa experiments;
- offline-first emergency communications.

## Corpus snapshot

Local research corpus generated with `tools/research/radiolink-youtube-corpus.sh`:

- **548** unique videos/streams/shorts;
- approximately **1,358,267 words**;
- **512** transcripts from YouTube captions;
- **36** transcripts generated with `whisper.cpp`;
- approximately **359,076 timestamped segments** in the local segment corpus.

The raw corpus is intentionally **not committed to Git**. It remains a local research asset because of its size and source-content/licensing considerations.

Git contains only findings, source references, derived requirements and decisions.

## Research methodology

The corpus is mined through several lenses:

1. **Explicit pain** — difficult, annoying, unreliable, brittle, complicated, failed, did not work.
2. **Workarounds** — scripts, custom applications, adapters, alternate hardware, manual procedures and field cards.
3. **Repeated integration work** — audio, PTT, CAT, USB, Bluetooth, KISS, TNC and modem configuration.
4. **Operational simplification** — plug-and-play, zero configuration, automatic detection and mode switching.
5. **Field reliability** — power, reconnection, known-good hardware, offline operation and predictable state.
6. **Interface boundaries** — places where one implementation can be replaced by another behind a compatible interface.
7. **Market signals** — situations where the creator repeatedly builds a missing layer instead of simply selecting an existing product.

Findings are recorded in [`FINDINGS.md`](FINDINGS.md). Product opportunities derived from those findings are recorded separately in [`MARKET-OPPORTUNITIES.md`](MARKET-OPPORTUNITIES.md).

## High-value source sequence

### 2022-04-25 — Why are digital data modes so difficult?

Video: https://www.youtube.com/watch?v=hL-TrpGzEkg

The video frames the core problem directly: practical digital-radio operation requires a radio, interface, radio-specific cables, operating-system configuration and multiple software components. The creator describes the workflow as unnecessarily complicated and demonstrates two very different station configurations for similar packet/APRS goals.

**RadioLink relevance:** strong validation for abstraction, capability-driven configuration and mission-oriented UX.

---

### 2022-03-04 — The EmComm Tools Story

Video: https://www.youtube.com/watch?v=HFdXTk8Zp5c

The creator explains that radio procedures and message formats decay when not practiced frequently. EmComm Tools emerged to hide startup scripts, service orchestration and message-format details. The desired user experience is described in terms of outcomes such as sending an e-mail, sending a text message or finding a station rather than remembering which application and configuration implements each task.

Offline operation and rapid deployment are treated as core principles.

**RadioLink relevance:** direct validation for the Operations Engine, operational-service abstraction, templates/profiles and offline-first operation.

---

### 2024-07-03 — How to send email with the DigiRig Lite on Android

Video: https://www.youtube.com/watch?v=zQ3Og2r1yj8

The Android + DigiRig Lite workflow demonstrates that the smartphone can already serve as the primary radio computer, but practical operation depends on application-specific audio levels, channel selection, radio volume, PTT behavior and a radio-specific cable. The creator notes that WoAD requires a different media-volume level from APRSdroid and uses reference cards to remember settings.

**RadioLink relevance:** strong validation for device/interface profiles, stored calibration values and guided setup.

---

### 2024-11-24 — Packet Radio Field Test — EmComm Tools Mode Switcher

Video: https://www.youtube.com/watch?v=feVLHX2ypYw

The mode-switching workflow detects CAT and audio, switches from APRS to Winlink and explicitly stops/restarts services under the hood. The creator characterizes the target platform as plug-and-play and near-zero configuration.

**RadioLink relevance:** direct validation for lifecycle management, resource ownership, capability detection and recovery to a known state.

---

### 2025-03-01 — A breakthrough! Full Emergency Comms on a Phone

Video: https://www.youtube.com/watch?v=RDgd4QLNL8M

A PinePhone running a full Linux stack is used as a rapid prototype for EmComm Tools on a phone. The experiment validates that much of the Linux radio stack can technically run on mobile hardware and that Bluetooth packet operation is possible, but the initial experience is essentially a desktop/Linux stack transplanted onto a small device.

**RadioLink relevance:** validates the smartphone-as-computer goal while warning against treating mobile as a reduced desktop.

---

### 2025-03-15 — Radio Packet Terminal on your Phone using WoAD

Video: https://www.youtube.com/watch?v=pcdh4R6kdvo

The creator demonstrates a BTECH UV-Pro with embedded KISS TNC connected to Android through Bluetooth and uses WoAD for packet-terminal operation. The workflow can select either audio/DigiRig or KISS/Bluetooth as the TNC path.

The explicit field objective is to operate with the everyday phone and HT without additional computer hardware and cabling.

**RadioLink relevance:** strong validation for BLE/KISS as a high-value mobile path and for keeping TNC/provider selection below the application layer.

---

### 2025-12-24 — 2025 TTP Recap

Video: https://www.youtube.com/watch?v=bnsv_gGBzjM

The retrospective concludes that the PinePhone experiment did not work out as a practical product path. Desktop amateur-radio applications such as JS8Call and FLDigi were not designed for a small screen, terminal workflows were challenging, the keyboard implementation was poor and total runtime with the tested phone/keyboard configuration was under roughly two hours.

**RadioLink relevance:** strong evidence for mobile-native UX over Linux-desktop porting and for treating power/lifecycle constraints as product requirements.

---

### 2026-06-17 — Reticulum — Plug-and-Play Configuration Demo

Video: https://www.youtube.com/watch?v=UsP5BGs2uuA

The creator applies the same plug-and-play concept to LoRa/Reticulum devices: compatible hardware is detected when connected and interfaces are generated/configured automatically. In this setup the creator explicitly prefers a wired USB connection and says Bluetooth is not planned for that workflow.

**RadioLink relevance:** reinforces transport abstraction and the importance of USB as a first-class path rather than merely a fallback.

---

### 2026-08-05 — I am DONE with Reticulum

Video: https://www.youtube.com/watch?v=E8yarbRFtb4

The creator states a strong preference for wired connections in emergency-communications use and describes Bluetooth as hit-or-miss and brittle in personal experience.

**RadioLink relevance:** BLE remains strategically important for mobile convenience, but connection recovery, health monitoring and wired fallback must be first-class requirements.

---

### 2026-08-20 / 2026-09-02 — Mercury HF / Winlink experiments

Videos:

- https://www.youtube.com/watch?v=GQ9odnuUcQE
- https://www.youtube.com/watch?v=4LaFSDcLHus

The Mercury HF experiments focus on replacing or complementing VARA HF with a modern open-source/open-protocol modem while keeping existing operational workflows useful. Mercury is integrated with Winlink, BBS and EmComm Tools; audio-device detection and configuration are automated.

The creator's strongest objection to VARA is not simply that it is proprietary software, but that the available protocol documentation does not make independent compatible reimplementation straightforward.

**RadioLink relevance:** validates a `TNC/Modem Provider` boundary and the value of documented interoperable interfaces.

## Research rules for RadioLink

- Treat creator observations as **field evidence and hypotheses**, not universal truths.
- Promote a finding into the product only when it fits RadioLink's architecture and product goals.
- Preserve contradictory observations. Example: BLE is both a highly valuable cable-free workflow and a reliability concern in later field use.
- Prefer repeated patterns over isolated preferences.
- Cross-check findings against other independent sources such as KM4ACK before treating a market hypothesis as broadly validated.
- Do not copy EmComm Tools' Bash/Linux architecture. Generalize useful orchestration concepts into the platform-neutral RadioLink Core.

## Traceability model

```text
Source video / corpus
        ↓
Research finding
        ↓
Product implication
        ↓
ADR / architecture / roadmap item
        ↓
Implementation + field validation
```

The purpose of this directory is to preserve that chain so future contributors can understand **why** a RadioLink design decision exists, not only what the current code implements.
