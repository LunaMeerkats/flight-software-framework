# Stage 4 copied message-topology ownership

Date: **2026-10-04**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Verify that both public messaging constructors retain their original topology
following caller configuration and topic buffer mutation and destruction.
ADR-0010 and ADR-0011 already define copied, immutable topology; existing tests
drop configuration arrays but do not mutate their static or still-live topics.
This is the recorded next Stage 4 ownership checkpoint.

Starting revision: `565c9d44a0cc18bf9717c64416e5f265b5dda9e2`, clean and
equal to refreshed `origin/codex/nightly`; hosted run 37019302807 succeeded.
Initial locked local baseline: 135 tests, Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, Clippy 0.1.98.

## Acceptance criteria

- Standalone construction retains original identities, registration-order
  outcomes, capacities, topics, and exact FIFO after caller storage changes.
- Runtime construction retains original capacities and subscriptions through
  lifecycle availability, saturation, stop clearing, and restart reconnection.
- Replacement caller topics acquire no subscribers in either owner.
- Focused and full locked checks, source form, links, rendered documents, test
  references, and full diff review pass without new exceptions or dependencies.

## Files and verification

Extend `tests/message_bus.rs` and `tests/runtime_messaging.rs`. Add
`docs/verification/MESSAGE_TOPOLOGY_OWNERSHIP_REVIEW.md` and reconcile README,
roadmap, project state, traceability, and this plan. Contracts and requirements
remain unchanged; no production/API change or external research is planned.

```text
cargo test --locked --test message_bus --test runtime_messaging
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Set `RUSTDOCFLAGS=-D warnings` for documentation. Review physical/comment
widths and expectations separately from Cargo. Publish after local acceptance,
then inspect hosted CI at the exact published revision.

## Risks and safe stopping point

Topics are copied values with mission-defined equality. Plain-enum tests do not
prove deep isolation for topics referring to shared mutable state, allocation
accounting, issuer validation, dynamic subscription support, or human v0.1
acceptance. Stop after this checkpoint. If a regression fails, investigate the
copy boundary before changing production code; preserve unrelated work and
revert only this run's changes if necessary.

## Completed local verification

The focused targets pass 10 standalone and 12 runtime tests. The full locked
workspace baseline passes 137 tests; formatting, all-target check,
warnings-denied Clippy/rustdoc, and whitespace pass. Fixture extraction and a
shorter local owner binding keep the runtime scenario below the function-size
threshold without a new exception. No production, manifest, lockfile, lint,
workflow, or licence input changed.

Final review passes 33 Rust files and 12,483 physical lines with no physical
or comment-width findings, no block comments, and three unchanged fulfilled
expectations. All 270 relative links in 56 Markdown files resolve; 101 exact
traceability test references resolve to the 137 test functions. Six changed
Markdown documents render with PowerShell and receive HTML structural/content
review; pixel-level acceptance is not claimed. The complete diff and source
form are accepted.

## Publication and hosted verification

Checkpoint `ab9914618a739d4b7c8a46f0e14d7e12c55b6649` was published by
ordinary fast-forward; the remote head matched. Hosted push run
[37128965033](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37128965033)
passes the exact logged checkout, its Windows job, and every configured step:
137 workspace tests, both new regressions, 14 focused host-adapter tests, five
focused sample tests, warnings-denied documentation, executed sample, and both
whitespace checks. Runner 2.337.0 uses `windows-2025-vs2026` image
`20260925.250.1`; Rust/Cargo 1.99.0, rustfmt 1.10.0-stable, and Clippy 0.1.99
match the previous hosted checkpoint. Local tools remain unchanged. This
documentation-only follow-up records completed evidence; later revisions need
their own CI result. Human v0.1 acceptance remains open.
