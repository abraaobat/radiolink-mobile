# Baofeng UV-5 Mini — RadioLink Lab Profile

- **Registry priority:** P0
- **Validation state:** `LAB_AVAILABLE`
- **Intended role:** low-cost conventional HT compatibility target for the DigiRig/software-TNC path.
- **Firmware:** `UNVERIFIED`

## Planned validation path

```text
macOS → RadioLink CLI → USB/DigiRig → audio/PTT → software TNC → UV-5 Mini → RF
```

## Current observations

No transport, capability, cable, calibration or APRS/Packet claim is considered verified yet. Physical availability is the only validated fact in this profile.

## Next checks

- confirm exact model/hardware revision;
- identify the correct radio-side cable/interface;
- enumerate DigiRig USB interfaces on macOS;
- verify RX audio and PTT/TX audio;
- verify software-TNC KISS path;
- capture APRS/AX.25 RX fixture;
- perform controlled TX validation;
- record calibration/settings and compare with Quansheng profiles.

## Support statement

`NOT SUPPORTED` — bench validation pending.
