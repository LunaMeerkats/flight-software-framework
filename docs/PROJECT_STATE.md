# Project state

Last updated: **2026-11-23**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-019 revision 1
selects application-owned non-Copy output handoff through existing shared
lookup. The reviewed host v0.1 milestone remains the regression baseline.
Package 0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Implementation `6fef2a7896aad016e585c6b08b4ce773d7e55343` reached
  origin/main by fast-forward. Exact hosted run 45134831252 passed all 20
  steps, 304 workspace tests, separate adapter/sample tests, rustdoc and
  whitespace. All 222 sample lines match the input job.
- This run's pre-edit locked format/check/Clippy, 296 workspace tests,
  warnings-denied rustdoc and whitespace all passed.
- Final locked baseline passes 304 tests and all static/documentation checks.
  Identical independent binary check/Clippy/eight tests/run pass with exact
  source bytes/SHA256, metadata and seven-line stdout equality.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98. No toolchain or source-policy change is selected.
- Hosted tools remain Rust/Cargo 1.100.0, rustfmt 1.11.0-stable and Clippy
  0.1.100, runner 2.338.2 and Windows image `20261116.315.1`.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, events and optional validated configuration.
Hosts select callback and I/O turns. Shared application lookup exposes
retained values without mutable inner-owner access. Application-defined
shared methods may perform interior mutation and effects.

## Work in progress

No implementation remains. Codex accepts the
[owned-output recipe](OWNED_OUTPUT.md) after local/independent binary,
source/document/diff and exact hosted review. The
[focused record](verification/OWNED_OUTPUT_REVIEW.md) preserves actual
evidence and limitations. This record-only completion receives its own
review and exact hosted check.

## Highest risks and uncertainties

- Typed sink rejection must return the whole unaccepted record. Ambiguous
  external effects require another policy.
- Shared application methods can have interior mutation and other effects.
- Callbacks and sink turns must return. Logical bounds do not bound arbitrary
  allocation or execution time; local acceptance does not prove delivery.
- IDs remain runtime-local, with equal-slot cross-owner aliasing possible.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess structured diagnostics for returned message-callback errors against
the existing ordinary-work failure-event path before selecting another API.

## Latest run

2026-11-23. Owned-output consumer is implemented, verified, reviewed and
published. Exact implementation CI passes all 20 steps. Stop at this
bounded handoff checkpoint; no unrelated work follows.
