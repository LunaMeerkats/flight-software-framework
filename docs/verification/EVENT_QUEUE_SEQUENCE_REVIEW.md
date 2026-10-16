# Standalone event-queue sequence review

Date: **2026-10-17**
Status: **Checkpoint published; exact-revision hosted verification accepted**
Reviewed input: `21a0f02b5f2b23ef19813c7e37812f748157f812`
Reviewer: **Codex**, with independent event_gap and source_review agents

## Decision and gap

Keep the existing production contract from
[ADR-0013](../adr/0013-bounded-structured-event-queue.md). Eight existing
standalone tests already establish metadata, construction errors, exact bounds,
FIFO, saturation/retry, repeated reuse and copied-record ownership. No production
defect was found. The [resource review](EVENT_QUEUE_REVIEW.md) and
[ownership review](EVENT_RECORD_OWNERSHIP_REVIEW.md) remain distinct evidence.

The new checkpoint targets systematic composition of partial fills, empty reads,
rejections, slot reuse and repeated equal records. A single reduced model can
exercise those histories without widening runtime event production or adding an
API. Repeating another fixed saturation cycle would duplicate existing evidence;
mixing returned-work events, clocks and lifecycle in this same model would make
the objective unnecessarily broad. Existing local contracts settle the expected
outcomes; no external research, reused source or new ADR is needed.

## Frozen domain and independent oracle

Before implementation, PLANS.md freezes capacities 1, 2 and 3 and the alphabet
Emit(A), Emit(B), Dequeue, with every sequence of length zero through eight.
This is 29,523 independently initialized traces and 221,436 generated operations.
The two fixed records differ in all four metadata fields, with timestamp order
opposing A-then-B emission order. Repeated emission of one record is permitted;
the queue has no deduplication or severity-priority policy.

An append-only accepted-record ledger and consumed ordinal predict the retained
suffix. The oracle does not use VecDeque or production transition/storage helpers.
Every generated operation compares its exact result and observable capacity,
pending count and empty state, with capacity/trace/step failure context. Terminal
drains compare complete record equality and repeated empty reads. Those drains
occur only after each generated trace and are outside generated-operation counts.

## Verification and source review

Initial locked checks pass 148 workspace tests, formatting, all-target checking,
warnings-denied Clippy and rustdoc, working-tree and committed-tip whitespace.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
Fresh input hosted run 38740423411 passes its exact logged checkout and all 20
steps, 148 workspace tests, separate 14 adapter/five sample tests, rustdoc,
the unchanged sample report and both whitespace checks.

Focused test and focused warnings-denied Clippy pass. The final full locked
baseline passes 149 workspace tests, formatting, all-target check, Clippy,
warnings-denied rustdoc and whitespace. Root and independent source_review accept
the complete 205-line test at Git blob
`d865bbab4187415cd2460c8bd3ab13bfdbd06e54`. Independent enumeration confirms
the generated totals and 100,598 supplemental terminal dequeues, including two
empty reads per trace. The latter is a separately calculated observation count,
not another generated-domain count asserted by the Rust test.

The clean isolated source copy passes checking and the focused test. Six selected
production faults all compile and then fail behavioral assertions:

| Source fault | First observed witness |
| --- | --- |
| Reject at capacity minus one | Capacity 1, Emit(A), generated step 1 |
| Accept beyond the configured capacity | Capacity 1, Emit(A), Emit(A), generated step 2 |
| Remove newest rather than oldest | Capacity 2, Emit(B), Emit(A), terminal drain step 3 |
| Insert newest at the front | Capacity 2, Emit(B), Emit(A), terminal drain step 3 |
| Overwrite oldest while reporting QueueFull | Capacity 1, Emit(B), Emit(A), terminal drain step 3 |
| Report capacity plus one on rejection | Capacity 1, Emit(A), Emit(A), generated step 2 |

Every copy runs `cargo check --locked --test event_queue_sequences` followed by
`cargo test --locked --test event_queue_sequences`. Control exits are 0/0;
each mutant exits 0/101 with the listed result or full-record equality assertion.
Root independently inspects the raw patches, compile/test logs and matching
source/test hashes. Trial copies and manifests are retained under
target/review-2026-10-17-events/mutations/run-20261016-130459. These selected
faults are sensitivity evidence, not exhaustive mutation coverage.

Whole-tree audit covers 41 Rust files, 16,117 physical lines and 1,120
comment-only lines, with zero width findings and three unchanged fulfilled
expectations. All 363 relative links across 70 Markdown files resolve; 113
exact traceability references uniquely resolve to test functions. All six changed
documents pass rendered full-content/headings/code/link/table/nesting/encoding
inspection. This is rendered HTML structure/content review, without pixel or
human-review claims. Root and independent source_review accept the complete
source/document/diff within the frozen scope.

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

Rustdoc uses RUSTDOCFLAGS=-D warnings. Host adapters/sample, workflow and
ADR-0018/0019 probes are unchanged, so their separate local commands are not
triggered. Ignored review aids and raw logs reside under
target/review-2026-10-17-events; no checker is adopted as a new gate.

## Limits and disposition

This finite domain excludes arbitrary capacities, event content and identifier
types, construction failures, allocator behavior, shared referent isolation,
clock/identity provenance, runtime event-production ordering, filtering,
concurrency, panics/hangs, persistence and guaranteed delivery. Existing scenario
and owner-specific failure-event evidence remains separate. Test-harness ledger
retention is bounded by the frozen trace length and is not operational storage.

Production, public APIs, requirements, dependencies, licence, lint configuration
and workflow are unchanged. Local and exact-checkpoint hosted acceptance are
complete. This record does not establish human review or qualification.

## Publication and toolchain re-audit

Checkpoint `7137e924819b8cc35f0c4f3464f80c3d8df1b149` is published by ordinary
fast-forward; remote/local heads match and the tree is clean at publication.
[Push run 38905783350](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38905783350)
passes the exact logged checkout, job 116988877022 and all 20 reported steps.
Raw logs confirm 149 workspace tests including this model, separate 14 adapter/
five sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines
and both whitespace checks. Root and independent source_review inspect the raw
JSON/logs separately from local checks and review.

Hosted Rust/Cargo advances from 1.99.0 to 1.99.1, with compiler commit
58cf8f9c3 dated 2026-10-13. Rustfmt remains 1.10.0-stable and Clippy 0.1.99,
both at that compiler commit. Runner 2.337.0 uses windows-2025-vs2026 image
20261007.264.1, replacing 20260925.250.1. The required tracked-configuration and
41-file whole-tree source re-audit retains stable max_width=100, comment review
at 80, function threshold 60, the individual deny lint and three fulfilled
expectations, without source changes, added waivers or a toolchain pin. Local
execution remains on 1.98.0. Source review and completed hosted all-target checks
supply distinct evidence; neither claims local execution on the hosted toolchain.

This documentation follow-up records the checkpoint and refreshed environment,
preserves Rust/configuration/workflow blobs, and receives complete document/diff
review plus its own exact-revision CI. A later head cannot inherit this result.
The follow-up audit resolves 365 relative links/113 exact test references;
all seven run-changed rendered documents pass complete content/HTML structure
review. Root and independent source_review accept the complete follow-up diff.
Stop after this one accepted increment.
