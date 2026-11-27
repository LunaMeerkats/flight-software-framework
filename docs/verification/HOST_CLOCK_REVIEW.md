# Host elapsed-clock integration review

Date: **2026-11-27**
Reviewer: **Codex**, under ADR-0024/0025 routine autonomous review authority.
Scope: RFF-LIB-REQ-022 revision 1 and ADR-0047.

## Input and decision

Clean main `158ce34cd10517e52e0aa3548629e5e1ee43853a` matches refreshed
origin/main. The locked input baseline passes 324 tests and every static/
documentation check. Exact input run 45783611865/job 139784811815 passes
all 20 steps at the logged checkout; all 222 combined-sample lines are retained.
GitHub connector job summaries/logs and saved raw metadata/logs agree.

Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.
Input hosted tools are Rust/Cargo 1.100.0, rustfmt 1.11.0-stable, Clippy
0.1.100, runner 2.338.2 and image `20261116.315.1`.

Every runnable scheduling consumer previously used ManualClock. HostClock
closes the host-time adapter gap with one private native origin and existing
public interfaces. ADR-0047 compares consumer-local conversion, SystemTime,
regression clamping, fallible clocks and runtime-owned loops. Official Rust
Instant/sleep documentation informs only the conversion and host wait limits.
No dependency, service owner, workflow or lint-policy change is introduced.

## Local verification

Every final command below completes successfully:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --lib clock::tests
cargo test --locked --example host_clock
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo run --locked --example host_clock
git diff --check
```

RUSTDOCFLAGS is `-D warnings`. The full workspace passes 332 tests. Five
focused clock tests and three example tests pass. Four synthetic mapping
histories check seconds/nanoseconds,
equal observations, pre-origin saturation and unclamped regression. The
actual Clock entry point is bracketed between native samples using a synthetic
past origin, without waiting for positive elapsed time. The example's same
polling turn is checked with manual time for waiting/equality, no duplicate
work, overdue coalescing and consumed release on lifecycle rejection. Its live
test uses new/Default without sleeps or timing thresholds.

## Independent consumer

Executed the [guide recipe](../HOST_CLOCK.md#verify-identical-source-independently)
at `C:/Users/drrsm/AppData/Local/Temp/rff-host-clock-ad8eff284c784c89b93410b4006e8d20`.
After cargo new/add, all commands below pass with that absolute path
substituted for `<consumer>`:

```text
cargo check --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets
cargo clippy --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets -- -D warnings
cargo test --manifest-path <consumer>/Cargo.toml --locked --offline
cargo run --manifest-path <consumer>/Cargo.toml --locked --offline
cargo metadata --manifest-path <consumer>/Cargo.toml --locked --offline --format-version 1
```

All three identical tests pass. Metadata confirms one independent edition
2024 binary member, its sole framework path dependency, two graph packages
and no framework dependencies. Exact source bytes match: 6,198 bytes / 176
lines, SHA256
`94cd4c92297a4d702a06a2e11f5d702261ee817480d5f50c4c902a2ce18da1ca`.
Workspace and independent stdout each report five turns, three work calls and
Stopped. The check accepts counts one through five, rather than requiring
identical live counts. No minimum elapsed time or wall-clock deadline is tested.

## Review and publication boundary

Root and independent Codex source/design/documentation review finds no
actionable defect. The adapter stores one native origin, samples once per
read, and performs explicit saturating subtraction. It retains no time history
and does not start work. ManualClock and existing consumers retain their code.
Root and independent Codex accept the complete 16-file increment without
findings. Whole-tree review covers 62 Rust files / 31,021 physical lines and
1,625 comment-only lines, with zero width findings and three unchanged fulfilled
expectations. All 859 relative paths, 100 fragments and 292 exact test references
across 136 Markdown files resolve. All 12 changed documents pass rendered
content/structure review, and the generated HostClock API page is inspected.
This does not claim browser pixel QA. Exact publication evidence remains
pending. Raw logs and temporary review aids are retained under ignored
`target/review-2026-11-27/`; no new source checker is adopted.

Platform clock anomalies may regress elapsed observations; saturation does
not recover the clock. Same-origin pairing remains caller-owned. Finite turns
bound callbacks, not elapsed runtime; clock, callback, wait and output calls
must return. This is controlled consumer evidence, not general adoption,
API stability, real-time behavior, qualification or release readiness.
