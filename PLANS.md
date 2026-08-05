# Plan: bounded LC1 lifecycle registry

Status: **Complete**
Date: **2026-08-05**

## Objective

Record the approved project identity, licensing intent, lifecycle, application
identity, message-bus, and configuration decisions, then implement only the
smallest logical lifecycle and identity slice that can be exhaustively tested.

## Context

The user approved the name Rust Flight Framework, the LC1 stop-gated lifecycle,
runtime-local application identity, bounded per-application inbox semantics,
and monotonic configuration revision allocation with one-level rollback. The
repository previously contained architecture documents but no Rust code.

The approved licensing expression is `MIT OR Apache-2.0`. Applying it remains
blocked on the exact copyright-holder text, so this slice remains unpublished
and does not add licence files or a Cargo licence declaration.

## Acceptance criteria

- The approved decisions are recorded in focused ADRs with limitations and
  revisit conditions.
- One unpublished, dependency-free Rust package exposes a finite-capacity
  lifecycle registry.
- Registration produces opaque identifiers that remain stable and unreused for
  the registry's lifetime.
- The logical LC1 transitions `Registered -> Running -> Stopped -> Running`
  are explicit; every invalid state/operation pair returns a typed error without
  changing state.
- Capacity exhaustion and zero capacity return typed errors without mutation.
- Tests demonstrate the complete transition table and independent lifecycle
  records.
- Formatting, build, lint, tests, documentation, Markdown links, and repository
  consistency checks pass before local commits are created.
- Traceability calls RFF-REQ-002 only partially verified: no application object,
  execution boundary, returned-error ingress, or host mission exists yet.

## Proposed files and components

- `Cargo.toml`, `src/lib.rs`, and `src/lifecycle.rs` for the minimal library.
- `tests/lifecycle_registry.rs` for public-boundary behavior.
- ADR-0002 through ADR-0005 for the approved decisions.
- Existing project, roadmap, requirement, provenance, and verification records
  for truthful state updates.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo check --workspace --all-targets --all-features`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo doc --workspace --no-deps`.
- Resolve relative Markdown links and cross-check requirement/source identifiers.
- Run `git diff --check` and review the complete diff.

## Known risks

- An identifier from one registry may numerically alias an identifier from
  another; validity is scoped to the originating registry and the API does not
  yet encode that origin.
- The `Failed` state is represented for an exhaustive lifecycle model, but this
  slice intentionally provides no failure-ingress operation.
- The future application execution boundary may require a compatible extension
  or a revision of this logical registry API.
- The approved bus and configuration policies have no implementation evidence
  in this slice.

## Safe rollback or stopping point

Stop with the bounded logical registry and decision records. Do not add an
application callback API, message bus, configuration service, concurrency,
external dependency, or speculative portability layer in this increment.

## Result

The stopping point was reached in commit
`0844d7c21a715492a7754072c0d80fa2d7b812fe`. The package remains unpublished
and dependency-free. The exhaustive 12-pair lifecycle table and four public API
tests passed, for six tests total. Formatting, build, lint, documentation,
relative-link, requirement/source-identifier, and staged-diff checks also
passed. RFF-REQ-002 is recorded as partially verified; no application execution,
returned-error containment, bus, or configuration behavior is claimed.
