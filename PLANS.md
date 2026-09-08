# Bounded host command and telemetry adapters

Date: **2026-09-09**
Status: **Complete**

## Objective and context

Implement the ADR-0019 two-byte echo command and telemetry adapter pair using
the existing messaging runtime. Its grammar and borrowed output ownership
already have a decision and executable probe. The clean starting commit is
`3674863865d68ced666a1fc0b3e478b672fc79c3` on `codex/nightly`. All six baseline
commands pass with 83 tests on unchanged rustc/cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. Existing source policy needs no cleanup.

## Acceptance criteria

- Decode exact command length, identifier, and percentage in that order before
  publication; reject malformed input without changing retained state.
- Revalidate internal topic, one-byte length, and percentage before business
  logic or output occupancy; preserve concrete errors and ordered reports.
- Compose independently defined echo and telemetry applications with two
  capacity-one inboxes and a borrowed capacity-one host output mailbox.
- Keep ingress, each dispatch, and consume-once host drain separate; prove
  exact output, slot reuse, saturation, lifecycle retention, and replay.
- Keep full-output failure terminal with exact selected-inbox clearing and
  old host output retained. Do not retry implicitly or claim execution rollback.
- Share actual mission source between the executable example and integration
  tests. Add no library API, dependency, unsafe code, callback I/O, thread,
  general protocol, or complete v0.1 sample claim.
- Pass the baseline, focused tests, example execution, ADR-0019 experiment,
  source-form and complete diff review, link audit, and document rendering.

## Components and source placement

Use Cargo's multi-file example convention at `examples/host-echo/main.rs`.
`mission.rs` owns composition and ingress; its `codec.rs` and `applications.rs`
children separate record validation from application/output behavior. Mission
items remain private to their target, with crate visibility where needed.
`tests/host_adapters.rs` loads `mission.rs` with one explicit relative `#[path]`,
exercising the same code without a library export, copied implementation,
generated source, or extra crate. The Rust Reference documents the relative
path rule; Cargo documents multi-file targets. An initial test compilation
exposed different child lookup under the attributed parent. Explicit child
paths in `mission.rs` keep both target forms on the same two child files.

Update ADR-0019, README, architecture, requirements, roadmap, traceability,
project state, source register, and AGENTS.md. Review aids stay under ignored
`target/nightly-2026-09-09`; no new checker gate is adopted.

## Verification and limits

Run the six AGENTS.md baseline commands serially with rustdoc warnings denied,
plus `cargo test --test host_adapters`, `cargo run --example host-echo`, and
the explicit build, standalone rustdoc, extracted rustfmt and Clippy commands
in the host-mailbox experiment. Audit all handwritten Rust widths and inspect
changed rendered documents.

The normal topology cannot deliver a wrong topic or have an absent command
subscriber; tests may explicitly compose alternate topology for those cases.
The one-byte message bound rejects oversized internal payloads at construction.
No allocator-injection seam exists: review error preservation in source and
exercise full/unavailable reports; do not claim injected allocation failure.
Business-call cardinality combines the single-call source path with exact
one-message/output tests, without adding operational instrumentation.

## Risks and safe stopping point

The returned two-byte array is the verified output boundary. Printing it in
the host example is a diagnostic outside callbacks, with no retry or delivery
guarantee. The grammar is local and unfrozen. The full service sample, CI, and
v0.1 architecture review remain separate work.

Stop after the adapter pair is tested, documented, reviewed, and committed
locally. If implementation cannot preserve the baseline, remove only this
run's incomplete implementation safely and retain useful findings. No push.

## Outcome

Implemented the private codec, echo and telemetry applications, fixed mission
composition, ingress adapter, and host drain. Thirteen integration tests share
the example source and cover every valid percentage, all unsupported command
identifiers, all out-of-range percentages, validation precedence and retained
state, explicit dispatch, original reports, saturation, lifecycle retention,
and replay. The first test build exposed child-module lookup under `#[path]`;
explicit child paths resolved it without changing the runtime library.

The required baseline passes with 96 tests. Focused adapter tests, example
execution, library build, standalone mailbox rustdoc, extracted rustfmt and
warnings-denied Clippy pass. The unchanged 185-line probe passes its source
width audit. All 27 handwritten Rust files have no physical/comment-only width
findings; the three existing expectations remain unchanged and fulfilled.
Independent full implementation/test/diff review found no remaining defect.
One stale experiment status was corrected during the document review.

Generated-HTML source hashes, complete normalized content, headings, lists,
code, and tables match changed documents. The link/source/traceability audit
passes. Browser inspection at 1,280 pixels checks headings and page/table
overflow. Screenshot capture failed through both documented APIs with
`Page.captureScreenshot` timeouts. The narrow document-review adaptation is
generated-content comparison plus browser DOM/layout inspection; screenshot
visual QA is not claimed. No browser safety policy was bypassed.

The stopping point is this verified adapter pair, implemented and tested in
local commit `2c337f311da7d62230d626bd10c7fbe1383b586e`. The final audit covers
32 Markdown files, 99 resolving links, eight requirement rows, 26 sources,
60 exact test references, and 11 changed rendered documents. The full service
sample, CI, and v0.1 architecture review remain separate. Nothing is pushed.
