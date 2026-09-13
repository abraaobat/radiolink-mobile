# ADR-0006 — Meshtastic-first Mesh and TAK-compatible Situational Awareness

**Status:** Accepted  
**Date:** 2026-09-12

## Context

RadioLink is already designed as a cross-platform, offline-first radio platform with protocol and service logic separated from devices, transports and TNC/modem providers.

The product can naturally extend beyond APRS/Packet into resilient team communication and map-centered situational awareness. Meshtastic demonstrates that low-power external nodes can provide decentralized LoRa mesh communication, while the TAK ecosystem demonstrates a useful event model and operational UX for positions, chat, markers, routes and alerts.

These are related but separate concerns:

- ATAK is a moving-map/situational-awareness client and does not itself provide the LoRa mesh routing layer;
- Meshtastic provides the mesh and already has TAK/CoT interoperability paths;
- low-bandwidth LoRa links cannot carry every type of ATAK/TAK content efficiently;
- implementing mesh routing on the smartphone would create unnecessary lifecycle and background-execution fragility.

Earlier ADRs intentionally kept LoRa/mesh outside the initial MVP. The question is now whether to keep the idea indefinitely in Labs, build a new proprietary mesh, embed ATAK wholesale or define a bounded post-MVP expansion.

## Decision

### 1. Promote a bounded Mesh + Situational expansion

RadioLink will add two post-MVP roadmap phases:

- **RadioLink Mesh**, beginning with Meshtastic interoperability;
- **RadioLink Situational**, with offline maps and a transport-neutral event model plus optional CoT/TAK interoperability.

This decision promotes the specific Meshtastic/TAK-compatible direction out of unscoped Labs. It does not make mesh a dependency or exit criterion of the initial APRS/Packet MVP.

### 2. Use Meshtastic before inventing a RadioLink mesh protocol

The first mesh provider will communicate with Meshtastic-compatible nodes through documented BLE/USB APIs and data models where practical.

A RadioLink-specific mesh protocol requires a future ADR showing a concrete unmet need, interoperability plan, spectrum/airtime impact and maintenance justification.

### 3. Keep RF mesh forwarding on the external node

The external LoRa node owns radio modulation, mesh routing/rebroadcasting and store/forward behavior supported by its firmware.

The RadioLink host owns:

- application UX;
- event/state storage;
- map rendering;
- message composition;
- optional, explicit bridging;
- diagnostics and configuration.

This boundary keeps the smartphone workload modest and prevents mobile background restrictions from becoming a prerequisite for mesh continuity.

### 4. Add a provider parallel to the TNC/modem provider

Mesh traffic will use a `MeshProvider` abstraction rather than being forced through `TncBackend`.

```text
Operational service
   ├── TNC/Modem Provider → AX.25/APRS/Packet RF
   ├── Mesh Provider → Meshtastic/LoRa mesh
   └── Network Provider → IP/TAK Server where applicable
```

Host-to-local-node connectivity still uses the existing Transport Manager for BLE/USB lifecycle and diagnostics.

### 5. Build a RadioLink-native situational module, not a full ATAK clone

RadioLink will define a `SituationalEvent` model for supported position, chat, marker, route/shape and alert workflows.

CoT will be an interoperability codec/adapter. ATAK-CIV/WinTAK/TAK Server are reference interoperability targets, not mandatory runtimes and not UI templates to copy wholesale.

### 6. Make delivery capability and airtime explicit

Each provider advertises supported event types, payload constraints, delivery semantics and health.

Normal LoRa delivery is restricted to compact operational events. Maps, map tiles, high-resolution images, video, real-time audio and large unrestricted files must use another provider or a user-confirmed slow path.

### 7. Preserve a lightweight mobile baseline

Optional modules are lazy-loaded. Queues are bounded. Repeated events are deduplicated. Position rates adapt to movement, battery and channel conditions. Offline maps prefer efficient preloaded/vector data.

Support claims require representative Android/iOS CPU, memory, battery, background/reconnect and map-rendering measurements.

## Consequences

### Positive

- RadioLink gains resilient off-grid team messaging and position sharing without requiring Internet infrastructure.
- Existing Meshtastic hardware and networks become useful instead of creating an isolated ecosystem.
- TAK/CoT interoperability becomes possible without coupling RadioLink to ATAK itself.
- Mesh routing continues when the phone sleeps or disconnects.
- A common event model can bridge selected information between Meshtastic, APRS and IP/TAK providers.
- Smartphone processing remains dominated by normal map/GPS/BLE work rather than mesh routing.

### Trade-offs

- Provider resolution now includes delivery semantics, payload limits and bridge policy, not only physical transport.
- CoT and Meshtastic mappings are lossy for some event types and require explicit capability reporting.
- LoRa airtime, congestion and regulatory configuration are stricter constraints than CPU.
- Offline maps can create significant storage/battery load if not bounded.
- Android and iOS have different background Bluetooth/location behavior.
- Supporting Meshtastic interoperability adds upstream-version and protocol-compatibility work.

## Guardrails

- Mesh/Situational work must not block the initial KISS/AX.25/APRS MVP.
- Do not call ATAK a mesh protocol or assume that a TAK client provides radio routing.
- Do not make the smartphone the required always-on mesh relay.
- Do not send maps, video or other large content over LoRa by default.
- Do not silently drop unsupported event fields during provider conversion.
- Do not bridge position or messages between networks without explicit operator policy.
- Do not expose keys, credentials or TAK certificates in normal logs/diagnostics.
- Do not claim full ATAK compatibility from a partial CoT implementation.
- Reticulum and alternative mesh protocols remain separate Labs/research decisions.

## Supersession scope

ADR-0001, ADR-0002 and ADR-0004 remain valid: LoRa/mesh is still outside the **initial MVP**. This ADR supersedes only the indefinite deferral of a specific Meshtastic/TAK-compatible direction by assigning it bounded post-MVP phases and architectural guardrails.

## Detailed design reference

See `docs/MESH-AND-SITUATIONAL-AWARENESS.md`.

