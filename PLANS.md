# Plan: add a bounded structured-event queue

Status: **In progress**
Date: **2026-08-27**

## Objective

Record and implement the smallest standalone structured-event path needed for
Stage 2: fixed structured metadata, explicit framework timestamps, a positive
pre-reserved queue bound, FIFO consumption, and caller-visible reject-newest
saturation.

This is the highest-value next step because the roadmap and project state both
select finite events before injected time and scheduling. A standalone queue
defines the timestamp consumer and overflow contract without prematurely
changing application callbacks or coupling diagnostics to the lifecycle-gated
message bus.

## Context

RFF-REQ-005 requires machine-inspectable source, severity, identifier, and
framework timestamp fields plus a finite storage or delivery policy. No event
representation, event bound, or overflow behavior currently exists. RFF-REQ-008
also remains partial until a returned application error produces a structured
event, but that integration requires a framework-owned timestamp source that is
outside this increment.

The existing message bus is not an event transport. It routes by application
subscriptions and lifecycle availability, while diagnostics must remain
observable without recursively publishing onto a saturated or unavailable
application path.

## Acceptance criteria

- `Event<Identifier>` stores an `EventSource`, severity, mission-defined copied
  identifier, and explicit `EventTimestamp` without requiring free-form log
  parsing.
- `EventTimestamp` represents elapsed duration from a framework clock origin.
  This slice accepts it explicitly and adds no clock, wall-clock access,
  scheduling behavior, or monotonicity claim.
- `EventQueue` requires a positive event-record capacity, reserves that storage
  during construction, and never treats allocator capacity as its logical
  bound.
- Emission accepts events through the exact configured capacity in FIFO order.
  When full, it retains every older event, rejects the newest event, and returns
  a direct typed outcome containing the configured capacity.
- Saturation emits no recursive diagnostic, retries nothing, spills nowhere,
  and creates no hidden work.
- Dequeue frees one logical slot so a later event can be accepted.
- Public tests assert every structured field, zero-capacity rejection,
  exact-capacity acceptance, one-over-capacity rejection, retained FIFO order,
  and acceptance after dequeue.
- No dependency, thread, executor, runtime integration, application service
  context, event text payload, filter, fan-out, clock, scheduler, protocol, or
  compatibility claim is added.
- RFF-REQ-005 is recorded as partially implemented and partially verified,
  never fully verified. RFF-REQ-008 remains partial until a later runtime
  failure emits an event with framework-supplied time.
- The complete documented baseline and repository document/source-form audits
  pass, and the complete diff contains no unrelated implementation change.

## Files and components

- `src/events.rs`: structured event vocabulary and bounded FIFO queue.
- `src/lib.rs`: narrow public re-exports.
- `tests/event_queue.rs`: public-API field, boundary, overflow, and FIFO
  evidence.
- `docs/adr/0013-bounded-structured-event-queue.md`: representation, timestamp
  boundary, reject-newest policy, alternatives, risks, and revisit conditions.
- `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, `docs/ROADMAP.md`,
  `docs/PROJECT_STATE.md`, and `docs/verification/TRACEABILITY.md`: truthful
  current behavior, limits, evidence, and next step.
- `PLANS.md`: this bounded plan and final result.

No new external source is required. The slice uses the event-service
responsibility already recorded under `SRC-NASA-CFE`; the representation and
overflow policy are independent local decisions.

## Verification approach

- Run focused public event-queue tests while implementing.
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
- Review the complete diff for accidental API commitments, unbounded storage,
  hidden allocation after construction, stale claims, and changes outside the
  objective.

## Risks and safe stopping point

Reject-newest can omit a later high-severity event when the queue is saturated;
the explicit outcome makes that loss visible, while preserving the earlier
causal trace. A caller can still ignore the outcome, so this is not a durable
audit log or guaranteed delivery mechanism.

The timestamp type defines only elapsed framework time. Until an injected clock
owns its production, callers can supply non-monotonic values and RFF-REQ-005
remains partial. Mission identifiers are copied in-process values, not protocol
or cFS identifiers. Events removed from the queue become caller-owned and fall
outside the framework storage bound.

Stop after the standalone queue, tests, decision record, and durable state are
coherent. Do not add runtime failure emission, an application context, injected
time, filtering, fan-out, persistence, automatic draining, or scheduling in
this run.

## Result

Pending implementation and verification.
