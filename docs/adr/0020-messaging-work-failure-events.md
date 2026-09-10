# ADR-0020: Messaging-owned work failure events

- Status: Accepted and implemented
- Date: 2026-09-10
- Scope: One opt-in returned-work event through the existing messaging owner

## Context

The host adapter pair owns a `MessagingRuntime`. A full service sample must
also demonstrate time, configuration, and cooperative failure events. The
inner `Runtime` already supports direct failure reporting, but is deliberately
private so callers cannot bypass inbox cleanup. Scheduling has the same
composition gap and remains a separate prerequisite.

[ADR-0011](0011-runtime-owned-message-availability.md) assigns lifecycle and
inbox coordination to the messaging owner.
[ADR-0016](0016-returned-work-failure-events.md) requires a future
messaging-aware event path to preserve that owner's clearing rules.
[ADR-0018](0018-configuration-aware-work-context.md) keeps all ordinary work
on the same callback and configuration view. These existing decisions are
sufficient for this increment; no new upstream behavior is assumed.

## Decision

Add `MessagingRuntime::work_with_failure_event` with the same application,
mission event identifier, borrowed clock, and borrowed event queue inputs as
the direct runtime operation. Its error is the existing
`MessagingOperationError<RuntimeWorkEventError<A::WorkError, EventId>>`.
No new public error type, constructor, service owner, or runtime escape is
needed.

The operation first calls `MessagingRuntime::work` exactly once:

1. Successful work returns `Running` without reading the clock, emitting an
   event, or consuming inbox deliveries.
2. Lifecycle rejection returns the original error with zero discarded
   deliveries and no clock read or event attempt.
3. A cooperative returned work error commits only the selected application to
   terminal `Failed` and clears that application's queued deliveries.
4. After clearing completes, read the supplied clock once and attempt one
   application-sourced error event with the supplied identifier.
5. Return the original work error, exact discarded-delivery count, and exact
   event plus its `Recorded` or `QueueFull` outcome. Preserve peer queues and
   the active configuration/history. Perform no automatic retry or rollback.

The event source comes from the application identity in the runtime work
error, rather than a separate caller-selected source. A crate-private
`RuntimeWorkEventError::from_work_error` contains the shared construction and
emission policy. Both owners call it only after their ordinary work operation
returns. Lifecycle errors produce no event. Existing direct-runtime reporting
keeps its behavior and error type.

The public method belongs in `messaging_runtime.rs` with its owning work
operation. The shared event policy remains in `runtime_events.rs`. This is a
concrete service interaction, not a general work-executor trait or callback
framework. The original error chain remains inspectable through the nested
standard `Error::source` implementations.

## Alternatives considered

- **Call the inner runtime's event method, then clear:** fewer changed lines,
  but invokes the injected clock before inbox cleanup. Reject that order;
  the existing messaging work boundary must finish before reporting effects.
- **Construct an event in the host after matching the error:** possible with
  today's public data, but every mission would reproduce source, timestamp,
  rejection, and saturation policy. Select one owner-preserving operation to
  keep those semantics shared with direct work.
- **Expose the inner runtime mutably:** rejects the owner invariant because
  work/stop failure could bypass selected-inbox clearing.
- **Replace wrappers with a unified multi-service owner:** could reduce future
  forwarding, but would rewrite working lifecycle, messaging, configuration,
  and scheduling boundaries for one concrete missing operation. Defer until
  measured composition problems justify that wider design.
- **Add messaging scheduling and scheduled events together:** both require
  explicit schedule-consumption/error contracts. Keep them separate so this
  increment has one observable behavior and a small review surface.

## Evidence and acceptance

The clean pre-change baseline passes 96 tests. Focused public-API evidence is
added in `tests/messaging_work_events.rs` for exact error/event fields and
clock-read counts, selected-inbox clearing, retained peer delivery and later
work, no event on success/rejection, queue saturation with explicit retry, and
configuration visibility/history across failure. The existing direct event,
messaging, and configuration suites remain regression evidence.

All six focused tests and the required Cargo baseline pass, with 102 tests in
the full suite. The shared direct-runtime event tests continue to pass.
Source review confirms owner cleanup precedes the injected clock call; the
public tests inspect returned state and counts rather than accessing the
mutably borrowed owner from inside that clock. Source-form, document, and
complete-diff review results are recorded in the project state and plan.

## Consequences, risks, and revisit conditions

Reporting remains opt-in and limited to cooperative ordinary-work errors.
Message callbacks, lifecycle operations, and scheduled work gain no event
behavior. [ADR-0021](0021-messaging-owned-scheduled-work.md) subsequently adds
messaging-owned scheduling without events. The full sample still requires an
explicit mission driver.

Event storage can reject the newest event; its caller-owned copy is outside
the queue's retained-record bound. Callbacks and clocks may panic or fail to
terminate. The operation does not contain those failures or provide execution
rollback, recovery, physical delivery, or real-time guarantees. Application
and clock identities retain caller-scoped origin limitations.

Revisit when a concrete mission needs scheduled failure reporting, additional
callback events, application-authored events, or an owner redesign that
demonstrably simplifies composition. Preserve original errors, finite storage,
owner cleanup before reporting, and healthy peer progress in any extension.
