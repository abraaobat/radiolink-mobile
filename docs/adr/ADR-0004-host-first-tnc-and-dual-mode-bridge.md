# ADR-0004 — Host-first TNC and dual-mode RadioLink Bridge

- **Status:** Accepted
- **Date:** 2026-09-05

## Context

RadioLink is based on the principle that the smartphone or computer is the main computing platform and the radio is primarily the RF endpoint.

During the first RadioLink Bridge design, an ESP32-S3 accessory was being treated as a Mobilinkd-like TNC that would perform AFSK, AX.25, KISS and BLE transport locally.

That architecture is valid for cable-free mobile operation, but it is not necessary simply because the host lacks processing power. Modern smartphones and computers can perform AFSK/AX.25 modem work comfortably. Moving the modem into the ESP32 too early also makes hardware bring-up harder because audio-level, PTT, codec and modem-firmware problems become coupled.

A conventional radio still needs an interface for:

- receive audio;
- transmit audio;
- PTT;
- safe signal conditioning;
- optional CAT/control;
- transport to the host.

The project therefore needs to distinguish the **physical radio interface problem** from the **TNC/modem placement decision**.

## Decision

RadioLink adopts a **host-first TNC strategy**.

For conventional-radio development, the first reference path will use the smartphone/computer — initially macOS/Linux with Direwolf or equivalent — as the software TNC/modem.

```text
Radio
  │
audio RX/TX + PTT
  │
RadioLink Bridge / compatible interface
  │
USB audio/control
  │
Host software TNC
  │
KISS / AX.25
  │
RadioLink
```

The RadioLink Bridge will be designed as a dual-mode accessory.

### Mode A — host-TNC interface mode

The Bridge provides the radio edge:

- audio RX;
- audio TX;
- PTT;
- signal conditioning;
- host transport;
- optional CAT/control.

The host performs modem/TNC processing.

### Mode B — embedded-TNC mode

After Mode A is validated, the same hardware may add:

- AFSK modulation/demodulation;
- AX.25 frame handling where required;
- KISS;
- BLE KISS transport.

This creates a Mobilinkd-like cable-free operating mode for Android/iOS and other hosts.

Both modes must present compatible semantics through the shared `TncBackend` / `TncTransport` abstraction so APRS, Packet and future Winlink modules do not care where the modem resides.

## Raw Bluetooth audio

Raw Bluetooth audio is not selected as the primary AFSK transport.

Reasons include possible:

- codec compression;
- resampling;
- AGC/noise suppression;
- buffering;
- latency variation;
- platform-specific Bluetooth audio behavior.

BLE KISS/data is preferred for cable-free Packet/APRS operation. USB audio/PTT is preferred as the deterministic reference path for software-TNC bring-up.

Wi-Fi/raw-audio transport may be explored later but is outside the initial MVP.

## Consequences

### Positive

- Uses the processing power already available on phones/computers.
- Separates radio-interface debugging from modem-firmware debugging.
- Allows Direwolf to serve as a known-good reference implementation.
- Makes the Bridge useful before embedded TNC firmware is complete.
- Preserves the final cable-free BLE KISS goal.
- Keeps APRS/Packet logic independent of TNC placement.
- Makes future native RadioLink software modem work possible without redesigning application modules.

### Negative

- The first conventional-radio PoC uses a USB cable between Bridge/interface and desktop host.
- Mobile host software-TNC support still requires platform-specific audio/PTT research.
- The Bridge firmware will eventually support two operational modes rather than one.

## Validation order

```text
1. radio electrical interface
2. RX/TX audio conditioning
3. PTT
4. USB audio/control
5. host software-TNC validation
6. real APRS RX/TX
7. embedded AFSK modem
8. KISS
9. BLE KISS
10. mobile cable-free validation
```

## Guardrails

1. The host remains the preferred computing layer.
2. Embedded TNC mode is an optimization for portability/cable-free use, not a mandatory architectural center.
3. The Bridge must never require embedded TNC mode to be useful as a radio interface.
4. Bluetooth capability does not imply reliable modem audio or KISS capability.
5. APRS/Packet modules must consume a generic TNC backend.
