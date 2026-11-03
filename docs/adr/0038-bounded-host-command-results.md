# ADR-0038: Bounded host command results

- Status: Implemented; local checks/complete Codex review pass, publication pending
- Date: 2026-11-04
- Scope: Serial correlated-command consumer under RFF-LIB-REQ-013

## Context

ADR-0037 correlates pending-operation controls but deliberately supplies no
feedback. A busy New is consumed and unmatched controls leave state unchanged.
Delivered means only command enqueue. The copied test observation cannot drive
host policy, so a host lacks a supported way to decide whether to resubmit.
Root Codex and independent consumer_assessment select this recorded adoption gap.

## Decision

Compose one capacity-one typed result mailbox with existing public APIs. The
producer and serial host share an `Rc<Cell<Option<CommandResult>>>`. Record the
exact command after successful handling, with reply acceptance flags, busy
pending ID, matching cancellation or no matching operation. Capture flags before
clearing completed pending state. The host explicitly takes the result and uses
one bounded turn: skip producer dispatch if occupied; otherwise dispatch at most
one command through its existing owner and preserve the complete error wrapper.
The gate precedes lifecycle validation; ResultPending is no lifecycle assertion.

The callback also checks the slot before parsing or effects. Direct dispatch
bypassing the gate returns ResultOccupied, retains the old result and uses
ordinary terminal failure/selected-inbox cleanup. Malformed or routing failures
return their runtime errors and produce no fresh success result; earlier peer
effects survive. Stop/restart retains mailbox and pending state; cleared
undispatched commands have no result. The old seven test bodies remain regression
histories, with their test helper explicitly consuming each result; new tests
exercise operational feedback.

## Alternatives considered

- Shared typed single slot: selected. Fixed storage and serial host gating
  avoid result loss or overwrite without new framework interfaces.
- Bus-routed acknowledgements: defer. Publication may be full/unavailable and
  needs its own pending-result, retry and duplicate-effect contract. No current
  application subscriber requires that transport.
- Generic framework callback return values: defer. It would expand the
  callback/runtime API before mission composition has been shown insufficient.
- Drop/overwrite the oldest result or fail when ordinarily full: reject as
  normal policy; the host can preserve results and queued inputs by gating.
- Persistent/multiple-issuer identity: defer; it does not answer whether this
  command executed and no current consumer needs it.

## Evidence and consequences

Official Rust Cell get/set/take and Rc documentation inform copied shared
single-thread state (SRC-RUST-COMMAND-RESULTS). The mailbox capacity, gate and
outcome vocabulary are local choices. No upstream code is reused. This adds one
consumer result record and one fixed host/app shared allocation at construction;
commands add no queue or growing result history. Existing command inbox bounds
and at most two selective publication attempts per applicable command remain.
The [guide](../CORRELATED_COMMANDS.md) and
[focused review](../verification/COMMAND_RESULTS_REVIEW.md) record 14 focused
and identical independent tests, the 243-test locked baseline, matching
bytes/hashes/metadata and root/independent complete source/document acceptance.
Publication and exact hosted verification remain pending.

## Risks and revisit conditions

Host access to the shared cell is trusted mission composition; it can discard a
result or bypass the gate. This is a single-thread consumer, not synchronization
or a new concurrency commitment. Results acknowledge only the stated local
effect; accepted subscriber copies can later be cleared or fail processing.
Repeated New can duplicate effects, and control results are not proof of
eventual delivery or durable cancellation. Retain ADR-0037 session-ID limits.
Revisit for an actual bus-result subscriber, concurrent dispatch, persistent
results, multiple outstanding commands or a demonstrated missing public API.
Separate consequential authorization gates remain unchanged.
