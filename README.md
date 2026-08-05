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

The repository contains an initial research and architecture baseline plus the
first bounded Rust behavior: a no-dependency `LifecycleRegistry` that allocates
runtime-local identities and enforces the successful LC1 lifecycle sequence.
This is only a logical lifecycle-record slice, not an executable application
runtime or sample mission.

Returned application errors, application-object ownership, the message bus, and
configuration lifecycle remain unimplemented. Traceability distinguishes this
partial evidence from complete v0.1 requirements.

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
- [Research sources and provenance](docs/research/SOURCES.md)
- [Verification traceability](docs/verification/TRACEABILITY.md)
- [Contributor and automation guidance](AGENTS.md)

## Licence status

The approved intent is dual licensing under `MIT OR Apache-2.0`, but the exact
copyright holder has not yet been confirmed and licence files have not been
added. Until that is completed, do not assume permission to redistribute or
reuse repository content. Cargo publication remains disabled.
