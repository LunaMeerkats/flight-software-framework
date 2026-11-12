# Observe overdue application progress on completed host turns

## Objective and acceptance

Demonstrate that a serial host using public APIs can detect missing sample
output while the application continues returning successfully. Follow
RFF-LIB-REQ-016 and [ADR-0041](docs/adr/0041-host-observed-application-progress.md).

Use one application, a bounded sample slot, a ManualClock and a capacity-one
event queue. Verify the inclusive threshold, fresh-sample precedence, a
second overdue interval after fresh progress, and a rejected warning without
automatic retry. Exercise the existing typed callback error and later
lifecycle rejection at the same clock reading.

## Verification and stopping point

Run the public integration test inside and outside the workspace with
identical source. Run the locked baseline, review source form and the
complete diff, render the changed guidance and inspect exact hosted CI.
Stop at the serial consumer policy. If verification fails, keep the
evidence and withdraw only this run's incomplete changes.

## Result

The locked local baseline passes 268 tests. All three focused tests pass
with byte-identical source in the workspace and an independent package.
See the [focused review](docs/verification/HOST_PROGRESS_REVIEW.md).
