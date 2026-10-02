# Returned-stop failure and peer dispatch review

Date: **2026-10-03**
Scope: **ADR-0007/0011 cooperative stop failure under the messaging owner**

## Inspected boundary and decision

[ADR-0007](../adr/0007-operation-specific-owned-stop-boundary.md) commits
terminal `Failed` when an application returns a concrete stop error.
[ADR-0011](../adr/0011-runtime-owned-message-availability.md) then clears only
that application's queued deliveries before returning. Peers retain their
lifecycle state and inbox contents. Their message callbacks remain governed by
[ADR-0012](../adr/0012-application-message-dispatch.md).

The existing `returned_stop_error_clears_only_the_failed_endpoint` test
observes one discarded delivery, peer counts, and later publication. It does
not observe the retained peer payloads or later peer message callbacks. This
checkpoint supplements that evidence with one chronological public scenario
in `tests/messaging_stop_failure.rs`; it retains the existing production API
and stop policy.

## Executed public observations

`returned_stop_failure_preserves_peer_fifo_and_terminal_callback_gates`
executes the following scenario through the public messaging-owner API.

The selected application and peer start with inbox capacities two and three.
Two distinct messages fill the selected inbox and occupy two peer slots. A
returned stop error retains the exact application identity and concrete
cause, reports two discarded deliveries, leaves the selected application
`Failed` with no queued messages, and leaves the peer `Running` with both older
messages.

A later publication reports `Unavailable` for the failed subscriber and
`Delivered` for the peer in registration order. Failed stop, restart, ordinary
work, and message dispatch each return the corresponding lifecycle error
with zero discard and no further callback invocation. The peer dispatches
exactly the two retained messages followed by the new message, one per call,
then returns `InboxEmpty` without another callback and completes ordinary work.

The returned stop error also exposes the exact standard source chain
`MessagingOperationError -> RuntimeStopError -> MissionStopError`, terminating
at the concrete mission cause. The observed messages have distinct payloads,
including one- and two-byte lengths; each message callback observation includes
the complete message and selected application identity.

## Evidence limits

These observations concern cooperative returned errors and the owner's queued
message records. They do not prove application-internal cleanup, external
resource disposal, secure payload erasure, panic or hang containment, retry,
recovery, guaranteed delivery, API stabilization, or human v0.1 acceptance.
Application identity remains caller-scoped. No event-emission path is added to
stop.

## Verification

The initial locked baseline passes 134 tests at
`2b138e11415695129e6162954600911f71ec2eaf`. The focused target passes one
test and the final locked baseline passes 135 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. Formatting, all-target checking,
warnings-denied Clippy/rustdoc, and whitespace checks pass. No production
defect was found, and no lint exception or dependency was introduced.

```text
cargo test --locked --test messaging_stop_failure
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc runs with `RUSTDOCFLAGS=-D warnings`. Source form, expectations,
relative Markdown links, changed rendered documents, exact test references,
and the complete diff are reviewed separately from hosted CI.

The whole-tree source review passes 33 Rust files with zero physical/comment
width findings, no block comments, and three unchanged fulfilled expectations.
The document audit passes 55 Markdown files and 261 relative links. Six
changed documents render successfully in PowerShell and receive structural
review; pixel-level visual acceptance is not claimed. Publication and hosted
verification remain pending.
