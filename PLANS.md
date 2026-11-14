# Report sustained downstream congestion on periodic host turns

## Objective and acceptance

A host that intentionally pauses sample observation needs a distinct way to
see a downstream slot that stays occupied. Extend the existing public-API
periodic consumer under RFF-LIB-REQ-016 revision 3 and
[ADR-0041](docs/adr/0041-host-observed-application-progress.md#sustained-downstream-congestion).

Use the first observed deferral and a single warning attempt at or after a
ten-second host threshold. Return the exact event and queue outcome. Retain
the pending sample, release and input, leave lifecycle handling with the
runtime and re-arm when the host observes space. Verify equality, queue
saturation and suppression, rearming and the existing sample-window histories.

## Verification and stopping point

Run the locked baseline, focused tests and independent package recipe with
identical source. Review the complete diff, links, rendered guidance and Rust
source form. Inspect exact hosted CI after the ordinary main push. Stop at
the demonstrated serial host policy. If verification fails, preserve the
evidence and withdraw only this run's incomplete changes.
