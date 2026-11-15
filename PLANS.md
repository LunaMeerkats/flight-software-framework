# Retain a diagnostic across sink rejection

## Objective and acceptance

A host that dequeues an accepted diagnostic needs a way to keep it when its
sink returns busy. Extend the public-API sample consumer under
RFF-LIB-REQ-016 revision 4 and
[ADR-0041](docs/adr/0041-host-observed-application-progress.md#hand-off-diagnostics-to-a-fallible-sink).

Use one host-owned typed sink and one optional pending event. Each output
turn offers at most one record. Retain the event on rejection, return it on
acceptance, and leave work and output turns independently selected. Verify
repeated rejection, FIFO recovery and idle output. Exercise both queue
saturation on work failure and handoff of an accepted failure event.

## Verification and stopping point

Run the locked baseline, focused tests and independent package recipe with
identical source. Review the complete diff, links, rendered guidance and Rust
source form. Inspect exact hosted CI after the ordinary main push. Stop at
the demonstrated serial handoff. A sink call must return for the serial host
to continue, and a retry requires a definite rejection. If verification fails,
preserve the evidence and withdraw only this run's incomplete changes.

## Result

The locked local baseline passes 274 tests. All nine host progress tests
pass with identical source in the workspace and an independent package.
[Diagnostic handoff review](docs/verification/HOST_PROGRESS_REVIEW.md#diagnostic-handoff-follow-up)
