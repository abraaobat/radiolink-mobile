# RadioLink Nominal Device Registry

This registry records specific hardware selected to prove RadioLink architecture. Listing is not a support claim.

## P0 — Current lab hardware

| Device | Role | Initial class target | Current state | Immediate validation target |
|---|---|---|---|---|
| Quansheng UV-K1 2 MB | inexpensive conventional HT baseline | Class D — DigiRig/USB Audio + software TNC | `LAB_AVAILABLE` | macOS CLI → DigiRig → audio/PTT → software TNC → APRS/Packet |
| Quansheng UV-K5 V3 | second conventional HT baseline | Class D — DigiRig/USB Audio + software TNC | `LAB_AVAILABLE` | reproduce same service through a second radio profile |
| Baofeng UV-5 Mini | low-cost conventional HT compatibility target | Class D candidate | `LAB_AVAILABLE` | determine cable/audio/PTT profile and validate receive/transmit path |
| QYT KT-7900D | mobile/base-form conventional-radio target | Class D candidate | `LAB_AVAILABLE` | validate that the same software-TNC/provider architecture is not HT-specific |

Capabilities in the P0 profiles remain `UNVERIFIED` until bench testing. Physical possession is evidence only of lab availability.

## P1 — Reference targets

| Device | Why it matters | Architectural role | Current state |
|---|---|---|---|
| BTECH UV-Pro | research evidence shows the value of phone + embedded Bluetooth TNC/APRS workflow | Class A — Embedded BLE KISS/TNC reference | `CANDIDATE` |
| VGC/Vero VR-N76 family | useful firmware-aware BLE/TNC reference; research shows service capability can change by firmware | Class A + firmware-aware profile reference | `CANDIDATE` |
| Kenwood TH-D75 | strong multi-transport example: KISS/TNC over Bluetooth while GPS/context may use USB | Class G — Multi-transport/composite device | `CANDIDATE` |

P1 devices should be acquired, borrowed or otherwise tested when practical, but they do not block P0 development.

## P2 — Boundary / negative-control target

| Device | Why it matters | Architectural role | Current state |
|---|---|---|---|
| Retevis Ailunce HA2 | research example where Bluetooth capabilities do not imply Bluetooth TNC/KISS exposure | Class E / capability-model negative control | `RESEARCHED` |

This device is useful even if never becomes a primary supported radio because it tests whether RadioLink reports capabilities accurately rather than inferring them from marketing labels.

## Watchlist

These remain research candidates and have no assigned compatibility class until evidence is sufficient:

- Radtel RT-660;
- Baofeng DM-UV32.

Watchlist status must not be interpreted as a procurement recommendation or support promise.

## Reference interfaces / accessories

These are not radios but are important to the validation matrix:

| Interface | Role | State |
|---|---|---|
| DigiRig Mobile | current USB audio/PTT/serial-class lab interface for conventional radios | `LAB_AVAILABLE` |
| Mobilinkd TNC4 | external BLE KISS TNC architectural/interoperability reference | `RESEARCHED` |
| RadioLink Bridge | future BLE + USB-C + KISS/audio/PTT/CAT-capable accessory hypothesis | `PLANNED` |

## Promotion rule

A device profile advances only when evidence exists for the exact combination being claimed:

```text
device + firmware + host + transport + interface/cable + provider + service
```

Examples of separate claims:

- `UV-K1 + DigiRig + macOS + software TNC + APRS RX`;
- `UV-K1 + DigiRig + macOS + software TNC + APRS TX`;
- `TH-D75 + BLE + KISS + APRS`;
- `TH-D75 + USB + GPS context`.

Success in one combination does not automatically validate another.

## Research basis

Primary source-derived reference-device evidence is recorded in:

- `docs/research/KM4ACK-DEEP-MINING.md`;
- `docs/research/EVIDENCE.md`;
- `docs/research/SYNTHESIS.md`.

The current P0 inventory is a lab-planning input and must be validated experimentally.
