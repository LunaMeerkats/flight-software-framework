# ADR-0001: Caller-driven host runtime for initial slices

- Status: Accepted for v0.1 experiments
- Date: 2026-08-05
- Scope: Initial host execution and scheduling model

## Context

The framework needs lifecycle supervision, bounded messaging, injected time,
and repeatable tests. No executable workload yet demonstrates a need for
parallel application work or a general async I/O executor. Choosing either now
would also choose shutdown, cancellation, scheduling, and failure semantics
before their requirements are understood.

NASA cFE is C-based and uses Executive Services over OSAL task and operating-
system abstractions across host and RTOS targets. This project adopts the
responsibility for runtime-owned lifecycle, but it is not required to reproduce
that execution mechanism. The official source observations used here are recorded in
[the source register](../research/SOURCES.md).

## Decision

For the first v0.1 vertical slices, use a serial, caller-driven host runtime:

- the host or test harness explicitly asks the runtime to advance work;
- application work executes one item at a time in a documented stable order;
- lifecycle state and application identity are owned by the runtime;
- applications receive explicit service access and return typed outcomes;
- framework time and external effects are injected;
- the runtime starts no hidden threads and owns no global executor; and
- conforming applications keep blocking external I/O outside the initial
  execution path.

The exact Rust traits and public methods are intentionally not decided by this
ADR. The first lifecycle tests should discover the smallest useful API.

“Deterministic” is limited to repeatable framework-observable ordering for
conforming, terminating applications whose external effects are controlled,
given the same initial state, simulated time, and ordered inputs. Rust cannot
prevent application code from bypassing framework adapters to read system time,
perform I/O, spawn threads, or obtain randomness. This decision does not claim
real-time deadlines, bounded wall-clock execution, OS scheduling control, fault
tolerance, or containment of an application that never returns.

## Alternatives considered

### One operating-system thread per application

This may eventually isolate blocking workloads, but immediately requires
join/stop ownership, cross-thread queues, scheduling assumptions, and panic
handling. There is no current workload or latency evidence to justify it.

### General async runtime

This may suit a future I/O-heavy host, but it would introduce a major dependency
and commit to executor, timer, cancellation, and task-lifecycle semantics. The
current behavior can be tested without those commitments.

### Executor abstraction with multiple backends from the start

This appears portable but has no second backend or concrete variation to inform
the interface. It would be speculative indirection.

## Evidence

- Official cFE documentation separates executive lifecycle, bus, event, table,
  and time responsibilities and encourages published inter-application message
  interfaces. (`SRC-NASA-CFE`)
- Official OSAL and PSP repositories distinguish OS abstraction from
  platform-specific facilities. (`SRC-NASA-OSAL`, `SRC-NASA-PSP`)
- Rust's standard library provides a stable bounded channel primitive, but a
  primitive alone does not define publish/subscribe or supervisor policy.
  (`SRC-RUST-CHANNEL`)

The decision is primarily a local risk-reduction choice, not a claim that NASA
or Rust documentation prescribes this architecture.

## Consequences

Positive consequences:

- transition, delivery, and timestamp ordering can be asserted directly;
- the runtime itself creates no background work;
- tests need no wall-clock sleeps; and
- concurrency dependencies and cancellation policy remain deferred.

Costs and risks:

- work is not parallel and one non-returning application blocks progress;
- host I/O must remain outside the step path or be made non-blocking explicitly;
- returned errors are the initial failure boundary, not arbitrary fault
  isolation; and
- later concurrency may require internal redesign; and
- application code that bypasses the explicit adapters falls outside the
  repeatability contract.

## Revisit conditions

Revisit this decision when a concrete, tested workload demonstrates at least one
of the following:

- required progress while another application waits on external I/O;
- a measured latency or throughput constraint the serial model cannot meet;
- a real platform/RTOS integration with incompatible execution requirements; or
- progress-isolation requirements that need threads, or stronger fault-
  containment requirements that need a process or platform isolation boundary.

Any replacement ADR must define ownership, shutdown, cancellation, queue bounds,
panic behavior, time semantics, and deterministic-test strategy.
