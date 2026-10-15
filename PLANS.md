# Runtime configuration/work sequence checkpoint

Date: **2026-10-16**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Strengthen existing RFF-REQ-006 integration evidence with generated direct-runtime
configuration, work and selected lifecycle histories. Standalone configuration
sequences already check table transitions; runtime scenarios check individual
contracts. The lifecycle scenario never changes configuration while stopped.
Generated histories compose that missing path with restart visibility,
returned-work failure, peer observations and retained configuration lineage.
ADR-0005/0017/0018 and the existing LC1 contracts supply the expected behavior.

Input `ce8bd7e86423a69435c6ba4f71a011df5b62ce99` is clean on codex/nightly and
matches refreshed origin. Fresh input hosted run 38570555687 succeeds at that
exact revision with all 20 reported steps. Initial locked local format/check/
Clippy/test, warnings-denied rustdoc and whitespace pass 147 workspace tests.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
Root Codex and independent gap_review/baseline_review inspect the existing gap
and source policy. ADR-0024 supplies routine autonomous review authority.

## Frozen domain and oracle

- One direct Runtime, two initially Running applications, one constructor-owned
  table with bound two, initial [1] at revision one. The pure fixture validator
  accepts nonempty content with a nonzero first byte and rejects [0].
- Eight operations: ReplaceShort=[1], ReplaceExact=[2,3], Reject=[0], Rollback,
  WorkSelected, WorkPeer, StopSelected and RestartSelected. Independently
  initialize every sequence of length zero through five for both selected
  application indices: 74,898 traces and 363,792 generated operations.
- The selected application returns its concrete error on its second invoked
  work callback; its peer always succeeds. Stop/restart callbacks succeed and
  preserve the same application value. Rejected requests invoke no callback.
- An independent append-only acceptance/predecessor ledger and movement history
  predict configuration identity and rollback eligibility. Independent lifecycle
  and callback histories predict results, copied work views and callback counts.
  Do not call production transition logic or copy its storage representation.
- Compare exact results, lifecycle states and complete callback logs after every
  generated operation. Observe accepted revision and used bytes from callbacks,
  including the callback returning the application error. Inspect error sources.
- After each trace, separate WorkPeer/Rollback/WorkPeer/ReplaceShort/WorkPeer/
  Rollback/WorkPeer probes expose current content, retained history and next
  revision. These 524,286 supplemental operations are outside generated counts
  and never induce an unattempted selected failure.

The eight-operation domain deliberately includes stop/restart because
WorkSelected/StopSelected/ReplaceExact/RestartSelected/WorkSelected fits the
bound and observes fresh configuration and retained callback count on failure.
A six-operation failure-only alphabet would leave the stopped-change gap open.
Oversize, exhaustion and alternate service owners retain separate evidence.

## Components and acceptance

gap_review implements tests/runtime_configuration_sequences.rs only. Root Codex
owns a focused verification review and updates layout, traceability, roadmap,
concise project state and CI evidence. Independent review checks oracle
independence, source form, the complete diff and selected isolated source faults.
No production/API, requirement, dependency, licence, lint or workflow change is
planned. Existing repository contracts suffice; no external research or source
reuse is needed.

Acceptance requires frozen counts/outcomes, focused and full locked checks,
whole-tree width/waiver review, relative-link and traceability resolution,
changed rendered-document inspection and complete source/diff review. Selected
isolated faults must compile then fail behavioral assertions. Publish accepted
commits by ordinary fast-forward; inspect hosted CI at each exact revision.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_configuration_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host adapters/sample,
ADR-0018/0019 experiments and workflow do not trigger their extra local commands.
Ignored review aids remain under target/review-2026-10-16-runtime-config.

## Risks and safe stopping point

The finite domain excludes arbitrary candidates, validators, capacities,
initial rejection, unknown/Registered states, other callback failures, messaging,
scheduling, events, revision exhaustion, allocator failures, panics/hangs and concurrency.
Fixture purity/termination does not establish those properties for mission
validators or callbacks. No operational suitability or recovery claim follows.

If a mismatch appears, retain its smallest witness and resolve it within this
objective. Preserve unrelated changes. Stop after this one reviewed checkpoint.

## Local evidence

The focused model and focused warnings-denied Clippy pass. Initial/final full
locked baselines pass 147/148 workspace tests, format, all-target check, Clippy,
warnings-denied rustdoc and whitespace. One initial collapsible fixture condition
was corrected before final verification and all sensitivity trials. Root and
parallel baseline_review accept oracle/source at blob
`ca64c5a09f1ab93e046e612fd869937b8b3ecff6`. Whole-tree review covers 40 Rust
files/15,912 physical lines/1,113 comment-only lines, no width findings and
three unchanged fulfilled expectations. Links and 112 exact test references
resolve. Six selected compiling production faults fail behavioral assertions;
clean isolated control passes. The focused review records exact trial scope,
commands and limits. Root independently checks raw logs, patches and hashes.
Root and parallel baseline_review accept the complete source/document/diff.
All 357 relative links across 69 Markdown files and 112 exact function references
resolve; all six changed rendered documents pass full content/HTML structure
inspection, with no pixel or human-review claim. Publication and exact-revision
hosted execution are recorded below.

## Publication and stopping point

Checkpoint `283b07c280746dd6ac7c26b60bdaf0ab14e5bad3` is published by ordinary
fast-forward; local and remote heads match and the tree is clean at publication.
[Push run 38739984030](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38739984030)
passes its exact logged checkout, Windows job 116438836697 and all 20 steps.
Raw logs confirm 148 workspace tests including this model, separate 14 adapter/
five sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines
and both whitespace checks. Root inspects raw JSON/logs separately from local
checks; parallel baseline_review supplies independent review.

Hosted runner 2.337.0, windows-2025-vs2026 image 20260925.250.1,
Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the input.
Local execution remains on 1.98.0. This documentation follow-up preserves source
and receives full review and its own exact-revision hosted CI. The checkpoint
result does not cover a later revision. Stop after this one completed increment.

Follow-up audit resolves 358 relative links/112 exact traceability functions;
all seven run-changed rendered documents pass complete content/HTML structure
inspection. Test and production blobs remain unchanged from local acceptance.
