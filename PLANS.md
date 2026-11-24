# Report returned message-callback failures

## Objective and selection

Add one opt-in message-dispatch failure-event operation under
RFF-LIB-REQ-020 revision 1. Clean input main
`151511aec980dffe41dc25ab5d406798437459ee` matches fetched origin/main.
The owned-output increment is complete. Ordinary work has a standard
failure-event path; message callbacks require missions to repeat that
source, timestamp, saturation and error-preservation policy.

Select a small owner-preserving API over another host-only policy recipe.
Do not translate message errors to work errors, expose the inner runtime,
or introduce a generic callback framework. ADR-0045 compares alternatives.

## Acceptance and components

- Delegate once to dispatch_one; complete selected lifecycle/inbox cleanup
  before reading the supplied clock once and attempting one Error event.
- Preserve the exact message error, discard count and event attempt through
  a distinct MessageDispatchEventError nested in MessagingOperationError.
- Empty/success/lifecycle rejection reads no clock and emits nothing.
- Preserve self/peer publication semantics and reject-newest saturation;
  explicit caller retry retains the original event timestamp.
- Public-only tests cover these paths, ownership accessors and error chains.
  Run identical tests as an independent path-dependent Cargo consumer.
- Update requirements, architecture, guide, ADR, traceability and concise
  state. No dependency, workflow, historical-consumer or lint changes.

## Verification and stopping point

Establish the locked pre-edit baseline, then run focused tests and the
locked format/check/Clippy/workspace-test/warnings-denied-rustdoc/whitespace
baseline. Check/lint/test the independent consumer, inspect metadata and
compare exact test source bytes/SHA256. Audit handwritten Rust widths,
waivers, links and exact test references. Render and inspect changed docs;
review the complete diff with an independent Codex reviewer.

After checks/review pass, commit and ordinary fast-forward push main.
Inspect exact-head hosted CI and record actual evidence. Stop after this
increment. Preserve the input baseline; revert only current-run changes if
no healthy result is possible. Callbacks/clocks must return; reporting
does not add rollback, recovery, guaranteed delivery or origin validation.

## Local result

The locked input baseline passes 304 tests. Final format/check/Clippy,
311 workspace tests, warnings-denied rustdoc and whitespace pass, along
with all seven focused histories. Identical independent consumer check,
Clippy/seven tests/metadata pass with exact bytes/SHA256. Root and
independent Codex accept source and tests. Whole-tree width/link/test-
reference and rendered-document review complete before publication;
the [focused record](docs/verification/MESSAGE_FAILURE_EVENTS_REVIEW.md)
preserves exact results and limits. No implementation remains. Commit,
ordinary source publication and exact-head hosted inspection follow review.
