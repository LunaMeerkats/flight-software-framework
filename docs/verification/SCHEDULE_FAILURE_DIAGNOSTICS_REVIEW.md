# Scheduled-failure diagnostics review

Date: **2026-09-28**
Scope: **Existing ADR-0015/0021 error-source and final-consumption contract**

## Decision and inspected boundary

Retain the existing direct and messaging-owned finite scheduled-work error
types. Close the direct path's public diagnostic-evidence gap by executing both
of its `std::error::Error::source` chains. No production mismatch was found in
`src/scheduling.rs` at starting revision
`bb3ec40cac4d72e9ca8b9d38c3a191fa55cdb070`. This review supplements
RFF-REQ-004, [ADR-0015](../adr/0015-caller-driven-scheduled-work.md), and
[ADR-0021](../adr/0021-messaging-owned-scheduled-work.md) without changing
behavior.

For a due item, `ScheduledWorkError` retains the consumed item, the one observed
clock reading, and the exact `RuntimeWorkError`. Its standard source is that
runtime error. The runtime error's source is then either the exact
`LifecycleError` that rejected work or the concrete error returned by the
application. The existing messaging wrapper adds one outer
`MessagingOperationError` source while retaining the same inner chain and the
selected-inbox discard count.

## Executed public observations

Two existing direct public tests now inspect the previously implicit source
contract before consuming their typed errors:

- `lifecycle_rejection_consumes_one_item_without_blocking_a_due_peer` proves
  `ScheduledWorkError -> RuntimeWorkError -> LifecycleError`, exact stopped
  state, final item consumption, callback suppression, and later peer progress.
- `returned_work_error_consumes_one_item_without_blocking_a_due_peer` proves
  `ScheduledWorkError -> RuntimeWorkError -> TestApplicationError`, terminal
  failure of only the selected application, final consumption, and later peer
  progress.

The unchanged messaging-owned regression
`returned_failure_clears_exact_selected_inbox_and_preserves_peer_fifo` continues
to prove its additional wrapper source, exact two-message cleanup, complete
inner scheduled/runtime/application chain, and retained peer FIFO.

## Alternatives and limits

Flattening nested errors would lose owner-specific context. Adding public
conversion methods or changing error types is unnecessary because the typed
accessors and standard source traversal already expose the required context.
Adding scheduled failure events would introduce separate reporting-order and
timestamp decisions explicitly deferred by ADR-0015 and ADR-0021.

This evidence covers cooperative returned errors and lifecycle rejection. It
does not add retry, recurrence, event reporting, panic or hang containment,
execution rollback, API stabilization, or human acceptance. Final schedule
consumption means the caller must configure a separate item for any explicit
retry.

No new external source was needed. The observations follow accepted local ADRs
and executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 134 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. After the assertions were added,
`cargo test --locked --test scheduled_work` passes 11 direct tests and
`cargo test --locked --test messaging_scheduled_work` passes nine
messaging-owned tests. The final locked baseline passes 134 workspace tests.
Formatting, all-target checking, warnings-denied Clippy/rustdoc, and whitespace
checks pass without a new exception.

Source, link, traceability, and generated-document audits pass: 32 Rust files
with zero physical or comment-width findings, no block comments, and three
unchanged fulfilled expectations; 51 Markdown files, 237 resolving relative
links, 35 source definitions, and 101 exact traceability test references.
PowerShell Markdown rendering for the five changed controlling documents passes
structural inspection. Pixel-level visual acceptance is not claimed.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. Publication and any
exact-revision hosted result remain pending. A local pass does not establish
hosted CI or human review.
