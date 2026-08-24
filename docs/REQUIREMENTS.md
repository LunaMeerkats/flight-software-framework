# v0.1 requirements

Status: **Provisional, partially implemented**
Last reviewed: **2026-08-25**

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
policy shall not silently depend on a library default.

Acceptance: tests fill each relevant queue to its exact limit and assert the
publisher-visible result and unaffected-subscriber behavior.

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
public APIs. Human-approved decisions now settle:

- RFF-REQ-002 lifecycle and identity in
  [ADR-0003](adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md);
- RFF-REQ-003 capacity, FIFO, saturation, fan-out, and unavailable endpoints in
  [ADR-0004](adr/0004-bounded-application-inboxes.md); and
- RFF-REQ-006 revision and rollback behavior in
  [ADR-0005](adr/0005-configuration-revisions-and-rollback.md).

Before implementing RFF-REQ-007, record the selected host adapter's input
grammar and validation boundary. The approved ADRs define observable behavior,
not a broad Rust API.

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
