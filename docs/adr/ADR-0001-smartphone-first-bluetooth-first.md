# ADR-0001 — Smartphone-first, Bluetooth-first Architecture

- **Status:** Accepted
- **Date:** 2026-09-05

## Context

The project originated from the question of whether DigiPi-like capabilities could run directly on Android/iPhone. Reproducing DigiPi on a phone would bring Linux/Raspberry Pi assumptions, platform-specific constraints and unnecessary system complexity into a mobile product.

Modern smartphones already provide CPU, display, touch UI, GPS, storage, Wi-Fi/cellular connectivity, Bluetooth and battery management. Some modern amateur radios and TNCs already expose data and control functions over Bluetooth/BLE.

## Decision

RadioLink Mobile will be designed around the following principle:

> **The smartphone is the computer. The radio is the RF endpoint. Bluetooth/BLE is the preferred bridge.**

The mobile app owns:
- user interface;
- station/message models;
- APRS/AX.25/KISS protocol processing where appropriate;
- local storage;
- GPS/location integration;
- orchestration and device capability handling.

The radio/TNC owns:
- RF transmission/reception;
- hardware-specific modulation/TNC functions when available;
- hardware control exposed by its supported interface.

For legacy radios, a future RadioLink Bridge may provide Bluetooth, TNC/PTT/audio/CAT adaptation, but it must remain an accessory rather than becoming the application's main computer.

## Consequences

### Positive

- Removes Raspberry Pi as a required component.
- Enables clean Android/iOS user experiences.
- Reduces cables and external modules for compatible radios.
- Creates a reusable driver architecture for multiple radios/TNCs.
- Makes offline field operation practical.
- Keeps protocol complexity behind a modern mobile interface.

### Trade-offs

- Android and iOS have different Bluetooth/background constraints.
- Some radios expose proprietary or undocumented Bluetooth services.
- Not every legacy radio can participate without an adapter.
- Feature parity may vary by device capability.

## Rejected alternative

### Run DigiPi/Linux stack directly on the smartphone

Rejected as the primary architecture because it would couple the project to Linux daemons, audio/device workarounds and platform restrictions while producing a less native user experience.

DigiPi remains a separate and valid Raspberry Pi/Linux solution; RadioLink Mobile is not intended to replace it feature-for-feature.

## Guardrails

1. No Raspberry Pi dependency in the core mobile product.
2. No requirement for Linux daemons for core APRS workflows.
3. Device-specific behavior lives in drivers.
4. Bluetooth/BLE is preferred for initial hardware integrations.
5. New protocol/hardware features must not compromise the simple mobile UX.
6. DMR, LoRa and mesh are out of the initial MVP and require separate future decisions.
