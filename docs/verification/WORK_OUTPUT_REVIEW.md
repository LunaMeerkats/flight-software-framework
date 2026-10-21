# Ordinary-work output consumer review

Date: **2026-10-22**
Status: **Local verification and autonomous review complete; publication pending**

## Objective and input

RFF-LIB-REQ-003 revision 1 demonstrates configured ordinary-work output through
public APIs with explicit single-subscriber host transfer. Clean input
0848e32821046b91048ad607a7aba267a8850fb4 matches refreshed origin/codex/nightly.
Exact [input run 39429862057](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39429862057)
is re-read this run: successful push event, exact head, one Windows job
118651777288 and all 20 successful steps. Fresh initial locked local baseline
passes 158 workspace tests, warnings-denied rustdoc and both whitespace checks.
Raw logs are retained under target/review-2026-10-22-work-output (ignored aids).

Root Codex and independent consumer_need assess the concrete gap and alternatives
before source implementation. [ADR-0028](../adr/0028-host-mediated-work-output.md)
selects a capacity-one caller-owned slot rather than broadening ordinary-work
service access without a consumer requiring in-callback publication feedback.
Official Rust Cell and Cargo example sources are recorded in SRC-RUST-WORK-OUTPUT.

## Verification boundary

The [guide](../ORDINARY_WORK_OUTPUT.md) and
[public-only source](../../examples/work_output.rs) are the selected consumer.
The producer stages a record containing configuration acceptance revision and
one percentage byte; only the value byte is transmitted. Both application
inboxes have capacity one, only the sink subscribes, one host output slot holds
the delivered byte, and two separate observation cells retain work observations.
The pending, output and observation cells are caller-owned. Work observes
the current configuration even when
pending-slot saturation subsequently returns an error. The old pending record
survives configuration replacement, stop/restart and terminal producer failure.

Transfer publishes a copy, then checks sole-destination identity and acceptance
before clearing. Full/unavailable outcomes return Retained without failing the
producer. UnexpectedDestination is a separate error after possible delivery;
the host stops and must not blindly retry it. No actual allocation failure is
injected; Message/Publish error preservation is source-review evidence only.
No nonzero selected-inbox discard is claimed: the producer has no subscriptions.
Existing runtime regressions retain that separate cleanup evidence.

## Source and tests

Source Git blob: `6e70364bc192d4ab6b9e3761391360f8fae3bc1b`.
Source SHA256:
`49babbd3d287cb2bd503c2e01db3290aa0ce11926233b125bd99db80da5a1f32`.

Ten focused regressions exercise the requirement:

- Staging occurs before publication; empty transfer does nothing; dispatch and
  host drain remain distinct from inbox acceptance.
- Rejected configuration remains rejected and accepted replacement leaves the
  pending revision/value unchanged until transfer; later work sees revision two.
- Full and unavailable destinations retain pending output. After explicit drain
  or start, retry transfers the retained value without terminal producer failure.
- Producer stop/restart retains pending output; lifecycle-rejected work does
  not change its observation. Explicit transfer after stop is permitted.
- Occupied-slot work returns the exact typed PendingFull error through
  MessagingOperationError and RuntimeWorkError source chains, retains the old
  output, commits Failed and leaves peer work/dispatch available. Transferring
  that old record after failure is deliberate host policy.
- Scheduled waiting leaves the item untouched; success consumes once; a work
  error or lifecycle rejection consumes the due item and allows the next peer.
- Empty/invalid sink input and full host output retain older host values with
  typed errors; the producer can still work.
- Wrong expected destination returns the actual post-publication report without
  clearing pending, even though the real subscriber already accepted delivery.

Exact test names are mapped in [traceability](TRACEABILITY.md). No old test,
library behavior, dependency, licence, lint policy or workflow is changed.

## Independent package

Actual package:
`C:/Users/drrsm/AppData/Local/Temp/rff-work-output-168578b9afaf4fed9452e5003c866594`.
The guide's commands are executed with $framework set to this checkout and
$consumer set to that unused path:

```powershell
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/work_output.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/work_output.rs", "$consumer/src/main.rs"
```

All commands pass, including ten independent tests. Both source hashes match
the identity above. Metadata confirms one independent workspace member, two
graph packages, the sole framework path dependency and no library dependencies.
Independent and workspace stdout exactly match the guide's three lines, ending
in [42, 7]. Source is public-only and imports no private host sample modules.

## Local baseline and source review

The following commands pass; rustdoc uses RUSTDOCFLAGS=-D warnings:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --example work_output
cargo run --locked --example work_output
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Initial/final workspace totals are 158/168, including all ten new regressions.
Focused and independent tests each pass ten. The author additionally runs
source-only rustfmt and all-target focused Clippy with warnings denied. An
initial combined schedule test exceeded the function threshold after lifecycle
coverage was added; it is split into independent success, callback-failure and
lifecycle-rejection scenarios. No new waiver is needed. A temporary evidence
inspection ran before the workspace run log existed; repeating it after command
completion passes. These are recorded orchestration/source-review corrections,
not suppressed baseline failures.

Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
Whole-tree inventory covers 46 Rust files, 18,794 physical/1,164 comment-only
lines, zero width findings and three unchanged fulfilled expectations. Relative
links and exact traceability test names resolve. Twelve changed Markdown
documents pass full rendered text, headings, code, table, link and HTML structure
inspection. This is content/structure review, not pixel or human acceptance.
Generated crate documentation is inspected separately. Host sample, workflow
and ADR-0018/0019 probe sources are unchanged, so their extra conditional local
commands are not triggered. Hosted CI still executes its configured host checks.

Root Codex reviews the complete new source, retained tests, ownership/error
contracts, source form and complete diff. Independent Codex independent_review
accepts the design/source, actual external metadata/hashes/command logs and raw
168-test baseline. Its findings correct
scheduled-instant terminology and explicitly enumerate all caller-owned cells.
Root and independent_review accept complete source/document/diff and actual
baseline/consumer evidence with no remaining blocking findings. The staged diff
and refreshed rendered documents are checked before the publication commit.
No runtime capability, licence, dependency, lint or workflow change is made.
