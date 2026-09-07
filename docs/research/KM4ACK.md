# KM4ACK — Research Source Record

**Status:** Active research source  
**Source type:** YouTube technical corpus  
**Research date:** 2026-09-06  
**Project:** RadioLink Platform

## Purpose

KM4ACK is used as an independent source to validate, qualify or contradict findings first observed in The Tech Prepper corpus and to discover new RadioLink-relevant problems.

The channel is broader than The Tech Prepper, so RadioLink uses a screening stage before deep mining.

## Corpus snapshot

Local corpus generated with `tools/research/radiolink-youtube-corpus.sh`:

- **828** unique videos/streams/shorts;
- approximately **1,142,114 transcript words**;
- **797** transcripts from YouTube captions;
- **31** transcripts generated with `whisper.cpp`;
- source date range in corpus: **2007-11-06 through 2026-09-04**;
- corpus extraction completed with **0 failures**.

Raw transcripts remain outside Git. Git stores derived findings, evidence references and research decisions only.

## Initial screening

The first pass uses title plus the opening transcript/topic signal. It is intentionally conservative and can be overridden during manual review.

| Class | Meaning | Videos | Deep mine |
|---|---|---:|---|
| A | Direct RadioLink core relevance | 154 | Yes |
| B | Adjacent integration/off-grid/software relevance | 209 | Yes |
| C | Radio/field context | 330 | Selectively |
| D | Low immediate RadioLink relevance | 135 | No by default |

Primary deep-mining universe: **363 A+B videos**.

A screening class is not a permanent exclusion. Clickbait/generic titles can hide relevant material, so C/D items can be promoted when later evidence points to them.

## Cross-source validation — first pass

### Strongly confirmed

The KM4ACK corpus independently supports these existing The Tech Prepper findings:

- digital-mode setup is integration-heavy;
- interface/cable/radio fragmentation creates repeated configuration work;
- known-good settings and automation reduce failure risk;
- smartphones can be primary field computers;
- KISS/TNC abstraction enables the same application over different transports;
- USB remains strategically important;
- offline-first operation has real field value;
- outcome-oriented wrappers/scripts are repeatedly built to hide configuration complexity;
- store-and-forward/BBS concepts remain operationally interesting.

### Qualified rather than simply confirmed

Bluetooth is strongly validated as a convenient path, but KM4ACK adds a different nuance: capability availability depends heavily on the exact radio, firmware and Bluetooth profile. A radio may expose Bluetooth for programming/headset/PTT while not exposing its TNC at all.

The Tech Prepper's stronger claim about Bluetooth brittleness remains valuable, but KM4ACK currently contributes more evidence about **capability fragmentation and firmware dependence** than about Bluetooth being universally unreliable.

## High-value evidence sequence

### 2020-01-17 — Easy Raspberry Pi Build for Ham Radio Digital Operations | Build-a-Pi

Source: https://www.youtube.com/watch?v=gBokG9FI-NM

Shows substantial pre-configuration of Direwolf, AX.25 and radio applications specifically to make digital operation easier for users.

### 2019-12-10 / 2020-05-19 — Pat Menu

Sources:

- https://www.youtube.com/watch?v=xXJsJKgU-mc
- https://www.youtube.com/watch?v=rpVfRUBdZ7E

The menu exists to make Pat Winlink easier and to avoid direct editing of a configuration file where a small syntax mistake can prevent Pat from loading.

### 2022-09-02 — Digirig vs Mobilinkd

Source: https://www.youtube.com/watch?v=LWhqdyLdCX4

Directly compares a true Bluetooth TNC with a USB sound-card/serial interface. Mobilinkd is compact and phone-friendly but mode-limited; DigiRig is broader and battery-free but needs a host and software modem. This strongly supports RadioLink's provider/transport separation and future Bridge direction.

### 2022-09-23 / 2022-09-30 — WoAD + Mobilinkd

Sources:

- https://www.youtube.com/watch?v=NFiaV0zYM9I
- https://www.youtube.com/watch?v=X0SyJ0cj4ho

Shows a compact Winlink field kit centered on an Android phone and a KISS TNC. WoAD can select Bluetooth, USB or TCP/IP as the connection type while keeping KISS as the TNC protocol.

### 2024-03-29 — Kenwood TH-D75 on Linux via Bluetooth for APRS & GPS

Source: https://www.youtube.com/watch?v=ignNN6OPygU

Important multi-transport case: the demonstrated configuration uses **Bluetooth for KISS/TNC** and **USB-C for GPS data** because both data functions cannot be obtained through the Bluetooth connection simultaneously in that workflow.

### 2024-12-20 — VGC Now Fully Supports Winlink

Source: https://www.youtube.com/watch?v=5b6GDOZxpfo

Shows a radio that previously worked with APRS over Bluetooth but not Winlink over Bluetooth; a later firmware revision enabled the missing workflow. Device capability profiles therefore need firmware/version context.

### 2025-07-06 — BTECH UV Pro Bluetooth APRS on a Budget!

Source: https://www.youtube.com/watch?v=mpSXoxxnK7o

Demonstrates the value of an embedded Bluetooth TNC and explicitly uses the phone to provide a better APRS experience than the radio's own interface.

### 2025-10-07 — Stop Suffering with APRS UIs – Use This Setup Instead

Source: https://www.youtube.com/watch?v=e3S_gpQvfi4

Strongly supports the companion/mobile UI thesis: multiple APRS-capable radios are criticized for poor message-entry/user-interface ergonomics even when their RF/TNC capabilities are useful.

### 2026-01-06 — Retevis Ailunce HA2 APRS test

Source: https://www.youtube.com/watch?v=AZoYdirQPlw

Critical capability-model evidence: the radio has Bluetooth for programming/headset/PTT, but the Bluetooth connection does **not** expose the TNC. `Bluetooth = true` therefore cannot imply `KISS/TNC = true`.

### 2026-03-17 — Send REAL RF APRS Packets From Your Phone Over WiFi

Source: https://www.youtube.com/watch?v=Xfn9AlTCorI

Demonstrates APRSDroid using KISS over TCP/IP to a Direwolf-based RF endpoint. This reinforces the idea that the logical TNC/provider interface can remain stable while the physical/network transport changes.

### 2026-06-02 — MY OFF-GRID COMMS SERVER

Source: https://www.youtube.com/watch?v=EQLv6mPvy1g

Explicitly designs a communications server to operate completely offline and without Internet, including APRS and local reference/tool services.

### 2026-06-24 — The Reason Your USB Device Won't Connect

Source: https://www.youtube.com/watch?v=465FRVrBe20

Identifies a recurring operational support failure: charge-only or otherwise unsuitable USB cables can make radios/devices appear broken or undiscoverable. Transport diagnostics should distinguish cable/link problems from protocol/device failures.

## New KM4ACK-specific hypotheses

### RL-KM-001 — Capabilities may be composed across simultaneous transports

A single radio may expose useful functions on different paths at the same time. The TH-D75 case demonstrates KISS over Bluetooth plus GPS data over USB-C.

**Implication:** RadioLink should allow one logical `RadioDevice` to aggregate capabilities from more than one active transport instead of forcing one transport per device/session.

### RL-KM-002 — Firmware version is part of device capability identity

The VGC Winlink example shows that firmware can materially change available Bluetooth/TNC workflows.

**Implication:** RadioLink Profiles should include firmware version/range and capability validation status by firmware.

### RL-KM-003 — Transport diagnostics are a product feature

USB data-vs-charge cable failures are common enough to deserve first-class diagnostics.

**Implication:** `radiolink capabilities`, `radiolink transports` and future UI diagnostics should explain whether the host sees USB power, enumeration, serial/audio interfaces and usable data paths.

### RL-KM-004 — The phone is often the best UI even when the radio already has APRS

Several APRS-capable radios still provide poor text-entry/message UX. The external phone application can be the superior human interface while the radio remains RF/TNC hardware.

**Implication:** RadioLink should not assume that an embedded APRS implementation eliminates the need for a companion app.

### RL-KM-005 — Network KISS is useful as a logical transport

KISS over TCP/IP allows the same APRS client logic to operate against a remote/local Direwolf TNC endpoint.

**Implication:** Keep network/TCP KISS in the transport/provider abstraction for desktop, node and Labs scenarios without changing the three official *physical radio I/O* paths.

## Next mining stage

1. Deep mine all A videos.
2. Mine B videos for repeated pain/workaround/product signals.
3. Manually review C/D false negatives discovered by evidence search.
4. Link each accepted evidence item to `FINDINGS.md`.
5. Promote cross-source findings to source-neutral `RL-RS-*` synthesis findings.
6. Record contradictory evidence explicitly rather than silently reconciling it.
