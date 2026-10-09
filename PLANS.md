# Finite message-bus sequence checkpoint

Date: **2026-10-10**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Add one dependency-free public-API reference-model test for composition of
standalone message publication and dequeue. The completed
[host review](docs/verification/HOST_V0_1_REVIEW.md) identifies this distinct
gap beyond selected scenario regressions. Requirements and ADR-0004/0010 define
the oracle; this checkpoint changes no runtime behavior or API.

Input `81ca906e97c28db53ca601d34c1b2d1b2e7926fb` is clean on codex/nightly,
equal to refreshed origin. Fresh locked baseline passes 141 workspace tests;
exact input hosted run 37782992360 succeeds. Local tool versions remain
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, Clippy 0.1.98.

## Frozen exploration contract

- Three registration-ordered inboxes: Command only/capacity 1, Telemetry
  only/capacity 2, and both topics/capacity 2. All are available.
- Six operations: publish Command, Telemetry, or Unrouted; dequeue inbox 0,
  1, or 2. Each publication carries its operation position as one payload byte.
- Enumerate every operation sequence of lengths zero through six, starting
  each from empty inboxes: 55,987 traces and 324,726 generated operations.
  Final observational drains are additional and excluded from that count.
- Independent explicit routes are Command=[0,2], Telemetry=[1,2], Unrouted=[].
  Accepted-delivery logs and consumed cursors model outstanding records without
  reusing production queue, message, report, or routing implementation.
- Compare every ordered outcome and classification. After every operation
  compare all logical capacities and pending counts. Compare each dequeue's
  complete semantic topic/payload. Drain every outstanding suffix at trace end,
  then assert empty dequeue and zero pending for every inbox.

## Components and acceptance

Add `tests/message_bus_sequences.rs` as one coherent integration target.
Record finite coverage, reviewers, evidence and limits in
`docs/verification/MESSAGE_SEQUENCE_REVIEW.md`; update traceability, project
state, roadmap and this plan. Reconcile ADR-0023's observed default-branch
revisit without altering the workflow or repository settings.

Root Codex implements; parallel Codex reviewers independently assess the
contract, durable state, oracle independence and complete diff. Acceptance
requires the frozen enumeration count, exact reports/FIFO, focused and full
locked baseline, source-form/width/waiver review, resolving Markdown links,
exact traceability functions and changed rendered-document inspection. Publish
only a coherent verified checkpoint, then inspect exact-head hosted CI.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test message_bus_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc runs with `RUSTDOCFLAGS=-D warnings`. Unchanged adapters/sample,
ADR-0018/0019 experiments and workflow do not trigger extra local commands.
Temporary audit/render aids remain ignored under target/review-2026-10-10-model.

## Risks and safe stopping point

This is exhaustive only over the declared finite alphabet, topology and length
bound. It does not prove longer traces, arbitrary capacities/topics/payloads,
lifecycle/dispatch behavior, allocation failures, concurrency, real-time
behavior, or whole-process resource bounds. No external research, dependency,
lint gate, exception, or public API change is needed. If a mismatch appears,
retain the smallest failing trace and address it within this objective; do not
hide it by changing the oracle to match accidental behavior. Stop after this
one tested/documented checkpoint. Revert only current-run edits if necessary.

## Completed local evidence

The new integration target passes one test over all frozen traces and both
asserted counts. Initial/final locked baselines pass 141/142 workspace tests,
formatting, all-target check, warnings-denied Clippy/rustdoc and whitespace.
Six isolated source fault trials compile and fail behavioral assertions;
production source is unchanged. Independent Codex model and source reviews
accept enumeration and invariants; the opening comment is wrapped to policy
and diagnostic context is improved. Source audit finds zero width debt across
34 Rust files/12,913 lines with three unchanged fulfilled expectations.
Relative links and 106 exact traceability function references resolve.
Seven checkpoint documents pass rendered structure/content inspection, and root
and parallel Codex complete-diff reviews accept the checkpoint. No pixel-level
acceptance is claimed. Exact publication evidence is recorded in the
[sequence review](docs/verification/MESSAGE_SEQUENCE_REVIEW.md).

## Publication and stopping point

Checkpoint `82078b3656481ec9527aaa059e159ca446c54aec` was published by
ordinary fast-forward; local/remote heads matched and the tree was clean.
[Hosted run 37935055379](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37935055379)
passes the exact logged checkout and all 20 reported steps, including 142
workspace tests, separate 14 adapters/five sample tests, warnings-denied
rustdoc, executed sample and both whitespace checks. Runner/image/tools match
the input baseline. This follow-up records that checkpoint's evidence; its
later revision requires separate hosted verification. Stop after this one
finite sequence checkpoint; do not start the next model in this run.
