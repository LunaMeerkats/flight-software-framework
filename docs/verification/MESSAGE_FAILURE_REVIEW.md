# Returned-message failure review

Date: **2026-09-15**
Status: **Complete: published checkpoint has successful exact-revision CI**

## Reviewed boundary and decision

This is one Stage 4 lifecycle/dispatch checkpoint, based on source at
`ee63b4a7a7ec6c556bc76536298211fd3ff0d253` and the test-only changes in this
increment. It reviews `MessagingRuntime::dispatch_one`, its runtime callback
owner, the publish-only context, and selected-inbox cleanup against
[ADR-0012](../adr/0012-application-message-dispatch.md) and
[ADR-0011](../adr/0011-runtime-owned-message-availability.md).

No production contract defect was found. The existing failure test observed
three pending peer records but consumed only the first. Its error assertion
followed the second source hop from a separately constructed expected error.
This increment strengthens those observations and adds a focused post-failure
availability regression. Retain immediate publication and selected cleanup;
transactional publication, production fault hooks, and a new testing framework
would change or enlarge the established boundary without a demonstrated need.
ADR-0012's historical outcome list is clarified to match the existing source:
the callback closure releases the in-flight message before state commitment.
This precision correction changes neither runtime code nor public behavior.

Repository contracts are the authoritative sources for this checkpoint. No
new external research, upstream claim, source reuse, dependency, public API,
or architecture decision is introduced. RFF-REQ-003 retains its meaning;
RFF-REQ-008's separate returned-work event boundary is not broadened.

## Source-order and ownership review

1. `dispatch_one` validates identity and `Running` state before touching the
   inbox, then refreshes every preallocated dispatch-state slot from the
   runtime. Rejected calls do not invoke the callback or clear queues.
2. `Runtime::invoke_running` borrows the retained application. The closure
   removes only the selected inbox's oldest message and passes a borrow to
   `handle_message`. Empty inboxes return without a callback.
3. Context publication uses that serial snapshot and the ordinary bounded bus.
   It reserves its complete report before enqueue and reports each matching
   destination in topology order. Accepted publications take effect at once.
4. On a returned callback error, the closure releases the in-flight record,
   the runtime commits the selected state to `Failed`, and the messaging owner
   clears only that endpoint's remaining queue. The discard count excludes the
   attempted input but includes accepted self-publications and older queued
   records. Peer effects remain accepted.
5. The caller receives `MessagingOperationError<MessageDispatchError<E>>` with
   the concrete source and exact clear count. Later dispatch refreshes the
   snapshot, so a healthy callback sees the failed endpoint as unavailable.

Temporal ordering in this list is source-inspection evidence. Public tests
observe callback traces and state after return; they do not inspect the
exclusively borrowed owner during cleanup or lifecycle commitment.

The retained operational queue bound remains the sum of configured inbox slots
plus one serial in-flight delivery. A dispatch snapshot has one state per
application; each returned publication report has at most one outcome per
matching endpoint. Caller/application-retained messages, reports, error data,
topic effects, allocator overhead, and stack use are outside those bounds.
This review neither proves allocation-free dispatch nor injects reservation
failure. The [constructor review](MESSAGING_CONSTRUCTION_REVIEW.md) retains
its separate capacity-overflow evidence and allocator-error limits.

## Public regression evidence

- `callback_error_clears_selected_queue_but_retains_peer_publication` checks
  the actual returned source chain and consuming operation-error recovery,
  exact two-record selected cleanup, and accepted self/peer publication.
  It consumes the complete peer sequence:
  `Input(1)`, `Input(2)`, then `Reply(2)`, including exact identity, topic,
  payload, remaining counts, and empty-without-another-callback behavior.
  Redispatching the failed application leaves its observations unchanged.
- `dispatch_refreshes_peer_availability_after_message_failure` checks that
  callback publication after a peer's returned message error observes the
  failed destination as unavailable while the healthy endpoint still accepts
  delivery. It distinguishes a fresh snapshot from the preceding callback's
  `Running` snapshot.

These controlled fixtures use a small copied application error. They do not
newly establish non-`Copy` error destruction/ownership, rollback of external
effects, configuration retention during message failure, event reporting for
message errors, panic/hang containment, concurrency, or allocator exhaustion.

## Verification and continuation

Initial required locked baseline: 117 passing tests. The focused command
`cargo test --locked --test message_dispatch` passes all six tests; the final
workspace suite passes 118 tests. These local commands complete successfully:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Local Rust/Cargo are 1.98.0,
rustfmt is 1.9.0-stable, and Clippy is 0.1.98. Source widths, relative links,
exact traceability references, and rendered source-content comparison pass.
All eight changed documents pass browser DOM/layout and screenshot inspection
at 1280px without page overflow or console warnings/errors. Independent
complete-diff review found no blocking defect; its source-order clarification
is incorporated in ADR-0012. No changed
adapter, sample, workflow, or ADR-0018/0019 probe triggers separate local
commands. The complete workspace suite still includes both host test targets.

Next select another bounded Stage 4 contract, such as finite schedule
construction or event-queue resource/failure behavior, after reconciling its
existing tests. The broader audit and human v0.1 entry-point/architecture
acceptance remain open. Revisit this checkpoint if dispatch becomes concurrent,
publication gains transactional semantics, lifecycle availability changes, or
the callback receives broader authority.

## Published checkpoint evidence

Commit `63b149fcaeaae5c9842d75a5493e8809da6a9eba` contains the strengthened
failure test, new availability regression, and reviewed checkpoint. Ordinary
fast-forward publication succeeded and the remote head matched. On 2026-09-14
UTC (2026-09-15 Sydney),
[hosted run 34901286073](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/34901286073)
completed successfully for that exact push/checkout revision: one Windows job
and all 20 steps. Logs confirm 118 workspace tests, both targeted regression
names, 14 focused adapter tests, five focused sample tests, the sample, and
every configured baseline command.

The runner is 2.337.0, image `windows-2025-vs2026` version `20260907.229.1`
(requested label `windows-2025`). Actual rustc/Cargo are 1.98.1, rustfmt is
1.9.0-stable, and Clippy is 0.1.98. These match the previously reviewed hosted
environment; local Rust/Cargo remain 1.98.0. The all-target hosted baseline and
whole-tree source review preserve the existing policy without a new waiver.
Logs are retained locally under `target/review-2026-09-15`.

This documentation-only follow-up records completed evidence without changing
Rust, Cargo, workflow, or lint inputs. It does not establish CI success for a
later commit, human v0.1 acceptance, or broader scope.
