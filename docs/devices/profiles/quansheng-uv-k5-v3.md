# Quansheng UV-K5 V3 — RadioLink Lab Profile

- **Registry priority:** P0
- **Validation state:** `LAB_AVAILABLE`
- **Intended role:** second conventional HT baseline to prove that the software-TNC/audio path is not tied to one Quansheng model.
- **Firmware:** `UNVERIFIED`

## Planned validation path

```text
macOS → RadioLink CLI → USB/DigiRig → audio/PTT → software TNC → UV-K5 V3 → RF
```

## Current observations

No transport, capability, cable, calibration or APRS/Packet claim is considered verified yet. Physical availability is the only validated fact in this profile.

## Next checks

- identify exact hardware/firmware build;
- identify/document exact DigiRig cable/interface combination;
- verify RX audio;
- verify PTT/TX audio;
- verify software-TNC/KISS path;
- capture APRS/AX.25 RX fixture;
- perform controlled TX validation;
- compare calibration/settings against UV-K1 profile.

## Support statement

`NOT SUPPORTED` — bench validation pending.
