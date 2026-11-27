# Project state

Last updated: **2026-11-27**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-022 revision 1
adds a host elapsed-clock adapter and finite public-only polling consumer.
Reviewed host v0.1 remains the regression baseline. Package 0.0.0 stays
unpublished with provisional APIs.

## Verified baseline

- Clean input `158ce34cd10517e52e0aa3548629e5e1ee43853a` matches refreshed
  origin/main. Exact hosted run 45783611865 passes all 20 steps at that SHA.
- Locked input format/check/Clippy, 324 workspace tests, warnings-denied
  rustdoc and whitespace pass. Local tools remain Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98.
- Input hosted tools are Rust/Cargo 1.100.0, rustfmt 1.11.0-stable,
  Clippy 0.1.100, runner 2.338.2 and image `20261116.315.1`.
- Final locked baseline passes 332 workspace tests and every static/document
  check. Five focused clock tests and three example tests pass. Identical source
  passes check/Clippy/three tests/run independently; source bytes/hash and
  workspace/dependency metadata match. Both live runs stop after five turns.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, events and optional validated configuration.
HostClock adds a private native origin behind the existing Clock boundary.
Hosts still select callbacks, waiting and I/O. ManualClock controls test time;
HostClock observes elapsed host time without owning an execution loop.

## Work in progress

No implementation remains. Root and independent Codex accept the complete
source/document/diff and 12 rendered-document reviews without findings.
Whole-tree source-form/link/test-reference checks pass. Source publication
and exact hosted inspection remain. The
[focused review](verification/HOST_CLOCK_REVIEW.md) records actual results.

## Highest risks and uncertainties

- Host clock readings inherit platform anomalies and timing limitations;
  saturation does not supply recovery or same-origin validation.
- Callback/clock/writer/wait turns must return. Finite turns do not bound
  wall-clock execution, and queue acceptance does not prove delivery.
- Reporting roles need explicit retention bounds. Application IDs remain
  runtime-local, with equal-slot cross-owner aliasing.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess a concrete multi-application startup/cleanup consumer need before
adding further lifecycle wrappers or host orchestration APIs.

## Latest run

2026-11-27. Host elapsed-clock integration passes local and independent
verification and complete review. Publication remains. Stop at this bounded
increment.
