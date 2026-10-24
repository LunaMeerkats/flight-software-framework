# Host-owned periodic work release review

Date: **2026-10-25** (run started before local midnight)
Status: **Local verification and complete review passed; publication pending**

## Objective and input

RFF-LIB-REQ-005 revision 1 and
[ADR-0030](../adr/0030-host-owned-periodic-releases.md) precede source
implementation. A recurring output/housekeeping consumer needs reusable
late-poll/overflow policy without repeatedly allocating finite agendas. Root
Codex and independent repeated_work_assessment compare host arithmetic, agenda
reconstruction, the standalone cursor and paired execution wrappers. The
selected cursor leaves work/lifecycle/error ownership in the existing APIs.

Input is clean codex/nightly at
86d82c9b336ba013f7e460fc309a6fb8fcfe08b6, matching refreshed origin. The exact
[input run 39959863270](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39959863270)
passes all 20 steps at that head; the connector's live job-step read agrees.
Fresh local locked fmt/check/Clippy/test and warnings-denied rustdoc pass all
176 workspace tests, plus both whitespace checks. Local tools remain Rust/Cargo
1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. Hosted evidence is separate.
Ignored raw logs/aids are under target/review-2026-10-24-repeated-work; its path
retains the run's pre-midnight start date.

## Contract and evidence boundary

The [public cursor](../../src/scheduling/periodic.rs) retains one optional
ScheduledWork and a positive interval inline. One active poll reads once,
waits without mutation or consumes one release before re-arming from observed
time. Overflow returns that final due item with no next release; exhausted
polls read no clock. It neither executes callbacks nor touches lifecycle or
inboxes. A host may discard releases and must select timer disposal/error policy.
Source contains no allocation, history, catch-up loop or new dependency.

The [consumer](../../tests/periodic_work.rs) uses public APIs and std only.
Two configured producers each borrow one pending output slot, one attempted
observation, a call counter and one delivery observation. The host drains output
between successful releases. Messaging scenarios use two capacity-two inboxes
and one-byte payloads. Both selected fault positions preserve older output,
clear exactly two selected deliveries, retain the peer FIFO and allow peer work.
The host disposes its selected timer explicitly. A separate scenario rejects
Registered work, discards a later release without invoking work, then executes
the following release. Polling itself never changes lifecycle or queues.

Existing one-shot tests remain regression evidence with unchanged meanings.

No origin authentication, fixed phase, delay from completion, execution-time
bound, fairness, retry, transaction, physical output or panic/hang containment
is provided. Logical cursor/queue bounds do not bound process memory or callback
allocation. APIs remain provisional; this is controlled consumer evidence, not
external adoption, operational suitability, qualification or release readiness.

## Exact local verification

The initial and final commands all complete successfully; the test total grows
from 176 to 186, retaining the old suite plus ten new tests:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

The rustdoc command runs with `$env:RUSTDOCFLAGS = '-D warnings'`. Focused
`cargo test --locked --test periodic_work` passes ten tests; the author's
`cargo clippy --locked --test periodic_work -- -D warnings` also passes. Its
initial drop_non_drop finding is resolved by explicit Option-slot disposal,
without a waiver. Author `cargo check --locked --lib` and
`cargo clippy --locked --lib -- -D warnings` pass. The full warning-free baseline
includes those targets. No workflow, host sample, old consumer source/guide or
ADR-0018/0019 borrowing probe changes trigger extra local conditional commands.
The hosted workflow still separately executes its existing host targets/sample.

Cursor Git blob: `722bf639893ecf65bd226030eaafc430a745aaad`.
Cursor SHA256: `069288a676ec1f66a9826c6431ae47e4ba161144a8b156d8f40a87e73caaeee8`.
Consumer Git blob: `ef13cabcad38370d5821f7345ec68f05421d8296`.
Consumer SHA256: `6ae4fb317b900a00f193679aa1822ca168477bb4ebd469c2ea66007bcca5adb9`.
These identify the reviewed source; publication identity is recorded separately.

## Actual independent consumer

The guide's commands are executed against this fresh package outside the tree:

```text
C:/Users/drrsm/AppData/Local/Temp/rff-periodic-work-1734625b55b744c0bc0a8ff7e92f6fa0
```

The exact commands substitute that path for `$consumer` and this repository for
`$framework`:

```powershell
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/periodic_work.rs" -Destination "$consumer/tests/periodic_work.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test periodic_work
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/periodic_work.rs", "$consumer/tests/periodic_work.rs"
```

Every command passes; all ten tests execute successfully. Actual metadata has
one independent workspace member, two graph packages and the sole framework
path dependency. The framework has no dependencies. Original/copied bytes and
the SHA256 above match exactly; tests import only public APIs and std, without
include/path access to private sample code. Independent Codex
repeated_work_assessment separately inspects the raw commands/logs, metadata and
source identity. This verifies this consumer, not general downstream adoption.

## Source, documentation and complete review

Whole-tree temporary review aids report 49 handwritten Rust files, 20,260
physical lines, 1,241 comment-only lines and zero width findings. All three
existing item-level expectations are unchanged and fulfilled under Clippy;
no new lint, threshold, dependency, unsafe code or checker policy is introduced.
88 Markdown files have 490 resolving relative links and all 150 exact
traceability test references resolve uniquely. Twelve changed Markdown documents
are rendered and inspected for complete text, headings, code, tables, links and
HTML structure. Generated public API text is also inspected. This is structural
and content review, not pixel-level or human acceptance.

Root Codex and independent Codex repeated_work_assessment accept the complete
16-file source/document/diff and actual local/independent evidence with no
remaining findings. The capacity-two inbox documentation correction is
incorporated. Independent review verifies the completed 186-test baseline,
ten focused/ten independent tests, metadata/hashes, whole-tree policy and
rendered/generated API evidence. This is actual Codex review, not human review.
Authorized publication and exact hosted evidence remain pending.
