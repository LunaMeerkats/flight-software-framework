# ADR-0047: Host elapsed-clock adapter

- Status: Accepted and implemented; local and independent verification pass
- Date: 2026-11-27
- Scope: RFF-LIB-REQ-022 revision 1; caller-owned host polling

## Context

The public Clock boundary already supports elapsed observations, events and
periodic releases. All runnable scheduling consumers use ManualClock; a host
consumer must write its own adapter before polling against actual elapsed time.
ADR-0014 explicitly left a narrow host adapter open. Configuration has existing
public consumer coverage, and no concrete start/restart diagnostic consumer
justifies another wrapper family. This closes a distinct integration gap.

## Decision

Add HostClock to the existing clock module and re-export it. Construction
captures one private std::time::Instant origin; new and Default each create
a fresh clock. Each Clock::now takes one host observation and maps it through
saturating_duration_since(origin). A pre-origin observation maps to zero.
No duration is added to a native Instant, and no time history is retained.
Do not implement Copy or Clone or expose the native origin in the public API.

Keep ManualClock for controlled tests. Clarify that Clock reads do not request
advancement: a host source progresses independently. Origins remain untagged;
the host pairs elapsed deadlines and timestamps with one clock. This extends
ADR-0014's adapter boundary without changing manual advancement or scheduling.

Add a standalone public-only example with five explicit polling turns, one
periodic cursor, caller-owned sleep between turns, one counter application and
explicit stop. Its poll turn accepts dyn Clock so deterministic tests exercise
the same work policy using ManualClock. Sleep and callbacks remain outside the
adapter. Live smoke checks assert finite call bounds, without timing thresholds.

## Alternatives considered

- Consumer-local Instant adapter: feasible, but repeats a small foundational
  conversion in every downstream host. A library implementation centralizes its
  origin and saturation contract without owning a loop.
- SystemTime/UTC conversion: rejected for this elapsed-time need. External epoch
  correlation is a separate contract and not required by existing consumers.
- Cache and clamp the last returned duration: deferred. It adds state and hides
  clock regressions behind a recovery policy. Current periodic polling already
  waits on a regressed reading; this adapter reports its defined conversion.
- Return a new clock-error type: deferred. Clock is presently infallible and
  saturation supplies a narrow specified mapping. Fault detection needs a
  concrete health consumer before changing every time consumer.
- Runtime-owned polling, threads or async timers: rejected for this increment.
  Hosts already control selection, waiting, callback errors and shutdown.

## Evidence

[Rust's Instant documentation](https://doc.rust-lang.org/std/time/struct.Instant.html)
(stable 1.100.0, accessed 2026-11-27) defines saturating duration subtraction.
It also documents platform clock anomalies, non-steady readings and unspecified
suspend accounting. SRC-RUST-HOST-CLOCK records the adopted conversion and limits.
No upstream source code is copied. Deterministic mapping tests cover exact
nanoseconds, equal observations, pre-origin saturation and an unclamped
regression. The [focused review](../verification/HOST_CLOCK_REVIEW.md) records
the actual local, downstream and hosted evidence as it becomes available.

## Consequences and risks

Only one origin is stored; there is no history, queue, background work or new
dependency. Host readings are not replay inputs or UTC, and do not guarantee
deadline execution. Saturation
does not repair a platform clock; readings may regress after an earlier positive
observation. Example completion requires clock, callback, wait and output calls
to return. Finite turn count does not bound elapsed execution time. API stability,
general adoption, qualification and real-time execution are not established.

## Revisit conditions

Revisit for a concrete clock-fault consumer, externally correlated epoch,
suspend-aware policy, shared-origin transfer, clock-domain validation or evidence
that the infallible Clock boundary prevents a needed integration. Platform or
embedded commitments retain their separate gates.
