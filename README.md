# Rust Flight Framework (working title)

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

The repository currently contains an initial research, requirements, and
architecture checkpoint. There is no Rust implementation or executable sample
yet, and no behavior is verified beyond repository/document consistency.

The first implementation increment is expected to prove one explicit
application lifecycle through a caller-driven host runtime. It will not create
a broad collection of placeholder subsystems.

## Start here

- [Project charter](docs/CHARTER.md)
- [Initial architecture analysis](docs/ARCHITECTURE.md)
- [v0.1 requirements](docs/REQUIREMENTS.md)
- [Roadmap](docs/ROADMAP.md)
- [Current project state](docs/PROJECT_STATE.md)
- [Architecture decisions](docs/adr/0001-caller-driven-host-runtime.md)
- [Research sources and provenance](docs/research/SOURCES.md)
- [Verification traceability](docs/verification/TRACEABILITY.md)
- [Contributor and automation guidance](AGENTS.md)

## Licence status

No project licence has been selected. Do not assume permission to redistribute
or reuse repository content. Licence selection requires explicit human review.
