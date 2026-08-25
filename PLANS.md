# Plan: dispatch one bounded application message

Status: **Complete**
Date: **2026-08-26**

## Objective

Implement the smallest application-facing messaging boundary: let a caller
select one running application, move at most its oldest queued delivery into one
in-flight callback, and let that callback publish through the same
lifecycle-owned bounded bus.

## Context

The clean pre-change baseline passes with 33 tests. ADR-0004 already places one
serial in-flight delivery outside configured inbox capacities. ADR-0010 proves
FIFO and bounded routing, while ADR-0011 proves runtime ownership, lifecycle
availability, clearing, and restart reconnection. Applications still cannot
consume or publish messages, so the strongest remaining RFF-REQ-003 evidence is
the borrowing and dispatch boundary that joins those completed slices.

The existing context-free `Application::work` remains useful for caller-selected
work that is not caused by a queued message. This increment will add a separate
message callback rather than prematurely forcing all future services into one
general context.

## Acceptance criteria

- Add a narrowly scoped application messaging trait whose callback receives
  only the selected in-flight message and a publish-only context.
- Add one caller-selected `MessagingRuntime` dispatch operation that validates
  identity and `Running` state before changing an inbox.
- Return an explicit no-message outcome without invoking application code.
- Remove at most the oldest queued message, keep it outside inbox capacity only
  for the synchronous callback, and expose no nested dequeue or dispatch path.
- Derive publication availability from a preallocated state snapshot refreshed
  immediately before the callback. The callback cannot mutate lifecycle state,
  so the snapshot is not a second lifecycle authority.
- Prove true self-publication can fill the slot freed by the in-flight message,
  while saturation, peer delivery, unavailable reporting, and FIFO remain
  explicit.
- On callback success, retain `Running` and keep accepted publications queued.
- On a returned callback error, preserve the concrete source, commit only the
  selected application to terminal `Failed`, drop the attempted in-flight
  message, clear only its queued inbox, and report the exact queued discard
  count. Peer publications already accepted during the callback remain.
- Reconcile the RFF-REQ-003 wording with the stronger application-consumption,
  self-publication, and one-in-flight evidence required by the accepted ADR.
- Add no dependency, automatic or batch dispatch, thread, executor, scheduler,
  clock, event service, protocol boundary, dynamic topology, or lint waiver.

## Files and components

- `src/runtime.rs`: one crate-private running-callback primitive reused by the
  existing context-free work operation.
- `src/messaging.rs`: internal runtime-inbox dequeue support.
- `src/application_messaging.rs`, `src/messaging_runtime.rs`, and `src/lib.rs`:
  application context, callback, dispatch outcome/error, state snapshot, and
  public exports.
- `tests/message_dispatch.rs`: public FIFO, empty/rejected dispatch,
  self-publication, lifecycle-availability refresh, peer-delivery, and
  callback-error evidence.
- `docs/adr/0012-application-message-dispatch.md`: borrowing, ownership,
  in-flight, publication, and failure decision.
- `AGENTS.md`, this plan, README, architecture, requirements,
  ADR-0004/0009/0010/0011, roadmap, project state, and traceability: truthful
  status and evidence.

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

- A preallocated lifecycle-state snapshot duplicates state transiently during a
  callback. It must be refreshed before every dispatch and remain inaccessible
  to applications except through publication availability.
- Publications are committed as they occur. A later callback error does not
  roll back peer deliveries; only the failed application's queued inbox is
  cleared.
- The in-flight message is attempted work, not a queued discard. It is dropped
  after either callback outcome and excluded from the returned clear count.
- The separate messaging callback expands the unpublished application surface.
  A future common service context should replace it only when more than one
  implemented service demonstrates a coherent shared borrowing shape.
- Per-publication report allocation and inline payload-copy costs are unchanged
  from ADR-0010.

## Safe rollback or stopping point

Stop after one-message dispatch, publish-only context, focused public tests,
ADR-0012, reconciled RFF-REQ-003 traceability, and the complete baseline are
coherent and verified. Do not add automatic dispatch, events, time, scheduling,
configuration, command/telemetry, or protocol work in this run.

## Result

Implementation commit `150b924e6391c9adcc14f23bf21138011b747313`
reached the intended dispatch stopping point. `MessagingRuntime::dispatch_one`
validates `Running`, refreshes lifecycle state, moves at most one oldest message
outside the inbox, and calls a separate `MessagingApplication` with a
publish-only context. Five focused tests bring the complete suite to 38 and
verify empty and rejected dispatch, FIFO one-at-a-time handling, bounded
self-publication and saturation, running-to-stopped availability refresh, and
exact selected-queue clearing with retained peer delivery after callback error.

Independent reviews corrected the public outcome name, stopped-peer evidence,
file mapping, comment width, peer-publication API documentation, and the
snapshot-length invariant. The RFF-REQ-003 wording and traceability now match
the demonstrated behavior. All required Cargo checks, warnings-denied rustdoc,
source widths, reasoned expectations, 49 relative links across 23 Markdown
files, rendered structure for all 13 changed documents, requirement/source
identifiers, tables, and whitespace checks pass. No dependency, thread,
executor, automatic dispatch, event, time, scheduling, protocol, licence
change, or push was added.
