# Inline message payload ownership review

Date: **2026-10-05**
Scope: **ADR-0010 construction and standalone accepted-delivery copies**

## Inspected boundary and decision

[ADR-0010](../adr/0010-bounded-message-routing-core.md) selects an inline
byte array with a private logical length and one copied message per accepted
destination. `Message::try_new` copies the supplied slice into its private
array. `MessageBus::publish_with_availability` passes `*message` into each
matching endpoint; `enqueue` stores that value in the bounded inbox.
[ADR-0011](../adr/0011-runtime-owned-message-availability.md) reuses this core.

Existing payload-boundary, FIFO, saturation, and dispatch tests do not overwrite
the source buffer or replace a publisher message while its deliveries remain
queued. This checkpoint adds public regressions for those two copy boundaries
in `tests/message_bus.rs`. Production behavior is already correct. It extends
RFF-REQ-003 evidence without changing requirements, APIs, dependencies, or
accepted architecture; no external research or source-register change is needed.

## Executed public observations

`inline_message_owns_payload_after_source_buffer_changes_and_drop` covers
empty, two-byte, and exact four-byte payloads, including an embedded zero byte.
After construction, it overwrites the source vector, clears it, and writes
different bytes. The message exposes exactly the original logical slice both
while the altered vector is alive and after that source storage leaves scope.
Its original Command topic and four-byte capacity remain observable.

`queued_copies_survive_publisher_replacement_and_independent_consumption`
publishes an exact-limit Command and a shorter Command through one reused
publisher binding into two capacity-two subscriber inboxes. Each publication
reports Complete and two Delivered outcomes in original registration order.
The binding is then replaced with an empty Unrouted message and leaves scope.
Both inboxes still contain two records.

The test consumes the first subscriber's oldest record and compares the entire
Message value with the original. Replacing that returned binding leaves its
next delivery and the peer's two queued records intact. Both subscribers then
return the exact original FIFO values and become empty. Complete equality also
compares private stored length and inline bytes; this is not a secure-erasure
observation. The short payload exposes no unused array suffix through `payload`.

## Evidence limits

The public payload is immutable. Replacing a binding exercises owned-value
independence, not mutation through a payload API. These standalone tests do not
independently execute runtime dispatch after publisher storage changes; existing
runtime/dispatch regressions remain their evidence. Plain enum topics do not
prove deep isolation of shared referents. Caller-retained messages/reports remain
outside the bus's storage bound. Allocator exhaustion, allocation counts,
whole-process memory, secure erasure, concurrency, API stabilization, and human
v0.1 acceptance remain unverified or out of scope.

## Verification

The initial locked baseline passes 137 tests at
`4cff66c2ee1f9931bd445b2a077e6028c6766729`. Focused
`cargo test --locked --test message_bus` passes all 12 tests, including both
new regressions. Stable rustfmt reformatted one chained call; the subsequent
format check and all-target warnings-denied Clippy pass without an exception.

```text
cargo test --locked --test message_bus
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. The final locked workspace passes
139 tests; formatting, all-target check, warnings-denied Clippy/rustdoc, and
whitespace pass. Source review covers 33 Rust files and 12,545 physical lines
with zero physical/comment-width findings, no block comments or allows, and
three unchanged fulfilled expectations. Both new tests need no waiver.

All 275 relative links across 57 Markdown files and both new exact traceability
references resolve. Six changed documents render with PowerShell and pass HTML
structure/content inspection; pixel-level acceptance is not claimed. Complete
diff and independent source review pass. The temporary inventory is a review
aid, not an adopted checker. Conditional ADR-0018/0019 probes, workflow inputs,
and sample source are unchanged; their extra local commands were not run.

## Publication and hosted result

Checkpoint `97e590f25d8a9845bb40452a50741639beea3999` was published by
ordinary fast-forward; the remote head matched. Hosted
[push run 37204799708](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37204799708)
uses the unchanged workflow at that revision. The `Record checkout` log contains
the exact checkpoint SHA. Its Windows job and every configured step pass:
139 workspace tests including both new regressions, 14 focused host-adapter
tests, five focused sample tests, warnings-denied documentation, the executed
sample, and both whitespace checks.

Runner 2.337.0 uses `windows-2025-vs2026` image `20260925.250.1`. Compiler
1.99.0 (`b940084d7`, 2026-09-28), Cargo 1.99.0 (`5f94df478`, 2026-08-27),
rustfmt 1.10.0-stable, and Clippy 0.1.99 match the previous hosted checkpoint.
Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
The existing source-policy re-audit remains in the CI baseline; no toolchain or
policy change is made here. This documentation follow-up records exact completed
evidence; later revisions require their own CI result. Human acceptance remains
open.
