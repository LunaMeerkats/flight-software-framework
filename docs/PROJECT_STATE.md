# Project state

Last updated: **2026-10-11**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for the experimental
serial host v0.1 target. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with exact evidence and
limitations. Package 0.0.0 remains unpublished; APIs and requirements are
provisional. Current work hardens existing behavior within those boundaries.

## Verified baseline

- Input `181cfa913c17177e2696d35fadb790883e4d0ed7` starts clean and equal to
  refreshed origin. Exact input hosted run 37935576798 succeeds; it predates
  this new lifecycle test.
- Initial/final locked local baselines pass 142/143 tests, formatting, all-target
  check, warnings-denied Clippy/rustdoc and whitespace. The focused target passes
  one test: 19,531 traces and 112,305 generated operations plus final observers.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Hosted tools remain separate evidence; no local 1.99.0 execution is claimed.
- Final source audit covers 35 Rust files/13,353 lines, zero width debt and three
  unchanged expectations. All 331 relative links and 107 traceability functions
  resolve. Production source, APIs, requirements, dependencies and policy are unchanged.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate standalone
and runtime-owned finite models strengthen existing messaging behavior.

## Work in progress

The [reduced lifecycle model](verification/RUNTIME_MESSAGE_SEQUENCE_REVIEW.md)
checks two initially Running successful passive applications, capacities [1,2],
one shared topic and publication/stop/restart sequences of lengths zero through
six. Independent states, accepted histories and discard watermarks predict
reports, exact clearing, peer retention and callback suppression. Final passive
dispatch observes retained FIFO at every independently enumerated prefix.
Independent oracle/source/complete-diff reviews, six isolated fault detections
and final locked baseline pass. Six changed documents pass rendered HTML
structure/content inspection; no pixel-level acceptance is claimed. Publication
and exact-head hosted verification remain pending.

## Highest risks and uncertainties

- Finite models cover only their frozen domains. The new lifecycle model excludes
  Registered/Failed, callback errors, generated dispatch/work and self-publication.
- Application/clock origin pairing is caller discipline; positional topology
  cannot detect swapped valid mission entries. Copied identifiers may share
  referents. Logical bounds do not bound process bytes, callbacks or duration.
- Panics/hangs and allocator exhaustion are uncontained/unverified; Failed is
  terminal. Stop clearing does not prove application/external cleanup.
- Immediate fan-out is nontransactional; accepted peer deliveries survive later
  failure. Saturated events may omit a record. One-shot errors are consumed
  without automatic retry or rollback. Host output does not prove physical delivery.
- Stable tools/images float. Local source/document and conditional probe reviews
  remain outside CI. Codex review does not establish independent human review.

## Important unresolved decisions

Major API freeze, significant unsafe code, runtime/code-generation/dependency/
security decisions, platform commitments, identity/licences, tags/releases/crate
publication and deployment retain separate gates. MSRV, broader contexts/events,
external I/O, hardware, RTOS and no_std remain undecided. Routine host acceptance
is complete under ADR-0024.

## Most likely next task

Consider a separate reduced model of returned ordinary-work failure, terminal
unavailability and peer retention. Freeze its fault/lifecycle alphabet first;
reassess against fresh evidence and preserve this increment's finite boundary.

## Latest run

2026-10-11: runtime lifecycle model, independent oracle review, six selected
fault detections and full locked baseline pass with 143 tests. Source/document/
complete-diff reviews pass; publication and exact-head hosted CI remain pending.
