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

## Retained command result follow-up

2026-11-08. Codex reviewed RFF-LIB-REQ-014 revision 2 under
[ADR-0039](../adr/0039-runnable-command-session.md#retained-command-result-follow-up).
The original take-before-write could lose the only host record after a
partially accepted diagnostic. The selected serial host policy keeps the typed
record in its existing slot, with one fixed 128-byte line and one byte offset.
Holding the scoped writer across explicit turns lets the host resume from the
accepted prefix without resubmitting the command. The fixed driver exits on an
error. Writer acceptance, flush and physical or durable delivery are separate.

Input `fb2d728954980e042b7ca1d6af4278ed3b958767` matches local, fetched and live
origin. Codex independently inspected [hosted run 42222654426](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42222654426),
job 127908227152. All 20 steps passed, with 257 workspace, 14 adapter and five
sample tests and the unchanged 222-line combined host sample. The locked local
input baseline also passed.

### Local and downstream evidence

All commands passed on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --example command_session
cargo run --locked --example command_session
git diff --check
```

Rustdoc used `RUSTDOCFLAGS=-D warnings`. The workspace passes 258 tests and the
example passes 12. The new history
`result_write_retry_preserves_the_slot_and_resumes_at_the_accepted_prefix`
checks positive short progress, WouldBlock, WriteZero, InvalidData and an exact
suffix. The queued command stays behind the occupied slot, then dispatches
once. A further turn on the completed presenter leaves the new result intact.

The independent binary is at
`C:/Users/drrsm/AppData/Local/Temp/rff-command-session-05c2e129e6b84459abd63f589fa6afbb`.
`$consumer` below denotes that path and `$framework` denotes the checkout path.

```powershell
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/command_session.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/command_session.rs", "$consumer/src/main.rs"
```

The independent binary passes 12 tests. Metadata shows one independent
workspace member, one binary, the sole local framework dependency and zero
framework dependencies. Source files match byte for byte with SHA256
`708aba56d8945633a0a53c51a704dcc1f6a09564bd948a501dea509beaaaeb47`
and Git blob `92e9225c30db5ffc3cc42ea45810d68307b91ad5`.
The input, workspace and independent stdout match byte for byte at 24 lines,
1,469 bytes, SHA256
`051de58c94bccc798114af77fc2c935c5f501d24edcef28a2ee580281cea2787`.
Raw local evidence is in ignored `target/review-2026-11-08-command-result-output`.

### Source and document acceptance

Root and independent Codex accepted the full source and documentation diff.
The whole-tree audit covered 55 Rust files, 25,526 physical lines and 1,398
comment-only lines, with zero width findings and three unchanged Clippy
expectations. All 222 traceability test references and 668 relative Markdown
links resolve. The nine changed documents pass rendered text and structure
review.

### Hosted implementation acceptance

The reviewed implementation `7918362f6a8da0701eaa1b0b0fcac431ee0f3a13`
was fast-forward pushed to `main`. The committed tree matches the reviewed
stage and its example bytes match the independent binary source.
[Exact hosted run 42375736424](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42375736424),
job 128412806896, passes all 20 steps, with 258 workspace tests including all
12 command-session tests, 14 focused adapter tests, five focused sample
tests, warnings-denied rustdoc and both whitespace checks. The 222-line
host sample matches the input run. Hosted Rust/Cargo 1.99.1, rustfmt
1.10.0-stable and Clippy 0.1.99 ran on runner 2.338.2, image
`windows-2025-vs2026` version `20261103.298.1`.

Codex accepts this bounded host-consumer checkpoint. This record revision
receives its own exact hosted inspection, tracked in the nightly run record.

## Retained subscriber output follow-up

2026-11-09. [ADR-0039](../adr/0039-runnable-command-session.md#retained-subscriber-output-follow-up)
and RFF-LIB-REQ-014 revision 3 extend the existing result presenter to each
subscriber's typed slot. One active 128-byte line holds the mutable writer
borrow across turns. The peer can stage output independently. Finishing the
active line clears only its slot and allows the next selected delivery to run.

### Input and local verification

Input `785c6610260c05c66b3c52207487084eb085a715` matches refreshed origin.
The locked local baseline passes. Independent Codex inspected
[hosted run 42375973171](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42375973171),
job 128413499054, with 20 successful steps, 258 workspace tests, 14 adapter
tests, five sample tests and the 222-line combined host sample.

The following checks use Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
0.1.98. Rustdoc uses `RUSTDOCFLAGS=-D warnings`.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --example command_session
cargo run --locked --example command_session
git diff --check
```

The new `subscriber_write_retry_preserves_independent_gates_and_line_order`
history exercises short progress, WouldBlock, zero progress and an invalid
count. It observes peer callback staging during the interrupted line, a
second reply queued behind the selected gate, and exact suffix acceptance.
It then verifies release of the selected gate, retention of the peer's older
output, and two ordered lines on one writer. The earlier result retry and
transcript tests still pass. The initial draft exceeded the 60-line Clippy
limit. Removing redundant assertions restores the warnings-denied check
without a new expectation.

### Independent consumer

The independent package is
`C:/Users/drrsm/AppData/Local/Temp/rff-command-session-6ab469a585a1467cbc25911996a1b058`.
The following commands from the [guide](../COMMAND_SESSION.md#verify-the-identical-independent-binary)
pass with `$consumer` set to that package and `$framework` to the checkout.

```powershell
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/command_session.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/command_session.rs", "$consumer/src/main.rs"
```

Metadata shows one independent workspace member, one binary, the sole local
framework dependency and zero framework dependencies. Both example test runs
pass 13 tests. Source files match byte for byte with SHA256
`38ac2009d8f98114e6b026f4ffb679ad251a2b65fc3c619995b1fffce70117e6`
and Git blob `b72e045daddba8f401daad2ba89100236da98e08`.
Input, workspace and independent stdout match byte for byte at 24 lines,
1,469 bytes, SHA256
`051de58c94bccc798114af77fc2c935c5f501d24edcef28a2ee580281cea2787`.
Raw logs are under ignored `target/review-2026-11-09-subscriber-output`.

### Scope

The fixed script exits on writer error. Recovery requires keeping the same
live presenter and writer. A pending line delays further writes through that
writer object while peer staging remains bounded. Writer acceptance measures
local progress, independently of flush and external delivery.

### Source and document acceptance

Root and independent Codex reviewed the complete implementation diff. The
audit covered 55 Rust files, 25,598 physical lines and 1,397 comment-only lines,
with zero width findings and three unchanged Clippy expectations. All 223
traceability test references, 671 relative Markdown links and 27 fragments
resolve. Ten changed Markdown documents pass rendered text and structure
review. The staged tree equals the committed tree and its example blob matches
the independent binary source byte for byte.

### Hosted implementation acceptance

Implementation `70180da0af12745b666962def0202f655a76ced4` was published to
`origin/main` by ordinary fast-forward.
[Exact hosted run 42483458584](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42483458584),
job 128740148623, passes all 20 steps, including 259 workspace tests, 14 adapter
tests, five sample tests, warnings-denied rustdoc and both whitespace checks.
The 222-line host sample matches the input run. Hosted Rust/Cargo 1.99.1,
rustfmt 1.10.0-stable and Clippy 0.1.99 ran on runner 2.338.2 with
`windows-2025-vs2026` image `20261103.298.1`, matching the input.

Codex accepts the bounded host-consumer checkpoint. This record changes only
documentation and receives its own exact hosted inspection. The nightly run
record retains the result for that revision.

## Retained ingress follow-up

Root Codex and independent Codex reviewed RFF-LIB-REQ-014 revision 4 against
[ADR-0039](../adr/0039-runnable-command-session.md#retained-ingress-follow-up).
Input `9dbab6ad0432902f319af304d64c35232bc2a2ea` matched refreshed origin.
[Exact input hosted run 42483635231](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42483635231)
passed 20 steps, 259 workspace tests, 14 adapter tests, five sample tests,
warnings-denied rustdoc, both whitespace checks and the 222-line sample.
The local input baseline passed after rerunning the workspace tests.
Windows Application Control returned OS error 4551 before the first run
could execute the existing message bus test binary. The same binary passed
on focused replay and the full replay completed without configuration changes.

Root Codex ran the following against the implementation on Windows with
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
$env:RUSTDOCFLAGS = '-D warnings'
cargo doc --locked --workspace --all-features --no-deps
git diff --check
cargo test --locked --example command_session
cargo run --locked --example command_session
```

All passed, with 260 workspace tests and 14 focused tests.
`finite_session_prints_exact_results_and_separate_subscriber_observations`
covers the appended `InboxFull`, occupied result gate, explicit retry and
exact FIFO results before separate replies. The new test
`pending_ingress_survives_stop_and_clears_only_after_admission` covers
`Unavailable`, same-command admission after restart, an empty turn, and
separate callback execution. The existing 13 test histories remain.
Independent source review found that the typed error propagates before slot
mutation. The normal configured session does not exercise that error branch.

Root Codex ran the complete
[independent binary recipe](../COMMAND_SESSION.md#verify-the-identical-independent-binary)
with `$framework` set to `E:/Projects/flight software framework` and
`$consumer` set to
`C:/Users/drrsm/AppData/Local/Temp/rff-command-session-8de3393b048644ba9df903b235a1abaa`:

```powershell
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/command_session.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/command_session.rs", "$consumer/src/main.rs"
```

All succeeded. Metadata confirms one independent binary member and one
framework path dependency. The framework has no dependencies. The source
bytes match, with SHA256
`80550edaa9d667a5f4d2a5b9020c65ea65f43c559a46d27448872cce254a0818`
and Git blob `cfb9febe7be6baf038d074379d52461bf5ee4c4e`. The independent
binary passed the same 14 tests. Both executable stdout captures match
byte for byte at 40 lines and 2,333 bytes. Their first 24 lines and 1,469
bytes match input stdout exactly. Raw local logs and captures are under
`target/review-ingress-recovery`.

The caller owns intake and may retry or abandon the saved command.
Admission is an inbox enqueue. Later stop or failure can discard that
copy. The scenario uses one trusted serial host and demonstrates local
results and reply observations.

Root and independent Codex reviewed the complete diff, including rendered
content and structure for ten changed documents. The implementation review
covered 55 Rust files, 25,708 physical lines and 1,398 comment-only lines,
with zero width findings and three unchanged fulfilled Clippy expectations.
All 679 relative paths, 34 fragments and 224 exact test references resolved.
The reviewed staged tree matches the published implementation. Its
command-session blob matches the verified source.

Implementation `b5e5ce0c0b48e23f4668fb512f3eab2094f4fc31` was pushed to
`origin/main` by ordinary fast-forward.
[Exact hosted run 42622185103](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42622185103)
and job 129176860796 pass all 20 steps, including 260 workspace tests,
14 focused adapter tests, five focused sample tests, warnings-denied
rustdoc and both whitespace checks. The 222-line combined host sample
matches the input run. Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable and
Clippy 0.1.99 ran on runner 2.338.2 with Windows image 20261103.298.1,
matching input.

Codex accepts the bounded checkpoint. This record changes only documentation
and receives its own exact hosted inspection. The nightly run record
retains that result.

## Framed input follow-up

Root Codex and independent Codex reviewed RFF-LIB-REQ-014 revision 5 and
[ADR-0039](../adr/0039-runnable-command-session.md#framed-input-follow-up).
Input `aaf4b92a15a7a53448e0a59169f87890931fee24` matched refreshed origin.
[Exact input run 42622569988](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42622569988),
job 129178163857, passed all 20 steps and 260 workspace tests. The input
local baseline and focused example passed.

Root Codex ran these checks on Windows with Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
$env:RUSTDOCFLAGS = '-D warnings'
cargo doc --locked --workspace --all-features --no-deps
git diff --check
cargo test --locked --example command_session
cargo run --locked --example command_session
```

All passed, with 264 workspace tests and 18 focused tests. The four added
tests in [traceability](TRACEABILITY.md#stage-5-framed-input--2026-11-11)
verify short reads and returned errors, the gated next record during
inbox rejection, validation before issuing an ID, clean end of input,
truncation and defensive handling of an impossible count. The fixed
saturation scenario now stages the saved New from an in-memory cursor
and preserves the previous 40-line, 2,333-byte stdout exactly.

Root Codex ran every command in the
[independent binary recipe](../COMMAND_SESSION.md#verify-the-identical-independent-binary)
with framework `E:/Projects/flight software framework` and consumer
`C:/Users/drrsm/AppData/Local/Temp/rff-command-session-608e87afd5e940a6bf396dacfbaa51fe`.
Creation, offline dependency setup, copy, locked offline check, warnings-denied
Clippy, 18 tests, run, metadata and hashing all passed. Metadata shows one
independent binary workspace member, one framework path dependency and
no framework dependencies. The example and independent main match byte
for byte with SHA256
`66986420c0de2c005f623f6ef2da58f85edcd5ec7ce50916175b4b268e37e354`
and Git blob `03e5221cc89a002f0eab52ea692e3e7bdc5b27f9`.
The independent and workspace stdout captures match exactly.

The same reader and session own frame progress and ID issuance through
each turn. An admission result describes enqueue. This test domain uses
finite in-memory input and injected errors. Blocking and transport policy
depend on the caller's reader.

### Source and hosted acceptance

Root Codex reviewed the complete diff. Independent Codex reviewed the
source, decisions, tests and all 12 changed Markdown documents. Rendered
content and structure match their Markdown. All 688 relative paths, 42
fragments and 228 exact test references resolve. The audit covers 55
Rust files, 25,949 physical lines and 1,399 comment-only lines with zero
width findings and three unchanged fulfilled Clippy expectations. The
staged tree matches the committed tree and its example blob matches the
verified consumer source.

Implementation `12931b7e936b35bd0f4c9ffd15510213a6629efe` was pushed to
`origin/main` by ordinary fast-forward.
[Exact hosted run 42822018637](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/42822018637),
job 129849666161, passes all 20 steps, including 264 workspace tests, 14
adapter tests, five sample tests, warnings-denied rustdoc and both
whitespace checks. All 18 example tests appear in the workspace job.
The 222-line combined sample matches input. Hosted Rust/Cargo 1.99.1,
rustfmt 1.10.0-stable and Clippy 0.1.99 ran on runner 2.338.2 with
`windows-2025-vs2026` image `20261103.298.1`, matching input.

Codex accepts the bounded host-consumer checkpoint. The documentation
record revision receives a separate exact hosted inspection.
