# Event-record ownership review

Date: **2026-10-06**
Scope: **Existing ADR-0013 copied emission and caller-owned dequeue**

## Decision and inspected boundary

Retain the current queue and strengthen explicit public ownership observations.
No production defect was found at starting revision
`d5ddb7e0869fe953ff54213d4f5311b21b9124e2`. This checkpoint supplements
RFF-REQ-005 and [ADR-0013](../adr/0013-bounded-structured-event-queue.md)
without changing their contracts, production code, or public APIs.

`EventQueue::emit` borrows the producer record and appends `*event` only when
the logical capacity permits it. `dequeue` removes and returns an owned value.
All four metadata fields are stored in the event value. These are inspected
source facts; no external technical question requires new research or an ADR.

The six earlier standalone tests cover metadata, FIFO, saturation, reservation
errors, and repeated slot reuse with unchanged producer bindings. The
`replay_scenario` fixture in `tests/scheduled_work.rs` already emits loop-local
records and returns their dequeued values after an emptied queue is destroyed.
This checkpoint targets the remaining explicit producer replacement and
nonempty-queue destruction observations, rather than claiming ownership was
wholly untested. The earlier [resource review](EVENT_QUEUE_REVIEW.md) remains
separate evidence for construction and saturation.

## Executed public observations

Two new regressions in `tests/event_queue.rs` exercise distinct boundaries:

- `queued_records_survive_producer_replacement_and_scope_exit` emits two
  events through one repeatedly replaced producer binding. The accepted
  records differ in source, severity, identifier, and timestamp. A third
  replacement is rejected while full and remains caller-visible. After the
  producer leaves scope, full-record equality verifies the original FIFO,
  including decreasing timestamps; the rejected replacement never appears.
  The queue reaches exhaustion and retains its logical capacity of two.
- `dequeued_record_survives_slot_reuse_and_queue_destruction` retains an
  application error record, refills its freed logical slot, and consumes the
  older remaining record. The queue is then destroyed with the replacement
  still pending. Equality and field access after that scope verify the retained
  record's original source, severity, identifier, and elapsed timestamp.

Both tests use the existing plain mission enum and immutable public record API.
They remain below the function-size threshold without a new expectation.

## Evidence limits

Producer binding replacement demonstrates value independence; it does not
exercise an in-place event mutation API. Arbitrary `Copy` identifiers can
carry shared referents; plain enum observations do not prove deep isolation
of those referents. Queue destruction is a scope/ownership observation, not
instrumented deallocation, secure erasure, allocator accounting, persistence,
or a whole-process memory bound. Caller-retained records remain outside the
queue's logical record bound.

This adds no event filtering, guaranteed delivery, automatic retry, concurrency,
clock/identity provenance, broader runtime event production, or fault
containment. Human v0.1 architecture acceptance remains open.

## Verification

The initial locked baseline passes 139 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. The focused target passes eight tests;
the final locked workspace baseline passes 141. Formatting, all-target check,
warnings-denied Clippy/rustdoc, and whitespace pass. Stable rustfmt accepted
the added source without a new waiver. Author and independent source/diff
reviews find no blocker. The final inventory covers 33 Rust files and 12,617
physical lines with zero width findings, no block comments or allows, and
three unchanged fulfilled expectations. All 280 relative links across 58
Markdown files and 105 exact traceability function references resolve.
Six changed documents pass rendered HTML structure/content inspection without
claiming pixel-level visual acceptance. Temporary aids and logs remain under
`target/review-2026-10-06`; no checker or new gate is adopted.

```text
cargo test --locked --test event_queue
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Host adapters/sample, workflow,
and ADR-0018/0019 probes are unchanged, so their separate local commands are
not triggered. The full suite includes both host test targets.

## Publication

Checkpoint `814adcafe892aad3ecf960054ec1c6ff64ac98c7` was published by
ordinary fast-forward and the remote head matched. Hosted push run
[37315061868](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37315061868)
passes the exact logged checkout, the unchanged workflow's single Windows job,
and all 20 steps. Logs show both new regressions among 141 workspace tests,
14 separately focused adapter tests, five sample tests, warnings-denied
documentation, the executed sample, and both whitespace checks. Focused
reruns do not add to the workspace total.

Runner 2.337.0 uses `windows-2025-vs2026` image `20260925.250.1`.
Rust/Cargo 1.99.0, rustfmt 1.10.0-stable, and Clippy 0.1.99 match the prior
checkpoint; local tools remain 1.98.0/1.9.0-stable/0.1.98. The existing
source-policy re-audit remains applicable without a new waiver or adaptation.
Full JSON and logs are under `target/review-2026-10-06`.

This documentation follow-up records exact checkpoint evidence without
changing Rust or CI inputs. Later revisions need their own CI result; source
publication does not complete human v0.1 architecture acceptance.
