# Read one framed request into the command session

## Objective and acceptance

Compose a fallible finite reader with the existing saved command slot under
RFF-LIB-REQ-014 revision 5 and [ADR-0039](docs/adr/0039-runnable-command-session.md#framed-input-follow-up).
Use one example-local two-byte New record and the session issuer. Keep
the reader and prefix together across turns and read only when the slot
is free. Keep read progress, admission, callback and output separate.

## Verification and stopping point

Input `aaf4b92a15a7a53448e0a59169f87890931fee24` matches refreshed origin.
The local locked baseline and exact hosted run 42622569988 pass. Retain
the exact 40-line command-session transcript. Test short and fallible reads,
framing rejection, finite input termination, ID exhaustion and the unread
next record while admission is blocked. Run the locked baseline and
independent binary recipe, audit source form and rendered docs, review
the complete diff and inspect exact hosted CI after publication.

Stop at the private consumer boundary. If verification cannot pass,
preserve evidence and withdraw only this run's incomplete edits.

## Result

The locked local baseline passes 264 tests. The example and independent
binary pass 18 focused tests and match the input transcript byte for byte.
Implementation `12931b7e936b35bd0f4c9ffd15510213a6629efe` passes all
20 steps of exact hosted run 42822018637.
See the [focused review](docs/verification/COMMAND_SESSION_REVIEW.md#framed-input-follow-up)
for local, downstream and hosted evidence.
