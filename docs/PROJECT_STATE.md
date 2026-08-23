# Project state

Last updated: **2026-08-23**

## Current milestone

Stage 1 is complete: the bounded logical LC1 model and owned synchronous
start/work/stop/in-place-restart boundary are implemented. Stage 2 bounded
interaction policies have recorded decisions, but implementation has not
started.

## Verified baseline

- Implementation commit `43ef56b92da18302eee0c0a0a1f071a29aae0ade`
  contains the caller-driven work boundary, complete RFF-REQ-002 integration
  evidence, and additional partial RFF-REQ-008 evidence.
- Formatting, linting with warnings denied, all 15 tests, and documentation
  generation pass on stable rustc/cargo 1.96.1.
- All 33 relative links resolve across 19 Markdown files; all eight requirement
  identifiers match traceability; all eight referenced source identifiers are
  defined in the 11-entry source register; both Markdown tables have consistent
  row shapes.
- Thirteen changed or controlling Markdown documents were rendered locally to
  HTML and inspected structurally. README, project state, ADR-0009, and
  traceability screenshots were also reviewed without a visible layout defect;
  the wide traceability table retains its expected horizontal scroll.

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 through ADR-0009 select finite-capacity static application ownership
  and distinct synchronous start, work, stop, and in-place restart callbacks
  with concrete returned errors.
- ADR-0004 defines bounded application inbox behavior, and ADR-0005 defines
  configuration revision/rollback behavior; neither service is implemented.

The unpublished, dependency-free library retains a standalone
`LifecycleRegistry` for logical transition evidence. `Runtime<A>` owns bounded
application records and invokes one caller-selected application callback only
after identity and lifecycle validation. Lifecycle success commits `Running`,
`Stopped`, or `Running`; work success retains `Running`. Any returned concrete
operation error is preserved in an operation-specific caller result while only
the selected record enters terminal `Failed`.

The runtime creates no threads or executor. Work has no automatic dispatch,
service context, schedule, fairness rule, message bus, clock, events, or
configuration access. Public integration tests run two independently defined
applications through registration, start, work, stop, restart, and work, so
RFF-REQ-002 is verified. RFF-REQ-008 remains partial because no structured
failure event exists.

## Work in progress

No implementation work is in progress. The caller-driven owned-work increment
is at its intended stopping point.

## Highest risks and uncertainties

- The pre-v0.1 context-free work callback may need a recorded revision when the
  first message, time, event, or configuration service crosses the application
  boundary.
- A returned work, stop, or restart error may follow partial application-
  internal mutation. The runtime records `Failed` but proves no rollback,
  cleanup, reinitialisation, or isolation.
- Four operation-specific error wrappers duplicate a small amount of display
  and source plumbing; current evidence favors clarity over a generic marker.
- Static mission composition sizes each runtime record to its concrete
  representation's largest variant; record capacity does not bound allocation
  inside application or error values.
- IDs are valid only with their origin registry/runtime, but the opaque index
  cannot detect a numerically aliased ID from another issuer.
- One blocking or non-returning callback prevents caller progress. Returned
  application errors do not contain panics, hangs, process failure, memory
  exhaustion, or hardware faults.
- Queue-slot limits will not constitute memory bounds until message payloads
  receive a separate enforced bound.

## Important unresolved decisions

- The exact copyright-holder text is required before adding the approved MIT
  and Apache-2.0 licence files and Cargo licence expression.
- The first bounded message representation, payload limit, and service-context
  borrowing shape are not selected.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.
- No CI configuration exists; all current verification is local.

## Most likely next tasks

1. Reorient from ADR-0004 and implement the smallest coherent bounded inbox and
   publish/subscribe behavior with exact capacity, reject-newest saturation,
   and publisher-visible results.
2. Introduce a work service context only when that inbox slice has a concrete
   message-delivery need, keeping borrowing and ownership explicit.
3. Follow bounded messaging with structured finite event delivery, then inject
   simulated time before scheduling work.

## Latest run

2026-08-23: Added one synchronous caller-selected work operation for `Running`
applications. Tests verify retained application state, typed callback
suppression for registered/stopped/failed/unknown targets, exact concrete work
error preservation, terminal `Failed`, successful subsequent peer work, and a
complete two-application LC1 scenario. Formatting, linting, 15 tests, rustdoc,
links, identifier consistency, Markdown structural rendering, and diff checks
passed. Stage 1 and RFF-REQ-002 are complete; no automatic dispatch, context,
messaging, clock, event service, dependency, thread, or executor was added.
