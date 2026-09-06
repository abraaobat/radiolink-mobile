# ADR-0003 — Rust as the RadioLink Shared Core

- **Status:** Accepted
- **Date:** 2026-09-05

## Context

RadioLink targets Android, iOS, Linux and macOS while sharing protocol, transport, session, device-capability and domain logic. The shared layer must also support a native CLI on Linux/macOS and leave room for future DSP/modem work.

## Decision

Use **Rust stable** as the language for the RadioLink Shared Core.

### Responsibilities of the Rust core

- KISS framing and streaming parsing
- AX.25 framing/parsing
- APRS encoding/decoding
- Packet/session abstractions
- TNC abstraction
- device capability model
- driver-independent radio/session domain logic
- diagnostics and shared logging interfaces
- shared test fixtures
- CLI implementation

### Host responsibilities

Bluetooth, USB permissions, location, notifications, background execution and native UI remain in platform host layers:

- Android: Kotlin
- iOS: Swift
- macOS: native host adapter and/or desktop shell
- Linux: native host adapter and/or desktop shell

The Rust core must not depend directly on Android/iOS/macOS/Linux Bluetooth APIs.

## Toolchain

- Rust stable
- Cargo workspace
- `tokio` for async runtime where required
- `serde` for serialization
- `thiserror` for library errors
- `anyhow` for application/CLI error context
- `tracing` + `tracing-subscriber` for structured diagnostics
- `clap` for CLI parsing
- unit/integration tests via `cargo test`
- binary protocol fixtures committed to the repository

## FFI strategy

Start with clean Rust crate boundaries and a host-facing API. Use a C-compatible ABI or UniFFI only after the core API stabilizes enough to justify generated bindings.

## Consequences

### Positive

- one low-level core across all target platforms;
- excellent binary/protocol handling;
- native Linux/macOS CLI without JVM/runtime dependency;
- strong memory safety;
- suitable base for future DSP/modem code;
- clear separation between platform hardware APIs and radio protocol logic.

### Trade-offs

- additional FFI work for Android/iOS;
- steeper learning curve than Kotlin-only development;
- UI remains platform-specific rather than fully shared.

## Guardrail

Do not move platform-specific Bluetooth/UI code into the Rust core merely to maximize code sharing. The objective is a portable **radio/protocol core**, not a monolithic cross-platform application runtime.
