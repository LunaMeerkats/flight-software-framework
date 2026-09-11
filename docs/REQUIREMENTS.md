# v0.1 requirements

Status: **Provisional, partially implemented**
Last reviewed: **2026-09-12**

These requirements define host-observable behavior. They do not specify cFS
compatibility, flight readiness, real-time performance, or certification.

## Terms

- **Explicit error:** a typed or otherwise programmatically distinguishable
  result; expected failure does not require parsing text and does not panic.
- **Bounded:** a finite configured maximum enforced by behavior, not merely an
  initial allocation capacity.
- **Repeatable:** identical initial state, simulated time, and ordered inputs
  produce the same defined observable trace in a test. A trace includes ordered
  application work, lifecycle outcomes, and structured event timestamps, but
  excludes unspecified host diagnostics. This is not a real-time determinism
  claim.

## Behavioral requirements

### RFF-REQ-001 — Scope truth

Top-level, sample, and generated project documentation shall identify the
framework as experimental host software and shall not describe it as
flight-qualified, safety-certified, NASA-endorsed, operationally suitable,
TRL-proven, or automatically compatible with cFS, cFE, OSAL, PSP, CCSDS, or an
RTOS.

Acceptance: a documented release review checks every user-facing entry point and
records the reviewed commit.

### RFF-REQ-002 — Explicit application lifecycle

A host scenario shall register at least two independently defined applications
and expose ordered registration, start, stop, and restart transitions. Expected
invalid transitions shall return explicit errors and leave the affected
application in a documented state.

Acceptance: state-machine tests cover every allowed and rejected transition;
an integration test observes both applications completing the scenario.

### RFF-REQ-003 — Bounded publish/subscribe

Publish/subscribe storage shall enforce configured finite capacity. Routing,
FIFO ordering, successful delivery, saturation, and known-but-unavailable
subscriber outcomes shall be explicit. The chosen backpressure or overflow
policy shall not silently depend on a library default. One caller-selected
dispatch shall present at most the oldest queued delivery to a running
application. The framework-supplied callback context shall expose message
publication only through the same bounded, lifecycle-aware mechanism.

Acceptance: tests fill each relevant queue to its exact limit and assert the
publisher-visible result and unaffected-subscriber behavior. Dispatch tests
assert oldest-only consumption, capacity-one self-publication, refreshed peer
availability, and explicit empty, lifecycle-rejected, and returned-error
outcomes.

### RFF-REQ-004 — Injected time and repeatable scheduling

Framework time shall be supplied through an injected clock. Under a simulated
clock, identical ordered inputs shall produce repeatable scheduled-work and
timestamp ordering without wall-clock sleeps.

Acceptance: tests advance time manually, cover equal-deadline ordering, and
replay the same scenario with an identical defined observable trace.

### RFF-REQ-005 — Structured events

The host scenario shall emit machine-inspectable events containing source,
severity, event identifier, and framework timestamp. Tests shall not need to
parse free-form log text to assert those fields.

Acceptance: tests assert structured values and the finite storage or delivery
policy for events.

### RFF-REQ-006 — Validated configuration lifecycle

At least one versioned runtime configuration shall support validation and
activation. Rejecting an invalid replacement shall leave the active value
unchanged. Rollback shall restore the previous accepted value.

Acceptance: state-machine and integration tests cover accepted activation,
rejection without mutation, version behavior, and rollback.

### RFF-REQ-007 — Command and telemetry boundary

The sample mission shall accept at least one validated host command and emit
observable telemetry through explicit boundary adapters. Malformed input shall
be rejected explicitly before reaching application business logic.

Acceptance: integration tests cover one valid command/telemetry path and
representative malformed inputs defined by the chosen adapter, including
length, identifier, or payload constraints when that boundary exposes them.

### RFF-REQ-008 — Defined application-error containment

When one application returns a defined runtime error, the runtime shall produce
a documented lifecycle outcome while another application remains operable.
This requirement does not claim containment of arbitrary panics, hangs, process
termination, memory exhaustion, or hardware faults.

Acceptance: a fault-injection integration test observes the failed
application's state/event and successful subsequent work by the other
application.

## Approved parameters and remaining refinement

These provisional requirements define behavior without freezing unnecessary
public APIs. Accepted project decisions now settle:

- RFF-REQ-002 lifecycle and identity in
  [ADR-0003](adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md);
- RFF-REQ-003 capacity, FIFO, saturation, fan-out, unavailable endpoints, and
  caller-selected one-message dispatch in
  [ADR-0004](adr/0004-bounded-application-inboxes.md) and
  [ADR-0012](adr/0012-application-message-dispatch.md);
- the RFF-REQ-004 elapsed-instant, injected-read, and manual-advance boundary in
  [ADR-0014](adr/0014-injected-manual-framework-clock.md), plus the finite
  one-shot scheduled-work, equal-time ordering, and replay boundary in
  [ADR-0015](adr/0015-caller-driven-scheduled-work.md), extended through the
  messaging owner with exact inbox cleanup in
  [ADR-0021](adr/0021-messaging-owned-scheduled-work.md);
- the RFF-REQ-005 and RFF-REQ-008 direct cooperative returned-work failure-event
  boundary in [ADR-0016](adr/0016-returned-work-failure-events.md), extended to
  messaging-owned ordinary work with exact inbox cleanup in
  [ADR-0020](adr/0020-messaging-work-failure-events.md); and
- RFF-REQ-006 revision and rollback behavior in
  [ADR-0005](adr/0005-configuration-revisions-and-rollback.md), with its
  standalone bounded snapshot core in
  [ADR-0017](adr/0017-bounded-configuration-snapshots.md) and runtime ownership,
  safe-point application visibility, restart retention, and no automatic error
  rollback implemented by
  [ADR-0018](adr/0018-configuration-aware-work-context.md).

[ADR-0019](adr/0019-host-command-telemetry-boundary.md) now selects the
RFF-REQ-007 demonstration grammar: exactly two command bytes, identifier `0x01`
and percentage `0..=100`, checked in length/identifier/value order before
publication or business logic. Matching telemetry uses identifier `0x81` and
the same percentage. A capacity-one host mailbox separates application
dispatch from host drain/encoding. Its returned full error uses existing
terminal failure semantics. The private `host-echo` example now implements
these adapters with shared-source integration evidence for RFF-REQ-007 at the
selected local slice/returned-array boundary. The full v0.1 sample gate remains
unchanged. [ADR-0022](adr/0022-combined-host-sample.md) now demonstrates the
required services in one executable with a shared, tested scenario driver.
CI and the documented human entry-point/architecture reviews remain pending.
The original ownership probe alone does not establish adapter correctness.

Do not use an implementation's accidental behavior to settle these parameters
after the fact.

## v0.1 release gates

These gates support the requirements but are not additional system behaviors:

- a documented executable host sample demonstrates RFF-REQ-002 through
  RFF-REQ-008 and carries the RFF-REQ-001 safety notice;
- RFF-REQ-001 is reviewed separately across all user-facing entry points;
- formatting, selected source-quality lints, warning-free checks, unit and
  integration tests, and documentation checks pass in CI;
- important requirements map to exact evidence in the traceability register;
- dependency licences and enabled features are reviewed;
- unsupported claims and known limitations receive a human architecture review.

Changes to requirement meaning or scope must update this file, the roadmap,
traceability, and project state in the same coherent increment.
