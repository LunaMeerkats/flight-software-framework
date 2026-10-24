# ADR-0030: Host-owned periodic work releases

- Status: Accepted and implemented; local/independent verification passed
- Date: 2026-10-25
- Scope: One recurring release cursor; existing work owners remain unchanged

## Context

The ordinary-work output consumer demonstrates a bounded producer whose host
transfers/drains output between work calls. A recurring housekeeping producer
needs this behavior beyond a predetermined finite agenda. WorkSchedule can be
rebuilt for every release, but that reserves/drops an agenda and leaves interval,
late-poll and overflow policy to each consumer. Direct clock gating avoids the
agenda but still duplicates those decisions. This is a source-based consumer
assessment by root Codex and independent repeated_work_assessment, not an
external adoption study. RFF-LIB-REQ-005 revision 1 precedes implementation.

## Decision

Add PeriodicWork, a standalone host-owned cursor containing one ScheduledWork
and a positive Duration interval. It does not own or invoke a runtime. Its poll
returns PeriodicWorkOutcome: Waiting, Released or Exhausted. Active polling reads
the injected clock exactly once. Future work is unchanged. Equality and overdue
work release one item, then re-arm from the observed instant plus the interval.
No missed-release count or backlog is retained. This is delay from observation,
not fixed phase, delay from callback completion, or a deadline guarantee.

The due item is released even if checked addition cannot represent the next
instant. Released carries next_scheduled_at=None in that case; future polls are
Exhausted without clock reads. Otherwise next_scheduled_at is strictly later
than the observed instant. Zero interval is a typed construction error. Storage
is fixed inline, with no allocation, queue, counter, retained history or growth.

The host explicitly invokes Runtime::work or MessagingRuntime::work using the
released item's application ID. Release is consumed before host action. The host
may discard it. No execution, automatic retry, rollback, failure event or timer
disabling is implied. The cursor is independent of lifecycle state: a later poll
can release again after a work rejection/failure. The demonstrated host drops its
selected timer after a terminal error and retains a healthy peer timer. Success
retains ordinary identity/configuration semantics; messaging failure retains its
exact selected-inbox cleanup. The existing one-shot API remains unchanged.

## Alternatives considered

- Rebuild finite WorkSchedule values: supported but unnecessary per-release
  reservation and duplicated recurrence policy for this consumer.
- Keep all recurrence arithmetic in each host: possible and remains useful for
  different policies; select the narrow reusable cursor for this tested policy.
- Add direct and messaging periodic-execution methods: defer. A separate release
  result composes both existing owners without another callback/error hierarchy.
- Preserve initial phase or catch up every missed release: defer. This producer
  wants one current observation, not stale work bursts; phase preservation needs
  different arithmetic and missed-period evidence.
- Use saturating addition: reject, because repeated polling at Duration::MAX
  would keep releasing at the same instant instead of exposing exhaustion.
- Introduce a timer registry, executor or generic policy trait: reject without
  a demonstrated multi-timer ownership or policy-composition need.

## Evidence and consequences

Official Rust Duration checked_add/is_zero documentation informs arithmetic and
validation only; SRC-RUST-PERIODIC-RELEASE records provenance. Scheduling policy
is locally chosen. No upstream implementation is copied and no dependency is
added. Public-only tests and their independent Cargo package, source/diff review
and hosted checks will be recorded in
[the focused review](../verification/PERIODIC_WORK_REVIEW.md).

## Risks and revisit conditions

The cursor has no application or clock origin identity. Hosts must pair them
correctly, choose polling order, drain output and handle work errors. A clock
that moves backward leaves the retained future release waiting. A slow callback
may make the next release overdue because re-arming happens before invocation.
There is no fairness, time bound, synchronization, physical delivery, process
memory bound, panic/hang containment, API stability or qualification claim.
Revisit for a concrete fixed-phase, missed-period accounting, cancellation,
runtime-owned configuration, multi-timer ordering or executor need. Separate
consequential authorization gates remain unchanged.
