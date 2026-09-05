# RadioLink Mobile — Roadmap Master

Status legend: `NOT STARTED` · `IN PROGRESS` · `DONE` · `BLOCKED`

## North Star

Deliver a mobile-native amateur radio experience where a user can connect a compatible radio/TNC by Bluetooth, receive/transmit APRS and exchange messages without Raspberry Pi, DigiPi or an external Linux computer.

---

## F0 — Project Foundation

**Status:** IN PROGRESS

### Goal
Establish the product boundary, repository, architecture and first validation path.

### Deliverables
- [x] Create public GitHub repository.
- [x] Define product vision and MVP guardrails.
- [x] Define smartphone-first / Bluetooth-first architecture.
- [x] Create Roadmap Master.
- [x] Record ADR-0001.
- [ ] Define coding/toolchain decision for shared core and mobile apps.
- [ ] Select first reference radio/TNC for PoC.
- [ ] Capture Bluetooth services/characteristics for first reference device.
- [ ] Define test packet fixtures for KISS/AX.25/APRS.

### Exit criteria
A developer can clone the repository and understand exactly what the MVP is, what it is not, which first hardware will be tested and what technical spike starts F1.

---

## F1 — Bluetooth Device Layer

**Status:** NOT STARTED

### Goal
Discover, connect and maintain a reliable session with the first supported Bluetooth/BLE radio or TNC.

### Deliverables
- [ ] Device scan.
- [ ] Pair/connect flow.
- [ ] Connection state machine.
- [ ] Reconnect behavior.
- [ ] Permissions handling for Android/iOS.
- [ ] Service/characteristic discovery.
- [ ] Raw RX/TX diagnostic console for development.
- [ ] Device capability descriptor.

### Exit criteria
The app can connect to the reference device, exchange raw data and recover from a normal disconnect without restarting the app.

---

## F2 — KISS Transport Core

**Status:** NOT STARTED

### Goal
Implement a platform-neutral KISS transport core.

### Deliverables
- [ ] KISS frame encoder.
- [ ] KISS frame decoder.
- [ ] Escaping/unescaping.
- [ ] Streaming parser for fragmented BLE notifications.
- [ ] Unit tests and captured frame fixtures.
- [ ] Driver-to-KISS adapter.

### Exit criteria
Known KISS frames can be encoded/decoded deterministically and real frames from the reference device reach the protocol core.

---

## F3 — AX.25 + APRS Receive

**Status:** NOT STARTED

### Goal
Decode real APRS traffic received over RF.

### Deliverables
- [ ] AX.25 address parsing.
- [ ] AX.25 UI frame parsing.
- [ ] APRS position decoding.
- [ ] APRS message decoding.
- [ ] Basic telemetry/status/text handling.
- [ ] Unknown/unsupported packet preservation for diagnostics.
- [ ] Real RF receive test.

### Exit criteria
The app receives RF APRS through the Bluetooth radio/TNC and displays decoded callsign and payload data correctly.

---

## F4 — APRS Stations + Map

**Status:** NOT STARTED

### Goal
Turn decoded APRS traffic into a useful field view.

### Deliverables
- [ ] Station list.
- [ ] Last-heard timestamp.
- [ ] Position history model.
- [ ] Map view.
- [ ] Station detail screen.
- [ ] Basic filters.
- [ ] Local persistence/cache.

### Exit criteria
Nearby APRS stations received by RF appear on a usable mobile list/map without requiring APRS-IS.

---

## F5 — APRS Beacon TX

**Status:** NOT STARTED

### Goal
Transmit the user's own APRS position through RF.

### Deliverables
- [ ] Callsign/SSID configuration.
- [ ] GPS integration.
- [ ] APRS position encoder.
- [ ] AX.25/KISS transmit path.
- [ ] Manual beacon.
- [ ] Configurable beacon interval baseline.
- [ ] TX state/error feedback.
- [ ] On-air validation with an independent receiver.

### Exit criteria
A beacon transmitted from the app is decoded correctly by another APRS station/TNC.

---

## F6 — APRS Messaging

**Status:** NOT STARTED

### Goal
Provide a clean chat-like RF messaging experience.

### Deliverables
- [ ] Conversation list.
- [ ] Message composer.
- [ ] APRS message encoder.
- [ ] Message receive pipeline.
- [ ] ACK/REJ handling where applicable.
- [ ] Delivery state model.
- [ ] Local conversation history.
- [ ] Callsign destination validation.

### Exit criteria
Two stations can exchange APRS messages using the app with understandable delivery state.

---

## F7 — Basic Radio Control

**Status:** NOT STARTED

### Goal
Expose useful radio controls when supported by a device driver without making them mandatory for APRS operation.

### Deliverables
- [ ] Capability-gated controls.
- [ ] Read current frequency/channel where supported.
- [ ] Set frequency/channel where supported and legally/technically appropriate.
- [ ] Basic mode/power/PTT hooks where the device safely exposes them.
- [ ] Clear unsupported-state UX.

### Exit criteria
The same UI can work with both data-only TNCs and richer radios without hard-coded assumptions.

---

## F8 — Android MVP

**Status:** NOT STARTED

### Goal
Package F1–F7 into a coherent Android field-ready MVP.

### Deliverables
- [ ] Android application shell.
- [ ] Bluetooth lifecycle hardened.
- [ ] Location/background policy implemented.
- [ ] Notifications/status.
- [ ] First-run onboarding.
- [ ] Radio connection screen.
- [ ] Stations/map/messages navigation.
- [ ] Device compatibility indication.
- [ ] Internal/beta distribution build.

### Exit criteria
A radioamateur can install the Android build and complete the core APRS workflow without developer tools.

---

## F9 — iOS MVP

**Status:** NOT STARTED

### Goal
Deliver equivalent core behavior on iPhone while respecting iOS Bluetooth/background constraints.

### Deliverables
- [ ] iOS application shell.
- [ ] CoreBluetooth implementation.
- [ ] Location/background behavior.
- [ ] Notifications/status.
- [ ] Shared protocol test vectors reused.
- [ ] First-run onboarding.
- [ ] Stations/map/messages navigation.
- [ ] TestFlight build.

### Exit criteria
An iPhone user can complete the same core APRS workflow with a supported Bluetooth radio/TNC.

---

## F10 — Winlink Packet

**Status:** NOT STARTED

### Goal
Extend the transport/core to support Packet-based Winlink workflows without changing the smartphone-first architecture.

### Deliverables
- [ ] Technical feasibility spike.
- [ ] Winlink session architecture decision.
- [ ] Packet connection transport.
- [ ] Message compose/inbox/outbox model.
- [ ] RMS connection configuration.
- [ ] On-air interoperability testing.

### Exit criteria
A supported mobile device/radio combination can complete a practical Winlink Packet exchange.

---

## F11 — Driver SDK + Compatibility Matrix

**Status:** NOT STARTED

### Goal
Make support for new radios/TNCs systematic instead of ad hoc.

### Deliverables
- [ ] Stable driver interface.
- [ ] Capability schema.
- [ ] Driver test harness.
- [ ] Device profile documentation template.
- [ ] Compatibility matrix.
- [ ] Reference drivers.
- [ ] Contribution guide for new devices.

### Initial candidate classes
- [ ] Embedded BLE KISS radio (e.g. BTECH UV-PRO class).
- [ ] BLE KISS TNC (e.g. Mobilinkd class).
- [ ] RadioLink Bridge.

### Exit criteria
A contributor can add a new compatible device without modifying APRS/UI core logic.

---

## F12 — RadioLink Bridge for Legacy Radios

**Status:** NOT STARTED

### Goal
Provide an optional compact Bluetooth accessory for radios that lack a native smartphone-friendly data interface.

### Guardrail
The Bridge is an accessory, not a second computer and not a replacement for the smartphone.

### Candidate responsibilities
- [ ] BLE transport.
- [ ] KISS/TNC function.
- [ ] PTT interface.
- [ ] Audio/data interface.
- [ ] Optional CAT/control.
- [ ] Firmware update path.

### Exit criteria
A conventional analog radio can join the same RadioLink Mobile experience with one small bridge accessory.

---

# Post-MVP Exploration

Only after the core product is stable:

- APRS-IS optional integration / iGate experiments.
- USB device transport as an alternative to Bluetooth.
- richer offline maps.
- additional packet speeds/modems.
- interoperability with RadioNode-BR.
- possible LoRa/DMR integrations as separate capability modules.

These items must not be allowed to expand the MVP before F8/F9 prove the core concept.

# Current execution order

```text
F0 → F1 → F2 → F3 → F4 → F5 → F6 → F7 → F8
                                      └──────→ F9

After core MVP validation:
F10 → F11 → F12
```

# Immediate next action

Complete F0 by selecting the first reference hardware and documenting its Bluetooth data path. The preferred first PoC should minimize custom hardware and expose a usable KISS/data transport over Bluetooth.
