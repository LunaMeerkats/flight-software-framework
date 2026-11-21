# Project state

Last updated: **2026-11-22**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-018 revision 1
adds shared inspection of runtime-owned applications through both owners.
The reviewed host v0.1 milestone remains the regression baseline. Package
0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Clean input `304570ec5341dfa65b8ab1da7a3e2b786afadb01` matches refreshed
  origin/main. Exact hosted run 44746983137 passed all 20 steps.
- This run's pre-edit locked format/check/Clippy, 285 workspace tests,
  warnings-denied rustdoc and whitespace all passed.
- Final locked baseline passes 296 tests, including seven new consumer tests
  and four rustdoc examples, with all static/documentation checks passing.
  One initial Windows Application Control block cleared on an unchanged retry.
- Identical independent consumer source passes all-target check/Clippy and
  seven tests; explicit negative borrowing probes and positive controls pass
  their expected outcomes. Hosted implementation evidence is pending.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98. No toolchain or source-policy change is selected.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Hosts select callback and I/O turns. Shared application
lookup exposes retained observations without mutable inner-owner access.
Existing output-slot and periodic retry policies remain unchanged.

## Work in progress

Publish the locally verified and root/independent Codex reviewed
[shared application inspection](APPLICATION_INSPECTION.md) under
[ADR-0043](adr/0043-shared-application-inspection.md). The
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

2026-11-22. Shared inspection passes the full local baseline, identical
independent consumer and borrowing probes. Root/independent source review
accepts the implementation and full source/document diff. Width/link/rendered
document reviews pass. Publication and exact hosted verification remain.
