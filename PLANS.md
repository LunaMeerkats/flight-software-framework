# Account for downstream deferral in periodic progress observation

## Objective and acceptance

Compose the existing host progress policy with a public periodic cursor
and one downstream sample slot. Keep the three directly selected work
histories. Follow RFF-LIB-REQ-016 revision 2 and
[ADR-0041](docs/adr/0041-host-observed-application-progress.md).

Verify that an occupied slot preserves the sample, next release and
application input through deferred turns. Exclude only the observed
blocked interval, then show late re-arming and an overdue warning at the
adjusted threshold. Preserve a returned error at equality and disable the
periodic cursor. Keep observation and callback admission in the serial host.

## Verification and stopping point

Run the locked baseline, focused workspace tests and the independent package
recipe with byte-identical source. Review source form, links, rendered
guidance and the complete diff. Inspect exact hosted CI after the source push.
Stop after one periodic consumer policy. If verification fails, retain the
evidence and withdraw only this run's incomplete changes.

## Result

The locked local baseline passes 270 tests. All five host progress tests
pass with identical source in the workspace and an independent package.
Implementation `7a6150524c5bfdbf0bbfb8cee818e66a5dced280` passed all 20
steps in hosted run 43431041267.
See the [periodic deferral review](docs/verification/HOST_PROGRESS_REVIEW.md#periodic-deferral-follow-up).
