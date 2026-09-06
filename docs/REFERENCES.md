# RadioLink Platform — Technical References

RadioLink uses external projects as engineering references, not as runtime dependencies or architectural templates to be copied wholesale.

## Reference roles

### DigiPi

**Role:** functional coverage and App Hub reference.

Use DigiPi to benchmark which amateur-radio workflows should be available from one place, including APRS, Packet/AX.25, Winlink, digital modes, logs, GPS and supporting tools.

RadioLink does **not** adopt DigiPi's Raspberry Pi appliance architecture as its own runtime model.

See also: `DIGIPI-BENCHMARK.md`.

---

### Mobilinkd TNC4

**Role:** hardware/TNC reference for the RadioNode-BR and for external BLE KISS TNC interoperability.

Key concepts to preserve as reference points:

- wireless host-to-TNC link;
- KISS TNC abstraction;
- radio-side audio/PTT interface;
- APRS and general Packet operation;
- 1200/9600 baud capability as a longer-term benchmark;
- compact portable implementation.

For RadioLink, Mobilinkd-class hardware should be represented as a capability-driven `TncTransport`, not as a special case inside APRS or Packet modules.

---

### HTCommander

Repository: https://github.com/Ylianst/HTCommander

**Role:** Bluetooth radio-integration, device-control and driver/protocol reference.

HTCommander is a multi-platform tool focused on control of radios including UV-Pro, GA-5WB, VR-N76 and VR-N7500-class devices. RadioLink should study it specifically for the Bluetooth Device Layer and radio-driver work.

#### What RadioLink should learn from HTCommander

- Bluetooth device discovery and connection lifecycle;
- device identification and profile selection;
- radio-specific protocol handling;
- separation between generic application behavior and per-radio behavior;
- capability-driven controls;
- connection/configuration UX patterns;
- multi-platform handling of supported radio families.

#### What HTCommander is not

HTCommander is **not** the architectural template for the whole RadioLink product and is not treated as the APRS/Packet/Winlink core.

Its main reference value is concentrated in:

- F2 — Bluetooth Device Layer;
- F9 — Radio Control Module;
- F13 — Driver SDK + Compatibility Matrix.

#### Guardrail

Bluetooth control support discovered through HTCommander or similar projects must not be interpreted as proof of KISS/TNC capability. RadioLink must continue to represent control, audio, serial/data, KISS and embedded-TNC capabilities independently.

---

## Consolidated reference model

```text
DigiPi
  -> App Hub / functional coverage

Mobilinkd TNC4
  -> BLE KISS TNC / portable radio interface

HTCommander
  -> Bluetooth radio integration / control / drivers

RadioLink Platform
  -> combines these lessons behind one capability-driven architecture
```

## Reference policy

When studying external projects:

1. Prefer documented protocols and public interfaces over implementation-specific assumptions.
2. Preserve RadioLink's platform-neutral Rust core boundaries.
3. Keep platform Bluetooth APIs in host/adapter layers.
4. Keep radio-specific behavior inside drivers/profiles.
5. Never equate Bluetooth presence with Packet/KISS/TNC support.
6. Record interoperability observations and test fixtures in the repository before promoting a device to a supported profile.
