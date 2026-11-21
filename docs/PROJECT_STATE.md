# Project state

Last updated: **2026-11-22**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-018 revision 1
adds shared inspection of runtime-owned applications through both owners.
The reviewed host v0.1 milestone remains the regression baseline. Package
0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Implementation `8ff46483d7cd63f173f993b0fc7882b68efd6d0a` reached
  origin/main by fast-forward. Exact hosted run 44959470170 passed all 20
  steps, 296 workspace tests, separate 14 adapter and five sample tests,
  rustdoc and whitespace. The 222-line sample matches the input run exactly.
- This run's pre-edit locked format/check/Clippy, 285 workspace tests,
  warnings-denied rustdoc and whitespace all passed.
- Final locked baseline passes 296 tests, including seven new consumer tests
  and four rustdoc examples, with all static/documentation checks passing.
  One initial Windows Application Control block cleared on an unchanged retry.
- Identical independent consumer source passes all-target check/Clippy and
  seven tests; explicit negative borrowing probes and positive controls pass
  their expected outcomes.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98. No toolchain or source-policy change is selected.
- Hosted Rust/Cargo 1.100.0, rustfmt 1.11.0-stable, Clippy 0.1.100,
  runner 2.338.2 and Windows image `20261116.315.1` match input.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Hosts select callback and I/O turns. Shared application
lookup exposes retained observations without mutable inner-owner access.
Existing output-slot and periodic retry policies remain unchanged.

## Work in progress

No implementation remains for this increment. Codex accepts
[shared application inspection](APPLICATION_INSPECTION.md) after recorded
local/independent consumer, compiler, source/document and exact hosted review.
This record-only completion receives its own review and hosted check. The
[focused review](verification/APPLICATION_INSPECTION_REVIEW.md) records actual
consumer/probe/independent package evidence and the transient environment block.

## Highest risks and uncertainties

- Shared application methods may have interior mutation and other effects.
- IDs remain runtime-local, with equal-slot cross-owner aliasing possible.
- Callbacks and writes must return. Logical bounds do not bound application
  allocation or execution time; admission does not prove external delivery.
- Shared inspection neither drains owned output nor establishes recovery.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Reassess downstream use after this inspection checkpoint, including whether
host-owned output recipes have a concrete ownership-transfer gap. Avoid
adding mutable access or further retry machinery without consumer evidence.

## Latest run

2026-11-22. Shared inspection is implemented, reviewed, verified and published.
The exact implementation CI passes. The initial local OS4551 block cleared
on one unchanged retry. Stop at this bounded inspection checkpoint.
