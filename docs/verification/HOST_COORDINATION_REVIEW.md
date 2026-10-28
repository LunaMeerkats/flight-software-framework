# Recurring host coordination review

Date: **2026-10-29**
Status: **Local and independent-consumer verification passed; publication pending**

## Objective and input

RFF-LIB-REQ-008/ADR-0033 extends the existing public-only work_output consumer
with bounded recurring host turns under downstream backpressure. Root Codex and
independent consumer_assessment compare paused polling, dropped blocked releases
and a framework runner before implementation. Requirements/decision/plan precede
source changes. No library API, dependency, workflow or configuration changes.

Clean input c899aef39a8df854d14b48484bd044ee14bba120 matches refreshed origin.
Fresh locked format/check/Clippy/test/rustdoc and both whitespace checks pass,
including 201 workspace tests. Exact input hosted run 40511047519/job 122212738703
passes all 20 steps. Root raw-log/metadata review and connector steps agree.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
Raw logs and temporary review aids stay under ignored
target/review-2026-10-29-host-coordination.

## Acceptance boundary

One turn attempts at most one eligible sink dispatch, one pending transfer and
one poll/work invocation. The caller drains output separately. Pending output
blocks polling; newly produced output remains pending for a later turn. The
pipeline has one host output, one sink-inbox message and one pending record.
Configuration changes preserve older pending provenance and affect resumed work.
A work error disables recurrence and retains the error source; earlier effects
remain committed. Sole-subscriber retry and caller-paired IDs/slots are required.

## Verification and tests

The following commands pass on the final implementation, with RUSTDOCFLAGS set
to -D warnings before rustdoc:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --example work_output
cargo run --locked --example work_output
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

The workspace passes 208 tests; all 17 focused tests pass. The original ten tests
are preserved. Seven new tests check the three-slot pipeline, zero blocked clock
reads and retained release, exact FIFO drain/resumption at 10 seconds re-armed
for 11, new configuration revision without overwriting old output, stopped-sink
retention/restart, lifecycle rejection after committed earlier effects, concrete
returned-work error/source and timer disable, topology failure after possible
delivery, and waiting/exhausted/disabled states. The runnable output adds:

```text
recurring work with deferred host drain -> sink: [42, 7, 9, 11]
```

The author corrected InboxFull variant naming, a collapsible conditional and a
62-line test before passing warnings-denied checks. No new waiver is needed.
An initial parent verification wrapper expanded an argument incorrectly; using
the guide's explicit commands fixes the wrapper and all checks pass. No product
failure is hidden by these corrections. Conditional adapter/sample, workflow and
borrowing-experiment commands are not triggered by the touched scope; hosted CI
still executes its focused host targets and sample.

## Independent package and source identity

Actual package:
C:/Users/drrsm/AppData/Local/Temp/rff-work-output-9c2f51d97fd74f1891533770de2a29f2.
The complete [guide recipe](../ORDINARY_WORK_OUTPUT.md#verify-the-independent-package)
executes cargo new/add, source copy, the following checks and metadata/hash reads:

```powershell
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/work_output.rs", "$consumer/src/main.rs"
```

Creation/add/copy/check/Clippy/17 tests/run pass. Normalized metadata confirms one
independent workspace member, two graph packages, sole framework path dependency
and zero library dependencies. Source bytes match; both SHA256 values are
1d74670e92bb65b6ad77f830fcd018ce4c4a7ca643958f6693ed1fded5e0fd6d.
The independent and workspace runs print the same four stdout lines, retaining
the safety notice and original ordinary/scheduled [42, 7] result.

## Source, document and publication review

Root Codex and independent coordination_review inspect source, ownership, errors,
operation counts, tests and existing contracts. Whole-tree review covers 51 Rust
files, 21,739 physical and 1,290 comment-only lines with zero width findings and
three unchanged fulfilled expectations. No library, manifest, lockfile, licence,
workflow or source-quality configuration changes. All 545 relative links and
172 exact test references resolve across 96 Markdown files. All 12 changed
rendered documents preserve content, headings, code, links, tables and nesting;
this is content/structure inspection, not pixel or human acceptance.

Root Codex and independent coordination_review accept the complete 13-file
source/document/diff and actual raw evidence with no remaining findings. The exact accepted patch is retained in ignored
review aids. Ordinary fast-forward publication and exact hosted inspection follow
local acceptance; no hosted pass is claimed here before that execution. Previous
evidence stays historical in [the original review](WORK_OUTPUT_REVIEW.md).

## Limitations

This is controlled public-API composition, not external adoption or API stability.
Operation counts and logical storage do not bound process memory, allocation,
blocking or callback duration. Cell is serial interior mutation, not
synchronization. Paused polling offers no fairness or drain deadline. Accepted
inbox/output values do not establish physical delivery. Effects are not
transactional; panics, hangs, process loss and allocator failure remain outside
the cooperative-error boundary. No qualification, operational, release or
compatibility claim is established. Separate consequential gates remain.
