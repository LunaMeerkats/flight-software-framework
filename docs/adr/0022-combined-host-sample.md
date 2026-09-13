# ADR-0022: Explicit combined host sample

- Status: Accepted and implemented
- Date: 2026-09-12
- Scope: One private, finite host scenario composing existing services

## Context

The configuration, adapter, messaging-event, and messaging-scheduling slices
now have separate evidence. The remaining Stage 3 question is whether one real
mission can compose them without bypassing service ownership. ADR-0018 keeps
configuration on ordinary work; ADR-0019 separates ingress, message dispatch,
and host drain; ADR-0020 and ADR-0021 expose the required owner operations.
No additional framework API or external source is needed to test composition.

## Decision

Extend the private `host-echo` example with one explicit scenario driver.
The binary and integration tests load the same `mission.rs` and `sample.rs`.
The library and the ADR-0019 two-byte grammar are unchanged.

The host owns the mailbox, a bounded work monitor, manual clock, finite
schedule, and event queue. `MessagingRuntime` owns the two applications,
capacity-one inboxes, and constructor-supplied configuration table. Both
applications borrow the monitor; telemetry also borrows the mailbox. Ordinary
work validates and copies its immutable revision/value into the role's slot.
Message callbacks retain their existing validation and echo/output behavior.

The configuration is exactly one byte in `0..=100`, initially `[10]`. It is an
observation value for this experiment, not an actuator setting, command limit,
physical quantity, or protocol. Replacements remain explicit host safe points.
The optional monitor lets adapter-only fixtures omit observation effects while
using the same configured mission. It retains one latest record per role;
reading consumes that record so stale data cannot establish later work.

The scenario order is fixed and visible in `sample.rs`:

1. Construct/register, then start both applications; record their states.
2. Reject an unknown command, then echo 0, 42, and 100 with separate ingress,
   echo dispatch, telemetry dispatch, and output drain for each command.
3. At manual time zero observe waiting. Advance to 10 ms and invoke two
   equal-time work items, echo then telemetry; observe completion and each
   callback's initial configuration.
4. Activate `[20]` as revision 2, reject `[101]` without mutation, roll back
   to revision 1, reject a second rollback, then activate `[30]` as revision 3.
   Invoke and freshly observe ordinary work after each relevant transition.
5. Stop both, restart both, then perform and freshly observe both work calls.
6. Queue telemetry 7 for the peer and command 9 for echo. Arm one cooperative
   echo failure, advance to 20 ms, and call ordinary work with failure-event
   reporting. Observe terminal failure and one discarded echo delivery.
7. Invoke healthy telemetry work and dispatch its retained record. Drain the
   one structured failure event and observe that the event queue is empty.

The fault switch belongs to the host fixture. Only valid echo ordinary work
consumes it, after recording configuration. It is separate from configuration
and message semantics. Scheduled work emits no failure event. The report keeps
the original nested work error and emission outcome alongside observed states,
records, timestamps, and configuration. It uses named fields and fixed arrays,
not a growing event log. Diagnostic stdout is outside the scenario callbacks.

## Alternatives considered

- **Add a new example with copied adapters:** isolates the earlier executable
  but creates two adapter implementations whose evidence can drift. Extend
  the existing shared mission and preserve its regression tests.
- **Create a unified service owner or general application context:** the
  current owner operations already compose; no new abstraction is justified.
- **Inject failure through a configuration magic value:** would conflate
  valid data and fault controls. Keep an explicit borrowed fixture switch.
- **Use host-owned active configuration as observation evidence:** would not
  establish callback visibility. Copy from the actual work context and consume
  each observation before the next phase.
- **Add scheduled events or physical output now:** introduces new timestamp,
  delivery, and failure policies. Use the existing ordinary-work event path
  and local returned-array output boundary.

## Evidence

The required baseline and focused sample/adapter results are recorded in
[the sample guide](../verification/HOST_SAMPLE.md) and
[traceability](../verification/TRACEABILITY.md). Five tests execute the same
complete driver and assert exact lifecycle states, adapter bytes, equal-time
ordering, callback configuration, rejection/rollback, original error chain, event fields, inbox
clearing, peer progress, and fresh-run report equality. A focused adapter test
also proves telemetry does not consume the armed echo fault and that the
switch is consume-once. Existing hostile-input and saturation tests remain.

## Consequences, risks, and revisit conditions

This completes the combined host behavior demonstration, not the v0.1 release.
CI was outstanding at this decision; the subsequent
[hosted baseline](../verification/CI_BASELINE.md) passed. The
[autonomous dependency and scope review](../verification/DEPENDENCY_SCOPE_REVIEW.md)
prepares evidence; human entry-point and architecture acceptance remain pending.
The scenario demonstrates repeatability only for its initial state, fixed
inputs, manual time, and caller order. It does not contain panics or hangs,
recover a failed application, establish real-time behavior, or guarantee
physical/event delivery. Later successful peer work is cooperative progress.

Two application records, two capacity-one inboxes with one-byte payloads, one
host record, two observation slots, two schedule items, a capacity-one event
queue, and active/rollback one-byte configuration snapshots bound retained
operational data. The fixed report also owns copied records/errors; it is
outside queue limits. Framework setup/report allocations and diagnostic output
remain ordinary host behavior, with no global allocation or stack guarantee.

Revisit if another concrete scenario needs a different command grammar,
configuration meaning, recurrence, richer observation storage, scheduled
events, or external I/O. Keep such needs separate from this fixed fixture.
