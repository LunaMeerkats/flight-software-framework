# Messaging construction resource review

Date: **2026-09-14**
Status: **Autonomous constructor checkpoint; wider Stage 4 review remains open**

## Scope and reviewed input

Reviewed `553fb50832a2aa4948944d051575cc5fb704ea26` on `codex/nightly`:
[message constructors](../../src/messaging.rs), the
[messaging owner](../../src/messaging_runtime.rs), and their
[standalone](../../tests/message_bus.rs) and
[owner](../../tests/runtime_messaging.rs) public tests. The requirements and
[ADR-0010](../adr/0010-bounded-message-routing-core.md),
[ADR-0011](../adr/0011-runtime-owned-message-availability.md), and
[ADR-0012](../adr/0012-application-message-dispatch.md) supply the contracts.
Runtime source and public APIs are unchanged by this checkpoint.

This is input to human v0.1 review, not that acceptance or a complete resource,
allocator, failure-path, or public-API audit. Existing publication, dispatch,
and lifecycle tests remain regression evidence; their full interaction review
is not closed here.

## Constructor findings

No implementation mismatch was found at this boundary. Source inspection
establishes the following order and ownership rules:

1. The standalone bus rejects empty topology, then validates each position's
   identity, positive capacity, and unique topics before reserving storage.
   Attachment first checks exact inbox count and registered states; its fresh
   bus assigns positional identities and validates capacities/topic uniqueness.
2. Both bus constructors reserve endpoint storage, then build endpoints in
   order by reserving/copying topics and reserving inbox slots. A later error
   returns no partial bus; constructed temporary endpoints leave scope. Tests
   do not instrument deallocation or establish secure erasure.
3. The owner reserves one dispatch-state entry per application after the bus
   succeeds. Any returned construction error carries the original runtime.
   Success freezes registration and exposes neither inner owner mutably.
   Runtime capacity can exceed the frozen registered count.

| Storage | Logical retained bound and ownership |
| --- | --- |
| Endpoints | One per supplied configuration; one per registered application through the owner |
| Topics | One copied immutable topic slice per endpoint; duplicates rejected |
| Inbox deliveries | At most each endpoint's configured positive slot count; each slot includes the full inline payload array |
| Dispatch states | One pre-reserved state per registered application through the owner |
| Publication report | At most one outcome per matching endpoint per call; returned storage becomes caller-owned |

Enqueue checks the configured logical capacity rather than allocator capacity.
Summed slot limits describe retained queued records, not total process bytes.
Caller-held inputs, reports, dequeued messages, referenced topic data, allocator
overhead, stack temporaries, and application/topic effects need separate budgets.

## Added evidence and limits

- `oversized_later_inbox_returns_exact_reservation_error` supplies a valid
  first inbox and `usize::MAX` slots for the second. It observes the exact
  `InboxStorageAllocationFailed` identity and requested capacity.
- `oversized_later_inbox_preserves_runtime_for_corrected_attachment` exercises
  that failure through the owner. It observes both registered identities and
  runtime count/capacity, consumes the returned runtime, attaches corrected
  capacities with empty inboxes, and demonstrates retained application-specific
  start failure plus healthy peer publication and work.

Both fixtures use a small nonzero-sized message; the requested storage cannot
be represented. This is capacity overflow, not allocator exhaustion, pressure
on available RAM, or failure after mission execution. The owner fixture has no
configuration table and does not newly prove its lineage preservation during
failed messaging attachment.

`EndpointStorageAllocationFailed`, `TopicStorageAllocationFailed`,
`DispatchStateStorageAllocationFailed`, and `ReportAllocationFailed` have
source-inspection evidence here, not injected-failure execution evidence.
Report reservation precedes enqueue in source; this run does not force it to
fail or establish general allocation-free operation.

## Source treatment and decision

Official Rust 1.98.1
[VecDeque reservation documentation](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.try_reserve_exact),
accessed 2026-09-14, describes returned reservation errors and possible excess
allocator capacity. `SRC-RUST-DEQUE-RESERVATION` in the
[source register](../research/SOURCES.md) records its treatment. Local Rust
1.98.0 execution separately verifies the concrete fixtures.

Retain the existing typed-error and ownership policy. Add public overflow
regressions instead of global allocator fault injection, which needs an
isolated harness and separate review to avoid interfering with unrelated
allocations. No new ADR, dependency, suppression, threshold, or production
failure-injection hook is needed to verify these existing contracts.

## Verification and continuation

The initial locked baseline passes 115 tests. The focused command
`cargo test --locked --test message_bus --test runtime_messaging` passes eight
standalone and 11 owner tests. The final workspace suite passes 117 tests.
The following local commands completed successfully:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. The complete diff, 32-file source
width/expectation audit, relative links, exact traceability references, and
six changed rendered documents pass source/content and browser DOM/layout
inspection. Screenshot capture timed out; pixel-level inspection is not
claimed. The
[completed local plan](https://github.com/LunaMeerkats/flight-software-framework/blob/4db1a8074b014abc1c9b5cf3df5daef3cfca2b55/PLANS.md)
records the adaptation and temporary aids at that immutable revision.

Next select one remaining lifecycle/dispatch interaction boundary or another
service's resource/failure contract. Revisit these findings if topology
mutation, allocation ownership, message representation, configuration attachment,
or concurrency changes. Human entry-point/architecture acceptance and the wider
Stage 4 audit remain open. No operational or compatibility claim follows.

## Published checkpoint evidence

Commit `4db1a8074b014abc1c9b5cf3df5daef3cfca2b55` contains both tests and
the reviewed checkpoint. Ordinary fast-forward publication succeeded, and
the remote head matched. On 2026-09-13 UTC (2026-09-14 Sydney),
[hosted run 34780028767](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/34780028767)
completed successfully with one job/all 20 steps. Job logs confirm that exact
checkout, both new regression names, 117 workspace tests, 14 adapter tests,
five sample tests, the sample executable, and every remaining baseline step.

The runner is 2.337.0, image `windows-2025-vs2026` version `20260907.229.1`
(requested label `windows-2025`). Actual rustc/Cargo are 1.98.1, rustfmt is
1.9.0-stable, and Clippy is 0.1.98. These match the prior hosted environment;
local Rust/Cargo remain 1.98.0. The whole-tree source-form audit and all-target
hosted checks preserve the existing policy without a new waiver or toolchain
pin. Logs are retained locally under `target/review-2026-09-14`.

This evidence-only follow-up changes no Rust source. The recorded run does not
establish CI success at a later commit, human acceptance, or wider scope.
