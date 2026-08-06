# Project state

Last updated: **2026-08-07**

## Current milestone

Stage 1 is in progress: one bounded application-lifecycle vertical slice. The
logical LC1 model and owned synchronous start/stop boundaries are implemented;
owned restart and application work are not.

## Verified baseline

- Commit `8899cf986ad35ea32b8aff7950a958ca80365d7d` contains the owned
  stop implementation and exact partial RFF-REQ-002/RFF-REQ-008 evidence.
- Formatting, linting with warnings denied, all 11 tests, and documentation
  generation pass on rustc/cargo 1.96.1.
- Relative links resolve across all 17 Markdown files; all eight requirement
  identifiers match traceability; every referenced source identifier is
  defined.
- Rendered review of the README, project state, ADR-0007, and traceability found
  the expected structure, no page overflow, and an internally scrollable
  traceability table.

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 selects finite-capacity static application ownership and concrete
  returned start errors; ADR-0007 extends that pre-v0.1 boundary with
  synchronous stop and a distinct concrete stop error.
- ADR-0004 defines bounded application inbox behavior, and ADR-0005 defines
  configuration revision/rollback behavior; neither service is implemented.

The unpublished, dependency-free library retains a standalone
`LifecycleRegistry` for logical transition evidence. `Runtime<A>` owns bounded
application records and invokes `Application::start` and `Application::stop`
synchronously after lifecycle validation. Successful callbacks commit
`Running` and `Stopped` respectively. Either returned concrete error is
preserved in an operation-specific caller result while the selected record
enters terminal `Failed`; peer records are unchanged. The runtime creates no
threads or executor and has no work dispatch, restart callback, service context,
message bus, clock, events, or configuration service.

## Work in progress

No implementation work is in progress. The synchronous owned stop increment is
at its intended stopping point.

## Highest risks and uncertainties

- The pre-v0.1 public trait may need a recorded revision when restart, work, or
  service-context behavior is demonstrated.
- A returned stop error may follow partial application-internal cleanup. The
  runtime records `Failed` but does not prove rollback, cleanup, or isolation.
- Static mission composition sizes each runtime record to its concrete
  representation's largest variant; record capacity does not bound allocation
  inside application or error values.
- IDs are valid only with their origin registry/runtime, but the opaque index
  cannot detect a numerically aliased ID from another issuer.
- A returned application error does not contain a panic, hang, process failure,
  memory exhaustion, or hardware fault.
- Queue-slot limits will not constitute memory bounds until message payloads
  receive a separate enforced bound.

## Important unresolved decisions

- The exact copyright-holder text is required before adding the approved MIT
  and Apache-2.0 licence files and Cargo licence expression.
- Owned restart, work, and future service-context shapes are not selected.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.
- No CI configuration exists; all current verification is local.

## Most likely next tasks

1. Add synchronous in-place owned restart proving `Stopped -> Running`, same-
   object retention, invalid callback suppression, and terminal `Failed` on a
   returned restart error.
2. Add caller-driven work and demonstrate a returned work error while a peer
   remains operable, without claiming arbitrary fault containment.
3. Reassess whether operation-specific error wrappers remain clearer than a
   shared abstraction after restart and work provide evidence.

## Latest run

2026-08-07: Added synchronous owned stop. Tests verify success, exact concrete
returned-error preservation, terminal `Failed`, peer stop after failure,
callback suppression for registered/stopped/failed/unknown requests, and the
stop-gated rejection of ordinary start from `Stopped`. Formatting, lint, 11
tests, documentation, links, identifier consistency, and diff checks passed;
restart and application work remain unimplemented.
