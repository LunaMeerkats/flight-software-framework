# Combined host sample

Date: **2026-09-12**
Status: **Complete**

## Objective and context

Compose the existing services in one executable, repeatable host scenario.
ADR-0020 and ADR-0021 resolved the recorded owner gaps. The next useful step
is evidence that the actual mission can use these APIs together, without a
new library abstraction. Keep the two-byte command/telemetry contract.

Started clean on `codex/nightly` at
`c8696f041d05a19aabb22c37871d1381425c06f9`. The six required baseline commands
pass with 109 tests. Toolchain unchanged: rustc/cargo 1.98.0, rustfmt
1.9.0-stable, Clippy 0.1.98. The initial audit passes for 29 Rust files with
zero width findings, three existing reasoned expectations, 34 Markdown files,
117 relative links, eight requirement rows, and 73 exact test references.
Existing formatting and function-size policies are already encoded.

## Acceptance criteria

- Use the existing two independently defined mission applications, one bounded
  configuration table, manual clock, two-item schedule, event queue, and host
  mailbox. Preserve the adapter grammar and all existing regression evidence.
- Make actual ordinary-work configuration observations visible through two
  fixed latest-value slots. Validate, activate, reject, roll back once, and
  demonstrate revision non-reuse through callback observations.
- Record both applications' registration/start/stop/restart states. Show
  waiting and equal-time scheduled work with explicit caller order.
- Inject one explicit cooperative echo error through ordinary work with event
  reporting. Preserve exact errors, selected-inbox clearing, timestamp, peer
  message and subsequent peer work. Do not add scheduled event behavior.
- Execute the same driver in the binary and integration tests; return a fixed
  structured report and compare fresh-run traces. Keep stdout outside callbacks.
- Add no public library API, dependency, unsafe code, suppression, recurrence,
  physical I/O adapter, or broader fault/recovery claim.
- Pass required baseline, focused adapter/sample tests, executable output,
  source/diff review, document audits, and rendered-document inspection.

## Components and verification

Private `mission/work.rs` owns bounded observations, validation, and the fault
fixture. `mission.rs` composes configured applications and ingress;
`applications.rs` keeps message behavior. `sample.rs` owns explicit scenario
order and its fixed report; `main.rs` prints diagnostics after the scenario.
`tests/host_sample.rs` loads the same driver. ADR-0022 records alternatives,
ownership, scope, and resource limits. Update the current state and traceability.

Run all AGENTS.md baseline commands with rustdoc warnings denied, plus
`cargo test --test host_adapters`, `cargo test --test host_sample`, and
`cargo run --example host-echo`. ADR-0018/0019 and their experiments remain
unchanged; their separate probe commands do not apply. Temporary audit and
render aids stay ignored under `target/nightly-2026-09-12`; no checker gate
is adopted.

## Risks and safe stopping point

The sample uses fixed local inputs and an explicit fault fixture. Observation
slots retain only latest values, and stdout errors cannot undo completed work.
No panic/hang containment, recovery, event/physical-delivery guarantee, or
real-time behavior follows from this scenario. CI and human architecture/scope
review remain release gates.

Stop after one coherent verified local commit. Preserve unexpected changes;
if completion fails, safely remove only current-run incomplete implementation
and retain useful decision evidence. Nothing is pushed.

## Outcome and verification

The combined sample completes one Stage 3 behavior checkpoint. It uses the
existing library APIs, preserves the command grammar, and adds five tests of
the shared complete driver plus one adapter fixture test. The initial and
final baseline pass; the final suite has 115 tests. All 14 adapter and five
sample tests pass. The executable completes with the three documented output
records and the expected configuration, lifecycle, schedule, and failure trace.

Passed commands, run serially from the repository root:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --test host_adapters
cargo test --test host_sample
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo run --example host-echo
git diff --check
```

Rustdoc uses `$env:RUSTDOCFLAGS = '-D warnings'`. Rustfmt also ran to format
the implementation before the final checks. ADR-0018/0019 and their probes
are unchanged and were not separately rerun. No CI run is claimed.

Review found that a retained observation could falsely establish later work.
Reads now consume the latest slot, so each scenario phase requires a fresh
callback observation. The final checks above include this correction.
Independent complete-diff review found no additional source defect and caught
one stale architecture statement denying the sample's existence; it is fixed.
All 32 handwritten Rust files meet physical/comment-only widths; three
pre-existing reasoned expectations remain fulfilled. New/touched source has
no block comments needing exceptional width treatment, waiver, or suppression.

Document audits pass for 36 Markdown files, 128 relative links, eight
requirement rows, 26 source IDs, and 79 exact test references. Generated HTML
content comparison passes for all 11 changed/new documents, including source
hashes, normalized text, headings, code, lists, and table cells.
Browser DOM inspection of these rendered documents at 1,280 pixels found no
page/table overflow; browser console warning/error inspection was empty.
The final evidence-only edits are rendered and checked again before commit.

The first browser selection timed out; Chrome was unavailable. Selecting the
available in-app browser succeeded. Its screenshot command then timed out at
`Page.captureScreenshot`. The narrow document-review adaptation is generated
content comparison plus rendered browser DOM/layout inspection. Screenshot
visual QA is not claimed; no alternative capture or policy bypass was used.

Review aids and logs remain ignored under `target/nightly-2026-09-12`.
The audit/render commands below passed with `review` and are rerun with `final`
after evidence is recorded. They remain temporary aids, not policy gates.

```text
& ./target/nightly-2026-09-12/render-documents.ps1 final
python target/nightly-2026-09-12/audit-documents.py final
python target/nightly-2026-09-12/review-rendered-content.py final c8696f0
```

This is a reviewed local increment. CI for these exact checks is the likely
next bounded objective; human release reviews remain separate. Nothing is
pushed.
