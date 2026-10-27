# Bounded host event presentation review

Date: **2026-10-28**
Status: **Final local/independent verification and Codex review pass; hosted inspection pending**

## Objective, contract and source identity

RFF-LIB-REQ-007 revision 1 and
[ADR-0032](../adr/0032-bounded-host-event-presentation.md) precede source changes.
Root Codex and consumer_assessment select a concrete byte-output consumer over
adding a library sink API or blindly retrying write_all. Existing event APIs
suffice. This is source-based need assessment, not external adoption research.

The clean input is 6ab84023892b19642b76705bd184a95bb43754df, matching refreshed
origin/codex/nightly. Live input run 40329895459/job 121596214543 passes all
20 steps, 193 workspace tests, separate 14 adapter/five sample tests and the
222-line sample. Root raw-log inspection and connector step summaries agree.
The connector's commit-run query returned an empty list; gh metadata and actual
job logs supplied the exact-revision evidence, rather than inferring no run.

The [public-only example](../../examples/event_output.rs) owns its writer, one
pending event, a 96-byte line and a byte cursor. A host turn attempts at most
one write and retains exact progress across errors. It never removes the next
event while one is pending. Rendering uses fixed scratch storage and explicit
mission names. Writer acceptance, flush and physical delivery remain distinct.
The [guide](../EVENT_OUTPUT.md) describes source association, storage, errors,
stream continuity and the runnable sample's stop-and-drop policy.

Reviewed example SHA256 is
16ff085415e6078c8d33dda201798b316009b147885a75f9917d8141fcc38817;
Git blob is 8eab43dd31c1bb6f34c486ebe5b53c87653ea561. Library source,
existing tests/examples, lockfile, licences and workflow are unchanged. Cargo
adds only the test-enabled example target; there is no dependency or waiver.

## Executed local verification

Initial locked baseline passes 193 tests. An intermediate full baseline passes
201, eight focused tests and the two-line example. After source-form review
corrects three width findings, the final commands below pass on the frozen source
with 201 workspace tests, eight focused tests and the exact two-line output:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --example event_output
cargo run --locked --example event_output
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

RUSTDOCFLAGS is set to -D warnings before rustdoc. Local tools remain
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. The first author
compile failed on ErrorKind conversion directly into `Box<dyn Error>`; explicit
io::Error conversion and accurate sample errors fix this before passing tests.
No host adapter/sample, old consumer guide, workflow or borrowing-experiment
change triggers additional local conditional commands. Hosted CI still executes
the focused host targets and sample separately.

## Public-only and independent consumer evidence

Eight tests observe exact FIFO bytes across short writes, Interrupted and
WouldBlock; zero progress and BrokenPipe with retained suffix; raw-event retention
on unknown-source rendering rejection; every finite label with Duration::MAX;
filter omission and idle turns without writer calls; queue refill while a record
is pending; the exact runnable output and separate flush-failure reporting.
The host owns retry policy. Tests use a scripted writer with an unbounded Vec
as an observation ledger only; the presenter uses fixed storage. The sample
stops on errors and drops volatile state rather than pretending to persist it.

Actual independent package:
C:/Users/drrsm/AppData/Local/Temp/rff-event-output-264c8f8447ac4cdcb99045e9cd129c43.
The [guide recipe](../EVENT_OUTPUT.md) executes from the framework checkout:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ('rff-event-output-' + [guid]::NewGuid().ToString('N'))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/event_output.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/event_output.rs", "$consumer/src/main.rs"
```

Creation/add/copy/check/Clippy/eight tests/run pass. Normalized Path comparison
confirms the independent workspace root, one member, two graph packages, sole
framework path dependency and no framework dependencies. Bytes/hash match the
example. After the width-only change, recopy and final check/Clippy/eight tests/run,
metadata, byte equality and the SHA256 above all pass. Both runs print exactly:

```text
t=0.000000000 source=framework severity=info event=started
t=0.250000000 source=worker severity=warning event=degraded
```

## Review and publication boundary

Root Codex reviews public ownership, error sources, bounded resources, exact
progress, source naming, full precision, tests and complete diff. Independent
output_review finds no behavioral defect; three width findings and the sample's
volatile error-exit wording are corrected. Final whole-tree review covers
51 Rust files, 21,222 physical and 1,284 comment-only lines with zero width findings and three unchanged fulfilled
expectations. All 531 relative links and 165 exact test references resolve
across 94 Markdown files. All 12 changed rendered documents pass full content
and structure inspection. Rendering caught and corrected an unescaped generic
type in this record; source content was unaffected. This is Codex review, not
human or pixel acceptance. Raw local/consumer/hosted logs and temporary review aids are retained
under ignored target/review-2026-10-28-event-presentation.

Root Codex and independent output_review accept the complete 14-file staged
source/document diff and actual final consumer/baseline/rendered evidence, with
no remaining findings. Root also accepts this review-attribution wording.
Ordinary fast-forward source publication and exact hosted inspection follow. No later revision inherits an
unobserved pass. The final documentation-only evidence completion receives its
own source/diff review and hosted inspection; exact final head/result is retained
in automation memory.

## Limitations

This verifies one controlled host consumer, not adoption or API stability.
Queue plus one pending record is a logical bound; fixed rendering scratch and
writer behavior do not bound process memory or blocking. Source labels are
caller-scoped. Standard Write contracts are assumed. Broken streams, reconnects,
flush failure, panics, process loss and durability require separate policies.
No framework wire format, reliable delivery, real-time behavior, qualification,
operational suitability or release readiness is established.
