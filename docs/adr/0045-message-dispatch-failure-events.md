# ADR-0045: Returned message-callback failure events

- Status: Accepted; implemented and reviewed by Codex
- Date: 2026-11-24
- Scope: RFF-LIB-REQ-020 revision 1

## Context

The preceding owned-output consumer completes RFF-LIB-REQ-019. A host
handling both ordinary work and queued commands can already request
structured diagnostics for a returned work error, but dispatch requires
the host to repeat source, severity, timestamp and saturation policy.
Message errors have a distinct type and cleanup contract from work errors.

ADR-0012 defines dispatch's one in-flight item, immediate publications,
terminal failure and selected-tail clearing. ADR-0016 and ADR-0020 define
opt-in failure reporting after the owning operation finishes. No new
upstream behavior or external design assumption is needed.

## Decision

Add MessagingRuntime::dispatch_one_with_failure_event. Delegate exactly
once to dispatch_one. Only its returned application error triggers one
clock read and one Error event with the application identity from that
error and the caller's event identifier. Failed commitment and selected
cleanup precede the injected clock call. Empty, successful and rejected
dispatches keep their existing behavior and have no event attempt.

Return `MessagingOperationError<MessageDispatchEventError<E, EventId>>`.
The outer wrapper preserves the original discarded-delivery count; the
inner wrapper preserves MessageDispatchError and the optional exact
FailureEventAttempt. Borrowing/consuming accessors and Error::source retain
typed inspection. QueueFull retains older events and the rejected record
with its original timestamp; retries remain explicit caller policy.

Keep the method beside dispatch_one in messaging_runtime.rs and event
errors in runtime_events.rs. One private FailureEventAttempt constructor
shares the established event-field/emission policy with ordinary work.
No permanent service ownership, mutable inner-runtime access, dependencies
or callback context expansion is required.

## Alternatives considered

- Host-side event construction: feasible through existing public data, but
  duplicates the policy already standardized for ordinary work. Prefer a
  small opt-in owner method for the same concrete diagnostic need.
- Reuse RuntimeWorkEventError by translating errors: rejected because it
  misstates the callback kind and loses the original message error shape.
- Report inside the callback or before owner cleanup: rejected because
  reporting effects could precede Failed commitment and exact clearing.
- Always emit from dispatch_one: changes an existing contract and requires
  service ownership for hosts that do not want diagnostics. Keep opt-in.
- Generic callback-event framework or combined service owner: unnecessary
  abstraction and wider rewrite for one well-defined missing operation.

## Evidence and consequences

The repository-local contracts at input
`151511aec980dffe41dc25ab5d406798437459ee` are the design sources, recorded
as SRC-RFF-MESSAGE-FAILURE in the [source register](../research/SOURCES.md).
The [consumer guide](../MESSAGE_FAILURE_EVENTS.md) and
[focused review](../verification/MESSAGE_FAILURE_EVENTS_REVIEW.md) connect
public-only tests, independent-package verification and actual review.

The locked local baseline passes 311 workspace tests. Seven focused tests
also pass as identical source in an independent edition 2024 package.
Root and independent Codex source review find no issues; exact source
identity, document review and publication evidence are in the focused record.

The clear count excludes the attempted in-flight item and includes accepted
self-publications remaining in the failed inbox. Accepted peer effects
are retained. Reporting adds no retry, rollback, recovery, panic/hang
containment or delivery guarantee. Caller-retained event copies are outside
the event queue's logical bound; callbacks and clocks must return. Identity
origin remains caller-scoped, including equal-slot cross-runtime aliases.

## Revisit conditions

Revisit for a concrete lifecycle-callback diagnostic need, application-
authored events, scheduled-event composition, priority/reserved capacity,
or consumer evidence that nested errors or service wiring are burdensome.
Preserve exact original errors, cleanup-before-reporting and finite queue
outcomes in any extension. This decision does not freeze the public API.
