# Explicit callback reply cancellation review

Date: 2026-11-02 Australia/Sydney
Scope: RFF-LIB-REQ-011 revision 1 and ADR-0036; public-only consumer policy

## Selection and baseline

Root Codex and independent consumer_assessment selected an explicit Cancel
command because a permanently Failed destination can indefinitely block the
single reply slot. Existing public APIs express the policy without a new library
service. Requirement and plan preceded implementation. ADR-0036 compares
explicit cancellation with lifecycle policy, host mutation and acknowledgements.
The source register records consulted official Rust Option and VecDeque facts;
the cancellation protocol is locally designed. No upstream code is reused.

Clean input main 1972d66f20d08ab3590e84035073bc4d015b35b9 matches refreshed
origin/main. Initial locked baseline passes 224 workspace tests and both
whitespace checks. Live input run 41204977193/job 124548108020 passes all 20
steps at that exact head; raw metadata/logs are retained. The connector returned
no runs for this commit, so GitHub CLI supplied the exact metadata and raw logs.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 remain distinct
from hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable and Clippy 0.1.99.

## Consumer behavior and focused evidence

Only tests/callback_publication.rs changes Rust source. Cancel clears the
application-owned Option on dispatch without a context/bus call. Existing seven
test bodies are retained unchanged; five new tests pass:

- `cancellation_preserves_accepted_copy_and_rejected_fifo_then_admits_new_reply`:
  partial acceptance preserves peer 42 and older rejected FIFO [3, 4], retry
  after cancellation adds nothing, and resubmitted 43 reaches both subscribers.
- `cancellation_unblocks_new_replies_after_permanent_subscriber_failure`:
  a concrete sink error enters terminal Failed; repeated cancellation frees the
  producer for 43 while retained peer 42 and failed output 7 remain unchanged.
- `cancellation_obeys_command_inbox_backpressure_and_dispatch_order`:
  an older Retry fills the capacity-one inbox; Cancel is rejected, then admitted
  after dispatch. Enqueue alone does not clear pending state; dispatch does.
- `producer_restart_retains_reply_but_discards_queued_cancellation`:
  stop reports one discarded Cancel; restart retains the pending reply. A later
  dispatched Cancel remains effective across another stop/restart.
- `idle_repeated_and_completed_cancellation_leave_accepted_copies_intact`:
  idle/repeated Cancel and later Retry preserve both already accepted copies.

```text
cargo fmt --all
cargo test --locked --test callback_publication
cargo clippy --locked --test callback_publication -- -D warnings
```

All commands pass; the focused target has 12 tests. Cancel has no special
priority, acknowledgement or operation ID. It targets the current reply when
dispatched; host ordering must prevent stale commands from cancelling later work.
No library, public API, dependency, workflow, licence or lint policy changes.

## Independent Cargo consumer

Actual package:
C:/Users/drrsm/AppData/Local/Temp/rff-callback-cancel-72a37e88f76144d38cbe84d8dd099895.
The guide's recipe was executed from the framework checkout, stopping on every
failure. With $consumer set to that path and $framework to the checkout:

```powershell
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/callback_publication.rs" -Destination "$consumer/tests/callback_publication.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
```

All commands pass: 12 copied tests and the separate generated smoke test.
Metadata confirms an independent workspace with one member, two packages, one
framework path dependency and no library dependencies. Exact source bytes match;
SHA256 is 056253bd812c43e8cec2fe4eaeb30e4117e0d5da79a5be3b124e42317ecfa2df.
Python compared bytes and hashlib SHA256 against metadata's actual resolved paths.

## Final local baseline and review

RUSTDOCFLAGS=-D warnings is set for documentation. These commands pass with
229 workspace tests, including the original seven callback regressions and five
new cancellation regressions. Source bytes are unchanged after verification.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Root Codex and independent cancellation_review accept the complete 13-file
source/document diff with no findings. The independent reviewer inspected original
test-body identity, actual local/consumer logs, metadata/source bytes and audit
results. Whole-tree audit finds 53 Rust files, 23,239 physical/1,357 comment-only
lines, zero width findings and three unchanged expectations. All 595 relative
links and 193 exact test references resolve across 104 Markdown files. All 12
changed documents pass rendered content/structure inspection: headings, prose,
code blocks, links and tables match, without malformed markup or replacement
characters. This is not pixel QA. Temporary audit aids are not adopted gates.
The exact accepted staged patch and source identities are retained under
target/review-2026-11-02-reply-control before commit. Review is Codex, not human
acceptance.

## Publication and limits

Publication and exact hosted inspection are pending; local results do not imply
a hosted pass. This record will be updated with the actual implementation SHA,
run/job and completed results after ordinary main publication.

Cancellation ends future attempts; it cannot retract queued/processed effects.
Command saturation, unavailable/Failed producers, FIFO order and lifecycle inbox
clearing can delay or prevent it. New replies may themselves become pending
against the same failed subscriber. This is controlled consumer evidence, not
external adoption, processing acknowledgement, stability, physical delivery,
timing, qualification or release readiness. Separate consequential gates remain.
