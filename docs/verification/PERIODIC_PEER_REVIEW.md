# Periodic peer work review

Date **2026-11-18**. Scope [RFF-LIB-REQ-017 revision 1](../REQUIREMENTS.md#rff-lib-req-017--continue-periodic-peer-work-after-a-returned-failure)
and [ADR-0042](../adr/0042-periodic-peer-work-after-failure.md).

## Input and local verification

Input `ca32a428314055f1b2f4eae5214462c6ab129fdb` matched refreshed
`origin/main`. Its [exact hosted run 43873841092](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43873841092),
job 133321631099, passed all 20 steps. The local input passed 276
workspace tests.

The following checks passed on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable
and Clippy 0.1.98. `RUSTDOCFLAGS=-D warnings` applied to the doc build.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo test --locked --example periodic_peer
cargo clippy --locked --example periodic_peer -- -D warnings
cargo run --locked --example periodic_peer
cargo test --locked --test host_adapters
cargo test --locked --test host_sample
cargo doc --locked --workspace --all-features --no-deps
cargo run --locked --example host-echo
git diff --check
```

There were 278 workspace tests, two focused example tests, 14 focused
adapter tests and five focused sample tests. The combined host sample
reproduced the input's 222 lines. The focused tests verify successful
failure-event admission and queue-full rejection in separate histories.
Across both, the selected application is Failed and its cursor is disabled.
The admitted history measures the original peer release and three successful
heartbeat callbacks across a separate diagnostic drain. The rejected history
verifies the original cause, rejected event attribution and retained warning.

## Independent binary

The [consumer recipe](../PERIODIC_PEER.md#verify-the-independent-binary)
passed at
`C:/Users/drrsm/AppData/Local/Temp/rff-periodic-peer-c5f188ddb13b4ffa8c5ae450326913ba`.
Locked offline check, Clippy, both tests, run and metadata succeeded.
Metadata confirmed one edition 2024 binary workspace member and only
the framework path dependency, with two packages in the resolved graph.
The 9,878 source bytes matched with SHA256
`c26ddd18dce5c8d4e631971a1b89a91a801b0034bd9b19609c3e52993bf4da2e`
and normalized Git blob `022c48c3ce3441018cd16e2d30294b55b5155fed`.
Both binaries emitted identical six-line, 233-byte stdout with SHA256
`b418960ccf5780482ee6b48a1362e57fa6e1302613a7120058fafa024211b402`.

## Review scope

Codex and an independent Codex reviewer accepted the complete implementation
diff. Review covered 57 Rust files, 27,205 physical lines and 1,411
comment-only lines with zero width findings and three unchanged fulfilled
Clippy expectations. All 726 relative paths, 74 fragments and 242 exact
test references resolved. The 11 changed Markdown files passed rendered
content and structure checks.

The comparison is limited to this manually advanced serial host with two
application records and one terminal failure. The host must keep polling the
healthy cursor, and callbacks and writes must return. The acceptance evidence
covers event storage and local dequeue.

Raw local results are under the ignored `target/review-2026-11-18-peer-work`
and `target/review-2026-11-18-document-audit` directories.

## Publication and hosted verification

Implementation `ac5b960a5e56b49cbc3a874cfbde2ce38311956b` reached
`origin/main` by ordinary fast-forward push. Its commit tree matched the
reviewed staged tree and the committed example matched the independent
consumer's Git blob.

[Exact hosted run 44089160905](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/44089160905),
job 134059454583, passed all 20 steps with 278 workspace tests including
both periodic peer tests, 14 adapter tests, five sample tests,
warnings-denied rustdoc and both whitespace checks. The combined sample
reproduced the input's 222 lines. Hosted Rust/Cargo 1.100.0, rustfmt
1.11.0-stable and Clippy 0.1.100 ran on runner 2.338.2 and
`windows-2025-vs2026` image `20261103.298.1`.

Codex accepts RFF-LIB-REQ-017 revision 1 for the recorded serial consumer.
The hosted test harness ran the two focused tests. The executable's
six-line trace was verified locally in both packages.

## Two-failure retry follow-up

Date **2026-11-19**. RFF-LIB-REQ-017 revision 2 and the
[ADR follow-up](../adr/0042-periodic-peer-work-after-failure.md#two-retained-failure-events--2026-11-19)
extend the same consumer. Input `c2da285952d99710adb61eb76ca872213e32edbd`
was clean and matched refreshed origin. Exact input push run 44089559879,
job 134060813535, passed all 20 steps and 278 workspace tests. The input
local locked 278-test baseline passed with the same local tools listed above.

### Behavior and evidence

The host registers two independently identified samplers and one healthy
heartbeat. Two inline optional event slots retain only QueueFull work-failure
attempts, in host observation order. An explicit retry offers one oldest
record; full preserves both and acceptance shifts only the remaining record.
Each failing cursor is disabled before capture. Only the two samplers can
return work errors, each at most once, so the fixed mission cannot overwrite
an occupied retry slot. This proof must be revisited if mission roles or
cursor re-enabling change.

The original two histories remain. The admitted-event history also checks
an empty retry. `two_rejected_failures_retry_in_observation_order_without_duplicates`
checks both role orders, exact typed causes and records, distinct original
timestamps, repeated full retries, healthy work with all event slots occupied,
one successful admission per turn, and repeated disabled/empty turns without
duplicates. `lifecycle_rejection_disables_only_selected_cursor_without_retry`
checks exclusion of a stopped application's lifecycle error. The runnable
trace test fixes the original six-line prefix and two completed retry admissions.

The locked local baseline passed 281 workspace tests, all-target check,
warnings-denied Clippy/rustdoc, formatting and whitespace. The focused example
passed five tests and ran successfully; example-only warnings-denied Clippy
also passed. Exact commands were:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --example periodic_peer
cargo clippy --locked --example periodic_peer -- -D warnings
cargo run --locked --example periodic_peer
cargo run --locked --example host-echo
git diff --check
```

`RUSTDOCFLAGS=-D warnings` applied to the doc command. The workspace suite
included 14 adapter and five sample tests. The combined sample matched the
input's 222 output lines. Separate adapter/sample commands were not repeated
locally because their source was unchanged; hosted CI runs them separately.
Initial focused compilation caught a missing test-only Clock import and an
unchecked must-use emission; both were corrected before the accepted runs.

### Independent binary and review

The unchanged [independent recipe](../PERIODIC_PEER.md#verify-the-independent-binary)
passed at
`C:/Users/drrsm/AppData/Local/Temp/rff-periodic-peer-430e490645ea4a7db217808fba91ffce`.
Commands used that path as `$consumer`: `cargo new --bin --edition 2024 --vcs
none $consumer`, then `cargo add --manifest-path "$consumer/Cargo.toml"
rust-flight-framework --path $framework --offline`, and an exact source copy.
Locked offline check and Clippy used `--all-targets`; Clippy denied warnings.
Locked offline test, run and `metadata --format-version 1` passed. Metadata
confirmed one edition-2024 binary workspace member, the framework as sole path
dependency and two packages in the graph.

Source bytes match exactly: 19,231 bytes, SHA256
`35682eda950c7b81a57eb8a117ff30880d86f3ef793174b5465bac11b5d1b877`,
normalized Git blob `83886bee28bbcb7e36fd7ac3ef030ab24495ab5c`.
All five independent tests pass. Workspace and independent native stdout match
all 14 lines / 817 bytes, SHA256
`7c11489344b9e63ef698251f17b4d5c3288ee686b8ae81801ba682c9de610b3c`.
The input's native six-line / 233-byte stdout is an exact prefix. The older
saved sample text used CRLF; combined-sample comparison therefore compares
its 222 lines rather than claiming raw byte equality across line endings.

Codex and an independent Codex reviewer inspected the complete source and
documentation diff, including the fixed capture bound, ordering, error
preservation, ownership, source form and test scope. This verifies the fixed
cooperative host policy, not a general retry queue or occurrence ordering
across all fresh and retained events. No new library API, dependency, lint
waiver or workflow change is made. Callbacks/writes must return and the
caller must select retry. Queue acceptance and dequeue establish no durable
or external delivery.

Whole-tree review covers 57 Rust files, 27,421 physical and 1,414 comment-only
lines with zero width findings and three unchanged fulfilled expectations.
All 737 relative paths, 81 fragments and 245 exact test references in 121
Markdown files resolve. All 12 changed Markdown files pass rendered content
and structure inspection, including text, headings, code, links, tables,
nesting and encoding. This is structural/content review without browser
pixel-layout QA. Temporary audit commands were:

```text
python target/review-2026-11-19-peer-retry-audit/inventory.py
python target/review-2026-11-19-peer-retry-audit/render.py c2da285952d99710adb61eb76ca872213e32edbd
python target/review-2026-11-19-peer-retry-audit/inspect_rendered.py c2da285952d99710adb61eb76ca872213e32edbd
```

Raw evidence is in ignored `target/review-2026-11-19-peer-retry` and
`target/review-2026-11-19-peer-retry-audit`. Publication results are recorded
after their actual completion.
