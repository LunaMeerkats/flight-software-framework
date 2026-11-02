# Correlated operation cancellation review

Date: 2026-11-03 Australia/Sydney
Scope: RFF-LIB-REQ-012 revision 1 and ADR-0037; separate public-only consumer

## Selection and baseline

Root Codex and independent consumer_assessment selected correlation because
ADR-0036's dispatch-time Cancel can clear a newer reply. A separate consumer
preserves the original policy and all 12 existing tests. Requirements, plan and
ADR precede implementation. Acknowledgements require a larger bounded feedback
contract; lifecycle abandonment does not reveal processing. Existing public APIs
suffice. Official Rust checked_add and Option inform finite overflow and matching
optional state; mission identity and cancellation policy are local choices.

Clean input main d56441235dc8670a14c8a54fadd4df4abeb6c9f3 matches refreshed
origin/main. Initial locked format/check/Clippy/229 workspace tests and
warnings-denied rustdoc pass, as do working/committed whitespace checks. Live
input run 41310258543/job 124858954952 passes all 20 steps at that checkout,
229 workspace/14 adapter/five sample tests and unchanged 222-line sample. Root
and independent verification_support inspect raw metadata/logs. Local tools are
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. Hosted tools remain
1.99.1/rustfmt 1.10.0-stable/Clippy 0.1.99, runner 2.338.2 and image
windows-2025-vs2026 20261021.278.1. These are separate environments.

Initial whole-tree audit: 53 Rust files, 23,239 physical/1,357 comment-only lines,
zero width findings and three existing expectations. Relative links and exact
test references resolve. Temporary review aids and raw logs are retained under
target/review-2026-11-03-correlated-cancellation, outside maintained source.

## Consumer evidence

Only tests/correlated_commands.rs adds Rust source. The original callback source,
library, Cargo graph, workflow, lint configuration and existing waivers are
unchanged. Seven focused tests pass:

- `delayed_cancel_cannot_erase_newer_operation_and_retry_skips_accepted_peer`:
  queues New(B), stale Cancel(A), rejects a third command at exact capacity two,
  and dispatches B without stale cancellation. Stale Retry(A) with spare capacity
  adds nothing; matching retry enqueues B behind older FIFO without repeating an
  accepted peer. Completion and delayed matching cancellation add no copies.
- `matching_cancellation_preserves_accepted_copy_and_rejected_fifo`: distinct-ID
  control leaves pending state; matching/idle/repeated controls preserve accepted
  42 and older [3,4], then a new operation delivers 43 to both destinations.
- `busy_new_command_requires_same_id_resubmission_after_pending_completion`:
  busy New(B) and controls for B leave A unchanged; after A completes, the host
  resubmits the retained B request with the same ID and observes its reply.
- `producer_restart_retains_operation_and_host_issuer_but_clears_commands`:
  stop clears exactly two queued controls, preserves pending A and host issuance,
  and rejects ingress while stopped. Restart exposes an empty command inbox;
  mismatched control leaves A for matching retry after the peer restarts.
- `session_issuer_uses_entire_nonzero_domain_then_repeatedly_rejects_exhaustion`:
  all 255 distinct sequential IDs are observed, later issuance repeatedly fails
  without wrap, and retained-ID retry/cancellation still work at exhaustion.
- `idle_repeated_and_completed_control_commands_leave_copies_intact`: no pending
  operation means control commands have no queue effects, including after both
  subscribers accepted a reply.
- `malformed_commands_are_rejected_before_reply_or_cancellation_effects`: nine
  malformed command shapes/nonzero constraints enter the callback's typed failure
  path, discard exactly one later command and preserve an accepted peer copy;
  a wrong-topic parser case is checked separately.

```text
cargo fmt --all
cargo test --locked --test correlated_commands
cargo clippy --locked --test correlated_commands -- -D warnings
```

All pass. Root review strengthened the stale-Retry observation by freeing
destination capacity before attempting it, so an unintended enqueue is visible.

## Independent Cargo consumer

Actual package:
C:/Users/drrsm/AppData/Local/Temp/rff-correlated-e522f508d4584dff9e17d0c86295779c.
The guide's exact recipe was executed, stopping on every command failure. With
$consumer set to that path and $framework to this checkout:

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

All Cargo commands pass: seven identical consumer tests plus the separate
generated smoke test. Metadata confirms one independent workspace member,
two packages, sole framework path dependency and no library dependencies.
Exact source bytes match; SHA256:
1765d490964e33a2e6b8cec1672be1d8f82f99f1e8f799f1e876c7b4bf4e58ee.
Canonical source Git blob: 96ec5ffeff461ba432b723b91038a6e3cfbddd39.

## Final baseline and review

The following exact commands pass after source freezes:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

RUSTDOCFLAGS=-D warnings is set before cargo doc. Final workspace count is 236,
including all seven new and 12 original callback tests. Whole-tree review finds
54 Rust files, 23,848 physical/1,361 comment-only lines, zero width findings and
three unchanged fulfilled expectations. All 200 exact test references and relative
Markdown links resolve. Changed Markdown renders and passes content/structure
inspection; this is not pixel QA. Temporary inventory/render helpers are review
aids, not newly adopted policy gates.

Root Codex reviews all new source, complete diff, requirements, resource/error
bounds, source ordering, naming, waivers, provenance and unsupported claims.
No findings remain. Independent Codex verification_support accepts the complete
13-file staged source/document diff with no findings, separately inspecting the
local/independent logs, source identity and unchanged original source/config.
All 12 changed documents pass rendered content/structure inspection; 612 relative
links and 200 exact test references resolve across 107 Markdown files. The
REQ-012 wording explicitly limits no-effect controls to stale/idle commands and
repeated cancellation after clearing; matching Retry may still make progress.
The reviewed source identity is the exact Git blob above; the accepted staged
patch is retained locally for comparison with the resulting commit. Review is
Codex provenance, not human review. Ordinary main publication and exact hosted
inspection remain pending; no hosted pass is claimed for uncommitted work.

## Limits

One session issuer, one pending operation and two destinations only. Correlation
does not acknowledge processing, authenticate commands, suppress duplicate New
commands or preserve identity across issuer reset. Ordinary admission/FIFO and
lifecycle clearing apply; cancellation cannot retract accepted copies. No
framework API, service, dependency or workflow change is intended. Controlled
host evidence does not establish adoption, stability, operational suitability or
qualification. Release and other consequential gates remain separate.
