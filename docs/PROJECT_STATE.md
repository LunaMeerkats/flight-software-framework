# Project state

Last updated: **2026-10-18**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with recorded evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input d1e4989b48f71651f393bdbf2503b265b027bf91 matches refreshed
  origin. Fresh exact-input hosted run 38906264892/job 116990487541 passes all
  20 steps, 149 workspace tests, separate 14 adapter/five sample tests, rustdoc,
  unchanged sample and both whitespace checks.
- Initial/final locked local format/check/Clippy/test/warnings-denied rustdoc and
  both whitespace checks pass 149/150 workspace tests. Focused runtime event model and
  focused Clippy pass. Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and
  Clippy 0.1.98 remain unchanged.
- Independent source/oracle review accepts test blob
  0a175efa31066e00e05eb6ac7a13203292b1258e. Fixed assertions check 39,216
  traces/189,546 generated operations; separate terminal observations comprise
  39,216 healthy peer calls and 116,928 dequeues. Clean isolated control passes;
  six selected compiling faults fail behavioral assertions. Root separately
  inspects raw logs/patches/hashes. Whole-tree audit covers 42 Rust files/16,520
  lines, zero width findings and three unchanged fulfilled expectations. All
  370 relative links/114 exact test references resolve; six changed rendered
  documents pass content/HTML structure inspection, without pixel/human claims.
- Input hosted runner 2.337.0, windows-2025-vs2026 image 20261007.264.1,
  Rust/Cargo 1.99.1, rustfmt 1.10.0-stable and Clippy 0.1.99 match the recorded
  baseline. Local checks do not claim execution on that hosted toolchain.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen messaging, configuration, lifecycle/work failures, messaging-owned
scheduling, direct configuration histories, standalone event histories and direct
runtime event/time/retry histories.

## Work in progress

The [runtime event sequence checkpoint](verification/RUNTIME_EVENT_SEQUENCE_REVIEW.md)
composes two distinct application failures, healthy peer work, manual time,
dequeue and explicit rejected-event retry through length five at event capacities
one and two. Separate symbolic records and failure history predict all effects.
Local source/oracle, fault sensitivity and final baseline acceptance are complete;
final document/diff review precedes publication. Production, APIs,
requirements, dependencies, licence, lint and workflow are unchanged.

## Highest risks and uncertainties

- Finite models cover only frozen domains. The direct event model excludes
  messaging ownership/cleanup, configuration, scheduling and arbitrary callbacks.
  Internal commit-before-clock/attempt-count evidence includes source review.
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

Assess whether messaging-owned event histories add meaningful integration
evidence beyond the direct model and existing inbox-cleanup/event scenarios.
Select that model only if its added evidence justifies the oracle machinery.

## Latest run

2026-10-18: the direct-runtime event/time/retry history model and full locked
local baseline pass 150 workspace tests. Independent oracle/source review accepts
the frozen domain; selected compiling fault trials pass their defined expectations.
Whole-tree and rendered-document review accept the local evidence. Final complete
diff review precedes publication and exact-revision hosted CI inspection. Stop
after this one checkpoint.
