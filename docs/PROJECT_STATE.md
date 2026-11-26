# Project state

Last updated: **2026-11-26**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-021 revision 1
adds opt-in returned-stop failure diagnostics through both runtime owners.
Reviewed host v0.1 remains the regression baseline. Package 0.0.0 stays
unpublished with provisional APIs.

## Verified baseline

- Clean input `8fca981837327828b3caf6245984f5fa639e4fd6` matches refreshed
  origin/main. Exact hosted run 45556097802 passes all 20 steps at that SHA.
- Pre-edit locked format/check/Clippy, 316 workspace tests, warnings-denied
  rustdoc and whitespace pass. Local tools remain Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98.
- Input hosted tools remain Rust/Cargo 1.100.0, rustfmt 1.11.0-stable,
  Clippy 0.1.100, runner 2.338.2 and image `20261116.315.1`.
- Final locked baseline passes 324 workspace tests and all static/documentation
  checks. All eight focused tests also pass as identical source in an independent
  Cargo package; bytes/SHA256 and workspace/dependency metadata match.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, events and optional validated configuration.
Hosts select callback and I/O turns. The opt-in stop path delegates once
through its owner before event construction. Direct success retains its
ApplicationState; messaging success retains MessagingStopOutcome. Failure
keeps the original stop error, clear count and exact event attempt.

## Work in progress

No implementation remains. Root and independent Codex accept the complete
source/document/diff review, source-form/link audit and all 12 rendered-document
reviews. Publish the verified increment and inspect exact-head hosted CI. The
[focused review](verification/STOP_FAILURE_EVENTS_REVIEW.md) records evidence.

## Highest risks and uncertainties

- Additional reporting roles need explicit host retention bounds; no implicit
  retry, recovery, priority or guaranteed diagnostic delivery is provided.
- Callback/clock/writer turns must return. Logical bounds do not bound arbitrary
  allocation or execution time; acceptance does not prove delivery.
- Application IDs remain runtime-local, with equal-slot cross-owner aliasing.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess a concrete consumer need for start/restart diagnostics or another
integration gap before adding further lifecycle wrappers or retry machinery.

## Latest run

2026-11-26. Stop diagnostics pass local/independent verification and complete
Codex review. Publication and exact-head hosted evidence are next. Stop at
this one bounded increment.
