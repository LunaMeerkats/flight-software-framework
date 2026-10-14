# Standalone configuration sequence checkpoint

Date: **2026-10-15**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Strengthen existing RFF-REQ-006 with an independent finite model of standalone
configuration replacement, rejection, predecessor eviction and consume-once
rollback. Existing tests cover selected chronological traces, including runtime
visibility and revision exhaustion, but do not generate these combined histories.
ADR-0005/0017 already define the contract; no production or requirement change
is planned. Existing runtime composition evidence remains separate.

Input `ecfe954cd7550a933bf08821d9a4154c71023953` is clean on codex/nightly and
matches refreshed origin. Fresh input hosted run 38404703855 succeeds at that
exact revision with all 20 steps. Initial locked local format/check/Clippy/test,
warnings-denied rustdoc and whitespace pass 146 workspace tests. Local
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
Root Codex and parallel config_gap/baseline_review reorient and audit;
ADR-0024 supplies routine review authority, separate from completed host review.

## Frozen domain and oracle

- One standalone table, inline bound two, initial accepted bytes [1] at revision
  one. One pure fixture validator accepts nonempty content with a nonzero first
  byte and rejects [0] with its concrete mission error.
- Five operations: ReplaceA=[1], ReplaceB=[2,3], Reject=[0], Oversize=[0,0,0],
  Rollback. Independently initialize every sequence of length zero through six:
  19,531 traces and 112,305 generated operations. Assert both aggregate counts.
- An append-only accepted history with predecessor references and movement
  history derives active identity and rollback eligibility. Successful acceptance
  count assigns revisions; the oracle does not reproduce the production
  active/rollback/high-water storage triple or call production transition logic.
- Compare exact results, concrete error/source relationships, used bytes and
  original revision initially and after every generated operation. Equal-content
  acceptance, exact bound/shorter lengths, failed operations, consumed rollback,
  history eviction and revision non-reuse compose within this domain.
- After every trace, supplemental Rollback/ReplaceA/Rollback probes expose
  retained history and the next unused revision. Their 58,593 observations are
  excluded from generated lengths and the 112,305 generated-operation count.

Length six reaches replacement, history eviction, rollback, rejection, fresh
replacement and a consumed rollback. The oversized candidate is also invalid
under the validator, making error precedence observable. Existing white-box
tests separately exercise revision exhaustion and validator suppression.

## Components and acceptance

Add tests/configuration_sequences.rs and a focused verification review. Update
AGENTS layout, traceability, roadmap, concise state and CI evidence. Root Codex
owns records; config_gap owns the test. Separate Codex reviewers inspect oracle
independence, isolated compiling source faults, source form and complete diff.
Existing repository contracts suffice; no external research, source reuse,
dependency, public API, ADR, requirement or lint waiver is needed.

Acceptance requires frozen counts/outcomes, focused and full locked checks,
whole-tree width/waiver review, relative-link and traceability resolution,
changed rendered-document inspection and complete source/diff review. Selected
isolated faults must compile then fail behavioral assertions without changing
tracked production. Ordinary fast-forward publication follows local acceptance;
inspect hosted CI at each exact published revision.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test configuration_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host adapters/sample,
ADR-0018/0019 experiments and workflow do not trigger their extra local commands.
Ignored review aids remain under target/review-2026-10-15-configuration.

## Risks and safe stopping point

The finite domain excludes other capacities, validators, candidate bytes,
initial failures, revision exhaustion, runtime lifecycle/work, messaging,
events/scheduling, allocation failures, panics/hangs and concurrency. Observable
results do not establish validator purity, termination, physical memory bounds
or operational suitability. Existing scenarios retain those separate boundaries.

If a mismatch appears, retain its smallest witness and resolve it within this
objective. Preserve unrelated changes. Stop after this one reviewed checkpoint.

## Local evidence

The implemented model and focused warnings-denied Clippy pass. Initial/final full
locked baselines pass 146/147 workspace tests, formatting, all-target check,
Clippy, warnings-denied rustdoc and both whitespace checks. Root and parallel
baseline_review accept the independent lineage oracle and complete test source;
blob `00ca28860a92cffa7af79e6108bf9c40d3d9c231`. The fixture now matches the
frozen validator policy after a review correction. Six selected final-copy
faults compile then fail model assertions; clean isolated control passes.
Root separately inspects logs, witnesses and final hashes; the focused review
records scope and trial commands. Production is untouched. Root and parallel
baseline_review accept the complete source/document/diff with no blocker.
Whole-tree audit covers 39 Rust files/15,333 lines and 1,105 comment-only lines,
zero width findings and three unchanged fulfilled expectations. All 352
relative links across 68 Markdown files and 111 exact traceability functions
resolve. Six changed documents pass complete rendered content/HTML structure
inspection, with no pixel or human-review claim. Publication and exact-revision
hosted execution are recorded separately below.

## Publication and stopping point

Checkpoint `56a68a0c7c860dee02ac42e441f9857421aad164` is published by ordinary
fast-forward; local/remote heads match and the tree is clean at publication.
[Push run 38569938802](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38569938802)
passes its exact logged checkout, Windows job 115872183297 and all 20 steps.
Raw logs confirm 147 workspace tests including this model, separate 14 adapter/
five sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines
and both whitespace checks. Root inspects JSON/logs separately from local checks;
parallel baseline_review performs the companion independent evidence review.

Hosted runner 2.337.0, windows-2025-vs2026 image 20260925.250.1,
Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the input.
Local execution remains on 1.98.0. This evidence follow-up preserves source and
receives review of all seven run-changed documents and its own exact-revision
hosted CI. Checkpoint CI does not cover a later documentation revision. Stop
after this one completed checkpoint.

Follow-up audit resolves 354 relative links/111 exact traceability functions;
all seven run-changed rendered documents pass complete content/HTML structure
inspection. Test and production blobs remain unchanged from local acceptance.
