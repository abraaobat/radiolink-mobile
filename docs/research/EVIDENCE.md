# RadioLink Research Evidence Ledger

**Status:** Active

This file records source-level evidence separately from findings. Evidence is what a source demonstrates; findings are RadioLink interpretations built from one or more evidence items.

## Evidence ID policy

- `EV-TP-*` — The Tech Prepper evidence.
- `EV-KM-*` — KM4ACK evidence.

Evidence entries should preserve source, date, video, observation and the finding(s) they support.

---

## EV-KM-001 — Pat Menu hides fragile Winlink configuration

**Source:** KM4ACK  
**Date:** 2019-12-10 / 2020-05-19  
**Videos:**
- https://www.youtube.com/watch?v=xXJsJKgU-mc
- https://www.youtube.com/watch?v=rpVfRUBdZ7E

**Observation:** Pat Menu was written to make Pat Winlink easier, start the required HF or packet components and avoid direct edits to a configuration file where a syntax error can prevent Pat from loading.

**Supports:** RL-TP-001, RL-TP-002, RL-TP-008; RL-RS-001, RL-RS-002.

---

## EV-KM-002 — Build-a-Pi preconfigures the digital stack

**Source:** KM4ACK  
**Date:** 2020-01-17  
**Video:** https://www.youtube.com/watch?v=gBokG9FI-NM

**Observation:** The Build-a-Pi workflow preconfigures Direwolf, AX.25 and related software specifically to reduce setup burden for operators.

**Supports:** RL-TP-001, RL-TP-008, RL-TP-011; RL-RS-001.

---

## EV-KM-003 — DigiRig and Mobilinkd solve different layers

**Source:** KM4ACK  
**Date:** 2022-09-02  
**Video:** https://www.youtube.com/watch?v=LWhqdyLdCX4

**Observation:** Mobilinkd is a compact Bluetooth true TNC suited to APRS/packet/Winlink, while DigiRig provides USB audio plus serial/CAT and enables software-modem workflows such as VARA FM. The two interfaces trade convenience, breadth, battery dependency and host requirements.

**Supports:** RL-TP-001, RL-TP-005, RL-TP-009, RL-TP-010; RL-RS-011; RadioLink Bridge/provider abstraction.

---

## EV-KM-004 — WoAD preserves KISS while transport changes

**Source:** KM4ACK  
**Date:** 2022-09-30  
**Video:** https://www.youtube.com/watch?v=X0SyJ0cj4ho

**Observation:** WoAD uses KISS as the TNC type while allowing Bluetooth, USB or TCP/IP connection types.

**Supports:** RL-TP-005, RL-TP-007, RL-TP-010; RL-RS-004, RL-RS-006.

---

## EV-KM-005 — Cell phone is the primary field communications computer

**Source:** KM4ACK  
**Date:** 2022-09-23  
**Video:** https://www.youtube.com/watch?v=NFiaV0zYM9I

**Observation:** A compact Winlink kit is explicitly designed around the cell phone as the primary communications device, using WoAD on Android and a small HT/TNC arrangement.

**Supports:** RL-TP-004, RL-TP-005; RL-RS-003.

---

## EV-KM-006 — One radio can require BLE and USB simultaneously

**Source:** KM4ACK  
**Date:** 2024-03-29  
**Video:** https://www.youtube.com/watch?v=ignNN6OPygU

**Observation:** The demonstrated TH-D75 workflow uses Bluetooth for KISS/TNC and USB-C for GPS because the required GPS and TNC data are not both available over the Bluetooth connection in that configuration.

**Supports:** RL-KM-001; RL-RS-007; capability composition across transports.

---

## EV-KM-007 — Firmware changes Bluetooth/TNC service capability

**Source:** KM4ACK  
**Date:** 2024-12-20  
**Video:** https://www.youtube.com/watch?v=5b6GDOZxpfo

**Observation:** The VGC radio initially worked with APRS over Bluetooth but not Winlink over Bluetooth. A later firmware revision enabled the Winlink/KISS workflow.

**Supports:** RL-KM-002; RL-RS-008; firmware-aware Profiles.

---

## EV-KM-008 — Bluetooth does not imply TNC exposure

**Source:** KM4ACK  
**Date:** 2026-01-06  
**Video:** https://www.youtube.com/watch?v=AZoYdirQPlw

**Observation:** The Retevis/Ailunce HA2 exposes Bluetooth for programming, headset and PTT use but does not expose TNC access over Bluetooth.

**Supports:** ADR-0004 guardrail; RL-RS-005; capability-driven device model.

---

## EV-KM-009 — Radio-native APRS UX remains poor

**Source:** KM4ACK  
**Date:** 2025-10-07  
**Video:** https://www.youtube.com/watch?v=e3S_gpQvfi4

**Observation:** Multiple APRS-capable radios are criticized for poor text entry and messaging UI. A phone/client is used to improve the human interface while retaining the radio as RF/TNC hardware.

**Supports:** RL-KM-004; RL-RS-010; mission-first/mobile-native UX.

---

## EV-KM-010 — Embedded Bluetooth TNC improves field portability

**Source:** KM4ACK  
**Date:** 2025-07-06  
**Video:** https://www.youtube.com/watch?v=mpSXoxxnK7o

**Observation:** BTECH UV-Pro is selected in part because its Bluetooth TNC allows a phone application such as APRSDroid to provide the APRS workflow without a separate host/interface.

**Supports:** RL-TP-004, RL-TP-005; RL-RS-003, RL-RS-005.

---

## EV-KM-011 — KISS over TCP/IP preserves APRS client behavior

**Source:** KM4ACK  
**Date:** 2026-03-17  
**Video:** https://www.youtube.com/watch?v=Xfn9AlTCorI

**Observation:** APRSDroid uses KISS over TCP/IP to a Direwolf-based RF endpoint, demonstrating that the logical TNC boundary survives a network transport change.

**Supports:** RL-KM-005; RL-RS-004; provider/transport separation.

---

## EV-KM-012 — Offline communications server is an explicit design goal

**Source:** KM4ACK  
**Date:** 2026-06-02  
**Video:** https://www.youtube.com/watch?v=EQLv6mPvy1g

**Observation:** The server is explicitly designed to run completely offline and without Internet while providing APRS and local communication/reference services.

**Supports:** RL-TP-003; RL-RS-009; RL-KM-012.

---

## EV-KM-013 — USB cable type is a common transport failure

**Source:** KM4ACK  
**Date:** 2026-06-24  
**Video:** https://www.youtube.com/watch?v=465FRVrBe20

**Observation:** Charge-only or otherwise unsuitable USB cables repeatedly cause devices/radios to appear not to connect to a computer.

**Supports:** RL-KM-003, RL-KM-010; layered transport diagnostics.

---

## EV-KM-014 — Modern BBS/store-and-forward remains active

**Source:** KM4ACK  
**Date:** 2025-06-03  
**Video:** https://www.youtube.com/watch?v=knJzfDi6qfQ

**Observation:** A Meshtastic BBS is configured for local messaging/information functions and can also ingest selected JS8Call group data.

**Supports:** RL-TP-012; RL-RS-012; Labs messaging/BBS hypothesis.

---

## EV-KM-015 — Experienced Linux user still scripts Bluetooth onboarding

**Source:** KM4ACK  
**Date:** 2024-03-30  
**Video:** https://www.youtube.com/watch?v=78naxbNr9pI

**Observation:** The creator describes multiple commands required to connect the TH-D75 on Linux and writes a script because repeatedly remembering/executing them is undesirable even for an experienced Linux user.

**Supports:** RL-RS-001, RL-RS-002; zero-configuration/onboarding requirement.

---

## EV-KM-016 — Phone GPS is reused as a better context source

**Source:** KM4ACK  
**Date:** 2021-02-16  
**Video:** https://www.youtube.com/watch?v=rkmQxvT8BpQ

**Observation:** GPS data is streamed from the phone to another radio-computing host because the phone GPS is described as better than cheap USB GPS units and is normally already locked to satellites.

**Supports:** RL-KM-006; provider-based location architecture.

---

## EV-KM-017 — Radio GPS/GNSS can replace a dedicated USB GPS

**Source:** KM4ACK  
**Date:** 2025-10-24  
**Video:** https://www.youtube.com/watch?v=C2XRUmPRCC8

**Observation:** GPS/GNSS from a radio is piped into the host for digital applications, replacing a standalone USB GPS. The workflow depends on several correct radio-side settings.

**Supports:** RL-KM-006, RL-KM-007; context providers and executable Profiles.

---

## EV-KM-018 — Field operation uses multiple interchangeable GPS/time sources

**Source:** KM4ACK  
**Date:** 2025-05-16  
**Video:** https://www.youtube.com/watch?v=gOpLeRiyJ74

**Observation:** Several different field GPS/time approaches are compared for applications that require accurate time or position, reinforcing that context should not be owned by one specific radio or dongle.

**Supports:** RL-KM-006; future `LocationProvider` / `TimeProvider` registry.

---

## EV-KM-019 — APRS direct messaging has an availability gap

**Source:** KM4ACK  
**Date:** 2024-03-10  
**Video:** https://www.youtube.com/watch?v=UdaCcWFrYMY

**Observation:** A store-and-forward service is demonstrated specifically to solve the case where the APRS recipient is not simultaneously on-air/in-range and therefore misses a direct message.

**Supports:** RL-KM-008; RL-RS-012; future resilient messaging service.

---

## EV-KM-020 — Winlink can be reached through APRS messaging

**Source:** KM4ACK  
**Date:** 2025-01-10 / 2026-03-24  
**Videos:**
- https://www.youtube.com/watch?v=47OGdmm-jEY
- https://www.youtube.com/watch?v=CUKA4mwhbys

**Observation:** The operator can query/send Winlink-related messages through APRS/APRSLink mechanisms, demonstrating that user messaging intent can cross protocol/service boundaries.

**Supports:** RL-KM-008; future service-level messaging abstraction.

---

## EV-KM-021 — Raspberry Pi is not the durable center of the workflow

**Source:** KM4ACK  
**Date:** 2024-07-12  
**Video:** https://www.youtube.com/watch?v=K_0fSk63LFs

**Observation:** The creator substantially moves away from Raspberry Pi because availability, total system cost and ARM compatibility layers make inexpensive x86 laptops/mini-PCs more practical for many workflows.

**Supports:** RL-KM-009; source-neutral host/platform independence.

---

## EV-KM-022 — Emergency communications need frequent practice

**Source:** KM4ACK  
**Date:** 2025-04-18  
**Video:** https://www.youtube.com/watch?v=0eDifEto4bw

**Observation:** Lack of regular opportunities to practice emergency communications is identified as an operational problem; repeated exercises are used to build real proficiency.

**Supports:** RL-KM-011; future readiness/self-test hypothesis.

---

## EV-KM-023 — Weather/data retrieval is needed when cellular Internet fails

**Source:** KM4ACK  
**Date:** 2022-12-30  
**Video:** https://www.youtube.com/watch?v=ipjPa87cg6w

**Observation:** Multiple RF-based methods are explored to obtain weather information when cellular data is unavailable.

**Supports:** RL-KM-012; future offline Data Services hypothesis.

---

# The Tech Prepper evidence added for cross-source synthesis

## EV-TP-001 — Fragmented tools are aggregated behind one operational surface

**Source:** The Tech Prepper  
**Date:** 2025-11-14  
**Video:** https://www.youtube.com/watch?v=ATYguXQeau0

**Observation:** EmComm Tools is explicitly framed as an aggregation/orchestration solution to the same fragmentation problem seen in digital radio applications and hardware.

**Supports:** RL-RS-001, RL-RS-002.

---

## EV-TP-002 — Raspberry Pi field reliability/platform fit was rejected

**Source:** The Tech Prepper  
**Date:** 2025-11-14  
**Video:** https://www.youtube.com/watch?v=ATYguXQeau0

**Observation:** Raspberry Pi is explicitly rejected as the current reference platform after practical field use, including high-temperature failures; the project standardizes on 64-bit host-class hardware instead.

**Supports:** source-neutral host/platform independence.

---

## EV-TP-003 — APRS position may originate from the phone GPS

**Source:** The Tech Prepper  
**Date:** 2025-11-14  
**Video:** https://www.youtube.com/watch?v=ATYguXQeau0

**Observation:** APRS positioning is described as using geo-coordinates supplied by a GPS source such as the phone, reinforcing that location is contextual input rather than exclusively a radio-owned capability.

**Supports:** RL-KM-006; provider-based location synthesis.

---

## EV-TP-004 — Wired plug-and-play is preferred over Bluetooth in a field network workflow

**Source:** The Tech Prepper  
**Date:** 2026-06-17  
**Video:** https://www.youtube.com/watch?v=UsP5BGs2uuA

**Observation:** A USB-connected RNode is deliberately used for a plug-and-play field workflow while Bluetooth is not selected, reinforcing deterministic transport discovery and wired fallback/value.

**Supports:** RL-RS-005, RL-RS-006, layered diagnostics.

---

## EV-TP-005 — Communications proficiency is built through repeated field exercises

**Source:** The Tech Prepper  
**Representative sources:**
- `Daily Comms Practice - No Cellphone!` — 2025-01-12  
  https://www.youtube.com/watch?v=F0HBAWYzNvE
- `Digital Comms Field Exercise Part 3 - After Action Report` — 2022-05-31  
  https://www.youtube.com/watch?v=HtM0drzD2kw

**Observation:** Repeated practice, field exercises and after-action review are treated as necessary to make communication workflows reliable in actual use.

**Supports:** RL-KM-011; future readiness/self-test synthesis.

---

## Evidence quality rules

1. Prefer direct demonstrations over passing mentions.
2. Preserve contradictory observations.
3. Record firmware/platform context when capability depends on version.
4. Do not promote a product requirement from a single evidence item without architectural justification.
5. Add timestamp/deep-link references when segment files are available for evidence promoted into an ADR or implementation requirement.
6. Distinguish source observation from RadioLink interpretation.
7. Preserve negative/rejected hypotheses so future research does not unknowingly repeat them.
