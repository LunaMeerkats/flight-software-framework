# ADR-0015: Finite caller-driven scheduled work

- Status: Accepted and implemented as the completing RFF-REQ-004 slice
- Date: 2026-08-29
- Scope: One-shot application work ordering under injected elapsed time

## Context

RFF-REQ-004 requires manually advanced time, stable ordering for work scheduled
at the same instant, and an identical replayed observable trace. ADR-0014
provides elapsed `FrameworkInstant` readings and an injected `Clock`, but it
does not select due work or invoke applications. ADR-0009 provides one
caller-selected running-only `Runtime::work` operation with exact lifecycle and
returned-error behavior, but it has no time relationship.

A first schedule must join those proven boundaries without choosing periodic
phase, missed-period recovery, automatic draining, a shared application
context, or a concurrency model. It also must not let a stopped or failed
application remain at the front of the schedule indefinitely.

## Decision

Add `ScheduledWork` as one copied `(ApplicationId, FrameworkInstant)` item. Its
scheduled instant is the earliest elapsed time at which the work may be
attempted. It is a one-shot release, not a wall-clock value, execution-time
bound, completion deadline, or real-time guarantee.

`WorkSchedule` copies one complete finite agenda at construction:

- an empty agenda is valid and already complete;
- scheduled instants must be nondecreasing;
- equal-time items retain caller configuration order;
- storage for the full item count is reserved before the schedule is returned;
- no public operation adds items or grows the agenda; and
- attempted items remain in fixed storage behind a private forward-only cursor
  until the schedule is dropped.

`Runtime::run_next_scheduled_work` remains serial and caller-driven. One call:

1. Returns `Complete` without reading the clock when no item remains.
2. Otherwise reads the injected clock exactly once.
3. Returns `Waiting` without mutation when the next instant is in the future.
4. Treats equality and overdue instants as due.
5. Consumes exactly that one item before delegating to `Runtime::work`.
6. Returns `Completed` with the successful lifecycle state, or a
   `ScheduledWorkError` containing the item, observed instant, and exact
   `RuntimeWorkError` on rejection or failure.

Consumption is final for successful work, lifecycle rejection, and a returned
application error. Retry is never implicit; it requires another configured
item. This prevents one unavailable application from blocking later due peers
and preserves the existing `Runtime::work` lifecycle semantics. A callback
panic or non-returning call remains outside the cooperative contract; because
the item is consumed before invocation, unwinding also does not leave it at the
schedule head.

The runtime borrows rather than owns the clock for this operation. This lets a
host use the same manual authority to advance time and timestamp events, while
avoiding a second owning runtime wrapper beside `MessagingRuntime`. The caller
must pair schedule items and instants with the intended runtime and clock
origins. Neither `ApplicationId` nor `FrameworkInstant` encodes that origin.

This first operation applies to the lifecycle-only `Runtime`. A future
messaging-aware scheduled operation must delegate through
`MessagingRuntime::work`, not bypass its inbox-clearing behavior by reaching an
inner runtime directly.

## Alternatives considered

### Periodic interval configuration

Deferred. A periodic schedule must define phase, drift, arithmetic overflow,
reconfiguration, and whether missed releases catch up individually, coalesce,
or skip. A finite agenda proves the required time and ordering seam without
settling those policies.

### Drain every currently due item in one call

Rejected for this slice. Batch draining would reduce caller control, allow one
request to perform an agenda-sized amount of work, and require a multi-result
and stop-on-error policy. One item per call matches ADR-0001 and makes progress
observable after every application callback.

### Leave a rejected or failed item pending

Rejected. A stopped or terminally failed application could permanently block a
due peer at the same or later instant. Explicit final consumption makes the
attempt and lack of automatic retry visible.

### Silently skip non-running applications

Rejected. Delegating to `Runtime::work` preserves its typed `NotRunning` result
and avoids creating a second lifecycle interpretation inside the scheduler.

### Sort arbitrary input or use a priority queue

Rejected. Requiring nondecreasing configuration makes equal-time order
explicit, rejects mistakes rather than silently rewriting mission intent, and
needs neither a heap nor a dependency.

### Add a clock-owning scheduled runtime wrapper

Deferred. A second wrapper owning `Runtime` would compete with the existing
messaging owner before a combined service composition exists. The borrowed
clock operation is smaller and keeps host-controlled time available for event
timestamps.

## Evidence

Seven public integration tests demonstrate:

- typed rejection of descending instants and an explicit complete empty agenda;
- zero clock reads for a complete schedule and exactly one read for each
  waiting or due-item decision;
- waiting before an item, inclusive execution at its exact instant, and no
  hidden manual-clock movement;
- exactly one work callback per call, stable caller order at equal instants,
  and an overdue item remaining due;
- final consumption after a lifecycle rejection and successful subsequent work
  by an equal-time peer;
- exact returned work-error preservation, terminal failure of only the selected
  application, and successful subsequent equal-time peer work; and
- identical work outcomes, application callback order, observed lifecycle
  states, and clock-captured structured-event timestamp traces from two fresh
  runs with identical manual advances and ordered calls.

Together with the injected-clock evidence from ADR-0014, these tests verify
RFF-REQ-004's current finite scheduled-work requirement without wall-clock
sleeps. The test-owned event composition does not make event emission a runtime
service, so RFF-REQ-005 and RFF-REQ-008 remain partial.

No external source prescribes this local scheduling policy. The decision
composes already recorded project behavior and uses only stable standard-
library storage.

## Subsequent direct failure-event boundary

[ADR-0016](0016-returned-work-failure-events.md) later verifies RFF-REQ-005 and
RFF-REQ-008 for an opt-in direct `Runtime::work` error. Scheduled errors still
follow this ADR's exact final-consumption behavior and do not emit an event;
messaging-aware or scheduled event integration remains a separate decision.

## Consequences and risks

- The agenda is bounded by its immutable configured item count, but retaining
  consumed items means memory is released only when the schedule is dropped.
- A long overdue agenda requires one explicit caller request per item. There is
  no automatic catch-up loop, throughput guarantee, priority, or fairness claim.
- A schedule can contain a same-position identity from another runtime or an
  instant from another clock origin. The existing caller-scoped contracts
  cannot detect either misuse.
- Constructing an agenda does not validate lifecycle state. State is checked at
  the due attempt so later stop, restart, or failure remains observable.
- Work may mutate application state before returning an error. Final item
  consumption adds no rollback, cleanup, retry, panic containment, hang
  containment, or fault-tolerance guarantee.
- The fixed one-shot shape is deliberately narrower than a likely future
  mission schedule and may be revised before v0.1.

## Revisit conditions

Revisit when the sample mission needs recurring releases, missed-period policy,
runtime-owned schedule configuration, messaging-aware scheduled work, dynamic
reconfiguration, a stable multi-service context, priority or fairness, clock-
domain identity, or concurrency. Any periodic replacement must retain explicit
bounds, overflow behavior, lifecycle outcomes, equal-time ordering, and a
manual-time replay strategy.
