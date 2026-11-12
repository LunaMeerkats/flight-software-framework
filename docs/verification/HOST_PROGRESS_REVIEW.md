# Host progress observation review

Date: **2026-11-13**

## Objective and input

[RFF-LIB-REQ-016](../REQUIREMENTS.md#rff-lib-req-016--observe-application-progress-on-host-turns)
and [ADR-0041](../adr/0041-host-observed-application-progress.md) define a
serial host policy for observing overdue output across completed work turns.
Input `336ecfa17a22fabe93cd376e84f9194a1eaa8fbe` matched refreshed origin.
[Exact input run 43045448136](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43045448136),
job 130608057591, passed all 20 steps and 265 workspace tests. The locked
local input baseline also passed with 265 workspace tests.

## Contract and evidence

The [consumer](../../tests/host_progress.rs) uses one manual clock, one
application record, one optional sample, one window start, one warning
flag and one event queue entry. It uses public framework APIs and std.
The three histories verify fresh progress at the inclusive threshold,
the reset window, suppression and rearming, observation on a late turn,
rejection at capacity, typed failure and later lifecycle rejection.

The two-second window and one-attempt rule are consumer choices. Queue
acceptance establishes local storage. A rejected warning stays visible
in the turn result. Progress observation requires a completed callback
and depends on the host choosing when to call work.

## Exact local verification

Local tools were Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.
The locked baseline, warnings-denied documentation and focused test passed.
The workspace passed 268 tests including all three focused tests.

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
$env:RUSTDOCFLAGS = '-D warnings'
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test host_progress
git diff --check
```

## Independent package

The [guide recipe](../HOST_PROGRESS.md#verify-the-consumer-outside-the-workspace)
ran from this checkout with the following package and manifest.

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = 'C:/Users/drrsm/AppData/Local/Temp/rff-host-progress-3123b8d0f894468d9e5fe1a26cac43c1'
$manifest = "$consumer/Cargo.toml"
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path $manifest rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/host_progress.rs" -Destination "$consumer/tests/host_progress.rs"
cargo check --manifest-path $manifest --locked --offline --all-targets
cargo clippy --manifest-path $manifest --locked --offline --all-targets -- -D warnings
cargo test --manifest-path $manifest --locked --offline --test host_progress
cargo metadata --manifest-path $manifest --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/host_progress.rs", "$consumer/tests/host_progress.rs"
[System.Linq.Enumerable]::SequenceEqual[byte](
    [System.IO.File]::ReadAllBytes("$framework/tests/host_progress.rs"),
    [System.IO.File]::ReadAllBytes("$consumer/tests/host_progress.rs")
)
```

Creation, dependency setup, source copy, check, lint, all three tests,
metadata and hashing passed. Metadata has one independent workspace member
and two graph packages. The sole dependency is this checkout's framework
library. Windows metadata returned native path separators, so the
workspace and dependency paths were checked in normalized form. Source
bytes match with SHA256
`af2ca80ef1218e19ea0389d54f23af965ee8ddfe4f4d7f37be1547b2ee9afdfd`
and normalized Git blob `2fe5df093fe4922d0d08b2a072eb59de307b54dd`.

## Source review and publication

Codex and an independent Codex reviewer accepted the complete implementation
diff. The audit covers 56 Rust files, 26,331 physical lines and 1,402
comment-only lines, with no width findings and three unchanged fulfilled
Clippy expectations. All 721 relative paths, 48 fragments and 232 exact test
references resolve. All 11 changed Markdown files passed rendered content
and structure review. The committed tree and test blob match the reviewed
staged tree and verified consumer source.

Implementation `d8b77724ea1b63a954c5becb64371c8e37de35ea` was published
by an ordinary fast-forward push to `origin/main`.
[Exact hosted run 43225151848](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43225151848),
job 131206316119, passed all 20 steps, including 268 workspace tests
with all three host progress tests, 14 adapter tests, five sample tests,
warnings-denied rustdoc and both whitespace checks. The combined host
sample reproduced the input's 222 lines. Hosted Rust/Cargo 1.99.1,
rustfmt 1.10.0-stable and Clippy 0.1.99 ran on runner 2.338.2 and
`windows-2025-vs2026` image `20261103.298.1`, matching input.

Codex accepts the completed-turn consumer checkpoint. The record revision
receives separate hosted inspection. Ignored raw logs and review aids are
under `target/review-2026-11-13-host-progress`.
