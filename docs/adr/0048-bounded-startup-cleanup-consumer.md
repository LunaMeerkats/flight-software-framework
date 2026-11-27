# ADR-0048: Bounded consumer-owned startup and cleanup

- Status: Accepted; implemented with local and independent verification
- Date: 2026-11-28
- Scope: Fixed three-application host composition, RFF-LIB-REQ-023 revision 1

## Context

The public composition example uses infallible lifecycle callbacks and
propagates lifecycle errors immediately. Individual start/stop tests establish
the owner contracts but do not demonstrate a host preserving the original
startup failure while attempting all cleanup of a partially started mission.
The practical-library direction calls for useful downstream composition.

## Decision

Add a standalone public-only runnable consumer with three distinct initially
Registered applications. Attempt startup in fixed order until the first
error. Retain the exact typed start error and stop only the successfully
started prefix in reverse order. Retain each stop result in a fixed inline
report and continue after errors. After complete startup, perform the same
reverse shutdown on a separate explicit host call.

Use existing direct Runtime operations. Keep concrete mission policy in the
example; no library API, dependency, event wrapper, hidden thread or recovery
mechanism is needed. Preserve non-Copy errors without conversion to strings.
The guide makes identity/state/order assumptions and resource bounds explicit.

## Alternatives considered

- Generic framework startup group: deferred. Dependencies, mixed initial
  states, reusable group ownership, retry and compensation need demonstrated
  policy before a public abstraction is justified.
- Return immediately with the startup error: loses the host's opportunity to
  attempt cleanup of earlier successful starts and preserve secondary errors.
- Stop cleanup at its first failure: rejected because earlier starts remain
  eligible for a stop attempt, and their outcomes fit the same finite report.
- MessagingRuntime: unnecessary for this lifecycle consumer. Inbox discard
  counts and routing are already separately demonstrated.
- New start-failure event wrapper: does not resolve partial startup cleanup.

## Evidence

Existing ADR-0003/0006/0007 contracts and Runtime start/stop are authoritative.
The composition example and guide identify the adoption gap. No external
technical uncertainty, upstream cFS behavior or source reuse is involved.
The focused review records actual tests and independent-package evidence.

## Consequences and risks

The fixed consumer holds at most one start error and three stop outcomes.
Callbacks must return. Stop success establishes callback return only. Failed
startup can leave application effects, and terminal Failed prevents ordinary
stop/restart recovery. The recipe does not offer transactionality, arbitrary
resource cleanup, dependency ordering discovery or general adoption evidence.

## Revisit conditions

Reconsider a library abstraction only when another concrete consumer needs
shared group semantics. Different cleanup eligibility, application dependencies,
retry or external compensation require a new explicit policy and evidence.
