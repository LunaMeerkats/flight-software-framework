# Callback-owned selective publication review

Date: 2026-11-01. Reviewer: Codex (root and independent agents), not human review.
Scope: RFF-LIB-REQ-010 revision 1 and
[ADR-0035](../adr/0035-callback-owned-selective-publication.md).

## Input and selection

Clean main 3d547b75cdae424b123927fde09a915299b940a8 matches refreshed origin/main.
Initial locked format/check/Clippy/test/warnings-denied rustdoc and both whitespace
checks pass with 217 workspace tests. Local Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98 are unchanged. The tracked policy still forbids
unsafe code, denies too_many_lines at 60 and configures stable max_width 100.

Live [input hosted run](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/41136402063)
tests that exact main checkout: all 20 steps, 217 workspace/14 adapter/five sample
tests, warnings-denied docs, both whitespace checks and unchanged 222-line sample.
Raw log inspection confirms Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy
0.1.99, runner 2.338.2, windows-2025-vs2026 image 20261030.294.1. Local execution
does not claim those hosted tools.

Root and independent consumer_assessment compare host-owned retry, separate
topics, deduplication and a coordinator against one additive callback method.
Select application-owned reply retry through the existing snapshot and routing
primitive. Requirement, decision and plan precede implementation. Official Rust
borrowing/queue sources inform reuse, not mission policy or compatibility.

## Implementation and consumer evidence

ApplicationMessageContext::publish_to delegates the existing selective primitive
with the same dispatch-state slice as publish. No routing implementation, queue
owner, report, dependency, unsafe code, workflow or ordinary-work path is added.
The [public-only consumer](../../tests/callback_publication.rs) owns one reply
and two acceptance flags, with fixed copied test observations. Busy new-reply
commands are consumed without buffering; the host explicitly resubmits them.

Source identities at review:

| File | Git blob | SHA256 |
| --- | --- | --- |
| src/application_messaging.rs | bbcd670288614ee00b9f4472ece0c268ada0d0c7 | 1b6d9b9c943f99834bfd473d12a1e88dd0c844399cb69f1d06379cbb90bd75e2 |
| tests/callback_publication.rs | 29d0d14e4583f7aabf9b6bc5f6304a7f014d7fd1 | b1ade3e6d37c73f787dfcee6ba854d376c7260891731c2f22e62c5ca0a640b80 |

Seven focused tests pass. They observe older rejected-side FIFO [3,4] before
reply 42, repeated full/unavailable attempts without duplicate accepted peers,
busy new-reply consumption/resubmission, lifecycle snapshot refresh through
Registered/Running/Stopped/Failed, exact retained other-topic byte 7, concrete
routing error/source chains, capacity-one self-publication and exact two-copy
callback-owner clearing while the accepted peer's reply 42 survives.

Independent package:
C:/Users/drrsm/AppData/Local/Temp/rff-callback-3a3cba880a6149708b9b49b571cbb825.
Check, warnings-denied Clippy and all seven identical consumer tests pass. The
generated cargo-new smoke test separately passes one test. Metadata confirms one
independent workspace member, two graph packages, the sole framework path
dependency and zero library dependencies. Source bytes and SHA256 match above.
No maintained second crate is added.

Actual commands (stop on failure), with the recorded path assigned to $consumer:

```text
cargo test --locked --test callback_publication
cargo clippy --locked --test callback_publication -- -D warnings
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 src/application_messaging.rs, tests/callback_publication.rs
```

The guide's Copy-Item operation copied the exact test source into the independent
package; inspection also compares bytes. Initial fixture setup/lint failures
were corrected before these passing checks, without policy waivers.

## Full baseline and review

The final locked baseline passes all commands below, with 224 workspace tests.
RUSTDOCFLAGS=-D warnings is set for the documentation command; local tool
versions remain the input versions. Source is unchanged after these checks.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Root and independent callback_review accept the source and consumer with no
remaining findings. Whole-tree audit agrees on 53 Rust files, 23,070 physical
and 1,355 comment-only lines, zero width findings and three unchanged existing
expectations. Relative links and all 188 exact test references resolve. All 12
changed Markdown documents pass rendered content/heading/code/link/table and
nesting inspection after correcting unquoted generic type syntax. Generated
context rustdoc exposes the method, snapshot, owner cleanup and typed error links.
This is content/structure inspection, not pixel QA or human acceptance. Root
complete source/document/diff review accepts the bounded change with no findings;
independent callback_review also accepts the complete evidence/document diff
after clarifying busy-command consumption and the pending hosted status.
Temporary raw aids are under target/review-2026-11-01-callback-publication.

## Publication and exact hosted acceptance

Implementation e2ad3484530091e9b51e5f61ead43318187e802e exactly matches the
accepted staged patch. An ordinary fast-forward push publishes it to origin/main;
the clean local and live remote heads agree. Exact main push
[run 41204737701](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/41204737701),
job 124547413239 passes all 20 steps: 224 workspace tests including all seven
new regressions, separate 14 adapter/five sample tests, warnings-denied rustdoc,
both whitespace checks and unchanged 222-line combined sample. Root actual raw
checkout/result/tool/image logs and connector step summaries agree.

Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99 and runner 2.338.2 match
input. This job uses windows-2025-vs2026 image 20261021.278.1, differing from
input 20261030.294.1; the exact image is recorded rather than assumed unchanged.
Local Rust 1.98.0 execution remains separate. Independent callback_review also
accepts the raw exact-checkout/result/tool/image logs with no findings. The
record-only completion passes the same source/link audit (583 relative links)
and rendered content/structure inspection of all 13 run-changed documents; root
complete-diff review accepts it. Independent callback_review accepts the complete
seven-document completion with no findings. Completion changes no source or
consumer bytes; its own exact hosted result is recorded in automation memory
and the end-of-run report. No further behavior is selected in this run.

## Limits and disposition

The controlled consumer demonstrates application-owned enqueue acceptance only.
Retry commands are host-initiated. Later lifecycle clearing may discard accepted
copies; callback effects are nontransactional and repeated calls may duplicate.
IDs remain caller-scoped. No new ordinary-work service, queue owner, thread,
dependency, fairness, timing, delivery, adoption, stability or qualification claim.
Separate consequential authorization gates remain unchanged.
