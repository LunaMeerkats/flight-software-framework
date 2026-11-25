# Command-session failure diagnostics

## Objective and selection

Compose the completed message-failure API into the runnable command session
under RFF-LIB-REQ-014 revision 7. Clean input main
`d252d50d77230de7299ed3ed20d6d4fcda5f0d27` matches refreshed origin/main.
The existing consumer demonstrates command results and reply output, but its
callback failures have no executable structured diagnostic/output path.
Extend that consumer rather than copying its mission into another example
or adding a library presenter. ADR-0039 records the alternatives.

## Acceptance and components

- Preserve ordinary producer dispatch, historical tests and the exact prior
  transcript prefix. Add an opt-in diagnosed producer turn with the same
  occupied-result gate and the original typed failure/count/event outcome.
- Append a finite malformed-callback scenario: one consumed malformed head,
  one cleared tail, no success result, retained accepted peer replies and
  continued subscriber dispatch while diagnostic output is pending.
- Use a capacity-one event FIFO, one retained rejected failure and one
  output slot. Only one terminal producer failure can populate retention.
  Retry, dequeue and write are separate caller-selected turns; retries
  preserve the original event timestamp and never duplicate admission.
- Reuse the bounded writer/cursor policy with a compact diagnostic format.
  Preserve the output slot and suffix through short writes and errors;
  do not dequeue another event while that slot remains occupied.
- Update the guide, requirement, ADR, traceability and concise project state.
  No library API, dependencies, workflow, lint or historical fixture changes.

## Verification and stopping point

Establish the locked input baseline, then focused example tests/run and the
locked format/check/Clippy/workspace-test/warnings-denied-rustdoc/whitespace
baseline. Run the documented independent binary recipe and inspect exact
bytes/SHA256, workspace/dependency metadata and output/prefix equality.
Preserve message-event, callback and correlated-command regression evidence.
Review handwritten Rust widths/waivers, links, test references and rendered
changed documents, then obtain an independent complete diff review.

Commit and fast-forward push main after actual checks/review pass. Inspect
exact-head hosted CI and record results. Stop at this one consumer increment.
Preserve the baseline; revert only current-run implementation if necessary.
Callbacks, clocks and writers must return; no recovery, reliable transport,
durability or delivery guarantee follows from the controlled scenario.

## Local result

The locked input baseline passes 311 tests. All final baseline commands
pass with 316 workspace tests, plus 24 focused command tests, 14 adapters,
five sample tests and an unchanged 222-line combined sample. The identical
independent binary passes check/Clippy/24 tests/run/metadata; exact source
bytes/SHA256 and 65-line stdout match, preserving the 51-line input prefix.
Root and independent Codex accept source review without findings. The
[focused follow-up](docs/verification/COMMAND_SESSION_REVIEW.md#message-failure-diagnostics-follow-up)
preserves exact commands, paths and results. Whole-tree width/link/test-
reference and all 12 rendered-document content/structure reviews pass.
Exact-head publication evidence remains before stopping.
