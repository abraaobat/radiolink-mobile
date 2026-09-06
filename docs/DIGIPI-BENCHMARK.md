# DigiPi 2.2-1 → RadioLink Functional Benchmark

This document uses the DigiPi 2.2-1 hub as a **functional coverage benchmark**, not as an architectural template.

RadioLink should preserve the convenience of selecting multiple amateur-radio functions from one place while avoiding unnecessary Raspberry Pi/Linux-appliance concepts on Android, iOS, Linux and macOS.

## Benchmark principles

1. **Copy capabilities, not implementation.**
2. **One RadioLink hub, multiple selectable modules.**
3. **Shared RadioLink Core and TNC abstraction wherever practical.**
4. **Bluetooth/BLE KISS preferred; Direwolf + DigiRig supported as the universal desktop fallback.**
5. Features that only administer the Raspberry Pi itself are not product priorities for RadioLink.
6. A DigiPi feature can map to a native RadioLink module, an integration with an external engine, or be intentionally omitted.

## Priority legend

- **P0** — core platform / first usable product
- **P1** — high-value expansion
- **P2** — later digital-mode expansion
- **N/A** — operating-system/appliance administration, not a RadioLink product module

## Main DigiPi applications

| DigiPi 2.2-1 function | RadioLink mapping | Priority | Planned implementation | Status |
|---|---|---:|---|---|
| APRS TNC/iGate | APRS module + optional APRS-IS/iGate service | P0/P1 | Shared APRS core over generic `TncTransport`; iGate after RF APRS is stable | Planned |
| APRS TNC/Digipeater | APRS Digipeater mode | P1 | Shared AX.25/APRS core; capability-gated TX | Planned |
| APRS GPS Tracker | APRS Tracker | P0 | Host GPS/location + beacon scheduler + APRS encoder | Planned |
| APRS WebChat | APRS Messaging | P0 | Chat-like UI backed by APRS messaging/ACK model | Planned |
| APRS HF TNC/iGate | APRS HF mode | P2 | Audio/software-modem or supported external modem engine | Exploration |
| AX.25 Node Network | Packet/AX.25 module | P1 | Connected-mode AX.25, node/BBS interoperability | Planned |
| Winlink Email Server | Winlink server/gateway capability | P2 | Separate server-side feasibility study; not required for initial client | Exploration |
| Pat Winlink Email Client | Winlink module | P1 | Native RadioLink UX; Telnet first, Packet next, ARDOP later | Planned |
| ARDOP HF Modem | ARDOP transport engine | P1/P2 | External/open ARDOP engine first; native integration boundary | Planned |
| MERCURY HF Modem | Mercury modem integration | P2 | Evaluate protocol/engine availability and platform portability | Backlog |
| WSJTX FT8 | FT8 module | P2 | External engine integration or future native DSP path | Backlog |
| Slow Scan TV | SSTV module | P2 | Cross-platform SSTV codec/UI | Backlog |
| FLDigi | Multi-mode digital module | P2 | Prefer integration boundary rather than reimplementing every modem initially | Backlog |
| JS8Call | JS8 module | P2 | External engine integration first; shared RadioLink shell/UI where practical | Backlog |

## DigiPi utility links

| DigiPi utility | RadioLink mapping | Priority | Decision |
|---|---|---:|---|
| PatEmail | Winlink inbox/outbox/compose | P1 | Native RadioLink module |
| AX.25 | Packet diagnostics / terminal | P0/P1 | Native core + CLI/GUI tools |
| JS8Call | JS8 module | P2 | Future module |
| WSJTX | FT8 module | P2 | Future module |
| SSTV | SSTV module | P2 | Future module |
| FLDigi | Digital modes module | P2 | Future integration |
| WiFi | Host OS networking | N/A | Do not duplicate OS settings |
| Shell | RadioLink CLI / diagnostics | P0 | Provide RadioLink CLI, not a general shell |
| PacketLog | RF/KISS/AX.25 logs | P0 | Native diagnostics/logging |
| SysLog | RadioLink application logs | P0 | Native tracing/log viewer |
| Screen | Host OS / terminal multiplexer | N/A | No product module |
| Webchat | APRS Messaging | P0 | Native module |
| Audio | Audio device diagnostics | P1 | Needed for DigiRig/software-TNC path |
| SysInfo | RadioLink diagnostics | P1 | App/device diagnostics only, not full OS administration |
| GPS | Location service | P0 | Native host location adapter |
| Bluetooth | Device manager / capabilities | P0 | Native RadioLink device layer |
| Map | APRS/map module | P0 | Native map/station view |
| Refresh | Hub/device state refresh | P0 | Normal application behavior |
| Initialize | Onboarding/device setup | P0 | Guided setup, not Linux initialization |
| Restart | App/service restart if needed | N/A/P1 | No machine restart control required |
| Shutdown | Host OS shutdown | N/A | Omit |
| Save Configuration | Settings/profile persistence | P0 | Native RadioLink settings/profiles |

## RadioLink module groups derived from the benchmark

### Group A — Core radio/data platform (P0)

```text
RadioLink
├── Device Manager
│   ├── Bluetooth/BLE
│   ├── KISS
│   ├── TNC backend selection
│   ├── DigiRig/Direwolf fallback
│   └── Radio capabilities
├── APRS
│   ├── Receive
│   ├── Tracker / Beacon
│   ├── Map
│   ├── Messaging
│   └── Logs
├── Packet / AX.25
│   ├── Monitor
│   └── Terminal foundation
└── Diagnostics
    ├── KISS
    ├── PacketLog
    ├── Audio
    └── Application logs
```

### Group B — High-value expansion (P1)

```text
├── APRS iGate
├── APRS Digipeater
├── AX.25 connected mode
├── Winlink
│   ├── Telnet
│   ├── Packet
│   └── ARDOP
└── Radio Control
```

### Group C — Digital-mode expansion (P2)

```text
├── FT8
├── JS8
├── SSTV
├── FLDigi-class modes
├── Mercury
└── HF APRS / additional modem engines
```

## Coverage philosophy

RadioLink does **not** need to reproduce the DigiPi process model. For example, DigiPi may launch several independent Linux applications; RadioLink may instead expose one module shell backed by shared native code or external engines.

The compatibility goal is therefore expressed as:

> **Can a RadioLink user perform the same useful radio operation from the hub?**

—not—

> **Does RadioLink run the exact same Linux program?**

## Benchmark milestone levels

### Level 1 — Packet/APRS parity baseline
- BLE KISS and software-TNC backends
- APRS receive/transmit
- GPS tracker
- map
- APRS messaging
- KISS/Packet logs
- AX.25 monitor

### Level 2 — Communication hub baseline
- APRS iGate/digipeater
- connected AX.25
- Packet terminal
- Winlink Telnet/Packet
- radio-control module

### Level 3 — DigiPi-class digital hub expansion
- ARDOP
- SSTV
- FT8
- JS8
- FLDigi-class modes
- selected HF modem integrations

## Reference

Source benchmark: DigiPi 2.2-1 application hub screenshot supplied during RadioLink product design, September 2026.
