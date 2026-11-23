# Project state

Last updated: **2026-11-23**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-019 revision 1
selects application-owned non-Copy output handoff through existing shared
lookup. The reviewed host v0.1 milestone remains the regression baseline.
Package 0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Input `356697a288d11fc168e450cff0d2579f363557d1` matches refreshed
  origin/main. Exact hosted run 44959878838 passed all 20 steps and 296
  workspace tests, separate adapter/sample tests, rustdoc and whitespace.
- This run's pre-edit locked format/check/Clippy, 296 workspace tests,
  warnings-denied rustdoc and whitespace all passed.
- Final locked baseline passes 304 tests and all static/documentation checks.
  Identical independent binary check/Clippy/eight tests/run pass with exact
  source bytes/SHA256, metadata and seven-line stdout equality.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98. No toolchain or source-policy change is selected.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, events and optional validated configuration.
Hosts select callback and I/O turns. Shared application lookup exposes
retained values without mutable inner-owner access. Application-defined
shared methods may perform interior mutation and effects.

## Work in progress

The [owned-output recipe](OWNED_OUTPUT.md) is implemented and locally
verified. One private application Cell slot and one retained host slot move
a non-Copy record to a typed sink. Source and rendered-document review pass.
Publication and exact hosted verification remain pending in the
[focused record](verification/OWNED_OUTPUT_REVIEW.md).

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

After this bounded handoff checkpoint, reassess a concrete downstream need
before adding another service or retry policy.

## Latest run

2026-11-23. Owned-output consumer implemented. The final locked baseline
passes 304 tests, and the identical independent binary passes eight tests
and matches stdout. Source/document review accepts; publication and exact
hosted verification remain pending.
