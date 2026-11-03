# Bounded host command-result review

Date: 2026-11-04 Australia/Sydney
Scope: RFF-LIB-REQ-013 revision 1 and ADR-0038; correlated consumer extension

## Selection and starting evidence

Root Codex and independent consumer_assessment selected the recorded host
feedback gap: command enqueue alone cannot distinguish busy rejection from
staging, or matching cancellation from an unmatched control. Requirements,
ADR and plan precede source edits. One typed mailbox with serial host gating
fits existing public APIs; bus acknowledgements would need another bounded
pending-result/retry policy. A generic callback-result API and persistent IDs
are deferred until demonstrated consumer need.

Official Rust Cell/Rc documentation informs copied shared single-thread state;
result vocabulary, capacity and dispatch policy are local choices. See
SRC-RUST-COMMAND-RESULTS. No upstream implementation is reused.

Clean input main 4f64a4e70037609c6f382b20ff6c13967a54e05e matches fetched
origin/main. Initial locked baseline passes all 236 workspace tests plus format,
all-target check/Clippy, warnings-denied rustdoc and both whitespace checks.
Independent verification_review inspects raw metadata/logs for input run
41451380499/job 125297152054 at that exact checkout: all 20 steps pass,
236 workspace/14 adapter/five sample tests and unchanged 222-line sample.
Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.
Hosted tools remain 1.99.1/rustfmt 1.10.0-stable/Clippy 0.1.99, runner 2.338.2
and windows-2025-vs2026 image 20261021.278.1. The two environments are separate.

Initial whole-tree audit passes: 54 Rust files, 23,848 physical/1,361 comment-only
lines, zero width findings, three existing expectations, 107 Markdown files,
613 relative links and 200 exact test references. Temporary review aids/raw logs
live under target/review-2026-11-04-command-feedback, outside maintained source.

## Consumer evidence

The only changed Rust source is tests/correlated_commands.rs. All seven original
test bodies compare exactly after newline normalization against the input
revision; the test host now explicitly drains each successful command result.
Seven added regressions pass:

- `typed_results_report_busy_cancellation_and_same_id_resubmission`: partial
  reply, busy pending ID, unmatched control, matching cancellation and same-ID
  resubmission retain accepted peer copies and older destination FIFO.
- `retry_results_snapshot_partial_and_completed_acceptance_after_peer_clearing`:
  unavailable retry retains flags, restart allows completion, later peer stop
  clears its inbox without rewriting the saved result; completed controls report
  no matching operation.
- `blocked_result_turns_preserve_command_fifo_saturation_and_peer_queues`:
  repeated full-slot turns retain exact commands/result/operation/peer queues;
  ordinary capacity-two rejection and explicit drain restore FIFO progress.
- `malformed_command_has_no_result_and_clears_only_queued_producer_commands`:
  typed runtime failure discards exactly one queued command, adds no result and
  retains the already accepted peer copy.
- `occupied_result_bypass_fails_before_command_effects_and_preserves_old_result`:
  direct bypass retains result and pending cancellation target, fails selected
  producer, discards exactly one later input and preserves peer output.
- `stop_restart_retains_result_and_operation_but_unhandled_commands_have_no_result`:
  stop discards two queued controls, result gate precedes lifecycle checks,
  restart retains operation/issuer/result and cleared commands produce no result.
- `routing_failure_has_no_result_while_previously_accepted_peer_effect_survives`:
  second publication is unsubscribed after first succeeds; full typed error/source
  chain and exact queued clearing remain visible without a fresh result.

```text
cargo fmt --all
cargo test --locked --test correlated_commands
cargo clippy --locked --test correlated_commands -- -D warnings
```

All pass, with 14 focused tests. Source blob:
af81a948296ff8fb004787fcd5b55ef4ff0ac209.

## Independent Cargo consumer

Actual package:
C:/Users/drrsm/AppData/Local/Temp/rff-correlated-8cbeae94aefe4e82b3ac5d63d9de6d9c.
The guide's recipe is executed with $consumer set to that path and $framework
to this checkout, stopping after any failure:

```powershell
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/correlated_commands.rs" -Destination "$consumer/tests/correlated_commands.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/correlated_commands.rs", "$consumer/tests/correlated_commands.rs"
```

All Cargo commands pass: 14 identical tests plus the separate generated smoke
test. Metadata confirms one independent workspace member, two packages, sole
framework path dependency and no library dependencies. Exact source bytes match;
SHA256: 4c3953e934d622d5ab30ed4d0b2a0e6a31b2e5981ae07d324309e2ccf3e9dd0a.

## Final baseline and review

These commands pass on both input and final source; final workspace tests total
243. Rustdoc runs with $env:RUSTDOCFLAGS = "-D warnings":

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

The committed-tip check passes for input and published implementation. No
conditional host-adapter/sample or ADR-0018/0019 probes are required by unchanged
components.
Their unchanged behavior remains covered by workspace tests and hosted commands.
The workflow is unchanged; actionlint is not rerun.

Independent verification_review accepts the exact source blob above and unchanged
old test bodies with no findings. Root complete source/diff review agrees.
Implementation whole-tree review finds 54 Rust files, 24,240 physical/1,362
comment-only lines,
zero width findings and three unchanged fulfilled expectations. All 624 relative
links across 109 Markdown files and 207 exact test references resolve. All 12
changed rendered documents pass text/headings/code/links/tables/HTML structure
inspection; this is content/structure review, not pixel QA. The temporary review
aid is corrected to distinguish wrapped numeric prose from ordered-list starts.
Independent verification_review accepts the complete 13-file increment without
remaining findings, after correcting a guide method name to actual result.take().
Root Codex accepts the complete diff on the same evidence. This is autonomous
Codex review, not human review.

## Publication and hosted acceptance

Implementation 4ee3c5091ff49e85b9c9595958e040c051f8bec5 exactly matches the
independently accepted staged patch. Committed-tip whitespace passes; refreshed
origin is preserved and ordinary fast-forward main publication succeeds. Live
remote matches. Exact
[run 41641979417](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/41641979417),
job 125937510804, passes all 20 steps at that logged checkout: 243 workspace
tests including all 14 consumer cases, 14 adapters/five sample tests,
warnings-denied rustdoc, both whitespace checks and unchanged 222-line sample.
Root Codex and independent verification_review inspect raw metadata/logs, source
identity, all test results and the sample; no implementation findings remain.

Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99 and runner 2.338.2
match input. The windows-2025-vs2026 image advances from 20261021.278.1 to
20261030.294.1. An additional image-equality probe fails on this real runner drift;
the required hosted baseline passes. Record the new image rather than asserting
environment equality. No Rust toolchain/configuration change or new waiver is
needed; the current whole-tree audit remains separate evidence. Record-only
completion passes root/independent seven-document diff review, 625 relative links
and content/structure inspection of all 13 whole-run changed Markdown documents.
Rust inventory and 207 exact test references remain unchanged. This completion
receives separate publication/exact hosted inspection in
automation memory/report. Stop after this one verified checkpoint.

## Boundaries

Only the correlated consumer's operational feedback policy changes. Original
callback consumer/library/dependency/workflow/lint/licence state stays unchanged.
The result gate precedes lifecycle validation and bounds serial host dispatch;
direct bypass is a terminal callback error with ordinary selected queue clearing.
Malformed/routing failures produce no fresh success result, and accepted peer
effects survive. Result snapshots do not prove subscriber processing or retention
after later stop/failure. Host drain neglect can block future producer commands.
No authentication, duplicate suppression, durable identity, reliable transport,
concurrency, adoption, stability, qualification or release readiness is established.
