# Project state

Last updated: **2026-09-11**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Stage 3
has verified configuration and host adapters. ADR-0020 added messaging-owned
failure events; ADR-0021 now adds finite scheduled ordinary work through that
owner. The combined sample, CI, and v0.1 architecture review remain outstanding.

## Verified baseline

- Started clean on `codex/nightly` at
  `ca40d7beba8b17d5f722e633bbb45db06e6f62bc`; all six initial checks passed
  with 102 tests. Toolchain unchanged: rustc/cargo 1.98.0, rustfmt
  1.9.0-stable, Clippy 0.1.98. These are evidence, not an MSRV or pin.
- `cargo test --test messaging_scheduled_work` passes seven new tests for
  timing/consumption, exact errors and cleanup, lifecycle/unknown suppression,
  peer FIFO and later work, configuration history, and manual-reading replay.
- Final formatting, all-target/all-feature check, warnings-denied Clippy,
  109 tests, warnings-denied rustdoc, and Git whitespace checks pass.
- Independent complete-diff review found no actionable code defect. All 29 Rust
  files meet physical/comment widths after one 81-column comment was reflowed.
  Three reasoned expectations remain fulfilled; no dependency, unsafe code,
  public error type, mutable owner access, or lint waiver was added.
- Document audits pass: 34 Markdown files, 117 links, eight requirement rows,
  26 sources, 73 exact test references, and 12 changed content comparisons.
  Browser DOM layout review passes at 1,280 pixels with no overflow, heading
  gaps, or console warnings. Screenshot capture timed out; the plan records
  the narrow review adaptation, and screenshot-based visual QA is not claimed.
- Host adapters and ADR-0018/0019 experiments are unchanged and not separately
  rerun. The full suite still includes all 13 host-adapter tests.

## Current architecture

One unpublished, dependency-free safe-Rust library provides a finite LC1
runtime with synchronous lifecycle/work callbacks, bounded lifecycle-owned
inbox dispatch, injected manual time and finite one-shot scheduling, bounded
events, and constructor-owned optional configuration with immutable
ordinary-work visibility and consume-once rollback. The private `host-echo`
example provides the validated caller-framed command/telemetry adapter pair.

Both scheduled-work owners share one private timing/consumption decision.
The messaging operation delegates through its existing ordinary-work path,
retaining exact selected-inbox cleanup and configuration behavior. The nested
existing errors retain the consumed item, observed instant, work error, and
discard count. No mutable owner access is exposed. Failure-event reporting
remains a separate opt-in ordinary-work operation.

## Work in progress

No unfinished implementation remains. The messaging-owned scheduling increment
has reached its tested, documented, reviewed stopping point.

## Highest risks and uncertainties

- The full sample needs explicit ownership and driver order. Scheduled work
  does not emit failure events; the sample can use the existing separate
  ordinary-work event operation for its defined fault scenario.
- Callbacks and clocks may panic or hang. No rollback, recovery, real-time
  guarantee, implicit retry, recurrence, or event-delivery guarantee is added.
- Host output saturation terminally fails telemetry while retaining old output;
  drain cannot recover it. Physical delivery and stream framing remain absent.
- IDs, revisions, and instants do not encode owner origin; callbacks/validators
  need not terminate. Large inline bounds can exhaust stack resources.
- No CI currently executes the local baseline.

## Important unresolved decisions

The combined sample's explicit service composition and driver order remain to
be selected. Message/lifecycle configuration access, application-authored
events, scheduled event reporting, host event drain, physical I/O, MSRV,
hardware, RTOS, and no_std remain open. Local grammar and pre-v0.1 APIs remain
unfrozen.

## Most likely next tasks

1. Compose the documented combined sample with explicit ownership/driver order,
   including scheduled work and a separate cooperative work-failure event.
2. Establish CI for the required local baseline.
3. Record the v0.1 architecture review before broadening scope.

## Latest run

2026-09-11: Added messaging-owned one-shot scheduling and seven public
integration tests. Initial and final Cargo baselines and focused tests pass.
Source-form, document-content/browser-layout, and independent diff reviews
are complete, with the screenshot limitation recorded above. The scheduling
ownership prerequisite is resolved; the combined sample remains separate.
No push is authorized or performed.
