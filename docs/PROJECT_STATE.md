# Project state

Last updated: **2026-10-19**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with recorded evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input 15f5180f959b2be739ae9db6398a3842ad2bab9d matches refreshed
  origin. Fresh exact-input hosted run 39035292887/job 117403763701 passes all
  20 steps, 150 workspace tests, separate 14 adapter/five sample tests, rustdoc,
  unchanged 222-line sample and both whitespace checks.
- Initial/final locked local format/check/Clippy/test/warnings-denied rustdoc and
  both whitespace checks pass 150/151 workspace tests. Focused messaging event
  model and focused Clippy pass. Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable
  and Clippy 0.1.98 remain unchanged.
- Independent source/oracle review accepts test blob
  6d1d8c2dbcd45ca79d3914cdd7cc5ac73c3fa1af. Fixed assertions check 78,432
  traces/379,092 generated operations. Separate terminal probes comprise
  78,432 peer calls, 61,384 message dispatches, 156,864 final dispatch checks
  and 217,988 event dequeues. Clean isolated control passes; six selected
  compiling owner-wiring faults fail behavioral assertions. Root separately
  inspects raw logs/patches/hashes. Whole-tree audit covers 43 Rust files/17,101
  physical lines/1,138 comment-only lines, zero widths and three unchanged
  fulfilled expectations. All 375 relative links/115 exact test references
  resolve; seven run-changed rendered documents pass full content/HTML structure
  inspection. Root and independent Codex records_review accept the complete
  source/document diff and raw local/fault evidence.
- Input hosted runner 2.337.0, windows-2025-vs2026 image 20261007.264.1,
  Rust/Cargo 1.99.1, rustfmt 1.10.0-stable and Clippy 0.1.99 match the recorded
  baseline. Local checks do not claim execution on that hosted toolchain.
- Published checkpoint 27194287b8670b75efac05f3a3f0a7de38a312d5 has
  [successful hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39130580285):
  exact logged checkout, job 117684735602, all 20 steps, 151 workspace tests,
  separate 14 adapter/five sample tests, rustdoc, all 222 unchanged sample report
  lines and both whitespace checks. Root and independent Codex records_review
  inspect raw JSON/logs separately from local evidence. Tools/image match input.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen messaging, configuration, lifecycle/work failures, messaging-owned
scheduling, direct configuration histories, standalone/direct event histories,
and messaging-owned inbox/event history composition.

## Work in progress

The [messaging event history checkpoint](verification/MESSAGING_EVENT_SEQUENCE_REVIEW.md)
joins publication, selected work failure, healthy peer work/dispatch, manual time,
event dequeue and explicit retry through length five. Both fault roles and event
capacities one/two expose cleanup, exact discard counts, saturation and retained
peer FIFO. Local implementation, full baseline and complete source/document/diff
review pass. Publication and exact-checkpoint CI pass. This evidence follow-up
preserves Rust and receives its own complete review and exact-revision CI.
No unfinished implementation remains. Production, APIs, requirements,
dependencies, licence, lint and workflow are unchanged.

## Highest risks and uncertainties

- Finite models cover only frozen domains. The new model has one work fault,
  fixed capacities/topic and initially Running states. Other lifecycle/service
  composition retains separate evidence. Cleanup-before-clock and invisible
  full-queue attempt counts include source review.
- Application/clock origins and positional topology pairing are caller duties.
  Copied identifiers may share referents. Logical bounds do not bound process
  bytes, callback allocation or duration.
- Panics/hangs and allocator exhaustion are uncontained/unverified. Queue
  clearing does not prove application/external cleanup. Immediate fan-out is
  nontransactional; accepted peer deliveries survive later failure.
- Saturated events may omit records; explicit caller retry is optional. One-shot
  errors are consumed without retry/rollback. Host output does not prove physical
  delivery. Stable tools/images float; local reviews/conditional probes are outside CI.

## Important unresolved decisions

Major API freeze, significant unsafe code, runtime/code-generation/dependency/
security decisions, platform commitments, identity/licences, tags/releases/crate
publication and deployment retain separate gates. MSRV, broader service contexts,
external I/O, hardware, RTOS and no_std remain undecided. Routine host acceptance
is complete; autonomous Codex review does not establish human review.

## Most likely next task

Assess command-ingest malformed-byte coverage against existing adapter tests.
Choose exhaustive finite input checks or a bounded fuzzing experiment only if
it closes a demonstrated gap without adding unnecessary machinery.

## Latest run

2026-10-19: the messaging-owned inbox/event/time/retry history model and full
locked local baseline pass 151 workspace tests. Independent oracle/source review
accepts the frozen domain; six selected compiling owner faults fail behavioral
assertions. Whole-tree and rendered-document audits pass. Root and independent
Codex records_review accept the complete diff and exact-checkpoint hosted evidence.
Tools/image match input. This evidence follow-up receives its own review and
exact-revision CI. Stop after this one completed checkpoint.
