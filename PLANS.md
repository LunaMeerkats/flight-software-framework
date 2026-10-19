# Practical library direction checkpoint

Date: **2026-10-19**
Status: **Locally verified and independently reviewed; publication pending**

## Objective and authority

Record the user's direction to develop a practical reusable library and progress
beyond v0.1 using research and engineering judgment. The reviewed host milestone
is a baseline, not a product ceiling. Current maturity and all separate
consequential gates remain truthful. No runtime behavior or API is changed.

## Changes and acceptance

Add ADR-0025 and reconcile AGENTS.md, README, charter, requirements context,
architecture, roadmap, project state, traceability and crate-level documentation.
Update the existing nightly automation prompt through the supported tool,
preserving its schedule, model, reasoning effort, project, active state and
notification preference. Read back the saved prompt and unchanged settings.

Acceptance requires an explicit practical-library objective, autonomous routine
progression beyond v0.1, consumer-facing work selection, no fixed embedded-fork
rule, retained qualification/consequential boundaries, resolving links, rendered
content/structure review, source-width review, complete diff review and independent
Codex acceptance. Future consumer readiness must remain unclaimed until observed.

## Verification

Clean input 1809ce3e1cafd390c017debabfd9933146d70b61 has exact hosted run
39130850495 success. Establish and replay the locked baseline:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Host adapters/sample, workflow and ADR-0018/
0019 experiments are unchanged, so their conditional local probes are not triggered.
No new test is needed for a direction/documentation change. Record exact actual
results, source/diff review and automation readback before acceptance. Temporary
review aids and raw evidence belong under target/review-2026-10-19-practical-library.

## Risks and stopping point

Practical-library ambition is not evidence of current stability, downstream use,
qualification or permission to publish a crate. Scope must follow consumer needs
and research rather than unbounded feature lists. Preserve historical exact-domain
results. Stop after this one direction checkpoint is reviewed, verified, published
on codex/nightly by ordinary fast-forward and inspected in exact-revision hosted CI.

## Local result and saved instructions

Initial/final locked format/check/Clippy/test and warnings-denied rustdoc pass
151 workspace tests; working-tree and committed-tip whitespace checks pass.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
Whole-tree review covers 43 Rust files/17,104 physical lines/1,138 comment-only
lines with zero width findings and three unchanged fulfilled expectations.
All 388 relative links and 115 exact test references resolve. Ten changed Markdown
documents pass full content/headings/code/links/tables/HTML structure inspection;
generated crate documentation carries the same practical goal and current limits.
These are content/structure reviews, not pixel or human acceptance.

The supported automation update succeeds. Exact saved-prompt readback matches;
only prompt/updated_at change. Schedule, model, reasoning effort, project, ACTIVE
state and notification preference are preserved. Independent Codex
library_direction_review accepts the complete direction/source/document diff and
separately inspects the saved automation. No runtime/API change or new dependency
is introduced. Publication and exact-checkpoint hosted inspection remain.
