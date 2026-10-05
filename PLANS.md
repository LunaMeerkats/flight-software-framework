# Stage 4 event-record ownership

Date: **2026-10-06**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Verify ADR-0013's copied emission and caller-owned dequeue boundaries. Existing
standalone tests cover metadata, capacity, FIFO, saturation, and repeated reuse
with unchanged producer bindings. Scheduled replay indirectly covers loop-local
producer scope exit and a drained queue's destruction. The remaining explicit
gap is producer binding replacement and retained full records after a nonempty
queue is destroyed. No production change is expected.

Starting revision: `d5ddb7e0869fe953ff54213d4f5311b21b9124e2`, clean and
equal to refreshed `origin/codex/nightly`; hosted run 37204967826 succeeded.
The initial locked local baseline passes 139 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

## Acceptance criteria

- Accepted records retain every structured field and exact FIFO after one
  producer binding is replaced and leaves scope; a rejected replacement does
  not overwrite them.
- A dequeued record retains every field after the queue reuses its freed slot
  and is destroyed with a record still pending.
- Focused/full locked checks, whole-tree source form, links, rendered document
  inspection, and full diff review pass without new exceptions or dependencies.

## Files and verification

Extend `tests/event_queue.rs`; add
`docs/verification/EVENT_RECORD_OWNERSHIP_REVIEW.md`; reconcile README,
roadmap, project state, traceability, and this plan. Existing requirements,
accepted decisions, source register, production code, and workflow suffice.

```text
cargo test --locked --test event_queue
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Set `RUSTDOCFLAGS=-D warnings` for rustdoc. Audit physical/comment widths and
existing expectations separately. Inspect rendered Markdown HTML structure
and content; do not claim pixel-level visual acceptance. Publish after local
acceptance, then inspect exact-revision hosted checkout, steps, tools, and logs.

## Risks and safe stopping point

Events expose immutable fields: binding replacement proves value independence,
not in-place mutation. Plain enum identifiers do not prove independence of
referents within arbitrary copied identifiers. Queue destruction observations
do not instrument deallocation, bound caller retention, detect identity/clock
provenance, or establish secure erasure, concurrency, or human v0.1 acceptance.
Stop after this checkpoint. If a defect emerges, investigate only these owned
record boundaries and preserve unrelated work.

## Completed local verification

The focused target passes eight tests and the final locked workspace baseline
passes 141. Formatting, all-target check, warnings-denied Clippy/rustdoc, and
whitespace pass. Both added tests fit the function-size policy without a new
expectation. Author and independent source/diff review pass. The whole-tree
audit covers 33 Rust files and 12,617 physical lines with zero width findings,
no block comments or allows, and three unchanged fulfilled expectations. All
280 relative links across 58 Markdown files and 105 exact traceability
function references resolve. Six changed documents pass rendered HTML
structure/content inspection; no pixel-level acceptance is claimed. Temporary
review aids remain ignored under `target/review-2026-10-06` and do not adopt
a new checker.
Unchanged host sample/workflow and conditional ADR probes do not trigger their
separate local commands; the full suite includes both host test targets.

## Publication and hosted verification

Checkpoint `814adcafe892aad3ecf960054ec1c6ff64ac98c7` was published by
ordinary fast-forward and the remote head matched. Hosted push run
[37315061868](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37315061868)
passes its exact logged checkout, the single Windows job, and all 20 reported
steps. Both regressions pass among 141 workspace tests; the separate 14 adapter
tests, five sample tests, documentation, executed sample, and both whitespace
steps pass. Focused reruns do not increase the workspace total.

Runner 2.337.0 uses `windows-2025-vs2026` image `20260925.250.1`.
Rust/Cargo 1.99.0, rustfmt 1.10.0-stable, and Clippy 0.1.99 match the previous
hosted checkpoint; local tools remain unchanged. This documentation follow-up
records the completed checkpoint; later revisions need their own CI result.
Human v0.1 architecture acceptance remains open.
