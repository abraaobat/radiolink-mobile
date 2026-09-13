# RadioLink Mesh + Situational Awareness

## Purpose

Define the planned post-MVP expansion that combines resilient off-grid mesh communication with a lightweight, map-centered situational-awareness service.

The intent is to give RadioLink users a coherent way to exchange position, short messages, markers, routes and alerts over whichever delivery path is available, without turning the application into a full ATAK clone or making LoRa/Meshtastic a dependency of APRS, Packet or Winlink.

## Product decision

RadioLink will add two related but independent capabilities after the initial APRS/Packet transport architecture is proven:

1. **RadioLink Mesh** — a provider layer for mesh networks, beginning with Meshtastic interoperability over BLE/USB-connected LoRa nodes.
2. **RadioLink Situational** — an offline-first map and coordination module with a transport-neutral event model and optional CoT/TAK interoperability.

These capabilities are now planned roadmap work rather than unscoped Labs ideas. They remain outside the initial MVP and must not delay the first KISS/AX.25/APRS vertical slices.

## Terminology and boundaries

### Mesh

A network in which radio nodes can relay packets for other nodes. For the first implementation, routing/rebroadcasting remains in the external Meshtastic-compatible node firmware.

The phone or computer:

- connects to its local node over BLE or USB;
- sends and receives application payloads;
- displays nodes, messages and network health;
- may bridge explicitly selected events to another network;
- does not need to remain awake as the RF relay/router.

### Meshtastic interoperability

Compatibility with the established Meshtastic ecosystem, PhoneAPI/protobuf model and supported application ports where technically and legally appropriate.

RadioLink should interoperate before considering a RadioLink-specific mesh protocol. A custom protocol requires a separate decision and must demonstrate a clear capability that established protocols cannot provide.

### TAK / ATAK / CoT

ATAK is a moving-map and situational-awareness application. It can operate over IP networks and can use mesh networks through compatible transports or plugins; ATAK itself is not the LoRa mesh routing layer.

CoT, or Cursor on Target, is an interoperability format used by the TAK ecosystem. RadioLink may encode/decode a deliberately supported CoT subset and connect to a TAK Server, but RadioLink Situational remains its own cross-platform module.

## Target experience

```text
RadioLink
│
├── Mesh
│   ├── local node
│   ├── node list
│   ├── direct/channel messages
│   ├── network health
│   └── delivery/queue state
│
└── Situational
    ├── offline map
    ├── team/member positions
    ├── chat
    ├── markers
    ├── routes and simple shapes
    ├── alerts/emergencies
    └── mission/event history
```

The same high-level event may be delivered through different providers:

```text
Position / message / marker / alert
                 │
          Situational Event Core
        ┌────────┼───────────┐
        ▼        ▼           ▼
  Meshtastic   APRS      IP / TAK Server
     mesh     mapping       CoT
        │        │           │
   LoRa node   TNC/RF      network
```

Mappings are capability-dependent. APRS cannot represent every rich situational event, and low-bandwidth mesh links must not be treated as general-purpose IP pipes.

## Architectural additions

### `MeshProvider`

A provider parallel to, not inside, `TncBackend / ModemProvider`.

```text
MeshProvider
├── discoverLocalNode()
├── connect()/disconnect()
├── nodeDirectory()
├── send()/receive()
├── deliveryState()
├── channelCapabilities()
├── health()
└── diagnostics()
```

Initial implementation:

```text
MeshtasticMeshProvider
  ↓
BLE or USB PhoneAPI adapter
  ↓
Meshtastic-compatible LoRa node
  ↓
LoRa mesh
```

Future providers such as Reticulum require separate validation and do not change the initial Meshtastic-first decision.

### `SituationalEvent`

A transport-neutral domain model for:

- participant identity and position;
- text/GeoChat-style messages;
- markers and points of interest;
- routes and simple shapes;
- alerts/emergencies;
- timestamps, expiry/staleness and acknowledgements;
- source, provenance and delivery state.

Adapters map supported subsets to:

- Meshtastic TAK/application payloads;
- CoT for ATAK/WinTAK/TAK Server interoperability;
- APRS positions/messages where semantics are compatible;
- local persistence and export.

Unsupported fields must be preserved locally when possible and reported as not transferable on the selected provider rather than silently discarded.

## LoRa data policy

LoRa airtime and payload size, not smartphone CPU, are the primary constraints. RadioLink therefore classifies outbound content before selecting a provider.

### Appropriate for LoRa mesh

- short text;
- position and identity updates;
- compact markers;
- alerts/emergencies;
- concise route/shape updates when supported;
- small telemetry/status records;
- acknowledgements and delivery metadata.

### Not appropriate for normal LoRa mesh delivery

- base maps or map tiles;
- high-resolution photos;
- video;
- real-time voice/audio streams;
- large documents or unrestricted mission packages;
- frequent high-precision position updates from every participant.

Large content may be referenced by compact metadata and transferred later over Wi-Fi, cellular, local IP or another higher-bandwidth provider. Fragmented LoRa transfer, where supported, is an explicit slow-path operation with user-visible size/time/airtime impact.

## Performance and power model

The baseline RadioLink + Mesh + Situational workload should remain lightweight on a modern smartphone because mesh forwarding and LoRa modulation run on the external node.

Expected load classes:

| Function | Expected host load |
|---|---|
| KISS/AX.25/APRS codecs | very low |
| Mesh message/protobuf processing | very low |
| CoT/event conversion | low |
| Node directory, chat and persistence | low |
| GPS and continuous BLE session | low CPU; meaningful battery impact |
| Offline vector map and normal overlays | moderate |
| Large raster imagery/many overlays | moderate to high |
| Audio DSP/modems | moderate |
| Video, 3D and image analysis | high and outside the mesh baseline |

Implementation requirements:

- lazy-load optional modules and map layers;
- use bounded queues and backpressure;
- deduplicate repeated mesh/CoT events;
- apply expiry/staleness rules;
- adapt position frequency to movement, battery and channel conditions;
- prefer offline vector maps and preloaded content;
- avoid continuous redraw when state is unchanged;
- persist compact event/state data locally;
- expose battery, queue and channel-utilization diagnostics;
- benchmark CPU, memory, battery and reconnect behavior on representative Android/iOS devices before declaring support.

Mobile background-policy limitations are treated as a lifecycle/reliability problem. The external node remains capable of mesh participation even while the host application is suspended or disconnected.

## Security and privacy guardrails

- Position sharing is explicit, channel-scoped and visible to the operator.
- RadioLink must not imply end-to-end confidentiality merely because one transport supports encryption.
- Channel keys, credentials and TAK certificates must be stored using platform-secure facilities and never written to normal diagnostics.
- Bridges between mesh, APRS, local IP and TAK Server are opt-in and apply loop prevention, deduplication and scope filters.
- Emergency/priority events must not bypass operator-configured sharing boundaries.
- Region/frequency settings and encryption/identification behavior must remain compatible with applicable radio rules and operator configuration.

## Delivery phases

### Phase 1 — Mesh foundation

- `MeshProvider` API and fixtures;
- Meshtastic node connection over BLE/USB;
- node list, text, position and telemetry;
- delivery/queue/health diagnostics;
- offline two-node field validation.

### Phase 2 — Situational baseline

- offline map;
- participant position and staleness;
- chat, markers, routes and alerts;
- local event history;
- provider-aware send UI.

### Phase 3 — TAK interoperability

- supported CoT subset;
- TAK Server transport where appropriate;
- Meshtastic TAK payload mapping;
- interoperability tests with ATAK-CIV/WinTAK or another documented TAK client;
- explicit reporting for fields that cannot cross a selected low-bandwidth provider.

## Success criteria

This expansion is successful when:

1. two RadioLink users exchange position and text through two Meshtastic-compatible nodes with no Internet;
2. the local node continues mesh participation independently of the host app lifecycle;
3. RadioLink displays participants, messages, markers and alerts on an offline map;
4. a documented CoT subset interoperates with at least one reference TAK client/server path;
5. provider selection prevents unsuitable large payloads from being sent blindly over LoRa;
6. measured mobile CPU, memory and battery behavior stays within documented budgets on representative devices;
7. APRS, Packet and Winlink remain usable without installing or enabling Mesh/Situational modules.

