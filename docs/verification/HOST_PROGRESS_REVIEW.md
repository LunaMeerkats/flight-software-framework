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

## Periodic deferral follow-up

Date: **2026-11-14**

RFF-LIB-REQ-016 revision 2 and [ADR-0041](../adr/0041-host-observed-application-progress.md#periodic-work-under-downstream-backpressure)
compose the existing observer with a periodic cursor and one downstream
slot. The first observed blocked turn starts the pause. A later turn
with space preserves the elapsed time accrued before the pause and
polls the retained release. This demonstrates host observations and
local queue outcomes. The host must still regain control to observe work.

Input `93118f295cc89c9374e3429f2775939d6213f22b` matched refreshed origin.
[Exact input run 43367462734](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43367462734),
job 131695138725, passed all 20 steps and 268 workspace tests. The locked
local input baseline also passed with 268 workspace tests.

The local commands above passed with 270 workspace tests and five focused
tests. The same local Rust, Cargo, rustfmt and Clippy versions applied.
The two new histories check repeated deferral with retained input, sample
and release, a single late release, a waiting poll, the warning at the
adjusted threshold, and typed failure with cursor disablement at equality.
All three revision 1 histories also pass.

The independent-package commands above passed using
`C:/Users/drrsm/AppData/Local/Temp/rff-host-progress-09b8b356e5fc407dab37a3be1d4400c8`.
Offline locked check, Clippy, all five tests and metadata passed. Metadata
confirmed one independent workspace member and the sole framework path
dependency, with two graph packages. The 15,289 source bytes matched with
SHA256 `e2748f7913551a101fc50993e637decd4f3f5d461732e13651ee48107d912dfe`
and normalized Git blob `f1b516a18c9e1b16a22ad1dc9c53d47bd67aa2e1`.
Ignored raw logs and review aids are under
`target/review-2026-11-14-progress-deferral`.

Codex and an independent Codex reviewer accepted the full implementation
diff. The source audit covered 56 Rust files, 26,511 physical lines and
1,403 comment-only lines with zero width findings and three fulfilled
Clippy expectations. All 679 relative paths, 56 fragments and 234 exact
test references resolved. All ten changed Markdown files passed rendered
content and structure checks. The committed tree and test blob matched the
reviewed staged tree and independent consumer source.

Implementation `7a6150524c5bfdbf0bbfb8cee818e66a5dced280` reached
`origin/main` by ordinary fast-forward push.
[Exact hosted run 43431041267](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43431041267),
job 131904305232, passed all 20 steps with 270 workspace tests including
all five host progress tests, 14 adapter tests, five sample tests,
warnings-denied rustdoc and both whitespace checks. The combined host
sample matched the input's 222 lines. Hosted Rust/Cargo 1.100.0,
rustfmt 1.11.0-stable and Clippy 0.1.100 ran on runner 2.338.2 and
`windows-2025-vs2026` image `20261103.298.1`, matching input.

Codex accepts the periodic consumer checkpoint. The focused periodic
failure test covers a callback error. Cursor disablement after a lifecycle
rejection follows the same error branch by source review. The periodic
consumer depends on a returning callback and host turn timestamps.

## Downstream congestion follow-up

Date: **2026-11-15**

[RFF-LIB-REQ-016 revision 3](../REQUIREMENTS.md#rff-lib-req-016--observe-application-progress-on-host-turns)
and [ADR-0041](../adr/0041-host-observed-application-progress.md#sustained-downstream-congestion)
add one host warning attempt per observed deferral. A selected ten-second
threshold serves this consumer. The pending sample, next release, input and
sample-window state survive the warning branch. The warning uses the same
FIFO as sample warnings and runtime failure events.

Input `3acc1a9deb1ac0d113d96e6bd27908eaaa0878c2` matched refreshed origin.
[Exact input run 43431354539](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43431354539),
job 131905360022, passed all 20 steps with 270 workspace tests. The locked
local input baseline also passed with 270 workspace tests.

The local verification commands above passed with 272 workspace tests and
all seven host progress tests. Local tool versions were unchanged. The
focused histories check warning time and fields at equality and on a late
turn, queue acceptance and rejection, suppression, rearming, retained
state and draining at equality. The five prior histories also pass.
Additional local commands passed:

```powershell
cargo test --locked --test host_adapters
cargo test --locked --test host_sample
cargo run --locked --example host-echo
```

The adapter target passed 14 tests and the sample target passed five.
The complete 222-line local sample transcript matched the input.

The independent-package commands above passed using
`C:/Users/drrsm/AppData/Local/Temp/rff-host-progress-9c8e134d45b04c7280f834db78a7fa8d`.
Offline locked check, Clippy, all seven tests and metadata passed. Metadata
confirmed one independent workspace member and the sole framework path
dependency, with two graph packages. The 20,260 source bytes matched with
SHA256 `4476911cab3b973411aad3cbd18cf00a6cdf7a9ac0bc6c9f9cd5f7c1e473758d`
and normalized Git blob `06d42a0dffb1c25539e4dd64f712f083487923db`.
Ignored raw logs are under `target/review-2026-11-14-congestion`.

The new histories exercise the sample window through a state check on the
warning branch. The preceding shorter deferral history covers an empty
resumption and the adjusted sample threshold.

Codex and an independent Codex reviewer accepted the full implementation
diff. The audit covered 56 Rust files, 26,659 physical lines and 1,404
comment-only lines with zero width findings and three unchanged fulfilled
Clippy expectations. All 686 relative paths, 61 fragments and 236 exact
test references resolved. All ten changed Markdown files passed rendered
content and structure checks. The committed tree and test blob matched
the reviewed staged tree and verified consumer source.

Implementation `7e1a96738e5cab3b76936cdc2f2c5b45e0895065` reached
`origin/main` by ordinary fast-forward push.
[Exact hosted run 43597998608](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43597998608),
job 132456512701, passed all 20 steps with 272 workspace tests including
all seven host progress tests, 14 adapter tests, five sample tests,
warnings-denied rustdoc and both whitespace checks. The combined host
sample matched the input's 222 lines. Hosted Rust/Cargo 1.100.0,
rustfmt 1.11.0-stable and Clippy 0.1.100 ran on runner 2.338.2 and
`windows-2025-vs2026` image `20261103.298.1`, matching input.

Codex accepts the serial consumer checkpoint. The warning measures
elapsed time between host observations of the occupied slot. Event
acceptance establishes local queue storage. A full queue can reject a
warning or a later failure event.
