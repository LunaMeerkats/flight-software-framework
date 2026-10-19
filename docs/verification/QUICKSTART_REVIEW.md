# Public-API quickstart review

Date: **2026-10-20**
Status: **Local implementation and autonomous review accepted; publication pending**

## Objective, inputs and decisions

RFF-LIB-REQ-001 revision 1 enables a first downstream developer to compose a
bounded command handler through public APIs. Existing v0.1 requirement meanings
and tests remain unchanged. Clean input is
61e3f0e54509ab4a2f45b94648c83d66e245b233, matching refreshed origin.
Fresh exact input [hosted run 39248169214](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39248169214)
is successful at the logged checkout, job 118045097712, all 20 steps.

Codex root and consumer_assessment inspect README, public API and private sample.
The adoption gap is a small dependency/setup/composition recipe; this consumer
needs no new library service. [ADR-0026](../adr/0026-public-api-quickstart.md)
compares the tested example/external package, a permanent consumer crate, and
ordinary-work service expansion. Current official Cargo dependency/target/test
documentation is consulted; SRC-CARGO-CONSUMER-RECIPE records adoption. No upstream
source code, runtime behavior, new dependency, licence or workflow is changed.

## Implemented behavior and exact source

[quickstart.rs](../../examples/quickstart.rs) defines one independently authored
application borrowing one `Cell<Option<u8>>` host slot. One application/inbox and
one-byte payload have explicit finite limits. Registration precedes inbox
attachment and start. The driver queues 42, observes 43 rejected as InboxFull,
dispatches/drains 42, stops/restarts, then dispatches/drains 7 and stops.
Diagnostics occur outside callbacks. Callback validation/full-output errors are
typed, preserve prior output, and enter terminal Failed under existing contracts.
Setup error causes propagate while the one-shot demo deliberately tears down;
the guide explains ownership recovery available to integrating hosts.

Source Git blob: `985808cda0486af2e4510e29a6b1121d96c7f48f`.
Source SHA256:
`24bf847b2ce906217d37a20ab06dd051decae9caf65cc4fb8a9740ce33049156`.

The manifest sets test=true for this example. Workspace tests execute all three:

- `scenario_retains_older_input_and_reuses_restarted_inbox`
- `malformed_commands_fail_before_output_and_leave_terminal_state`
- `output_saturation_retains_older_value_after_callback_failure`

The last two use a capacity-one inbox and observe zero additional queued discards;
they do not newly establish clearing of multiple retained inputs. Existing tests
retain that separate evidence. The complete seven-line stdout is recorded in the
[getting-started guide](../GETTING_STARTED.md) and matches both executable forms.

## Independent consumer execution

Actual independent root:
`C:/Users/drrsm/AppData/Local/Temp/rff-consumer-16c7f92bc74e4e4494fa112d98a3fc2d`.
The guide's setup is executed using that unused temporary directory rather than
its suggested sibling name. Commands start in the framework checkout, with
$framework set to its absolute path and $consumer set to this external directory:

```powershell
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/quickstart.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
```

All final commands pass; three independent-consumer tests pass. Cargo add creates
its initial lockfile offline. Metadata confirms one external workspace member,
exactly two graph packages, the sole direct dependency pointing at this checkout,
and no library dependencies. SHA256 equality confirms the example and consumer
main source have identical bytes. The external package is generated verification
output, not a new maintained crate. The recipe needs no private sample files.

An initial local orchestration error appended --manifest-path after Clippy's
argument separator, so rustc rejected that option. Correcting option placement
and rerunning passes. This was an invocation error, not a library failure; raw
failed and corrected logs are retained separately.

## Local baseline and autonomous review

Initial locked baseline passes 151 tests. Final required commands are:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
cargo test --locked --example quickstart
cargo run --locked --example quickstart
```

Rustdoc runs with RUSTDOCFLAGS=-D warnings. Local Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98 are unchanged. Final full baseline passes 154
workspace tests; focused quickstart tests pass three and execution matches the
guide. Host sample/adapters, workflow and ADR-0018/0019 probes are unchanged;
their conditional local commands are not triggered. Hosted CI retains its
separately configured host/sample execution.

Codex root reviews full example, setup source/hash/metadata/logs and complete
diff. Independent Codex quickstart_review reviews the full source/design,
requirement, entry points and changed documentation and independently executes
the three focused tests. It accepts the exact source blob above with no blocking
findings. Independent quickstart_review then inspects actual external source
hashes, manifest/lockfile/metadata and raw successful local logs; full changed/new
documents, rendered guide/record and generated crate entry points; final state,
traceability and complete diff. It accepts completed local evidence with no
blocking findings. This is autonomous Codex review under ADR-0025, not human
acceptance.

Whole-tree audit covers 44 Rust files, 17,413 physical and 1,145 comment-only lines,
zero width findings and three unchanged fulfilled expectations. All 409 relative
links and 118 exact test references resolve. All eleven changed rendered Markdown
documents pass full content/headings/code/links/tables/HTML structure inspection;
generated crate documentation carries the recipe and current limits. These are
content/structure reviews, not pixel or human acceptance. The initial render
catches one unquoted generic type parsed as HTML; quoting it fixes the content
and the complete repeated inspection passes.

Review aids, original logs, metadata and source inventory are ignored under
`target/review-2026-10-20-downstream`. That folder and the temporary consumer may
be unavailable on another checkout; the commands and source hashes above make
the evidence repeatable. They are not new adopted checker gates.

## Limits and likely next step

This establishes one controlled independently packaged public-API composition,
not external adoption or general consumer readiness. It covers one homogeneous
application, caller-framed input and an in-process output slot. It adds no
transport, physical delivery, service context, scheduler or recovery machinery.
Existing owner-local identity, callback/allocator/resource and qualification
limits remain. No MSRV, API freeze, release or publication readiness is inferred.

Next, assess a consumer needing two independently defined applications or
ordinary-work output. Select an API/capability improvement only from a concrete
composition gap; preserve this simple recipe and all prior baseline evidence.
