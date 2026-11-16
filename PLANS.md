# Retry a work failure rejected by the event queue

## Objective and acceptance

The typed work error exposes an exact event even when the queue is full.
Extend `tests/host_progress.rs` and its guide under RFF-LIB-REQ-016 revision 5
and [ADR-0041](docs/adr/0041-host-observed-application-progress.md#retry-a-queue-rejected-work-failure).

Save the event in one caller-owned slot only after queue rejection. Re-offer
it on an explicit turn and clear the slot on queue acceptance. Verify a full
retry, admission while an older event is pending at the sink, exact ordering
and timestamp, the terminal work state, and empty retries for recorded or
absent initial attempts. Retain the previous nine histories.

## Verification and stopping point

Run the locked baseline, focused tests and independent package recipe with
identical source. Review the complete diff, links, rendered guidance and Rust
source form. Inspect exact hosted CI after the ordinary main push. Stop at
the demonstrated retry. The caller controls retry timing and the sink must
return for the serial host to continue. If verification fails, preserve the
evidence and withdraw only this run's incomplete changes.

## Result

The locked local baseline passes 276 tests. All 11 host progress tests pass
in the workspace and with identical source in an independent package.
Implementation `4801928432def9b0003eaf8f219d6adb27f4b77b` passed all 20
steps in hosted run 43873421289.
[Queue retry review](docs/verification/HOST_PROGRESS_REVIEW.md#queue-rejected-failure-follow-up)
