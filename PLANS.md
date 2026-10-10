# Runtime-owned lifecycle sequence checkpoint

Date: **2026-10-11**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Strengthen RFF-REQ-002/003 with one independent finite model of runtime-owned
availability and exact selected-inbox clearing. Existing scenario tests and the
standalone message model do not establish these compositions. Input
`181cfa913c17177e2696d35fadb790883e4d0ed7` is clean on codex/nightly, equal to
refreshed origin; hosted run 37935576798 succeeds for that exact head. The fresh
locked local baseline passes 142 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98, with no width or waiver changes.

## Frozen exploration contract

- Two successful passive applications, both initially Running after checked
  start setup, share one subscribed topic with inbox capacities [1,2].
- Five generated operations: Publish with its operation-position byte, Stop(0),
  Stop(1), Restart(0), Restart(1). Enumerate every sequence length zero through
  six: 19,531 traces and 112,305 generated operations.
- An independent Active/Paused model uses append-only accepted payload histories
  and discard watermarks. Stop while Active discards exactly the outstanding
  suffix and sets Paused; restart while Paused sets Active. Rejections preserve
  state and histories and predict zero discard with no callback.
- Hard-coded routes [0,1] predict ordered delivery/full/unavailable outcomes and
  classification. Compare both states, capacities, pending counts and lifecycle
  callback counters initially and after every generated operation.
- Supplemental final public dispatches compare complete retained topic/payload
  FIFO suffixes through passive callback observers. Check empty dispatch and
  callback suppression; Paused endpoints reject dispatch and remain empty.
  These observations are outside generated counts. Every prefix is independently
  enumerated, so final suffix observations cover every generated prefix.

Parallel Codex model_review independently proposed this domain. Adding generated
Dispatch operations would multiply the domain to 137,257 traces; a length-five
bound would omit six-step clear/reconnect reuse witnesses. More topics duplicate
standalone routing evidence. Keep the five-operation length-six contract.

## Components and acceptance

Add tests/runtime_messaging_sequences.rs, a lifecycle sequence review, and update
this plan, AGENTS layout, traceability, roadmap and concise project state. Keep
production source, requirements, public APIs, dependencies and policy unchanged.
Acceptance requires exact frozen counts and outcomes, observer FIFO, focused and
full locked checks, source/waiver/width and complete-diff review, resolving links,
exact traceability references and changed rendered-document structure/content.
Root Codex implements; parallel Codex reviewers assess model/source/evidence.
Selected isolated source fault trials check sensitivity without modifying the
production tree. Publish the accepted checkpoint by ordinary fast-forward and
inspect exact-head hosted CI, recording its distinct evidence.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_messaging_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host adapters/sample,
ADR-0018/0019 experiments and workflow do not trigger extra local commands.
Ignored review aids stay under target/review-2026-10-11-lifecycle.

## Limits and safe stopping point

The model excludes generated start, Registered/Failed states, callback errors,
work, callback publication, arbitrary topics/capacities, allocation failures,
concurrency, real-time and whole-process bounds. Final passive dispatch is only
an observer, not exhaustive dispatch-composition evidence. If a mismatch appears,
retain its smallest trace and resolve it within this objective. Stop after this
one tested/documented checkpoint; preserve unrelated changes and revert only
current-run implementation if the verified baseline cannot be restored.

## Completed local baseline

The focused model passes one test over all frozen traces and asserted counts.
Initial/final full locked baselines pass 142/143 workspace tests, formatting,
all-target check, warnings-denied Clippy/rustdoc and whitespace. Independent Codex
oracle review accepts the model; its optional final observer lifecycle/counter
check is included and passes. No function-size exception or production change
is added. Source audit covers 35 Rust files/13,353 physical lines with zero width
findings and three unchanged fulfilled expectations. All 331 relative links
across 64 Markdown files and 107 exact traceability function references resolve.
Six isolated source faults compile and fail behavioral assertions against final
test blob `71f5782c215de08f0e8034cacdead2b262c21555`; production is unchanged.
The [review](docs/verification/RUNTIME_MESSAGE_SEQUENCE_REVIEW.md) records exact
faults, small detected traces and ignored evidence paths.
Root and parallel Codex source/complete-diff reviews accept the checkpoint.
All six changed documents pass rendered HTML structure/content inspection;
pixel-level acceptance is not claimed. Publication and exact-head CI remain
separate evidence, completed below.

## Publication and stopping point

Checkpoint `c3eaa10cb013d1d4f5dbd11f6f0c4a3d4d98d37b` was published by ordinary
fast-forward. At checkpoint publication, local/remote heads matched and the
tree was clean.
[Hosted push run 38054603275](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38054603275)
passes the exact logged checkout, one Windows job 114220376706 and all 20
reported steps: 143 workspace tests including this model, separate 14 adapters/
five sample tests, warnings-denied rustdoc, sample execution and both whitespace
checks. Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo
1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the prior hosted baseline.
Local tools remain on 1.98.0; no local 1.99.0 claim is made.

This documentation follow-up records that checkpoint. Its later revision needs
its own hosted CI; it does not inherit this pass. All seven run-changed documents
are regenerated and pass structure/content inspection. Final audit resolves
332 relative links across 64 Markdown files and 107 exact traceability functions;
source widths and the three existing expectations remain unchanged.
Stop after this one lifecycle sequence checkpoint; begin no failure-model work.
