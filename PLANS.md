# Read-only subscription inspection

Status: Local and independent verification complete, 2026-11-06 Australia/Sydney.

## Objective and acceptance

Detect valid but misordered inbox configurations before command-session startup.
Add one read-only membership query to the existing messaging owners under
RFF-LIB-REQ-015 and [ADR-0040](docs/adr/0040-read-only-subscription-inspection.md).
Use the query at startup and verify a swapped configuration before any callback
runs. Preserve the original transcript, callbacks, routing and ownership APIs.

## Verification and stopping point

Input `ee1a8ed6cd6dacefc43835caf14d9d013dd5111b` matches fetched `origin/main`.
The local locked baseline passes on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and
Clippy 0.1.98. Exact input hosted run 41856418810 independently passes all
20 steps, 253 workspace tests and the 222-line sample comparison.

Run focused owner/session tests and the example. Verify the independent binary,
including source bytes, metadata and output. Run the locked baseline, inspect
source form and rendered Markdown, and review the complete diff. Commit on
`main`, push by fast-forward and check hosted CI for the exact revision.

Membership is a topology check. The host still pairs IDs with the owner that
issued them and supplies the expected routes. Stop at this query and consumer
check. If verification fails, preserve the useful evidence and withdraw only
this run's unfinished changes.

## Local result

All 256 locked workspace tests and warnings-denied rustdoc pass. The independent
binary passes 11 tests with the same source and output. Root and independent
Codex accept source. The
[focused record](docs/verification/COMMAND_SESSION_REVIEW.md#subscription-inspection-follow-up)
contains exact local commands, independent evidence and the hosted checkpoint.
