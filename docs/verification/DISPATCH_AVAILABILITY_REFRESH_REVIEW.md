# Dispatch availability refresh review

Date: **2026-10-08**
Status: **Local acceptance passed; publication and hosted verification pending**

## Contract and coverage reconciliation

[ADR-0012](../adr/0012-application-message-dispatch.md) refreshes a preallocated
lifecycle-state snapshot from the owning runtime before each accepted
dispatch. Callback publication uses that stable snapshot during the
synchronous call. [ADR-0011](../adr/0011-runtime-owned-message-availability.md)
makes only running endpoints available, clears an inbox on stop, and reconnects
it empty after successful in-place restart. Both contracts remain unchanged.

The existing `dispatch_refreshes_peer_availability_after_lifecycle_change`
regression covers a running peer becoming stopped between publisher callbacks.
The `stop_clears_one_inbox_and_restart_reconnects_it_empty` regression covers
external publication after restart. Host tests also cover stop/restart, but
do not perform a publisher callback while the peer is stopped to populate the
dispatch snapshot with unavailability. The remaining gap is callback
publication after that stopped snapshot must be replaced on the next dispatch.

## Executed observation

The existing `tests/message_dispatch.rs` regression now runs through:

| Peer state at publisher callback | Input payload | Reply outcome | Peer queued replies |
| --- | --- | --- | --- |
| Running initially | 1 | Delivered | One; subsequently cleared by stop |
| Stopped after clearing the initial reply | 2 | Unavailable | Zero |
| Running after in-place restart | 3 | Delivered | One, payload 4 |

The final peer callback observes exactly the new reply's application
identity, topic, and payload. An additional peer dispatch finds the inbox
empty without adding a callback trace. This checks both directions of refresh
and the absence of delayed replay of the unavailable publication.

The test checks exactly three full publisher reports: `Complete`,
`WhollyUndelivered`, then `Complete`, each containing only the peer's exact
delivery outcome. A focused report assertion helper keeps the chronological
scenario below the function-size threshold without a new lint expectation.
No production, API, dependency, requirement, ADR, or workflow change is needed.

## Evidence limits

This is controlled serial behavior with capacity-one inboxes and copied enum
topics. It does not measure allocation, scheduling latency, concurrency,
application-internal cleanup, or generic referent isolation. Successful restart
from `Stopped` does not recover terminal `Failed` applications. No automatic
retry/dispatch, external transport, identity provenance, API stabilization, or
human v0.1 acceptance is added.

## Verification

The initial locked baseline passes 141 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. The focused message-dispatch target
passes all six tests. The final locked baseline passes 141 tests with
formatting, all-target check, warnings-denied Clippy/rustdoc, and whitespace
passing. Executed commands:

```text
cargo test --locked --test message_dispatch
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Whole-tree source audit covers 33 Rust
files and 12,666 physical lines with zero width findings and three unchanged
fulfilled expectations. All 291 relative links across 60 Markdown files and
105 exact traceability function references resolve. Six changed documents pass
rendered HTML structure/content inspection; author and independent complete-
diff/source reviews pass. No pixel-level acceptance is claimed. Review aids/logs under
`target/review-2026-10-08` are ignored and do not adopt a checker.

Unchanged ADR-0018/0019 experiments, host adapters/sample, and workflow do not
trigger their separate local probes. The full workspace suite includes both
host test targets. Exact-checkout hosted acceptance remains pending.
