# Quansheng UV-K1 2 MB — RadioLink Lab Profile

- **Registry priority:** P0
- **Validation state:** `LAB_AVAILABLE`
- **Intended role:** first conventional-radio baseline for macOS CLI + DigiRig + software-TNC validation.
- **Firmware:** `UNVERIFIED`

## Planned validation path

```text
macOS → RadioLink CLI → USB/DigiRig → audio/PTT → software TNC → UV-K1 → RF
```

## Current observations

No transport, capability, cable, calibration or APRS/Packet claim is considered verified yet. Physical availability is the only validated fact in this profile.

## Next checks

- identify exact firmware/build;
- identify and document the exact DigiRig cable/interface combination;
- enumerate DigiRig USB interfaces on macOS;
- verify RX audio path;
- verify PTT/TX audio path;
- verify software-TNC KISS path;
- capture APRS/AX.25 RX fixture;
- perform controlled TX validation;
- record calibration and required radio-side settings.

## Support statement

`NOT SUPPORTED` — bench validation pending.
