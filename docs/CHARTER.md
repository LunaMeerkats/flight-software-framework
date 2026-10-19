# Project charter

## Purpose

Build a practical, reusable Rust library for composing and testing
flight-software-style applications that other developers can integrate into
their own projects. NASA cFS informs the responsibility set; the library designs
and verifies its own Rust-native behavior rather than pursuing source translation
or feature parity. Current experimental host maturity is a status, not the final
product ambition.

The initial host sample and reviewed v0.1 contracts establish a regression
baseline. [The user's 2026-10-19 direction](adr/0025-practical-library-development.md)
authorizes research-led autonomous progression beyond that scope. Later milestones
may add or refine useful capabilities, APIs and architecture with recorded needs,
decisions and verification. Each run remains one bounded, defensible increment.

## Practical-library outcomes

Evaluate future work against demonstrated developer use:

- A downstream application can compose the needed behavior through public APIs
  without copying private sample internals or modifying the library.
- Getting-started guidance, runnable examples and API documentation explain
  ownership, setup, errors, resource bounds and supported use cases.
- Realistic integration scenarios expose missing capabilities and unnecessary
  complexity; targeted hardening protects behavior those consumers depend on.
- Requirements, compatibility/version policy and support/toolchain choices are
  made explicit as evidence warrants them; they are not silently frozen.

These are development outcomes to establish, not claims that current adoption,
API stability or release readiness has already been demonstrated. Publication
and the separate consequential decisions retain their authorization gates.

## Safety, maturity, and affiliation

This project is:

- not flight-qualified or safety-certified;
- not a NASA product and not NASA-endorsed;
- not proven to any Technology Readiness Level;
- not suitable for operational spacecraft, safety-critical systems, or
  human-rated systems; and
- not automatically compatible with cFS, cFE, OSAL, PSP, CCSDS, or an RTOS.

Examples and test results demonstrate only their explicitly stated host
behavior. They are not evidence of real-time performance, fault tolerance,
flight readiness, standards compliance, or mission suitability.

## Design principles

1. Prefer one end-to-end behavior with evidence over a broad interface surface.
2. Use Rust ownership and explicit state transitions to make lifecycle and
   resource ownership visible.
3. Keep queues and operational resource policies bounded and observable.
4. Inject time and external effects; avoid ambient wall-clock and hidden work.
5. Separate framework policy, host services, future platform adapters, mission
   applications, and external protocol boundaries.
6. Use safe, stable Rust by default and add dependencies only for a demonstrated
   need.
7. Record assumptions, rejected alternatives, provenance, and exact verification
   results.
8. Revise architecture when tests or measured evidence contradict it.

## Initial non-goals

- Source or API compatibility with NASA cFS components.
- A complete reimplementation of the cFS ecosystem.
- Flight deployment, certification artefacts, or qualification claims.
- Hardware drivers, RTOS targets, `no_std`, distributed operation, or extensive
  CCSDS support.
- A general-purpose async runtime, plugin system, serializer, or code generator.
- Containment of arbitrary CPU hangs, process termination, memory exhaustion,
  or hardware faults in the first host implementation.

## Source and licence policy

Research follows [the source register](research/SOURCES.md). Official primary
sources take priority. Facts observed upstream must be distinguished from this
project's design choices. Substantial reused material would require explicit
provenance and licence analysis; none is currently planned.

The project name and recipient-choice `MIT OR Apache-2.0` licence have human
approval in [ADR-0002](adr/0002-project-name-and-licensing-intent.md). Repository
content is available under the [MIT licence](../LICENSE-MIT) or the
[Apache License, Version 2.0](../LICENSE-APACHE), at the recipient's option.
Public namespace or trademark clearance is not claimed.

## Success for v0.1

Success is the verified behavior in [the v0.1 requirements](REQUIREMENTS.md),
demonstrated by a documented host sample and exact traceability evidence. It is
not cFS parity, flight readiness, or a count of modules and changed lines.
