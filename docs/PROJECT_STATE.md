# Project state

Last updated: **2026-10-09**

## Current milestone

Stages 1 through 4 and the source-quality checkpoint are complete for the
experimental serial host v0.1 target. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts current scope entry points and architecture under ADR-0024 with exact
revision, reviewer identities, evidence, findings, and limitation dispositions.
The package remains unpublished at 0.0.0; APIs and requirements are provisional.

## Verified baseline

- Reviewed input `ba42f800d0b74637b28c833a55192fc541912fe5` started clean and
  equal to refreshed origin. Exact input hosted run 37675969512 is confirmed
  successful, including logged checkout, all 20 steps, 141 workspace tests,
  focused host targets, rustdoc, sample execution, and whitespace.
- Fresh locked local baseline passes 141 workspace tests, formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace. Focused
  adapters pass 14 and sample tests five; the executable reproduces the full
  documented report. Focused reruns do not add unique workspace tests.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, Clippy 0.1.98; hosted 1.99.0,
  1.10.0-stable, 0.1.99. Runner/image match the recorded source-policy re-audit.
- Whole-tree review covers 33 handwritten Rust files and 12,666 physical lines,
  no width findings, and the same three fulfilled function-size expectations.
  Fresh graph queries find one package, no external Cargo dependencies/features,
  one library/example and 16 integration targets. Approved licences are unchanged.
- All 320 relative links across 62 Markdown files and 105 exact traceability
  function references resolve. Eleven changed documents pass rendered HTML
  structure/content inspection; no pixel-level acceptance is claimed.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
The review accepts this coherent host boundary without freezing APIs or
introducing runtime, source, dependency, lint, or workflow changes.

## Work in progress

The actual review, documentation reconciliation, final local baseline, and
complete-diff acceptance are complete. Authorized publication and exact-head
hosted evidence are pending for this checkpoint. One nonblocking
source-register finding corrects the workflow push filter to codex/nightly.
No unfinished implementation remains.

## Highest risks and uncertainties

- Application identity and clock-origin pairing remain caller discipline;
  positional inbox configurations cannot detect swapped valid mission entries.
- Copied generic topic/event identifiers can retain arbitrary shared referents.
- Logical capacities do not bound whole-process bytes, inline stack use,
  callback allocations, caller archives/copies, or execution duration. Actual
  allocator exhaustion and allocation/deallocation counts remain unverified.
- Callback/clock panics and hangs remain outside containment. Stop clearing
  does not prove application-internal/external cleanup; Failed is terminal.
- Callback publications are immediate and non-transactional; accepted peer
  deliveries survive later failure. Event saturation may omit a later event.
- Scheduled errors are consumed without automatic retry, events, or rollback.
  Host output saturation is terminal; returned arrays are not physical delivery.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI. Autonomous review is not independent human review.

## Important unresolved decisions

These disclosed limits are accepted for the controlled host milestone, not for
an API freeze or operational use. Major API freeze, significant unsafe code,
major runtime/code-generation/dependency/security decisions, consequential
platform commitments, licences/identity, tags/releases/crate publication, and
deployment retain separate authorization gates. MSRV, context consolidation,
broader events, external I/O, hardware, RTOS and no_std remain undecided.

## Most likely next task

Freeze and implement one dependency-free finite reference-model test of
bounded message publication/dequeue sequences. Compare publisher reports and
independent FIFO contents over the declared operation alphabet and bounds;
report finite coverage precisely. Reassess if new evidence changes priority.

## Latest run

2026-10-09: actual delegated host v0.1 review accepts RFF-REQ-001 and the routine
entry-point/architecture gates; Stage 4 is complete for the host target.
Fresh local checks/sample and exact input hosted evidence pass. Known limits
receive explicit dispositions; the source-register branch wording is corrected.
This is one review checkpoint, with no implementation or scope expansion.
