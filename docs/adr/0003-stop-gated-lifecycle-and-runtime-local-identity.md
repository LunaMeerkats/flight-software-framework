# ADR-0003: Stop-gated lifecycle and runtime-local identity

- Status: Accepted; logical and owned lifecycle/work behavior implemented
- Date: 2026-08-05
- Scope: v0.1 application lifecycle and logical identity

## Context

RFF-REQ-002 requires explicit registration, start, stop, restart, invalid-
transition behavior, and two independently defined applications. The caller-
driven runtime selected by ADR-0001 does not need asynchronous transient states.
The first implementation also needs bounded identity storage without committing
to an application trait, factory, or recovery mechanism.

## Decision

LC1 has exactly four stable states: `Registered`, `Running`, `Stopped`, and
`Failed`. Its successful lifecycle transitions are:

| Current state | Operation | Resulting state |
| --- | --- | --- |
| `Registered` | `start` | `Running` |
| `Running` | `stop` | `Stopped` |
| `Stopped` | `restart` | `Running` |

All other lifecycle operation/state pairs return a typed error and leave the
state unchanged. There are no public `Starting`, `Stopping`, or `Restarting`
states because operations are serial and synchronous.

When the application-execution boundary is added:

- successful work retains `Running`;
- a returned start, work, stop, or restart error enters terminal `Failed`;
- invalid transitions invoke no application operation;
- only `Running` applications receive work;
- `restart` is valid only after an explicit successful `stop` and reuses the
  same application object; and
- arbitrary panic, hang, process failure, and hardware fault remain outside
  LC1.

Logical application identity is:

- allocated opaquely and monotonically in successful registration order;
- scoped by caller contract to the issuing registry/runtime; the initial slot
  key does not encode or validate its origin;
- stable across lifecycle changes and never reused because v0.1 has no
  deregistration;
- unrelated to mission, protocol, or persistent identifiers; and
- bounded by a positive mission-configured registry capacity reserved at
  construction. A full registration attempt returns a typed error without
  mutation.

## Current implementation boundary

The standalone `LifecycleRegistry` owns only bounded logical records and
successful LC1 transitions. The separate finite-capacity `Runtime<A>` owns
application values and implements synchronous start, caller-selected work,
stop, and in-place restart. Lifecycle success enters `Running`, `Stopped`, or
`Running`; work success retains `Running`; any returned concrete operation
error enters terminal `Failed`. Restart and work mutably borrow the retained
application value. A public integration test observes two independently defined
applications completing the full LC1 sequence with work before and after
restart, so RFF-REQ-002 is verified. At this lifecycle checkpoint RFF-REQ-008
remained partial because returned failure produced no structured event.
[ADR-0016](0016-returned-work-failure-events.md) later verifies it at the direct
cooperative returned-work boundary.

## Alternatives considered

- Composite restart from `Running`: rejected because it introduces two-phase
  stop/start failure without providing a clean-state guarantee.
- Fresh-incarnation restart: deferred because it requires a construction
  factory, generation identity, and service cleanup policy.
- Caller-supplied or persistent IDs: deferred to avoid coupling lifecycle to
  mission configuration or protocol namespaces.
- Generational/reusable slots: unnecessary without deregistration.

## Consequences and revisit conditions

The transition matrix is finite and exhaustively testable, registration order is
stable, and the first crate needs no external dependencies. Equal-position keys
from different registries can compare equal and address the corresponding slot;
cross-registry mixing is a caller error, not an origin-checked failure.

Revisit when recovery from `Failed`, restart-from-running, deregistration,
persistent mission identity, or a real application construction boundary is
required.
