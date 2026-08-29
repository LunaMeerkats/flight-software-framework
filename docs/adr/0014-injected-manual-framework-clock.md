# ADR-0014: Injected manual framework clock

- Status: Accepted and implemented as the first RFF-REQ-004 slice
- Date: 2026-08-28
- Scope: Elapsed framework-time observation and controlled manual advancement

## Context

RFF-REQ-004 requires injected framework time and repeatable behavior under a
simulated clock. RFF-REQ-005 requires structured events to carry framework
timestamps. ADR-0013 established `EventTimestamp` as an elapsed duration but
left every producer able to invent that value directly. The repository has no
clock, scheduler, wall-clock adapter, or runtime-owned event path.

The first time boundary must support the event timestamp consumer without
settling equal-deadline scheduling, overdue-work behavior, host-clock mapping,
or an application service context. It also must not make time progress as a
side effect of observation.

## Decision

Add three pieces of elapsed-time vocabulary:

- `FrameworkInstant` is a copied, ordered `Duration` from the origin of one
  framework clock. It is not a wall-clock value and carries no clock identity,
  so its ordering contract applies only to readings governed by the same
  origin.
- `Clock` is an object-safe injected boundary with one read-only `now` method.
  Reading time neither advances it nor performs hidden work.
- `ManualClock` is the only implementation in this slice. It starts at zero by
  default, can start at an explicit elapsed value for controlled scenario
  setup, and moves only through caller-requested `Duration` advances.

Manual advancement is nondecreasing rather than strictly increasing. A zero
advance succeeds without changing the current instant. Positive additions use
`Duration::checked_add`. Overflow returns
`ManualClockAdvanceError::Overflow { current, advance }` before assignment, so
the clock retains its prior reading. The clock is not `Clone` or `Copy`; it is
one mutable time authority unless a mission explicitly constructs another.

Keep `EventTimestamp` as the event-field wrapper rather than making the clock
return event-specific vocabulary. It can capture an injected `Clock` reading,
convert from a `FrameworkInstant`, or retain the existing explicit elapsed
constructor for standalone and replayed records. The latter path means only
clock-captured timestamps have framework-production evidence.

Do not attach the clock to `EventQueue`. The queue stores already-complete
records and applies its independent FIFO and overflow policy. Timestamping at
queue insertion would combine producer and storage responsibilities and make a
rejected event's retry time ambiguous. Public tests instead compose an injected
clock, structured events, and the unchanged bounded queue explicitly.

This slice adds no host clock, automatic ticking, sleep, scheduler, deadline,
runtime integration, application context, returned-error event, thread,
executor, dependency, real-time claim, or cross-clock ordering guarantee.
RFF-REQ-004 and RFF-REQ-005 remain partial.

## Alternatives considered

### Make `Clock::now` return `EventTimestamp`

Rejected. Scheduled work is the next recorded time consumer. Making the time
provider depend on its first event consumer would either spread event-specific
vocabulary into scheduling or force immediate public API churn.

### Use `std::time::Instant` or `SystemTime` directly

Rejected for controlled tests. `Instant` is tied to a host monotonic clock and
`SystemTime` can move non-monotonically. Neither lets tests advance framework
time explicitly without wall-clock waiting. They remain possible inputs to a
future narrow host adapter, not the framework time contract. (`SRC-RUST-TIME`)

### Advance automatically on every read

Rejected. Observation would mutate state, identical reads could not represent
equal timestamps or equal deadlines, and test traces would depend on incidental
access count rather than explicit scenario inputs.

### Saturate or panic on advancement overflow

Rejected. Saturation would hide the rejected advance and make time appear to
move successfully. A panic would turn an expected representational boundary
into process-level control flow. A typed error preserves both inputs and the
unchanged clock state. Stable `Duration::checked_add` provides this boundary.
(`SRC-RUST-TIME`)

### Add scheduling or runtime event emission with the clock

Deferred. Equal-deadline order, missed deadlines, work selection, event
identifiers, event saturation, returned-error preservation, and peer progress
are separate policies. Combining them would exceed one reviewable increment.

## Evidence

Public-API tests demonstrate:

- a new clock starts at zero and repeated reads or event timestamp captures do
  not advance it;
- explicit positive advances accumulate exactly and zero advance is a
  successful no-op;
- an advance beyond `Duration::MAX` returns the exact typed error and preserves
  the maximum current instant;
- two fresh clocks given the same ordered advances produce identical instant
  traces; and
- equal clock readings produce equal event timestamps, a later explicit
  advance produces the exact later timestamp, and the existing event queue
  retains emission-order FIFO.

The implementation uses safe stable Rust and adds no dependency or ambient time
read.

## Subsequent scheduling boundary

[ADR-0015](0015-caller-driven-scheduled-work.md) later composes this clock with
a finite one-shot work agenda. Its equal-time ordering and replay evidence
verify RFF-REQ-004. Statements above that RFF-REQ-004 remained partial describe
this decision's original checkpoint. RFF-REQ-005 remains partial because the
replay's events are test-owned observations rather than runtime/application
event emission.

## Subsequent returned-work event boundary

[ADR-0016](0016-returned-work-failure-events.md) later uses one injected clock
read after a cooperative returned work error and attempts the resulting event
through the bounded queue. This verifies RFF-REQ-005 and RFF-REQ-008 only at
that direct returned-work boundary. It does not make the runtime a permanent
clock owner or add general application event emission.

## Consequences and risks

- A caller can compare `FrameworkInstant` values from unrelated clock origins;
  the type cannot detect that misuse. Runtime composition must eventually own
  one clock or otherwise establish the domain explicitly.
- A caller can construct a manual clock at a nonzero elapsed value. This helps
  focused and replayed scenarios but is not a wall-clock restore or persistence
  contract.
- `EventTimestamp::from_elapsed` still permits arbitrary explicit values.
  Framework-owned event integration must use the injected capture path before
  RFF-REQ-005 can claim framework timestamp production in a host scenario.
- At this decision's checkpoint, repeatable clock traces did not prove
  repeatable scheduled work. ADR-0015 later supplies due-work selection,
  equal-time ordering, replay, and lifecycle evidence.
- No elapsed duration corresponds to UTC, mission epoch, oscillator accuracy,
  leap seconds, host suspend behavior, or a real-time guarantee in this slice.

## Revisit conditions

Revisit this decision when caller-driven scheduling defines deadlines and
equal-time ordering, runtime-owned events choose their identifiers and
saturation behavior, a sample mission composes one authoritative clock, a host
adapter must map elapsed time to an external epoch, or a concrete consumer needs
clock-domain identity or stronger arithmetic than ordered observation.
