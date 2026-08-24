# Plan: couple messaging availability to runtime lifecycle

Status: **Complete**
Date: **2026-08-25**

## Objective

Implement the smallest runtime-owned lifecycle integration for the bounded
message core: couple one fresh inbox to every registered application, derive
delivery availability from runtime state, clear queued deliveries when an
endpoint stops or enters terminal `Failed`, report the exact discarded count,
and reconnect an empty inbox after a successful stopped-to-running restart.

## Context

The clean pre-change baseline passes with 23 tests. ADR-0004 already defines
unavailable endpoint reporting, queue clearing, and restart reconnection, while
ADR-0010 deliberately stops at a detached available-endpoint routing core.
Stage 2 cannot safely add an application work context until runtime and message
ownership make lifecycle synchronization non-optional.

This slice records the previously implicit availability table: only `Running`
applications are available. `Registered`, `Stopped`, and terminal `Failed`
applications remain known subscribers but reject delivery as unavailable.
Failed applications cannot restart under LC1; reconnection applies only to a
successful `Stopped -> Running` restart.

## Acceptance criteria

- Add an owning integration type that consumes a fully composed `Runtime` and
  builds a fresh empty bus without exposing either inner component mutably.
- Require exactly one inbox configuration per registered application, assign
  identities internally in registration order, and reject attachment unless
  every runtime record is still `Registered`.
- Preserve ownership of the supplied runtime when attachment or message-bus
  construction fails.
- Treat only `Running` endpoints as available during publication. Include
  matching unavailable subscribers in the ordered report with a distinct
  `Unavailable` status.
- Clear only the selected inbox after successful stop or any returned callback
  error that commits terminal `Failed`; leave queues unchanged after lifecycle
  rejection.
- Return the exact discarded-delivery count on successful stop and alongside
  callback errors without losing the existing concrete error or source chain.
- Reconnect an empty inbox only after successful restart from `Stopped`; never
  reconnect terminal `Failed` records.
- Test construction coupling, registered/stopped/failed outcomes, exact
  clearing, unaffected-peer behavior, lifecycle rejection, and empty restart
  reconnection through the public API.
- Keep RFF-REQ-003 partial because application self-publication, one in-flight
  dispatch, and a messaging work context remain unimplemented.
- Add no dependency, thread, executor, dynamic topology, protocol identifier,
  event service, time service, scheduler, or lint suppression.

## Files and components

- `src/messaging.rs`: runtime-ordered inbox definitions, unavailable delivery
  outcome, state-aware internal publication, and exact queue clearing.
- `src/messaging_runtime.rs`: ownership coupling, lifecycle delegation, and
  discarded-delivery reporting.
- `src/runtime.rs` and `src/lib.rs`: narrow integration support and intentional
  public exports.
- `tests/runtime_messaging.rs`: public lifecycle/message integration evidence.
- `docs/adr/0011-runtime-owned-message-availability.md`: state table,
  construction, operation ordering, result shape, and deferred boundaries.
- README, architecture, ADR-0004, ADR-0010, roadmap, project state, and
  traceability: truthful implementation status and evidence.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo check --workspace --all-targets --all-features`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run warnings-denied all-feature rustdoc generation.
- Audit handwritten Rust physical and comment-only widths and all reasoned
  expectations.
- Resolve relative Markdown links; render changed documents; check requirement
  and source identifiers and table shapes.
- Run `git diff --check` and review the complete diff.

## Known risks

- The integration freezes topology only after registration; future mission
  composition may need a more direct configuration builder, but no current
  behavior justifies one.
- The detached `MessageBus` remains public for routing-core evidence and still
  models all endpoints as available. Lifecycle claims apply only to the owning
  integration type.
- The provisional integration exposes publication and queue observation but no
  application dispatch or service-context borrowing.
- Clearing occurs after a synchronous callback returns and the runtime commits
  its state. Panics, hangs, process failure, and application-internal cleanup
  remain outside this cooperative boundary.
- Per-publication report allocation and inline payload-copy costs are unchanged
  from ADR-0010.

## Safe rollback or stopping point

Stop after lifecycle-derived availability, exact clearing, restart
reconnection, focused public tests, the ADR, and partial traceability are
coherent and verified. Do not add application message access, automatic
dispatch, events, time, scheduling, configuration, or protocol work in this
run.

## Result

Implementation commit `65bd4fa458bb6f82fe73af291f90e90ee582e3d5`
reached the intended ownership stopping point. `MessagingRuntime` consumes a
fully composed still-registered runtime, constructs one fresh inbox per record,
derives availability from lifecycle state, returns exact stop/returned-error
clearing counts, and reconnects an empty inbox only after successful restart
from `Stopped`. Ten focused tests bring the complete suite to 33.

Independent review corrected stale standalone-bus wording, strengthened the
clearing evidence from one to two queued deliveries, asserted registration-order
identities and distinct positional capacities, narrowed topology guarantees,
and restored public-first helper placement. All required Cargo checks,
warnings-denied rustdoc, source widths, reasoned expectations, relative links,
rendered document structure, identifiers, tables, and whitespace checks pass.
No dependency, thread, executor, work context, dispatch, protocol boundary,
licence change, or push was added.
