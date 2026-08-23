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

The repository contains an initial research and architecture baseline plus five
bounded Rust lifecycle and work increments. A no-dependency `LifecycleRegistry`
verifies the logical LC1 transition table. A finite-capacity `Runtime<A>` owns
statically composed application values and executes start, caller-selected
work, stop, and in-place restart synchronously. Successful lifecycle operations
enter `Running`, `Stopped`, and `Running`; successful work retains `Running`.
Restart and work retain application-owned state, while a returned concrete
operation error enters terminal `Failed` without mutating peer records.

A public integration test runs two independently defined applications through
registration, start, work, stop, restart, and work, completing the bounded
RFF-REQ-002 lifecycle evidence. The runtime still has no automatic dispatch and
is not a sample mission. The message bus, time, events, configuration, and
command/telemetry boundaries remain unimplemented. Traceability distinguishes
this evidence from broader v0.1 requirements and from containment of panics,
hangs, cleanup failures, or other arbitrary faults.

## Development

The package is unpublished and uses stable Rust with no external dependencies.

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
```

## Start here

- [Project charter](docs/CHARTER.md)
- [Initial architecture analysis](docs/ARCHITECTURE.md)
- [v0.1 requirements](docs/REQUIREMENTS.md)
- [Roadmap](docs/ROADMAP.md)
- [Current project state](docs/PROJECT_STATE.md)
- [Caller-driven runtime decision](docs/adr/0001-caller-driven-host-runtime.md)
- [Project identity and licensing intent](docs/adr/0002-project-name-and-licensing-intent.md)
- [Lifecycle and identity decision](docs/adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md)
- [Bounded inbox decision](docs/adr/0004-bounded-application-inboxes.md)
- [Configuration rollback decision](docs/adr/0005-configuration-revisions-and-rollback.md)
- [Initial application ownership decision](docs/adr/0006-static-application-ownership-for-initial-runtime.md)
- [Owned stop boundary decision](docs/adr/0007-operation-specific-owned-stop-boundary.md)
- [In-place owned restart decision](docs/adr/0008-distinct-in-place-owned-restart.md)
- [Caller-driven owned work decision](docs/adr/0009-caller-driven-owned-work.md)
- [Research sources and provenance](docs/research/SOURCES.md)
- [Verification traceability](docs/verification/TRACEABILITY.md)
- [Contributor and automation guidance](AGENTS.md)

## Licence status

The approved intent is dual licensing under `MIT OR Apache-2.0`, but the exact
copyright holder has not yet been confirmed and licence files have not been
added. Until that is completed, do not assume permission to redistribute or
reuse repository content. Cargo publication remains disabled.
