# Plan: add caller-driven scheduled work

Status: **In progress**
Date: **2026-08-29**

## Objective

Record and implement the smallest scheduled-work slice for Stage 2: a finite
ordered agenda of explicit application work instants and one caller-driven
runtime operation that attempts at most the next due item under an injected
clock.

This is the highest-value next step because the manual clock is verified but
RFF-REQ-004 still has no scheduled application work, equal-time ordering, or
replayed scheduled trace. A finite one-shot agenda can establish those
behaviors without prematurely choosing recurrence, missed-period recovery,
automatic dispatch, or concurrency.

## Context and decision boundary

ADR-0001 requires serial caller control and stable observable ordering.
ADR-0009 provides running-only `Runtime::work` with exact returned-error
behavior. ADR-0014 provides an injected read-only clock whose manual
implementation advances only on explicit caller requests.

The schedule will contain copied `(ApplicationId, FrameworkInstant)` work
items in nondecreasing scheduled-time order. Equal-time items retain caller
configuration order. One runtime call observes the clock once and either
reports completion, waits without mutation, or consumes and attempts exactly
one due or overdue item through the existing work boundary.

A due item is consumed before its callback result is returned. That applies to
successful work, a lifecycle rejection, and a returned application error, so a
stopped or failed application cannot indefinitely block later equal-time work.
The result retains the scheduled instant, observed instant, identity, and exact
runtime work error where applicable.

## Acceptance criteria

- `ScheduledWork` identifies one runtime-local application and one absolute
  elapsed `FrameworkInstant`; it is a one-shot release, not a wall-clock or
  completion-deadline guarantee.
- `WorkSchedule` owns a fixed finite agenda copied at construction. It accepts
  an empty agenda, rejects descending scheduled instants before construction,
  reserves storage for the full configured item count, and has no mutation API
  that can grow the agenda.
- Equal-time work retains configuration order. An overdue item remains due, and
  every call attempts at most one item.
- Before the next scheduled instant, the runtime reports the item and observed
  instant without consuming it, invoking application code, or advancing time.
- A due item delegates to `Runtime::work`. Success, lifecycle rejection, and a
  returned application error preserve the existing lifecycle behavior and all
  consume that one item before returning exact schedule metadata.
- A consumed rejected or failed item does not block a later due peer.
- Two fresh runtimes and manual clocks given identical configuration, explicit
  advances, and ordered calls produce identical work results, application work
  order, lifecycle-state outcomes, and clock-captured structured-event
  timestamp traces without sleeps.
- RFF-REQ-004 becomes verified by the combined clock and scheduled replay
  evidence. RFF-REQ-005 and RFF-REQ-008 remain partial because no runtime-owned
  event or returned-error event path is added.
- No periodic policy, dynamic scheduling, automatic draining, priority,
  fairness promise, execution-time bound, wall-clock adapter, application
  context, message dispatch, event integration, thread, executor, dependency,
  protocol, or real-time claim is added.
- The complete documented baseline and repository document/source-form audits
  pass, and the complete diff contains no unrelated implementation change.

## Files and components

- `src/scheduling.rs`: finite agenda, creation errors, scheduled outcomes and
  errors, and the caller-driven `Runtime` operation.
- `src/lib.rs`: narrow scheduling re-exports.
- `tests/scheduled_work.rs`: public API, lifecycle/error, equal-time, overdue,
  and replay evidence.
- `docs/adr/0015-caller-driven-scheduled-work.md`: due-work, ordering,
  consumption, clock-domain, and deferred recurrence decisions.
- `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, `docs/REQUIREMENTS.md`,
  `docs/ROADMAP.md`, `docs/PROJECT_STATE.md`, and
  `docs/verification/TRACEABILITY.md`: truthful current behavior, limits,
  evidence, and next step.
- `PLANS.md`: this bounded plan and final result.

No external research is required. The slice composes already recorded local
runtime and clock decisions using stable standard-library storage.

## Verification approach

- Run the focused scheduled-work integration tests while implementing.
- Run `cargo fmt --all -- --check`.
- Run `cargo check --workspace --all-targets --all-features`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo doc --workspace --all-features --no-deps` with
  `RUSTDOCFLAGS=-D warnings`.
- Run `git diff --check`.
- Re-audit handwritten Rust physical and comment-only widths and every reasoned
  Clippy expectation.
- Verify relative Markdown links, headings, tables, requirement rows, ADR and
  source identifiers, and changed-document structure.
- Review the complete diff for hidden time movement, multiple-work dispatch,
  unstable equal-time order, schedule head-of-line blocking, accidental
  recurrence or real-time claims, public API commitments, and scope drift.

## Risks and safe stopping point

`FrameworkInstant` and `ApplicationId` carry no owner identity. The schedule
cannot detect a work item or instant from another same-shaped runtime or clock;
their existing caller-scoped contracts continue to apply.

A finite one-shot agenda is intentionally not a periodic scheduler. It retains
all configured items until the agenda is dropped, including already consumed
items, so its allocation stays fixed and bounded by construction. A later
periodic design must separately decide phase, drift, missed-release catch-up or
coalescing, arithmetic overflow, and reconfiguration.

Consuming a due item before returning an error prevents head-of-line blocking
but means retry is never implicit. Applications may still mutate internally
before returning an error, and panics or non-returning callbacks remain outside
the cooperative boundary.

Stop after the finite agenda, one-item runtime operation, focused replay and
error evidence, decision record, and durable state are coherent. Do not add
periodic generation, automatic looping, runtime-owned events, failure events,
message dispatch, a shared service context, or concurrency in this run.
