# Project state

Last updated: **2026-10-28**

## Current milestone

Reviewed experimental serial host v0.1 remains the regression baseline. Stage 5
advances practical library use under ADR-0025. RFF-LIB-REQ-007 composes existing
event APIs into a public-only fallible host-output example. Package 0.0.0 stays
unpublished; APIs remain provisional.

## Verified baseline

- Clean input 6ab84023892b19642b76705bd184a95bb43754df matches refreshed origin.
  Live exact run 40329895459/job 121596214543 passes all 20 steps, 193 workspace
  tests, separate 14 adapter/five sample tests and the unchanged 222-line sample.
  Root raw logs and connector step inspection agree.
- Initial/final full locked baselines pass 193/201 tests, eight focused tests
  and the exact runnable output. The final identical independent Cargo consumer
  passes check/Clippy/eight tests/run and normalized metadata/byte/hash inspection.
  Source-form review corrects three width findings without a waiver.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools are 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99, runner
  2.338.0/windows-2025-vs2026 image 20261021.278.1. Local checks do not claim
  execution on those hosted tools. Actual evidence and limits are in
  [the focused record](verification/EVENT_OUTPUT_REVIEW.md).

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time, finite one-shot scheduling, bounded events and
optional configuration with immutable work visibility/one-use rollback. All work
shares selected identity/context construction. PeriodicWork supplies one
observed-time release for host delegation. EventFilter applies explicit optional
severity admission; runtime failure reports remain unfiltered. The new host
presenter owns one writer, one pending event and fixed bytes/cursor outside the
queue, with one write attempt per turn. No library API or runtime changes.

## Work in progress

RFF-LIB-REQ-007/ADR-0032 precede source changes. Implementation, final full
baseline and independent consumer verification pass. Source-form corrections and precise
sample error-exit guidance are complete. Root and independent output_review
accept the complete 14-file source/document diff and actual final evidence,
with no remaining findings. Source publication and exact hosted inspection
remain to complete.

## Highest risks and uncertainties

- Controlled consumers do not establish adoption, broad usability or stability.
  Writer acceptance is not flush or delivery. The runnable sample drops volatile
  pending data on errors; retained-turn retries require the same stream.
- Application/clock origin and inbox pairing remain caller duties. Logical bounds
  do not bound process memory, allocation, blocking or callback time.
- Periodic release is not execution; host polling/error/timer/output policies
  remain explicit. Filtering reserves no capacity for errors. Panics/hangs,
  allocator exhaustion and process failure are uncontained.
- Finite models prove frozen domains. Hosted tools/images float; source/doc
  reviews and conditional probes remain outside the CI job.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV, external I/O protocols, hardware, RTOS
and no_std remain undecided. Codex review does not establish human acceptance or
operational suitability.

## Most likely next task

Reassess concrete host coordination friction across work, bounded output and
message dispatch. Compare mission composition before adding a framework runner,
service context or new queue policy; do not add more model machinery by default.

## Latest run

2026-10-28: selected bounded event presentation after source-based consumer
assessment and official Rust writer research. Implemented public-only composition
with retained partial-write progress and explicit flush/error boundaries. Finish
actual verification, autonomous review and exact source-publication evidence,
then stop after this checkpoint.
