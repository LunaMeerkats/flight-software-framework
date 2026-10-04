# Stage 4 inline message payload ownership

Date: **2026-10-05**
Status: **Local verification complete; publication and hosted CI pending**

## Objective and context

Verify the two owned-copy boundaries selected by ADR-0010: constructing an
inline message from caller bytes and publishing one accepted copy per inbox.
Existing tests cover payload limits, FIFO, saturation, and copied topology,
but use unchanged publisher messages and literal payloads. This is the next
recorded Stage 4 ownership gap; no production change is expected.

Starting revision: `4cff66c2ee1f9931bd445b2a077e6028c6766729`, clean and
equal to refreshed `origin/codex/nightly`; hosted run 37129162904 succeeded.
The initial locked local baseline passes 137 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

## Acceptance criteria

- Empty, short, and exact-limit constructed payloads retain their original
  bytes and logical lengths after the source vector is overwritten and dropped.
- Two subscribers retain exact FIFO copies after the publisher binding is
  replaced and leaves scope. Consuming and replacing one returned message
  leaves the peer's queued copies unchanged.
- Existing payload-boundary, saturation, lifecycle, and dispatch tests pass.
- Focused/full locked checks, source form, links, rendered documents, and full
  diff review pass without new exceptions, APIs, or dependencies.

## Files and verification

Extend `tests/message_bus.rs`; add
`docs/verification/MESSAGE_PAYLOAD_OWNERSHIP_REVIEW.md`; reconcile README,
roadmap, project state, traceability, and this plan. Requirements, accepted
decisions, and source-register findings remain unchanged.

```text
cargo test --locked --test message_bus
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Set `RUSTDOCFLAGS=-D warnings` for rustdoc. Audit physical/comment widths and
existing expectations separately. Inspect generated Markdown HTML structure
and content; do not claim pixel-level visual acceptance. Publish after local
acceptance, then inspect exact-revision hosted checkout, steps, tools, and logs.

## Risks and safe stopping point

Messages expose immutable payloads: binding replacement exercises value
independence, not an in-place mutation API. Plain enum topics do not prove
isolation of shared referents. Standalone dequeue observations do not add
runtime-specific dispatch evidence, allocator accounting, secure erasure,
concurrency, or human v0.1 acceptance. Stop after this checkpoint. If tests
reveal a defect, investigate only the owned-copy boundary and preserve unrelated
work; revert only this run's implementation if a coherent result is impossible.

## Completed local verification

The focused target passes 12 tests; the final locked workspace passes 139.
Formatting, all-target check, warnings-denied Clippy/rustdoc, and whitespace
pass. The first format check requested one chained-call reflow; stable rustfmt
applied it, and subsequent checks pass. Both new tests remain below the
function-size threshold with no exception. Independent source review accepts
the two distinct copy observations and their evidence limits.

Final inventory covers 33 Rust files and 12,545 physical lines: zero
physical/comment-width findings, no block comments or allows, three unchanged
fulfilled expectations. All 275 relative links across 57 Markdown files and
both new exact traceability references resolve. Six changed documents render
with PowerShell; HTML structure/content inspection passes without pixel-level
acceptance. The temporary inventory is a review aid, not a new adopted checker.
Complete diff/source review passes. No production, public API, Cargo input,
licence, lint configuration, or workflow change is made. Conditional
ADR-0018/0019 probes and sample-only reruns are unchanged and not run locally.
