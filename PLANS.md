# Messaging-owned work failure events

Date: **2026-09-10**
Status: **Complete**

## Objective and context

Expose the existing cooperative returned-work failure event through the
messaging owner while preserving exact selected-inbox clearing. The full
service sample cannot currently reach `Runtime::work_with_failure_event` or
scheduled work after the runtime moves into `MessagingRuntime`. Its private
owner is deliberate: mutable escape would bypass lifecycle/inbox coordination.
This run implements only the event prerequisite; scheduling and the combined
sample remain separate bounded work.

The starting tree is clean on `codex/nightly` at
`4cb621aa50708fbc823dd50f110db7404a461889`. All six required baseline commands
pass with 96 tests on unchanged rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and
Clippy 0.1.98. The source-quality policy is already encoded.

## Acceptance criteria

- Add one opt-in `MessagingRuntime::work_with_failure_event` that delegates to
  the existing messaging work operation before reporting an application error.
- Preserve the order: callback return, terminal `Failed`, selected-inbox
  clearing, one clock read, one bounded event attempt. Preserve peer queues.
- Reuse `RuntimeWorkEventError` inside `MessagingOperationError`, retaining the
  complete work error, optional event attempt, and exact discarded count.
- Share the internal event construction with the direct runtime method, with
  no new public constructor, error type, mutable runtime escape, or dependency.
- Success and lifecycle rejection read no clock and emit nothing. Saturation
  retains old events and returns the rejected event without retry or recovery.
- Verify later peer work and dispatch, configuration visibility/history,
  unknown/registered/stopped/failed suppression, and original error sources.
- Pass baseline checks, focused integration tests, source/diff review, link and
  traceability audits, and changed rendered-document review.

## Components and verification

`src/messaging_runtime.rs` owns the new operation because it owns inbox
cleanup. `src/runtime_events.rs` keeps the single crate-private event-reporting
implementation. `tests/messaging_work_events.rs` provides public-API service
interaction evidence. ADR-0020 records alternatives and exact operation order;
README, architecture, roadmap, traceability, project state, source register,
and contributor guidance record the resulting scope.

Run all six AGENTS.md baseline commands with rustdoc warnings denied and
`cargo test --test messaging_work_events`. Host adapters and ADR-0018/0019
experiments are unchanged, so their extra commands are not required. Review
aids remain ignored under `target/nightly-2026-09-10`; no checker gate is added.

## Risks and safe stopping point

Only cooperative returned work errors are covered. Panics, hangs, clock
failures, message-callback events, scheduled events, and physical delivery are
outside this increment. Event storage can saturate. Existing owner-origin and
application partial-effect limitations remain. The full sample still needs
messaging-aware scheduling and an explicit mission driver.

Stop at one tested, documented, reviewed local commit on `codex/nightly`.
Preserve unexpected changes; if the implementation cannot pass, remove only
this run's incomplete edits safely and retain useful evidence. Do not push.

## Outcome

Implemented the one opt-in messaging-owned work event operation. It delegates
through existing work before the shared event policy, preserving exact inbox
cleanup, concrete errors, configuration history, and peer progress. Six focused
integration tests pass. The full baseline passes 102 tests, warnings-denied
Clippy/rustdoc, formatting, all-target check, and Git whitespace inspection.
The host adapters and ADR-0018/0019 probes are unchanged and were not separately
rerun; the full test suite includes all 13 existing adapter tests.

Independent complete-diff review found no actionable defect. All 28 handwritten
Rust files meet physical/comment-only width limits; the three existing reasoned
expectations remain unchanged and fulfilled. The document audit passes for
33 Markdown files, 107 relative links, eight requirement rows, 26 sources, and
66 exact test references. A temporary audit initially misclassified a method
name as a test reference; explicit invocation notation corrected the document
ambiguity without changing the checker or adopting a new gate.

All ten changed documents match generated content, heading/list/code/table
structure, and source hashes. Browser layout inspection at 1,280 pixels found
no page/table overflow, heading gaps, or console warnings. A screenshot of the
new decision's opening viewport was also inspected successfully; complete-page
screenshot coverage is not claimed. Review aids remain ignored under the run
directory. No document-review adaptation or browser policy bypass was needed.

The implementation reaches its tested and reviewed stopping point. The local
commit hash is recorded in the subsequent evidence update. Scheduling through
the messaging owner remains the likely next bounded increment. Nothing is
pushed.
