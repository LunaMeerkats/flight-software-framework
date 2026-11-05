# Runnable command-session review

Date: 2026-11-05 Australia/Sydney
Scope: RFF-LIB-REQ-014 revision 1 and ADR-0039
Status: Verified implementation checkpoint; complete Codex review and exact hosted pass

## Selection and input evidence

Root Codex and independent consumer_assessment selected the missing executable
composition of ingress, local results and reply output. Requirements, plan and
ADR precede source edits. Independently author a public-only tested example;
preserve historical callback/correlation test sources exactly. Extracting the
asserting fixture would couple operational guidance to test instrumentation;
existing public APIs suffice without a new framework service.

Official Cargo targets and Rust Cell/Rc/checked_add documentation inform target
placement, copied serial slots and checked exhaustion. See
SRC-RUST-COMMAND-SESSION. Grammar, capacities and host policy are local choices;
no upstream implementation or dependency is reused.

Clean input main faab18be2fb06a4bec43f0889338409c5b71b26a matches fetched origin.
Initial locked format/check/Clippy/243 workspace tests/warnings-denied rustdoc and
both whitespace checks pass. Local tools remain Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98. Independent verification_support inspects
raw logs/metadata and connector summaries for exact input run 41642653583,
job 125939797733: all 20 steps, 243 workspace/14 adapter/five sample tests, rustdoc
and whitespace pass. The 222-line sample equals prior run 41451380499.
Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.338.2 and
windows-2025-vs2026 image 20261030.294.1 remain separate from local evidence.

Initial whole-tree review: 54 Rust files/24,240 physical/1,362 comment-only lines,
zero width findings and three existing expectations; 109 Markdown files/625 relative
links/207 exact test references resolve. Temporary review scripts and raw logs
are under target/review-2026-11-05-command-session, outside maintained source.

## Commands and acceptance

Initial baseline commands completed successfully, with
$env:RUSTDOCFLAGS = "-D warnings" before rustdoc:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

## Executable consumer evidence

The new standalone examples/command_session.rs imports only public framework
APIs and std. Borrowed host Cell slots avoid shared-slot heap ownership. The
producer is registered after its two subscribers, supplying known destinations
without late-bound shared routing storage. Command inbox capacity is two; both
subscriber inboxes have capacity one; message payload bound is three bytes.
Every host turn dispatches at most one callback and pauses on its occupied slot.

These focused commands pass, including ten tests and a 24-line finite transcript:

```text
cargo fmt --all
cargo test --locked --example command_session
cargo run --locked --example command_session
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
```

The tests are:

- `finite_session_prints_exact_results_and_separate_subscriber_observations`.
- `occupied_result_preserves_command_fifo_and_rejected_host_request`.
- `occupied_output_preserves_older_reply_and_gates_next_subscriber_dispatch`.
- `malformed_ingress_is_rejected_before_queue_or_result_effects`.
- `malformed_callback_bypass_retains_peer_effect_and_reports_exact_cleanup`.
- `direct_producer_bypass_preserves_result_before_terminal_cleanup`.
- `direct_subscriber_bypass_preserves_old_output_and_peer_reply`.
- `matching_cancel_preserves_accepted_copy_and_stale_control_preserves_new_operation`.
- `restart_retains_result_operation_and_issuer_while_clearing_queued_command`.
- `session_issuer_uses_all_nonzero_ids_and_rejects_repeated_exhaustion`.

The exact transcript observes partial acceptance, queued busy New behind the
occupied result gate, explicit drain, Busy feedback, rejected-only retry and
same-ID resubmission. Separate tests observe ingress/result/output saturation,
FIFO, malformed ingress versus defensive callback failure, full-slot bypass
cleanup, correlation/cancellation, restart retention and all 255 IDs plus repeated
exhaustion. No test-only pending observation controls operational host policy.
Both tests/callback_publication.rs and tests/correlated_commands.rs retain exact
input source, including all earlier histories.

During implementation, the compiler rejected the initial wrong InboxEmpty enum
spelling and a width review found an 81-column comment; both were corrected.
Root and independent consumer_assessment found two driver enqueue calls ignoring
DeliveryStatus. Shared admit now checks/prints every admission; the final exact
transcript covers those paths. No open finding remains. No threshold, waiver,
library source, dependency, lockfile, licence or CI workflow changes.

## Independent binary

Actual package:
C:/Users/drrsm/AppData/Local/Temp/rff-command-session-992b65f2ad074bdab39b0fc34e4cc44c.
The guide's exact recipe was executed with $consumer set to that package and
$framework to this checkout, stopping after each command failure:

```powershell
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/command_session.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
```

All pass, including ten identical tests and exact 24-line stdout equality with
the workspace example. Metadata confirms one independent workspace member, two
packages, sole framework path dependency and no framework dependencies. Exact
source bytes match; SHA256:
3214993a9396f63e2016263a343956ff9e0087bff935de76d75c6fbb0e4395b4.
Canonical source blob: 3d79febf338feff3b395aec8d6a4e28749ca7d54.
Copy-Item/Get-FileHash also confirm the identical independent main source and
both SHA256 values; the temporary runner compares exact bytes directly. This verifies this consumer only.

## Final local baseline and review

The seven baseline commands above pass again on final source: 253 workspace tests,
including the ten new example tests, and warnings-denied rustdoc. Both prior
consumer sources remain unchanged. Conditional host-adapter/sample and standalone
ADR probes are not required by unchanged components; hosted CI retains its
configured separate host checks.

Root Codex and independent consumer_assessment accept the complete example at the
hash/blob above after reading all behavior/tests and source form. The independent
reviewer does not rerun Cargo and distinguishes root/implementation execution
evidence from source review. Root and independent verification_support accept
the complete 14-file source/document/evidence diff with no open findings.
Whole-tree review passes: 55 Rust files, 25,153 physical/1,371 comment-only
lines, zero width findings and three unchanged fulfilled expectations; 112
Markdown files, 643 relative links and 217 exact test references resolve.
All 12 changed documents render and pass content/structure inspection: headings,
code blocks, links, tables, prose, nesting and encoding match source. This is
rendered-content review, not pixel or human review. Source configuration retains
the reviewed local policy; no toolchain change or new waiver is introduced.
Minor prose wrapping is corrected before staging. Independent final staged
review accepts patch SHA256
5f5a4c66f572c429acd1db5e83a65337368a3c82212117128f614b4e945c167b.
The actual committed implementation patch compares byte-for-byte equal.

## Publication and hosted acceptance

Implementation 3d171dc22d9c2661baf4d077871ebc60cad83003 is published by ordinary
fast-forward to origin/main; the committed patch equals the accepted staged
patch exactly. Exact [push run 41855764215](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/41855764215),
job 126663830711, passes all 20 steps: 253 workspace tests including ten new session
cases and unchanged earlier consumers, 14 adapter/five sample tests,
warnings-denied rustdoc, both whitespace checks and the unchanged 222-line sample.
Root and independent verification_support accept actual checkout/log/source
metadata and the hosted result. Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable,
Clippy 0.1.99, runner 2.338.2 and windows-2025-vs2026 image 20261030.294.1
match the input run. Local tools remain separate on 1.98.0. This record-only
completion changes no source or configuration and receives separate complete
review, publication and exact hosted inspection in automation memory/report.

Completion rechecks preserve the source/hash/metadata boundary, with the same
55 Rust files and three fulfilled expectations; 112 Markdown files, 644 relative
links and 217 exact test references resolve. All 13 run-changed documents pass
rendered content/structure inspection. Only seven evidence documents change;
Cargo is not rerun locally for this record-only patch because its authenticated
source/configuration inputs are unchanged. Exact final hosted CI executes the
full baseline again; final revision/result remain in automation memory/report.

## Limits

Serial trusted host policy; local results describe command handling and inbox
acceptance snapshots. Subscriber callback output and host printing are separate
observations. No durable/physical delivery, duplicate suppression, persistence,
authentication, concurrency, timing or process-memory guarantee. The example is
controlled integration evidence rather than external adoption, API stability,
qualification, operational suitability or release readiness. Separate
consequential authorization gates remain unchanged.

## Subscription inspection follow-up

RFF-LIB-REQ-015 and [ADR-0040](../adr/0040-read-only-subscription-inspection.md)
add a startup check for required routes. The input is clean main
`ee1a8ed6cd6dacefc43835caf14d9d013dd5111b`. The initial locked local
baseline passes. Independent Codex rechecked [input hosted run 41856418810](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/41856418810)
against the actual checkout and raw job log. All 20 steps, 253 workspace tests,
14 adapter tests, five sample tests and the unchanged 222-line host sample pass.

The new public API inspects the copied topics of one inbox. The detached test
checks membership, absence, an unknown ID and preservation of a queued message
after the caller changes its topic array. The runtime test observes membership
before start, with a full running inbox, after stop/restart and after terminal
work failure. The command session rejects both a producer/subscriber swap and
a missing route on the second subscriber before any application starts. All
three inboxes remain empty. The original successful transcript and historical
callback/correlated consumers are preserved.

The check relies on the host's required-route list and its pairing of IDs
with the issuing runtime. A match describes configuration only. Availability,
capacity and eventual subscriber processing remain separate observations.

### Local and independent verification

On local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98, all of
the following pass. The focused command runs 13 detached bus, 13 runtime
messaging and 11 command-session tests. The workspace runs 256 tests.

```powershell
cargo fmt --all -- --check
cargo test --locked --test message_bus --test runtime_messaging --example command_session
cargo test --locked --example command_session
cargo run --locked --example command_session
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
$env:RUSTDOCFLAGS = "-D warnings"
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

The independent binary is at
`C:/Users/drrsm/AppData/Local/Temp/rff-command-session-1f095dcc645040e3ac10d574a220f29c`.
The guide's locked offline check, Clippy, test, run and metadata commands pass.
Metadata reports one independent binary workspace member and its sole framework
path dependency. The framework has no dependencies. Exact source bytes match
with SHA256 `090008b9ece268d7123d7ceb4121531aa8d9d8fa2fb0a41da43a066cb13a9215`.
The independent package passes all 11 tests. Both executables emit byte-identical
24-line stdout, with the same transcript as the earlier example.

Root Codex and an independent Codex reviewer accept the Rust source and tests.
The audit finds no width violations in 55 Rust files and retains the three
fulfilled lint expectations. All 653 relative Markdown paths and 220 exact
traceability test references resolve. All nine changed Markdown documents pass
rendered content and structure checks. This review examines rendered HTML
structure and text.

### Hosted verification and disposition

Implementation `97296afda530bd721c4421bd2aa97d35f6be7ce3` was published
by ordinary fast-forward to `origin/main`. Its committed diff matches the
reviewed staged diff. Exact
[push run 42032781940](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42032781940),
job 127263132675, passed all 20 steps. The raw log confirms the exact checkout,
256 workspace tests, 14 focused adapter tests, five focused sample tests,
warnings-denied rustdoc, both whitespace checks and the same 222-line host
sample. Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner
2.338.2 and windows-2025-vs2026 image 20261030.294.1 match the input run.

Root Codex and independent Codex review accept this bounded checkpoint. The
implementation fixes pre-start detection of the demonstrated missing routes.
The follow-up record changes only documentation. Its own exact hosted result
is tracked separately in the automation run record.
