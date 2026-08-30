# Project state

Last updated: **2026-08-31**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Stage 3
has begun with a standalone configuration snapshot core: bounded copied bytes,
validation before mutation, fresh acceptance revisions, and one consume-once
rollback slot. RFF-REQ-006 remains partial until runtime ownership and
application visibility are integrated. No sample mission exists yet.

## Verified baseline

- Starting commit `782149bd97ec0c8f1371d566f3a63eb5e3138b0b` passed the complete
  local formatting, all-target check, warnings-denied Clippy, 58-test,
  warnings-denied rustdoc, and whitespace baseline before this increment.
- The source-quality baseline uses stable rustfmt at 100 columns, Clippy's
  individual `too_many_lines` lint at 60, and forbidden local unsafe code.
  The active versions remain rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and
  Clippy 0.1.98; these are evidence, not an MSRV or toolchain pin.
- Prior bounded runtime, messaging, clock, schedule, and direct failure-event
  evidence is recorded with exact commits in the traceability register.
- Configuration-focused verification passed: 12 public integration tests and
  three private boundary tests. The complete format/check/Clippy/test/rustdoc
  and Git whitespace baseline passes with 73 tests and warnings denied.
- Configuration implementation commit
  `7dd237626f6a1207b6abffe3bee98830e8a9ada7` contains the core, tests, ADR-0017,
  and scope reconciliation. The evidence checkpoint reruns the same baseline.
- All 21 Rust files meet the physical/comment width review limits. All 75
  relative links resolve across 28 Markdown documents; source/requirement/test
  references and generated-HTML structure/content pass review. Browser visual
  QA remains blocked under the documented adaptation below.
- Repository content remains `MIT OR Apache-2.0`, with confirmed notice
  `Copyright 2026 Daniel Smith`; Cargo publication remains disabled.

## Current architecture

One unpublished, dependency-free safe-Rust package provides:

- A bounded logical LC1 registry and finite-capacity `Runtime<A>` owning
  statically composed applications, with synchronous start, caller-selected
  work, stop, and in-place restart. Returned errors are preserved and fail only
  the selected record; arbitrary panics or hangs are not contained.
- Inline-bounded messages and a bounded routing core with FIFO, reject-newest
  saturation, and explicit partial fan-out. `MessagingRuntime` owns one inbox
  per application, derives availability from lifecycle state, clears exactly
  after stop/failure, and reconnects an empty inbox after restart. Its separate
  application callback receives at most one oldest message and a publish-only
  context; accepted peer publications are not transactional.
- A bounded FIFO structured-event queue and an injected manual clock.
  `Runtime::work_with_failure_event` borrows them for one opt-in direct work
  operation, preserving the original error and exact event attempt even when
  storage is full. Other callbacks do not emit failure events.
- A fixed finite one-shot `WorkSchedule`. One caller request consumes at most
  one due/overdue item, with stable equal-time order and exact clock reads.
  Its replay evidence is limited to controlled inputs on bare `Runtime`.
- `ConfigurationTable<E, MAX_BYTES>` with immutable byte snapshots, one
  retained mission validator, table-local revisions, and one rollback slot.
  ADR-0017 implements the standalone core of ADR-0005 without changing its
  eventual runtime-owned contract.

RFF-REQ-002 through RFF-REQ-005 and RFF-REQ-008 are verified only at their
recorded boundaries. Neither runtime permanently owns a clock, event queue,
or configuration table. Runtime configuration access remains pending.

## Source-quality policy

Three existing item-level function-size expectations preserve cohesive
chronological test traces; new configuration code introduces no waiver.
Physical widths, comment prose, source order, module cohesion, names, and
abstraction boundaries remain manual review responsibilities. A strict width
checker, additional lints, complexity metric, dependency-policy tool, CI,
and toolchain pin remain separate measured increments.

## Work in progress

No implementation work remains in progress. The standalone configuration core
is complete; runtime configuration integration is the next likely slice.

## Highest risks and uncertainties

- Configuration retains at most two byte-bounded snapshots; a candidate and
  compiler temporaries can coexist during replacement. Large const bounds can
  exhaust stack resources. Caller copies and validator effects/errors are
  outside the table's retained content bound.
- Retaining a validation function fixes its identity, not purity, termination,
  or external-state stability. Rollback restores previously accepted bytes
  without revalidation. Revisions carry no table origin.
- `ApplicationId` and `FrameworkInstant` likewise carry no runtime/clock origin.
  Positional inbox topology can associate valid settings with the wrong app.
- Scheduling has no recurrence, dynamic reconfiguration, fairness, or deadline
  guarantee and retains consumed items until drop. Messaging-aware scheduling
  must preserve failed-endpoint clearing through `MessagingRuntime`.
- Event saturation can omit a later high-severity record. Direct failure-event
  reporting is opt-in, observes time after `Failed`, and offers no guaranteed
  delivery or automatic retry. Caller-owned event copies are outside the queue.
- A returned callback error can follow partial app mutation or peer
  publication; no cleanup, rollback, panic/hang containment, or general fault
  tolerance is demonstrated.
- No CI currently executes the local baseline.

## Important unresolved decisions

- ADR-0005 selects runtime ownership, but the owner representation, typed
  application access, validation context, and safe-point API remain unselected.
  Restart retention and explicit no-automatic-rollback behavior still need
  integration evidence.
- No RFF-REQ-007 host command/telemetry grammar or input boundary is selected.
- Periodic scheduling, messaging-aware due work, shared application service
  contexts, persistent event/clock owners, application-authored events, event
  filtering, and a host event drain remain unselected.
- Mission payload limits, external topic identities, wire representations,
  and minimum supported Rust version remain open.

## Most likely next tasks

1. Integrate configuration ownership and read-only application visibility at
   caller-driven work safe points, preserving restart and failure semantics.
2. Select the command/telemetry host grammar and validation boundary before
   implementing RFF-REQ-007.
3. Compose a small sample mission without widening existing event, scheduling,
   messaging, or containment claims.

## Latest run

2026-08-31: Implemented the bounded configuration lifecycle core and ADR-0017.
Twelve public tests cover validation, byte bounds, ownership, revision and
history transitions, and concrete errors; three private tests exercise checked
exhaustion, rejection precedence, and validator suppression. RFF-REQ-006 is
explicitly partial. All 73 tests and required Cargo/whitespace checks pass.
Browser visual inspection was blocked by local-file URL policy; no workaround
was attempted. This run uses generated-HTML structure and content inspection as
the documented document-review adaptation, without claiming browser visual QA.
Implementation commit `7dd237626f6a1207b6abffe3bee98830e8a9ada7` and its
evidence checkpoint remain local on `codex/nightly`. No push, dependency,
runtime context, parser, schema, or protocol behavior was added.
