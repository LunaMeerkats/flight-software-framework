# Startup and cleanup consumer review

Date: **2026-11-28**
Reviewer: **Codex**, under ADR-0024/0025 routine autonomous review authority.
Scope: RFF-LIB-REQ-023 revision 1 and ADR-0048.

## Input and decision

Clean main `82ed58a04aba1bcc0660c3bc386902ff3e0654ac` matches refreshed and
live origin/main. The locked input baseline passes 332 tests and all static/
document checks. Exact input run 45933553084/job 140291456844 passes all
20 steps at the logged checkout, including separate 14 adapter/five sample
tests. The 222-line combined sample is saved for comparison after publication.

Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.
Input hosted tools are Rust/Cargo 1.100.0, rustfmt 1.11.0-stable, Clippy
0.1.100, runner 2.338.2 and image `20261123.321.1`.

The composition example uses infallible lifecycle callbacks and immediate
error propagation. The new consumer closes the partial-start cleanup gap:
retain the original typed error, reverse the successful prefix and continue
cleanup while retaining secondary failures. Existing repository contracts
resolve the design; no external research or source reuse is needed.
ADR-0048 compares generic orchestration, fail-fast policies, messaging and
another diagnostic wrapper. No library API, dependency, lint or CI change.

## Local verification

Every command below completes successfully:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --example startup_cleanup
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo run --locked --example startup_cleanup
git diff --check
```

RUSTDOCFLAGS is `-D warnings`. The workspace passes 339 tests, including
seven new example histories. The author also runs
`cargo check --locked --example startup_cleanup --all-features` and
`cargo clippy --locked --example startup_cleanup --tests -- -D warnings`.
All checks pass after draft duplicate code, transcript length and function-size
issues are corrected, without a suppression or baseline policy change.

The tests cover complete startup followed by explicit reverse shutdown;
first/middle/last startup failure with untouched suffix and retained failed
application effects; multiple original non-Copy errors and standard source
chains surviving runtime destruction; successful earlier cleanup after a
failed stop; lifecycle-rejected startup excluded from cleanup; and all-error
normal shutdown. The executable's entire ten-line stdout is byte-exact in
its test. Trace observations and report identity/results remain separate.

## Independent consumer

Executed the [guide recipe](../STARTUP_CLEANUP.md#verify-identical-source-independently)
at `C:/Users/drrsm/AppData/Local/Temp/rff-startup-cleanup-0298a82756174a93ad6360a8bc4cd393`.
After cargo new/add, these commands pass with that absolute directory substituted for `<consumer>`:

```text
cargo check --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets
cargo clippy --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets -- -D warnings
cargo test --manifest-path <consumer>/Cargo.toml --locked --offline
cargo run --manifest-path <consumer>/Cargo.toml --locked --offline
cargo metadata --manifest-path <consumer>/Cargo.toml --locked --offline --format-version 1
```

All seven identical tests pass. Metadata confirms one independent edition-2024
binary member, its sole framework path dependency, two graph packages and no
framework dependencies. Exact source bytes match: 21,276 bytes / 596 lines,
SHA256 `4135d39c4275db23bbcee63245ea497dc6eacaf39eeae3aeb54af3b0f4308c47`.
Workspace and independent stdout match all ten lines. Success ends with three
Stopped states; the demonstrated startup/cleanup error path ends with three
Failed states. No failed-start compensation is claimed.

## Review and publication boundary

Root and independent Codex accept the complete 14-file source/document/diff
review without findings. Private token construction bounds the prefix at
zero through three; cleanup stores each Result and continues after failure.
Main flow precedes incidental helpers. No existing Rust consumer, library
source, lockfile, lint, dependency or workflow changes.

The whole-tree audit covers 63 Rust files / 31,617 physical lines and 1,637
comment-only lines, with zero width findings and three unchanged fulfilled
expectations. Across 139 Markdown files, all 880 relative paths, 101 fragments
and 299 unique exact test references resolve. All 12 changed documents pass
rendered content/structure review, including manual inspection of guide,
ADR and focused evidence. This does not claim browser pixel QA. Raw commands,
hashes, metadata and temporary review aids are retained under ignored
`target/review-2026-11-28/`; no new checker is adopted. Authorized publication
and exact hosted inspection remain separate from this local acceptance.

## Limits

The fixed helper assumes distinct same-runtime IDs in its intended order and
initial states. A consumed token prevents ordinary repeated cleanup; it does
not validate runtime origin or external lifecycle changes. Callbacks must
return. Failed startup may leave application effects; Failed cannot be stopped
or restarted through ordinary lifecycle operations. Stop success records the
callback result, not independent resource cleanup. No transaction, recovery,
compatibility, general adoption or release-readiness evidence is claimed.
