# Returned-stop failure diagnostics

## Objective and selection

Add opt-in shutdown failure events through both runtime owners under new
RFF-LIB-REQ-021 revision 1. Clean input main
`8fca981837327828b3caf6245984f5fa639e4fd6` matches refreshed origin/main.
Existing stop-failure evidence has nonempty selected cleanup and healthy peer
progress, but consumers must construct structured shutdown events themselves.
ADR-0046 compares the small stop path with host construction and broader
lifecycle wrappers. Preserve ordinary start/stop/restart and existing consumers.

## Acceptance and components

- Delegate once through each owner's stop; preserve direct ApplicationState
  and messaging MessagingStopOutcome on success.
- Return the exact RuntimeStopError, source chain, optional event attempt and
  messaging clear count. Report only returned application errors, after Failed
  commitment and owner cleanup, using one clock read and one Error event.
- Success and every rejected lifecycle state read no clock and emit nothing.
- Retain older events and rejected records at saturation; explicit retry keeps
  the original timestamp. Later rejected stops cannot duplicate reporting.
- Verify selected clearing, retained peer FIFO/work, ownership accessors and
  receiving-owner identity limits through public-only integration tests.
- Add a consumer guide, focused evidence, requirement, ADR and traceability.
  No dependencies, workflow changes, new service owner or implicit retry.

## Verification and stopping point

Run the documented locked input and final format/check/Clippy/workspace-test/
warnings-denied-rustdoc/whitespace baseline. Run focused stop-event tests and
identical source in an independent Cargo package; check/lint/test it and
inspect bytes/SHA256 and workspace/dependency metadata. Review whole-tree
widths, scoped expectations, Markdown links, rendered changed documents and
the complete diff with an independent Codex reviewer.

After local checks and reviews pass, commit on main and fast-forward push.
Inspect exact-head hosted CI and record its actual result. Stop at this one
increment. Preserve the baseline and revert only current-run implementation
if it cannot pass. Callbacks and clocks must return; reporting adds no recovery,
delivery guarantee or permission to release.

## Local result

The locked input baseline passes 316 tests. Final locked baseline commands
pass with 324 workspace tests, plus eight focused and eight identical
independent tests. Independent metadata and exact bytes/SHA256 match.
Root and independent Codex accept source/test/design review without findings.
The [focused review](docs/verification/STOP_FAILURE_EVENTS_REVIEW.md) records
actual commands, consumer path, results and limits. Whole-tree width/link/test-
reference audit and all 12 rendered-document content/structure reviews pass.
Root and independent Codex accept the complete implementation diff. Accepted
staged tree `e76dbfd7cfefe15476b42c599d158110feb663f1` matches implementation
`a81111c3e0293b6581931aa17745e633aeefdb56`, published by ordinary fast-forward.
Exact hosted run 45783032972/job 139782868369 passes all 20 steps, 324 workspace
tests and separate host checks. All 222 sample stdout lines match input.
Hosted tools remain unchanged; Windows image advances to `20261123.321.1`.
No implementation remains. Evidence-only completion receives separate review
and exact hosted inspection. Stop at this one increment.
