# Runtime-issued ordinary-work identity review

Date: **2026-10-24**
Status: **Local consumer verification and autonomous review complete**

## Objective and input

Complete the unfinished RFF-LIB-REQ-004 revision 1 increment from 2026-10-23.
Input HEAD a9afb328905eb2d413a868bc7e0371a06b5c6c20 matches refreshed
origin/codex/nightly. Existing requirement, ADR, guide and partial source edits
are preserved as unfinished automation work, rather than silently treated as
a clean input. The requirement and ADR precede source implementation.

Exact [input run 39606324526](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39606324526)
is re-read: successful push event at that head, Windows job 119243331220,
all 20 successful steps, 168 workspace tests, separate 14 adapter/five sample
tests and the 222-line sample report. Input hosted tools are Rust/Cargo 1.99.1,
rustfmt 1.10.0-stable and Clippy 0.1.99; runner 2.338.0 uses
windows-2025-vs2026 image 20261018.275.1. This verifies the committed input only.

Current initial format check finds accidental indentation in unfinished trait
rustdoc; correcting it restores the baseline. Fresh locked check/Clippy/test,
warnings-denied rustdoc and both whitespace checks pass with 168 workspace
tests before the new consumer tests. Local tools remain Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98. No local hosted-toolchain claim is made.
Raw logs and review aids are under target/review-2026-10-24-work-identity,
ignored and not newly adopted policy tools.

## Design and verification boundary

Root Codex and independent Codex context_review reassess the actual source gap.
[ADR-0029](../adr/0029-ordinary-work-identity.md) compares host staging,
post-registration identity wiring and broader context/service changes. Selected
identity attribution is the bounded API gap: checked mission-owned services
can already supply callback-time acceptance feedback. Official Rust Cell and
RefCell documentation is rechecked and registered in SRC-RUST-WORK-IDENTITY;
no upstream implementation is copied.

The [public-only tests](../../tests/work_identity.rs) borrow a caller-owned
capacity-one diagnostic queue and Cell-backed clock. Each application borrows
its own caller-owned observations for attempted/committed work, call count and
one message observation. There are two runtime records, optional one-byte
configuration, and two capacity-one one-byte inboxes in messaging scenarios.
The effect is a copied observation committed only after Recorded. QueueBusy and
QueueFull are explicit mission-selected terminal work errors. Failure-event
wrappers receive a separate capacity-one report queue, avoiding a queue borrow
held across a callback that needs the same RefCell.

The library adds one copied ApplicationId and one read-only accessor to a
privately constructed context. Runtime::work is the sole construction path;
both schedule owners and failure wrappers delegate through it. No new trait,
framework service/queue owner, crate, dependency, lint exception or workflow is
introduced. Ordinary-work callbacks may emit their own events using injected
mission services; the framework's automatic failure reporting remains opt-in.

Identity has no issuer provenance: equal-position keys from different runtimes
can compare equal. The queue bounds records, not process memory or callback
duration. Event acceptance before an effect establishes ordering, not atomicity,
physical delivery or rollback. Panics, hangs and allocation exhaustion remain
outside cooperative returned-error behavior. This fixed consumer does not
establish external adoption, general usability, stable APIs or qualification.

## Local and independent consumer verification

Consumer source blob: `216386faca71ee468c318a43fa13a354e5140753`.
Source SHA256: `7f94006884a4ee354fa2c451195b7d99858e339f0b5c98214a89de95411aee2c`.
Runtime source blob: `0c9cc37b42eabd5c0c7d73d5969ff18cecff269c`.
Runtime source SHA256: `641c16f1eeba814695a6f9a2442d636b61ec2a4c9555e4e9800a01c303225c3d`.

Eight focused tests pass, with exact names mapped in
[traceability](TRACEABILITY.md). They observe both IDs through direct and
messaging work, both failure-event wrappers and both finite schedule owners;
configuration revision/bytes remain visible alongside identity. They cover
in-place restart, registered/stopped/failed callback suppression, checked busy
and full queues, retained older diagnostics, typed source chains, exact selected
inbox clearing and healthy peer work/dispatch. Fault scenarios exercise both
selected positions. A due-work error consumes its item before the peer runs;
future direct work waits without a callback. No post-acceptance returned failure
is injected: wrapper failure scenarios reject the application's diagnostic on
its full queue and independently record the framework's error event.

The identical public-only tests pass in the actual independent package
`C:/Users/drrsm/AppData/Local/Temp/rff-work-identity-b0cf9a81c5b04d59854bb7844c5a2e34`.
The guide's commands execute with $framework set to this checkout and $consumer
set to that unused path:

```powershell
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/work_identity.rs" -Destination "$consumer/tests/work_identity.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test work_identity
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/work_identity.rs", "$consumer/tests/work_identity.rs"
```

All commands pass; the test target passes all eight. Actual manifest, lockfile,
metadata and hash inspection confirms one independent workspace member, two
graph packages, the sole framework path dependency and no library dependencies.
Copied and original source bytes and hashes match. The consumer imports only
public library APIs and std, with no private sample source inclusion.

Required local commands (rustdoc uses RUSTDOCFLAGS=-D warnings):

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test work_identity
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

All listed commands pass. Initial/final workspace totals are 168/176, with all
eight new regressions included. Focused and independent tests each pass eight.
The author also runs focused Clippy with warnings denied and a width audit
without waivers.
An initial compile typo dispatch_next is corrected to the existing dispatch_one
before the passing focused checks; no library workaround is introduced.

The active composition guide's stale configuration-only wording is corrected.
Its unchanged example passes four focused and four independent tests, check,
warnings-denied Clippy, executable run, metadata and source/output comparison.
Actual external package:
`C:/Users/drrsm/AppData/Local/Temp/rff-composition-20261024-c8de0301739a4657bca89899edd2258a`.
Both source SHA256 hashes are
`12eb88546081c9c0a67de4186cec0e73a9f9f6c893693df237db350bc7d8aef1`.
Metadata confirms one independent workspace member, two graph packages, the
sole library path dependency and no library dependencies. Both executable
stdout streams match the guide's three lines. The exact commands are the
unchanged recipe in [the composition guide](../COMPOSING_APPLICATIONS.md),
with an unused temporary destination; raw logs and commands are in the
run-scoped composition-results.md aid. This required conditional check is
current-library evidence, separate from its original checkpoint.

## Autonomous review and publication

Whole-tree review covers 47 Rust files, 19,499 physical/1,176 comment-only lines,
zero width findings and three unchanged fulfilled application-runtime
expectations. All 470 relative links in 85 Markdown files and 140 exact mapped
test names resolve. Thirteen changed rendered Markdown documents pass full
text, heading, code-block, table, link and HTML-structure inspection. This is
content/structure review, not pixel or human acceptance. Generated context API
documentation exposes the new accessor, its identity scope and configuration.
Workflow, host sample and ADR-0018/0019 probe sources are unchanged; actionlint,
extra host commands and standalone experiment probes are not triggered locally.
Hosted CI retains its configured separate host checks and sample execution.

Root Codex reads the full changed source and diff. Independent Codex
context_review checks the construction/delegation paths, requirement/ADR scope,
all final consumer source, source hashes, actual independent-package manifest,
metadata and raw check/Clippy/test logs. It accepts source and consumer evidence
with no outstanding correctness, source-form or claim findings, and separately
reads the 176-test baseline and warnings-denied documentation results. Review
corrects the guide's impossible same-queue wrapper borrowing suggestion and
active composition documentation's stale configuration-only description.

Root and independent context_review accept the complete 15-file staged
source/document/diff and actual baseline/consumer/rendered evidence with no
outstanding findings. Source hashes remain unchanged. Ordinary source
publication and exact hosted inspection remain pending; no new hosted result
is inferred from the input checkpoint.
