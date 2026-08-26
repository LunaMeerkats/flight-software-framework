# Rust Flight Framework

This repository is an experiment in building a small, Rust-native framework for
host-based flight-software research. It studies responsibilities also addressed
by NASA's Core Flight System (cFS), while independently choosing designs that
fit Rust's ownership, type, error, and testing models.

> **Safety and affiliation notice:** This project is not flight-qualified,
> safety-certified, a NASA product, or NASA-endorsed. It has no demonstrated
> Technology Readiness Level and is not suitable for operational spacecraft,
> safety-critical systems, or human-rated systems. It does not claim automatic
> compatibility with cFS, cFE, OSAL, PSP, CCSDS, or any RTOS.

## Current status

The repository contains an initial research and architecture baseline, five
bounded Rust lifecycle/work increments, and three Stage 2 messaging increments.
A no-dependency `LifecycleRegistry` verifies the logical LC1 transition table. A
finite-capacity `Runtime<A>` owns statically composed application values and
executes start, caller-selected work, stop, and in-place restart synchronously.
Successful lifecycle operations enter `Running`, `Stopped`, and `Running`;
successful work retains `Running`. Restart and work retain application-owned
state, while a returned concrete operation error enters terminal `Failed`
without mutating peer records.

The standalone `MessageBus` uses mission-selected topics and inline
const-bounded payloads. It copies immutable positive-capacity inbox topology,
preserves FIFO order across topics, applies reject-newest saturation, continues
fan-out to unaffected subscribers, and returns stable ordered outcomes. The
owning `MessagingRuntime` now consumes a fully composed still-registered
runtime, constructs exactly one fresh inbox per application, derives delivery
availability from lifecycle state, clears queues after successful stop or a
returned callback error with an exact discarded-delivery count, and reconnects
an empty inbox after successful restart. A separate `MessagingApplication`
callback now receives one oldest in-flight delivery plus a publish-only context.
It can self-publish through the same bounded bus without gaining lifecycle,
dequeue, or nested-dispatch access. Seven routing-core, ten runtime-messaging,
and five message-dispatch tests cover these boundaries.

A public integration test runs two independently defined applications through
registration, start, work, stop, restart, and work, completing the bounded
RFF-REQ-002 lifecycle evidence. The combined routing, lifecycle-availability,
and caller-selected dispatch evidence now verifies RFF-REQ-003, including
capacity-one self-publication, per-dispatch availability refresh, and exact
selected-queue clearing after a returned message error. The runtime still has
no automatic or batch dispatch. It is not yet a sample mission. Time, events,
configuration, and command/telemetry boundaries remain unimplemented.
Traceability distinguishes this evidence from containment of panics, hangs,
cleanup failures, or other arbitrary faults.

The first source-quality checkpoint tracks stable rustfmt at 100 columns and
denies Clippy functions over a 60-line review threshold across all targets.
Three cohesive chronological integration tests carry narrow reasoned
expectations; two avoidable expectations were retired through focused test
structure. Production code needs no function-length waiver. Source ordering,
module cohesion, names, comment prose, and exceptional physical lines remain
review responsibilities rather than unsupported automated claims.

## Development

The package is unpublished and uses stable Rust with no external dependencies.

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

Run rustdoc with warnings denied; in PowerShell, set
`$env:RUSTDOCFLAGS = "-D warnings"` before the documentation command. Contributor
guidance records the remaining structural and document checks.

## Start here

- [Project charter](docs/CHARTER.md)
- [Initial architecture analysis](docs/ARCHITECTURE.md)
- [v0.1 requirements](docs/REQUIREMENTS.md)
- [Roadmap](docs/ROADMAP.md)
- [Current project state](docs/PROJECT_STATE.md)
- [Caller-driven runtime decision](docs/adr/0001-caller-driven-host-runtime.md)
- [Project identity and dual licensing](docs/adr/0002-project-name-and-licensing-intent.md)
- [Lifecycle and identity decision](docs/adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md)
- [Bounded inbox decision](docs/adr/0004-bounded-application-inboxes.md)
- [Configuration rollback decision](docs/adr/0005-configuration-revisions-and-rollback.md)
- [Initial application ownership decision](docs/adr/0006-static-application-ownership-for-initial-runtime.md)
- [Owned stop boundary decision](docs/adr/0007-operation-specific-owned-stop-boundary.md)
- [In-place owned restart decision](docs/adr/0008-distinct-in-place-owned-restart.md)
- [Caller-driven owned work decision](docs/adr/0009-caller-driven-owned-work.md)
- [Bounded message-routing core decision](docs/adr/0010-bounded-message-routing-core.md)
- [Runtime-owned message availability decision](docs/adr/0011-runtime-owned-message-availability.md)
- [One-message application dispatch decision](docs/adr/0012-application-message-dispatch.md)
- [Research sources and provenance](docs/research/SOURCES.md)
- [Verification traceability](docs/verification/TRACEABILITY.md)
- [Source-quality baseline](docs/verification/SOURCE_QUALITY_BASELINE.md)
- [Contributor and automation guidance](AGENTS.md)

## Licence

This repository is licensed under either the
[MIT licence](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option. The confirmed
notice is `Copyright 2026 Daniel Smith`. Cargo publication remains disabled.
