# Standalone event-queue history checkpoint

Date: **2026-10-17**
Status: **Complete: checkpoint published and exact-revision CI passed**

## Objective and context

Supplement existing RFF-REQ-005 and ADR-0013 evidence with mixed partial-fill,
dequeue, rejection and repeated-equal-record histories. Existing eight queue
scenarios already verify capacity, FIFO, saturation/retry, reuse and ownership;
no production defect is identified. The value is systematic composition of
those operations, rather than another copy of the fixed saturation cycle.

Input 21a0f02b5f2b23ef19813c7e37812f748157f812 is clean on codex/nightly,
equal to refreshed origin. Fresh input run 38740423411 reports success at
that exact head. Initial locked format/check/Clippy/test, warnings-denied
rustdoc and both whitespace checks pass 148 workspace tests. Local tools are
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. Root Codex and
independent event_gap inspect the existing scenarios and contracts. ADR-0024
supplies routine autonomous review authority.

## Frozen domain and oracle

- One standalone queue at each capacity 1, 2 and 3.
- Emit(A), Emit(B), Dequeue; independently initialize every operation string
  of lengths zero through eight: 29,523 traces, 221,436 generated operations.
- Fixed A/B records differ in source, severity, mission identifier and elapsed
  timestamp; timestamp order opposes A-then-B emission order. Equal records
  may be emitted repeatedly. No implicit deduplication or priority applies.
- Independent append-only accepted-record ledger plus consumed ordinal predicts
  exact outcomes and remaining records, without VecDeque or production helpers.
- Compare exact emission/dequeue outcomes, capacity, pending and empty after
  every generated operation. Include capacity/trace/step in failure context.
- Only after each generated trace, drain all expected remaining records and
  repeat empty reads; keep these supplemental observations outside action counts.

## Files and acceptance

Add tests/event_queue_sequences.rs and a focused verification review; reconcile
AGENTS layout, project state, roadmap, traceability and CI evidence. No production,
API, requirement, dependency, licence, lint or workflow change. Existing local
contracts suffice; no new external research or source reuse is needed.

Acceptance requires fixed counts, focused and full locked checks, independent
oracle/source/complete-diff review, whole-tree width/waiver review, resolving
relative links and traceability references, and changed rendered-document
inspection. Selected isolated source faults must compile then fail behavioral
assertions; keep their exact patches and logs outside tracked source.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test event_queue_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host adapters/sample,
ADR-0018/0019 experiments and workflow do not trigger extra local commands.
Ignored aids/logs belong under target/review-2026-10-17-events.

## Risks and safe stopping point

The finite domain excludes arbitrary capacities/content/identifier types,
construction failures, allocation accounting, shared referent isolation,
clock/identity provenance, runtime emission order, filtering, concurrency,
panics/hangs, persistence and delivery guarantees. Existing scenario and
owner-specific event evidence remains separate. If a mismatch appears, retain
its smallest witness and resolve it within scope. Preserve unrelated changes.
Publish only accepted commits by ordinary fast-forward, inspect exact-head CI,
and stop after this one checkpoint.

## Local execution

tests/event_queue_sequences.rs implements the frozen domain at Git blob
d865bbab4187415cd2460c8bd3ab13bfdbd06e54. Focused formatting/test/Clippy and
the final full locked baseline pass 149 workspace tests. Fixed trace/action
counts pass. Root and independent source_review accept the separate symbolic
ledger/cursor oracle and public observations. Independent enumeration additionally
counts 100,598 supplemental terminal dequeues, outside generated counts.
Clean isolated control passes; six compiling faults fail behavioral assertions
for early/late saturation, newest dequeue, reversed insertion, full-queue
overwrite and incorrect rejection capacity. Root separately inspects raw logs,
patches and hash identity. Whole-tree audit covers 41 Rust files/16,117 physical
lines/1,120 comment-only lines, zero width findings and three unchanged fulfilled
expectations. All 363 relative links/113 exact test references resolve. All six
changed rendered documents pass full content/HTML structure inspection; no pixel
or human-review claim. Root and independent source_review accept the complete
source/document/diff.

## Publication and stopping point

Checkpoint 7137e924819b8cc35f0c4f3464f80c3d8df1b149 is published by ordinary
fast-forward and its remote head matches. [Push run 38905783350](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38905783350)
passes exact logged checkout, job 116988877022 and all 20 steps, 149 workspace
tests, separate 14 adapter/five sample tests, warnings-denied rustdoc, unchanged
222-line sample report and both whitespace checks. Root and independent Codex
review inspect raw hosted JSON/logs separately from local evidence.

Hosted Rust/Cargo advances to 1.99.1, rustfmt remains 1.10.0-stable and Clippy
0.1.99; runner 2.337.0 image advances to windows-2025-vs2026 20261007.264.1.
Required configuration and whole-tree re-audit retain the same policy and
three fulfilled expectations without source or waiver changes. Local tools stay
on 1.98.0. The focused review and CI baseline record exact environment evidence.
This documentation follow-up preserves Rust and receives complete review and
its own exact-revision hosted CI. Stop after this one completed checkpoint.

Follow-up audit resolves 365 relative links/113 exact test references; all seven
run-changed rendered documents pass full content/HTML structure inspection.
Root and independent source_review accept the complete follow-up diff.
