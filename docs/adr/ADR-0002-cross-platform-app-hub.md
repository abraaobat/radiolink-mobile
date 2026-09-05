# ADR-0002 — Cross-platform App Hub Architecture

**Status:** Accepted  
**Date:** 2026-09-05

## Context

RadioLink began as a smartphone-first concept inspired by replacing DigiPi with a cleaner phone-to-radio Bluetooth path. Further analysis showed that the most valuable DigiPi characteristic is not the Raspberry Pi itself, but the integrated experience of choosing among several amateur-radio applications from one place.

At the same time, the same Bluetooth KISS and protocol stack is useful on Android, iOS, Linux and macOS.

## Decision

RadioLink will be designed as a cross-platform application hub with first-class targets for:

- Android
- iOS
- Linux
- macOS

The product will expose multiple operating modules from one central shell/launcher, initially APRS, Packet/KISS diagnostics and Radio Control, with Winlink following after Packet support is stable.

Shared protocol/domain code will be preferred wherever technically practical. Platform-specific code will be limited mainly to Bluetooth/USB integration, permissions, lifecycle, background execution and native UI adaptation.

Linux and macOS may additionally expose CLI/headless workflows using the same core and drivers.

Bluetooth/BLE remains the preferred device transport. USB/audio remains a compatibility path.

## Consequences

### Positive

- One product instead of separate disconnected apps.
- Shared drivers/protocols across desktop and mobile.
- Desktop can serve as a lower-friction development and validation environment.
- Users retain DigiPi-like convenience without requiring a Raspberry Pi appliance.
- New operating modules can be added without redesigning the whole product.

### Trade-offs

- Cross-platform toolchain selection becomes a key F0 decision.
- Bluetooth behavior differs by platform and requires adapters.
- UI cannot be fully identical across desktop and mobile.
- Background/headless behavior will differ between iOS, Android and desktop systems.

## Guardrails

- Do not turn RadioLink into a generic Linux distribution.
- Do not ship unrelated third-party applications as separate experiences merely to increase feature count.
- Each new mode should integrate through the common module shell, shared device session and common capability model where practical.
- Do not let DMR, LoRa, mesh or infrastructure features expand the initial MVP.
