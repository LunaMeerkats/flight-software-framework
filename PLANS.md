# Messaging-owned one-shot scheduled work

Date: **2026-09-11**
Status: **Complete**

## Objective and context

Expose finite scheduled ordinary work through `MessagingRuntime`, preserving
one-shot consumption, exact errors, and selected-inbox cleanup. This is the
remaining recorded service-ownership prerequisite for the combined sample.
ADR-0015 requires this path to use `MessagingRuntime::work`; ADR-0011 owns
cleanup, and ADR-0018 keeps configuration on the same ordinary-work callback.
The existing source policy is encoded and needs no adoption change.

The starting tree is clean on `codex/nightly` at
`ca40d7beba8b17d5f722e633bbb45db06e6f62bc`. All six baseline commands pass
with 102 tests on unchanged rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and
Clippy 0.1.98.

## Acceptance criteria

- Add one `MessagingRuntime::run_next_scheduled_work` with the existing
  schedule, clock, outcome, and nested error types. Expose no mutable owner.
- Share the private schedule decision between owners: no clock read when
  complete, one read otherwise, no mutation while waiting, and at most one
  due or overdue item consumed before invoking ordinary work.
- Delegate to messaging-owned work so a returned error commits terminal
  `Failed`, clears only the selected inbox, and preserves the original error
  and exact discarded count. Lifecycle rejection consumes its item without
  invoking work or clearing any inbox. Later due peers remain operable.
- Preserve equal-time order, configuration visibility/history, explicit retry
  through separately configured items, and direct-runtime behavior.
- Verify complete/waiting boundaries, inclusive/overdue execution, exact clock
  reads, lifecycle/unknown rejection, error sources, peer FIFO, and replay.
- Add no scheduled event reporting, recurrence, new public error type,
  executor trait, dependency, unsafe code, lint waiver, or full-sample scope.
- Pass required baseline, focused tests, full-diff/source review, link and
  traceability audits, and changed rendered-document review.

## Components and verification

`src/scheduling.rs` owns the shared private clock/consumption decision and
existing scheduled error. `src/messaging_runtime.rs` owns delegation and inbox
error wrapping. `tests/messaging_scheduled_work.rs` exercises public service
interactions. ADR-0021 records the decision and alternatives. Update current
scope, requirement traceability, source provenance, and project state.

Run all six AGENTS.md baseline commands with rustdoc warnings denied and
`cargo test --test messaging_scheduled_work`. Existing direct scheduling tests
remain regression evidence. Host adapters and ADR-0018/0019 experiments remain
unchanged; their extra commands are not applicable. Ignored review aids belong
under `target/nightly-2026-09-11`; no checker gate is introduced.

## Risks and safe stopping point

Application/clock origins remain caller-scoped. Callback panic/hang containment,
rollback, recovery, timing guarantees, scheduled failure events, and automatic
retry remain outside scope. The full sample still needs explicit mission
driver order, followed by CI and architecture review.

Stop after one coherent tested, documented, reviewed local increment on
`codex/nightly`. If it cannot pass, remove only current-run incomplete edits
safely and preserve useful evidence and unexpected changes. Do not push.

## Outcome and verification

Implemented the one messaging-owned scheduled ordinary-work operation with
shared private timing/consumption. Seven focused tests cover clock boundaries,
order, lifecycle rejection, exact errors/cleanup, peer FIFO, configuration
history, and repeated manually controlled clock readings. The required full
baseline passes with 109 tests, including all seven existing direct-schedule
tests and all 13 unchanged host-adapter tests. No new public error type,
dependency, unsafe code, suppression, or mutable owner access was added.

Passed commands, run serially from the repository root:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --test messaging_scheduled_work
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
git diff --check
```

Rustdoc runs with `$env:RUSTDOCFLAGS = '-D warnings'`. The full baseline is
rechecked after the final Rust comment reflow. Host adapter execution and the
ADR-0018/0019 standalone probes are unchanged and not separately rerun.

Independent complete-diff review found no actionable code defect. It identified
stale direct-only event evidence wording, now corrected to reflect ADR-0020.
The first source audit found one new 81-column comment; reflow fixed it without
a policy exception. All 29 Rust files now meet physical/comment-only width
limits, and the three existing reasoned expectations remain fulfilled.
The document audit passes for 34 Markdown files, 117 relative links, eight
requirement rows, 26 sources, and 73 exact test references. All 12 changed
documents match rendered content, headings, lists, code, tables, and hashes.

The first browser initialization timed out and reset; opening the prepared local
review server then succeeded. Browser DOM layout inspection of all 12 changed
documents at 1,280 pixels found no page/table overflow, heading gaps, or console
warnings. Screenshot capture timed out. Generated-content and browser DOM
layout review are the narrow document-review adaptation for this run;
screenshot-based visual QA was not completed. No alternate screenshot mechanism
or browser policy bypass was used.

Ignored review aids and command logs remain under `target/nightly-2026-09-11`.
The render/audit commands below ran with `validated` and are rerun with `final`
after recording the evidence; they are review aids, not adopted checker gates.

```text
& ./target/nightly-2026-09-11/render-documents.ps1 final
python target/nightly-2026-09-11/audit-documents.py final
python target/nightly-2026-09-11/review-rendered-content.py final ca40d7b
```

The increment reaches its tested, documented stopping point. The likely next
bounded task is explicit composition of the combined sample. Nothing is pushed.
