# Stage 4 configuration work-view refresh

Date: **2026-10-07**
Status: **Local acceptance passed; publication and hosted CI pending**

## Objective and context

Verify ADR-0018's current used-byte prefix through the production ordinary-work
boundary. Standalone tests already cover shrinking storage snapshots. The
starting runtime baseline covered an initial short prefix and fixed-length
activation/rollback; the host sample uses one-byte values. Borrow escape and
copied-observation retention already have evidence. The remaining explicit gap
is callback visibility across
exact-bound, shorter, empty, and restored-shorter configuration transitions.

Starting revision: `b5f1f5dd770ffa8e3e310635dd0d03b678b91892`, clean and
equal to refreshed `origin/codex/nightly`; hosted run 37315596231 succeeded.
The initial locked local baseline passes 141 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

## Acceptance criteria

- Work observes revision one with four bytes, revision two with one byte,
  revision three with empty configured content, then restored revision two
  with one byte; no unused or stale bytes appear.
- Empty accepted content remains a configured `Some` view. Exact full copies
  and four callback invocations remain observable after the owner leaves scope.
- Reuse existing observing applications; change no production/API/dependency,
  lint, workflow, requirements, accepted decision, or borrowing experiment.
- Focused/full locked checks, source form, links, rendered documents, and
  complete diff review pass without new exceptions.

## Files and verification

Extend `tests/configuration_runtime.rs`; add
`docs/verification/CONFIGURATION_VIEW_REFRESH_REVIEW.md`; reconcile
README, roadmap, project state, traceability, and this plan.

```text
cargo test --locked --test configuration_runtime
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Set `RUSTDOCFLAGS=-D warnings`. Audit whole-tree physical/comment widths and
expectations separately; render changed Markdown and inspect its structure and
content. Publish after applicable local checks and source/diff review, then
inspect exact-revision hosted checkout, steps, tools, and logs. Unchanged
ADR-0018/0019 experiments and host sample do not trigger separate local probes.

## Risks and safe stopping point

Application-owned `Vec` copies and observation logs lie outside runtime storage
bounds. This is direct ordinary-work prefix-refresh evidence, not a new
configuration service, mutable runtime view, allocation/deallocation measure,
secure erasure, messaging-specific retention proof, or human v0.1 acceptance.
Stop after this checkpoint. If a defect emerges, investigate only the copied
view boundary and preserve unrelated work.

## Completed local checks

The focused configuration-runtime target passes 11 tests; initial and final
locked workspace baselines pass 141. Formatting, all-target check,
warnings-denied Clippy/rustdoc, and whitespace pass. Independent source review
accepts the strengthened regression without a new exception. Source audit
covers 33 Rust files and 12,632 physical lines with zero width findings and
three unchanged fulfilled expectations. All 286 relative links across 59
Markdown files and 105 exact traceability function references resolve.
Author and independent complete-diff/source reviews pass. Six changed documents
pass rendered HTML structure/content inspection; no pixel-level acceptance is
claimed. Publication and exact-revision hosted verification are pending.
