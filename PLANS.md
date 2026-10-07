# Stage 4 dispatch availability refresh after peer restart

Date: **2026-10-08**
Status: **Local acceptance passed; publication and hosted verification pending**

## Objective and context

Verify ADR-0012's per-dispatch lifecycle snapshot through a complete peer
stop/restart cycle. The existing dispatch regression covers a running peer
becoming unavailable after stop. External publication already covers restart
reconnection, but does not populate and then refresh a stopped dispatch
snapshot. Extend the existing callback-publication scenario to close that gap.

Starting revision: `b53f680ad938da6913cb774b204816902074322a`, clean and
equal to refreshed `origin/codex/nightly`; hosted run 37469088813 succeeded.
The initial locked local baseline passes 141 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

## Acceptance criteria

- Three publisher callbacks observe the peer as delivered, unavailable after
  stop, and delivered after successful in-place restart.
- Stop discards the initial queued reply. The stopped publication queues
  nothing; restart reconnects an empty inbox; the next callback delivers only
  the new reply, with exact topic/payload/application identity and no replay.
- Reuse the existing dispatch fixtures and test; retain production behavior,
  public API, dependencies, requirements, ADRs, lint policy, and workflow.
- Focused/full locked checks, source form, links, rendered documents, and
  complete diff review pass without a new exception.

## Files and verification

Extend `tests/message_dispatch.rs`; add
`docs/verification/DISPATCH_AVAILABILITY_REFRESH_REVIEW.md`; reconcile
README, roadmap, project state, traceability, and this plan.

```text
cargo test --locked --test message_dispatch
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
ADR-0018/0019 experiments, host sample, and workflow do not trigger extra probes.

## Risks and safe stopping point

This observes synchronous callback publication against refreshed lifecycle
state under controlled inputs. It does not add automatic dispatch, retry,
concurrency, failed-application recovery, identity provenance, real-time
behavior, or human v0.1 acceptance. Stop after this checkpoint. If a defect
emerges, investigate only the snapshot refresh boundary and preserve unrelated
work.

## Completed local checks

The focused message-dispatch target passes six tests; initial and final locked
workspace baselines pass 141. Formatting, all-target check, warnings-denied
Clippy/rustdoc, and whitespace pass. The chronological regression uses one
focused publication-report assertion helper and adds no lint expectation.
Whole-tree audit covers 33 Rust files and 12,666 physical lines with zero
width findings and three unchanged fulfilled expectations. All 291 relative
links across 60 Markdown files and 105 exact traceability function references
resolve. Six changed documents pass rendered HTML structure/content inspection;
author and independent complete-diff/source reviews pass. No pixel-level
acceptance is claimed. Publication and exact-checkout hosted CI are pending.
