# Research sources and provenance

Access date for this initial register: **2026-08-05**.

Only public primary sources are used below. “Adopt” means adopting a problem
boundary or locally designed behavior, not claiming compatibility. This
checkpoint uses factual paraphrases rather than translated NASA implementation
code. Any future source reuse must record provenance and licence obligations.

## SRC-NASA-CFS — Core Flight System bundle v7.0.1

- Title: Core Flight System — BUNDLE
- Organisation: NASA Goddard Space Flight Center Flight Software Systems Branch
- Source: <https://github.com/nasa/cFS/blob/088b2fa828db9ff7e00733f1908e0eeb59f66ce3/README.md>
- Version: official v7.0.1 tag, commit `088b2fa828db9ff7e00733f1908e0eeb59f66ce3`
- Informed: bundle composition; distinction between framework, mission-selected
  applications, lab examples, and a flight distribution; NASA's warning that
  the bundle is a starting point and mission verification remains necessary.
- Local treatment: adopt cautious scope and responsibility analysis; reject
  parity, operational-readiness, endorsement, and distribution-equivalence
  claims.

## SRC-NASA-CFE — cFE Application Developers Guide v7.0.1

- Title: cFE Application Developer's Guide
- Organisation: NASA
- Source: <https://github.com/nasa/cFE/blob/c5fb2b4d540bd55eb6c3707da7dd13eee679d4dd/docs/cFE%20Application%20Developers%20Guide.md>
- Version: cFE v7.0.1 tag, commit `c5fb2b4d540bd55eb6c3707da7dd13eee679d4dd`
- Informed: Executive, Software Bus, Event, Table, Time, and File Service
  responsibilities; application identity/lifecycle; published message
  interfaces; bounded pipe concepts; staged table validation/activation; time
  as a service.
- Local treatment: adopt the responsibility questions and separation concerns;
  independently design Rust behavior and reject API/source translation or
  implied behavioral compatibility.

## SRC-NASA-OSAL — Operating System Abstraction Layer v7.0.1

- Title: Core Flight System: Operating System Abstraction Layer
- Organisation: NASA
- Source: <https://github.com/nasa/osal/blob/d2d877a69cff47452bcca274b309147d48e6c16f/README.md>
- Version: OSAL v7.0.1 tag, commit `d2d877a69cff47452bcca274b309147d48e6c16f`
- Informed: OS services are a distinct abstraction responsibility in cFS.
- Local treatment: retain conceptual separation between framework policy and
  host OS mechanisms; reject an empty OSAL-shaped crate before a second backend
  or concrete port requires it.

## SRC-NASA-PSP — Platform Support Package v7.0.1

- Title: Core Flight System: Platform Support Package
- Organisation: NASA
- Source: <https://github.com/nasa/PSP/blob/c4b3b0b65b119e106481ad8e20976ae4d7f554e3/README.md>
- Version: PSP v7.0.1 tag, commit `c4b3b0b65b119e106481ad8e20976ae4d7f554e3`
- Informed: platform-specific functionality is distinct from the executive and
  OS abstractions.
- Local treatment: keep future platform effects behind narrow evidence-driven
  interfaces; reject claiming PSP equivalence or creating speculative hardware
  layers.

## SRC-RUST-UNSAFE — The Rust Reference: `unsafe`

- Title: The `unsafe` keyword
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/stable/reference/unsafe-keyword.html>
- Version: stable documentation accessed 2026-08-05
- Informed: unsafe code introduces proof obligations not verified by the
  compiler.
- Local treatment: ordinary local framework crates should forbid unsafe code;
  this policy does not inspect or constrain dependency internals. Any future
  local exception requires isolation, invariants, evidence, an ADR, and human
  review.

## SRC-RUST-CHANNEL — Rust standard-library synchronous channel

- Title: `std::sync::mpsc::sync_channel`
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html>
- Version: stable documentation accessed 2026-08-05
- Informed: stable Rust provides a bounded channel whose full-buffer behavior is
  blocking for `send`, with one receiver and cloneable senders.
- Local treatment: retain it as one future implementation option; reject using
  library defaults as the bus overflow contract or mistaking one-receiver
  channels for publish/subscribe fan-out.

## SRC-RUST-TIME — Rust standard-library time types

- Title: `std::time::Instant` and `std::time::SystemTime`
- Organisation: The Rust Project
- Sources: <https://doc.rust-lang.org/std/time/struct.Instant.html> and
  <https://doc.rust-lang.org/std/time/struct.SystemTime.html>
- Version: stable documentation accessed 2026-08-05
- Informed: monotonic and non-monotonic system-clock behavior have different
  semantics and platform considerations; wall-clock access is not a
  deterministic test clock.
- Local treatment: inject framework time and use manual simulated time in tests;
  do not infer real-time guarantees.

## SRC-RUST-CARGO — Cargo workspaces

- Title: Workspaces — The Cargo Book
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/cargo/reference/workspaces.html>
- Version: stable documentation accessed 2026-08-05
- Informed: workspaces share a lockfile and target directory and can centralise
  package metadata and lint declarations; each member must opt into workspace
  lints explicitly.
- Local treatment: use the smallest workspace needed once a crate boundary is
  proven, require each member to inherit workspace lints, and do not create a
  multi-crate topology in this checkpoint.
