# Initial architecture analysis

This document separates observed upstream responsibilities from proposed local
design. Source details and adoption notes are in [the source register](research/SOURCES.md).

## Observed responsibility boundaries

The following observations use the official NASA cFS v7.0.1 public release:

- The public cFS bundle contains cFE, OSAL, PSP, tools, and common/example
  applications. NASA distinguishes that bundle from a flight distribution,
  which selects applications for particular mission requirements, and describes
  the bundle as a starting point rather than a fully verified operational
  system. (`SRC-NASA-CFS`)
- cFE provides Executive, Software Bus, Event, Table, and Time services, plus a
  File Service API. (`SRC-NASA-CFE`)
- OSAL is a collection of operating-system abstraction APIs. (`SRC-NASA-OSAL`)
- PSP abstracts platform-specific functionality. (`SRC-NASA-PSP`)
- cFE guidance treats application identity and lifecycle as executive concerns;
  communication normally uses published message interfaces rather than direct
  calls into another application. (`SRC-NASA-CFE`)
- cFE software-bus pipes have explicit depths, and table loading distinguishes
  staged validation from activation. These are useful responsibility examples,
  not local API requirements. (`SRC-NASA-CFE`)

These facts do not make similarly named local components equivalent or
compatible. This analysis adapts problem boundaries; it does not establish or
claim implementation equivalence.

## Proposed Rust-native shape

The first host implementation should be a small composition, not a set of
one-to-one cFS component clones:

1. **Caller-driven runtime:** owns application registry, identity, lifecycle,
   deterministic ordering, and supervisory policy.
2. **Application boundary:** conforming application behavior accesses framework-
   managed effects through an explicit context and returns typed outcomes.
3. **Framework services:** bounded routing, structured events, clock access, and
   validated configuration are added only as vertical slices require them.
4. **Host adapters:** wall-clock, file, terminal, and future network access stay
   behind narrow interfaces; simulated adapters drive tests.
5. **Mission composition:** a sample selects applications, capacities, policies,
   schedules, and adapters without changing framework internals.
6. **External boundaries:** command ingestion, telemetry output, and any future
   protocol codecs validate external data outside core application logic.

Applications should not call one another directly. The runtime owns framework-
managed lifecycle, queue, and adapter resources; applications may own internal
state. Iteration order that affects observable behavior must be stable. The
initial runtime will not spawn hidden work.

## Chosen first execution model

[ADR-0001](adr/0001-caller-driven-host-runtime.md) selects a serial,
caller-driven host runtime for the first v0.1 slices. A host advances work using
an injected clock; deterministic tests explicitly advance a simulated clock.
This makes event and transition ordering testable without adopting an async
runtime or thread lifecycle early.

Here, **deterministic** means that conforming, terminating applications with
controlled external effects produce the same framework-observable ordering from
the same initial state, simulated time, and ordered inputs. It does not mean
hard real-time execution, bounded wall-clock latency, freedom from OS jitter, or
fault tolerance.

## Approved service policies

- [ADR-0003](adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md)
  defines the LC1 stop-gated lifecycle and bounded runtime-local identity.
- [ADR-0004](adr/0004-bounded-application-inboxes.md) defines one bounded inbox
  per application, non-blocking reject-newest overflow, and explicit partial
  fan-out reporting. It is not implemented.
- [ADR-0005](adr/0005-configuration-revisions-and-rollback.md) defines immutable
  monotonic snapshot revisions and one consume-once rollback slot. It is not
  implemented.

## Current implementation boundary

The first Rust slice is a bounded `LifecycleRegistry`. It reserves a positive
logical record limit, allocates opaque identities in registration order, and
enforces `Registered -> Running -> Stopped -> Running`. It does not own or invoke
application objects. Consequently, it does not yet prove returned-error
containment, actual object retention on restart, or a two-application host
scenario. Those limitations are intentional rather than hidden behind a
placeholder application abstraction.

## Alternatives kept open

- One OS thread per application may later help isolate blocking work, but it
  introduces shutdown, joining, scheduling, and queue semantics that need
  explicit evidence first.
- An async runtime may later suit many concurrent I/O sources, but no current
  workload justifies its dependency, executor, cancellation, or timing model.
- `no_std` or RTOS adapters may later be explored after the host interfaces are
  proven and a concrete target supplies constraints.
- Standard-library bounded channels are a future option, but publish/subscribe
  fan-out and overflow policy still require framework behavior; selecting a
  primitive is not the same as designing the bus. (`SRC-RUST-CHANNEL`)

## Major technical risks

- Premature public traits may freeze the wrong lifecycle and ownership model.
- “Deterministic” may be overstated unless every input and ordering source is
  controlled and the claim remains limited to repeatability.
- Bounded bus queues can coexist with unbounded event, telemetry, or diagnostic
  accumulation unless every operational path is reviewed.
- A returned application error is not equivalent to containing a panic, hang,
  process failure, memory exhaustion, or hardware fault.
- Platform seams created without a second backend may become decorative layers.
- Borrowed cFS terminology may cause compatibility or NASA-affiliation drift.
- Dependencies may silently define concurrency, allocation, or failure policy.
