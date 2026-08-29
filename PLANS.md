# Plan: report returned work errors as bounded events

Status: **Complete**
Date: **2026-08-30**

## Objective

Integrate one runtime-owned, clock-captured structured-event attempt for a
cooperative error returned by `Application::work`, while preserving the
complete original work error, the existing event-queue saturation policy, and
successful subsequent work by a healthy peer.

This is the highest-value next step because the standalone event queue, manual
clock, and returned-error lifecycle behavior are already verified, but
RFF-REQ-005 and RFF-REQ-008 still lack one composed runtime path that proves the
failed state, structured event, and peer progress together.

## Context and decision boundary

`Runtime::work` already validates `Running`, invokes exactly one synchronous
callback, preserves a concrete returned error, and commits only that record to
terminal `Failed`. ADR-0013 supplies a positive-capacity FIFO event queue with
caller-visible reject-newest saturation. ADR-0014 supplies injected elapsed
time and clock-captured event timestamps.

Add an opt-in `Runtime::work_with_failure_event` operation. It will delegate to
the existing work boundary. Success and lifecycle rejection will not read the
clock or attempt an event. A returned application work error will leave the
existing failed state in place, capture the injected clock once, construct one
application-sourced error event with a caller-supplied copied identifier, and
attempt to append it once to the supplied bounded event queue.

The returned error will retain the complete `RuntimeWorkError` plus the exact
event and `EventEmitOutcome`. This makes a rejected event available for
explicit caller handling or retry without replacing or hiding the application
error. The operation will not attach event storage or a clock permanently to
the runtime.

## Acceptance criteria

- Successful work retains `Running`, reads no clock, and emits no event.
- Unknown or non-running work is rejected through the existing lifecycle error,
  reads no clock, and emits no event.
- A cooperative returned work error commits only the selected application to
  `Failed` before one event timestamp is captured.
- The runtime-generated event contains `EventSource::Application` with the
  selected runtime-local identity, `EventSeverity::Error`, the caller-supplied
  copied identifier, and exactly one injected clock reading.
- The returned integration error retains the complete original
  `RuntimeWorkError`, event record, and event emission outcome.
- A queue with room records the event. A full queue retains its older event,
  reports `QueueFull`, and returns the rejected event for explicit retry.
- Event recording or saturation does not prevent a healthy running peer from
  completing later work.
- RFF-REQ-005 and RFF-REQ-008 become verified only at this cooperative returned-
  work-error boundary. No claim is made for application-authored events,
  start/stop/restart/message callback event emission, panic or hang containment,
  guaranteed diagnostic delivery, or fault tolerance.
- No event filter, fan-out, host drain adapter, shared service context, runtime
  clock owner, messaging integration, scheduled-event integration, thread,
  executor, dependency, protocol, or automatic dispatch is added.

## Files and components

- `src/runtime_events.rs`: failure-event attempt vocabulary and the integrated
  runtime work operation.
- `src/lib.rs`: narrow re-exports.
- `tests/runtime_events.rs`: recorded, saturated, retry, no-event, and peer-
  progress evidence.
- `docs/adr/0016-returned-work-failure-events.md`: operation ordering, ownership,
  saturation, and scope decision.
- `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, `docs/REQUIREMENTS.md`,
  `docs/ROADMAP.md`, `docs/PROJECT_STATE.md`, and
  `docs/verification/TRACEABILITY.md`: current behavior, evidence, limits, and
  next milestone.
- `PLANS.md`: this bounded plan and final result.

No external research is required. This slice composes only previously recorded
local runtime, event-queue, and injected-clock decisions.

## Verification approach

- Run the focused runtime-event integration tests while implementing.
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
  source identifiers, exact traceability test names, and changed-document
  structure.
- Review the complete diff for error replacement, duplicate clock reads or
  event attempts, events on lifecycle misuse, hidden retry, queue mutation on
  saturation, peer-state mutation, accidental API breadth, and scope drift.

## Risks and safe stopping point

Reject-newest saturation can omit the failure event from bounded storage. The
caller-visible attempt retains the rejected event, but this does not guarantee
delivery or require retry. The returned error also carries a caller-owned event
copy outside the queue bound, just as other caller-owned values are outside the
queue's retained-record limit.

`ApplicationId` and `FrameworkInstant` still carry no runtime or clock origin.
This operation constructs the event source from the selected record and reads
the supplied clock, but the caller remains responsible for supplying the
intended queue, identifier, and clock.

The application may mutate its own state before returning an error. This slice
adds no rollback, cleanup, panic containment, hang containment, or recovery
policy. It reports only direct `Runtime::work` errors; scheduled work and
`MessagingRuntime` require separate integration that preserves their existing
consumption and inbox-clearing semantics.

Stop after the direct work-error event path, focused fault-injection and
saturation evidence, ADR, durable state, and complete verification are
coherent. Do not add other callback events, application event APIs, host
adapters, configuration, or command/telemetry behavior in this run.

## Result

Implemented the direct returned-work failure-event boundary in commit
`a04c5bd3f929934b7578b14f181138ae0be56e9b`.
`Runtime::work_with_failure_event` delegates to the established work operation.
Success and lifecycle rejection read no clock and attempt no event. After a
cooperative application error commits only the selected record to `Failed`, the
integration captures one injected reading, constructs one application-sourced
error event, and calls the bounded queue exactly once.

`RuntimeWorkEventError` retains the complete original `RuntimeWorkError` and an
optional `FailureEventAttempt` containing the exact event and `Recorded` or
`QueueFull` outcome. A full queue preserves its older record and returns the
rejected event with its original timestamp for explicit retry. Nothing retries
implicitly, and event reporting leaves a healthy peer operable for later work
once the operation returns.

Four public integration tests prove exact fields and source chaining, one clock
read and queue insertion, no event on successful or lifecycle-rejected work,
unknown-identity and post-`Failed` suppression, exact saturation retention and
explicit retry, failed state, and later peer progress. The combined runtime and
existing bounded-queue evidence verifies RFF-REQ-005 and RFF-REQ-008 only at the
direct cooperative returned-work boundary. Stage 2 is recorded complete.

The required formatting, all-target checking, warnings-denied Clippy, 58-test,
warnings-denied rustdoc, and Git whitespace checks passed. The source-form audit
found 19 handwritten Rust files and 6,803 lines, no physical line over 100
columns, no comment-only line over 80, three existing narrow reasoned
expectations, no allow attributes, and no unsafe code use. The document audit
found 27 Markdown files, 71 resolving relative links, eight matched requirement
and traceability rows, 16 matching ADR identifiers, 21 source definitions with
no undefined reference, 39 exact traceability test names resolving to source,
consistent table shapes, and one top-level heading per document. All documents
rendered structurally.

No dependency, permanent runtime clock/event owner, other callback event path,
application-authored event API, host drain, automatic retry, thread, executor,
protocol, compatibility claim, release, or push was added.
