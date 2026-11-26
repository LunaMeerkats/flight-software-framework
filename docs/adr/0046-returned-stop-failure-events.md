# ADR-0046: Returned-stop failure events

- Status: Accepted; implemented and reviewed by Codex
- Date: 2026-11-26
- Scope: RFF-LIB-REQ-021 revision 1

## Context

A host shutting down an application needs to observe a returned stop error
alongside ordinary-work and message-callback diagnostics. Existing
[stop-failure evidence](../../tests/messaging_stop_failure.rs) already has
two selected deliveries cleared at failure and retained peer FIFO progress.
The public stop errors expose enough data for host construction, but no
opt-in operation supplies the established timestamp/severity/admission policy.
The preceding command-diagnostic consumer is complete at input
`8fca981837327828b3caf6245984f5fa639e4fd6`.

## Decision

Add Runtime::stop_with_failure_event and the matching MessagingRuntime
operation. Delegate exactly once through the existing owner's stop method.
Preserve direct ApplicationState and messaging MessagingStopOutcome on success.
Only a returned application error triggers one injected clock read and one
application-sourced Error event with the caller's event identifier. Failed
commitment and selected inbox cleanup precede the clock call.

A distinct `RuntimeStopEventError<E, EventId>` retains RuntimeStopError and
an optional exact FailureEventAttempt with borrowing/consuming accessors,
Display and Error::source. The messaging result retains its outer
MessagingOperationError and exact discarded-delivery count. Share existing
private event construction without translating stop errors into work errors.
Success and lifecycle rejection neither read time nor attempt an event.
Saturation preserves older events and the rejected original record; retry
remains a separately bounded, explicit host policy.

## Alternatives considered

- Host construction through existing public errors: feasible, but repeats
  the opt-in diagnostic policy already standardized for work and dispatch.
- Start/stop/restart wrappers together: defer. Stop has the concrete nonempty
  cleanup scenario; start/restart normally have empty unavailable inboxes.
- Always report from ordinary stop: would require injected services and alter
  existing consumers. Keep the ordinary operation unchanged.
- Generic lifecycle error/event framework: widens the public surface and
  obscures operation-specific errors without a demonstrated second need.

## Evidence and consequences

Repository-local contracts are authoritative: ADR-0007 owned stop, ADR-0011
messaging ownership, ADR-0016/0020 work reporting and ADR-0045 message reporting.
SRC-RFF-STOP-FAILURE in the [source register](../research/SOURCES.md) records
the exact input and adoption. No external behavioral assumption or source
reuse is needed. The [consumer guide](../STOP_FAILURE_EVENTS.md) and
[focused review](../verification/STOP_FAILURE_EVENTS_REVIEW.md) retain
actual public-only/independent-package checks and Codex review evidence.
The locked baseline passes 324 workspace tests; all eight focused histories
also pass as identical source in an independent Cargo package. Root and
independent Codex accept the complete source/document/diff and rendered review.
Publication evidence follows separately. No dependency, service owner, workflow
or lint policy change is selected.

Callbacks and clocks must return. Reporting adds no recovery, rollback,
implicit retry, reserved event capacity or delivery guarantee. Caller-owned
retention is outside the queue bound. Identity remains runtime-local;
equal-slot foreign IDs can address the receiving owner's record. The design
does not freeze the API or establish operational suitability.

## Revisit conditions

Revisit for a demonstrated start/restart diagnostic consumer, burdensome
nested errors, or a concrete need for application-authored lifecycle events.
Any extension must preserve original errors, owner cleanup before reporting
and finite queue outcomes. Additional reporting roles need explicit host
retention bounds rather than an assumed single-failure slot.
