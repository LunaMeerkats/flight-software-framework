# ADR-0033: Backpressure-aware recurring host coordination

- Status: Implemented; local and independent-consumer verification passed
- Date: 2026-10-29
- Scope: Existing public-only work_output consumer; library contracts unchanged

## Context

The work_output consumer stages one configured value and transfers it to one
subscriber. PeriodicWork supplies recurrence independently. Their composition
exposes a concrete gap: a retained transfer can occupy the producer slot at the
next release, while deferred host observation can occupy the sink output slot.
Blind work or dispatch then returns a terminal application error for ordinary
backpressure. Root Codex and independent consumer_assessment identify this by
source inspection, not external adoption. RFF-LIB-REQ-008 precedes source changes.

## Decision

Extend the existing example with a mission-owned bounded host turn. Dispatch
one sink message only if its observation slot is empty and it is Running.
Attempt the existing validated sole-subscriber transfer once. If pending output
remains, report backpressure without reading the clock or polling recurrence.
Otherwise poll once and explicitly invoke at most one released producer work
item. Leave new output pending for a later turn. The caller drains observation
outside the turn. Retain separate dispatch, transfer and production outcomes.

Keep the timer in an Option. A returned work error disables it before returning
the concrete error chain; earlier turn effects are not rolled back. Dispatch
or transfer errors end the turn before recurrence; topology errors may already
have delivered and must not be blindly retried. Paused polling preserves the
cursor; resumption uses existing overdue coalescing and observed-time re-arming.

## Alternatives considered

- Poll and discard blocked releases: valid for explicit omission policy, but
  this consumer preserves the next unpolled release until capacity is available.
- Add a framework runner/context or grow queues: defer. Existing APIs express
  this serial policy; more capacity postpones saturation without resolving it.
- Mission-owned bounded coordination: selected. It tests useful integration
  while keeping ordering, lifecycle and output policy visible to the consumer.

## Evidence, consequences and risks

Official Rust Cell and Option documentation informs shared serial slots and
explicit timer removal only; scheduling/backpressure policy is locally chosen.
SRC-RUST-HOST-COORDINATION records provenance. The identical complete example
must pass the independent-package recipe and the full repository baseline.
Actual tests, review and publication belong in the focused verification record.

The pipeline holds at most one output, one sink-inbox message and one pending
producer value. A turn is not transactional: successful earlier stages remain
committed after a later error. Caller-owned cells and IDs must be paired with
this runtime. Cell is not synchronization. There is no fairness, drain deadline,
process-memory bound, physical delivery, automatic recovery or qualification
claim. Revisit for measured multi-producer fairness, fan-out retry or an ordering
need this composition cannot express. Consequential authorization gates remain.
