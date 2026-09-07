# QYT KT-7900D — RadioLink Lab Profile

- **Registry priority:** P0
- **Validation state:** `LAB_AVAILABLE`
- **Intended role:** mobile/base-form conventional-radio target to prove that RadioLink's audio/PTT/software-TNC path is not specific to handheld radios.
- **Firmware:** `UNVERIFIED`

## Planned validation path

```text
macOS → RadioLink CLI → USB/DigiRig or validated interface → audio/PTT → software TNC → KT-7900D → RF
```

## Current observations

No interface, transport, capability, cable, calibration or APRS/Packet claim is considered verified yet. Physical availability is the only validated fact in this profile.

## Next checks

- identify exact hardware/firmware revision;
- determine the correct audio/PTT/interface path;
- document cable pinout/interface requirements;
- verify RX audio;
- verify PTT/TX audio;
- verify software-TNC/KISS path;
- capture APRS/AX.25 RX fixture;
- perform controlled TX validation;
- compare operational behavior with HT profiles.

## Support statement

`NOT SUPPORTED` — bench validation pending.
