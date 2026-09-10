# APRS support matrix

RadioLink implements APRS incrementally on top of the AX.25 UI-frame and KISS layers. Protocol support is only marked complete when deterministic tests exist; live RF compatibility remains a separate validation gate.

## Implemented information-field subset

| APRS data type | Meaning | Current support |
|---|---|---|
| `!` | Position without timestamp, no messaging | Uncompressed latitude/longitude, symbol table/code, comment |
| `=` | Position without timestamp, messaging capable | Uncompressed latitude/longitude, symbol table/code, comment |
| `>` | Status | Plain status and optional DHM `z` timestamp |
| `:` | Message | 9-character addressee, text, optional message ID, `ack` and `rej` |

## Explicitly not implemented yet

- timestamped position reports (`/` and `@`);
- compressed positions;
- Mic-E;
- objects/items;
- weather-specific payload interpretation;
- telemetry;
- queries;
- bulletins/announcements as higher-level message semantics;
- APRS 1.1/1.2 extensions beyond the core subset above.

Unsupported data type identifiers produce an explicit parser error instead of being silently misclassified.

## Layering

```text
KISS stream
  ↓
AX.25 UI frame
  ↓ information field
APRS parser
  ↓
Position | Status | Message
```

This keeps APRS logic independent of BLE, USB, Dire Wolf, an embedded TNC or a specific radio.

## Validation policy

Unit/CI validation proves codec behavior only. It does not claim RF interoperability. Captured real-world fixtures and live TNC/radio tests remain separate roadmap gates.
