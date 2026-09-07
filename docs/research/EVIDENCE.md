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

**Supports:** RL-TP-001, RL-TP-002, RL-TP-008; future RL-RS setup/orchestration synthesis.

---

## EV-KM-002 — Build-a-Pi preconfigures the digital stack

**Source:** KM4ACK  
**Date:** 2020-01-17  
**Video:** https://www.youtube.com/watch?v=gBokG9FI-NM

**Observation:** The Build-a-Pi workflow preconfigures Direwolf, AX.25 and related software specifically to reduce setup burden for operators.

**Supports:** RL-TP-001, RL-TP-008, RL-TP-011.

---

## EV-KM-003 — DigiRig and Mobilinkd solve different layers

**Source:** KM4ACK  
**Date:** 2022-09-02  
**Video:** https://www.youtube.com/watch?v=LWhqdyLdCX4

**Observation:** Mobilinkd is a compact Bluetooth true TNC suited to APRS/packet/Winlink, while DigiRig provides USB audio plus serial/CAT and enables software-modem workflows such as VARA FM. The two interfaces trade convenience, breadth, battery dependency and host requirements.

**Supports:** RL-TP-001, RL-TP-005, RL-TP-009, RL-TP-010; RadioLink Bridge/provider abstraction.

---

## EV-KM-004 — WoAD preserves KISS while transport changes

**Source:** KM4ACK  
**Date:** 2022-09-30  
**Video:** https://www.youtube.com/watch?v=X0SyJ0cj4ho

**Observation:** WoAD uses KISS as the TNC type while allowing Bluetooth, USB or TCP/IP connection types.

**Supports:** RL-TP-005, RL-TP-007, RL-TP-010; network/transport-neutral KISS architecture.

---

## EV-KM-005 — Cell phone is the primary field communications computer

**Source:** KM4ACK  
**Date:** 2022-09-23  
**Video:** https://www.youtube.com/watch?v=NFiaV0zYM9I

**Observation:** A compact Winlink kit is explicitly designed around the cell phone as the primary communications device, using WoAD on Android and a small HT/TNC arrangement.

**Supports:** RL-TP-004, RL-TP-005.

---

## EV-KM-006 — One radio can require BLE and USB simultaneously

**Source:** KM4ACK  
**Date:** 2024-03-29  
**Video:** https://www.youtube.com/watch?v=ignNN6OPygU

**Observation:** The demonstrated TH-D75 workflow uses Bluetooth for KISS/TNC and USB-C for GPS because the required GPS and TNC data are not both available over the Bluetooth connection in that configuration.

**Supports:** RL-KM-001; capability composition across transports; F1/F9/F13 architecture.

---

## EV-KM-007 — Firmware changes Bluetooth/TNC service capability

**Source:** KM4ACK  
**Date:** 2024-12-20  
**Video:** https://www.youtube.com/watch?v=5b6GDOZxpfo

**Observation:** The VGC radio initially worked with APRS over Bluetooth but not Winlink over Bluetooth. A later firmware revision enabled the Winlink/KISS workflow.

**Supports:** RL-KM-002; firmware-aware Profiles.

---

## EV-KM-008 — Bluetooth does not imply TNC exposure

**Source:** KM4ACK  
**Date:** 2026-01-06  
**Video:** https://www.youtube.com/watch?v=AZoYdirQPlw

**Observation:** The Retevis/Ailunce HA2 exposes Bluetooth for programming, headset and PTT use but does not expose TNC access over Bluetooth.

**Supports:** ADR-0004 guardrail; capability-driven device model; independent validation of `Bluetooth != KISS/TNC`.

---

## EV-KM-009 — Radio-native APRS UX remains poor

**Source:** KM4ACK  
**Date:** 2025-10-07  
**Video:** https://www.youtube.com/watch?v=e3S_gpQvfi4

**Observation:** Multiple APRS-capable radios are criticized for poor text entry and messaging UI. A phone/client is used to improve the human interface while retaining the radio as RF/TNC hardware.

**Supports:** RL-KM-004; mission-first/mobile-native UX.

---

## EV-KM-010 — Embedded Bluetooth TNC improves field portability

**Source:** KM4ACK  
**Date:** 2025-07-06  
**Video:** https://www.youtube.com/watch?v=mpSXoxxnK7o

**Observation:** BTECH UV-Pro is selected in part because its Bluetooth TNC allows a phone application such as APRSDroid to provide the APRS workflow without a separate host/interface.

**Supports:** RL-TP-004, RL-TP-005.

---

## EV-KM-011 — KISS over TCP/IP preserves APRS client behavior

**Source:** KM4ACK  
**Date:** 2026-03-17  
**Video:** https://www.youtube.com/watch?v=Xfn9AlTCorI

**Observation:** APRSDroid uses KISS over TCP/IP to a Direwolf-based RF endpoint, demonstrating that the logical TNC boundary survives a network transport change.

**Supports:** RL-KM-005; provider/transport separation.

---

## EV-KM-012 — Offline communications server is an explicit design goal

**Source:** KM4ACK  
**Date:** 2026-06-02  
**Video:** https://www.youtube.com/watch?v=EQLv6mPvy1g

**Observation:** The server is explicitly designed to run completely offline and without Internet while providing APRS and local communication/reference services.

**Supports:** RL-TP-003.

---

## EV-KM-013 — USB cable type is a common transport failure

**Source:** KM4ACK  
**Date:** 2026-06-24  
**Video:** https://www.youtube.com/watch?v=465FRVrBe20

**Observation:** Charge-only or otherwise unsuitable USB cables repeatedly cause devices/radios to appear not to connect to a computer.

**Supports:** RL-KM-003; transport diagnostics and human-readable error states.

---

## EV-KM-014 — Modern BBS/store-and-forward remains active

**Source:** KM4ACK  
**Date:** 2025-06-03  
**Video:** https://www.youtube.com/watch?v=knJzfDi6qfQ

**Observation:** A Meshtastic BBS is configured for local messaging/information functions and can also ingest selected JS8Call group data.

**Supports:** RL-TP-012; Labs messaging/BBS hypothesis.

---

## Evidence quality rules

1. Prefer direct demonstrations over passing mentions.
2. Preserve contradictory observations.
3. Record firmware/platform context when capability depends on version.
4. Do not promote a product requirement from a single evidence item without architectural justification.
5. Add timestamp/deep-link references when segment files are available for evidence promoted into an ADR or implementation requirement.
