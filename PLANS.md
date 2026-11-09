# Retain one command across ingress rejection

## Objective and acceptance

Use [ADR-0039](docs/adr/0039-runnable-command-session.md#retained-ingress-follow-up)
and RFF-LIB-REQ-014 revision 4 to show explicit host recovery from a full
command inbox. Retain one typed command and its original ID outside the
inbox. An explicit admission turn uses the current routing path and clears
that slot on acceptance before the host writes diagnostics.

## Verification and stopping point

Input `9dbab6ad0432902f319af304d64c35232bc2a2ea` matches refreshed origin.
Hosted run 42483635231 passes 20 steps and 259 workspace tests. The local
locked baseline passes on replay. Windows Application Control transiently
blocked the existing message bus test executable on the first suite run.

Keep the original transcript as a prefix. Run an appended saturation and
FIFO recovery scenario and a stopped-owner retention regression. Check the
locked baseline, exact independent Cargo recipe, source shape, rendered
documents and complete diff. Publish the verified increment on main and
inspect exact hosted CI. Stop at a single caller-owned slot. If the
increment cannot pass review, preserve evidence and withdraw only this
run's incomplete edits.

## Result

The locked local baseline passes 260 workspace tests. The workspace example
and identical independent binary pass 14 focused tests. Their 40-line output
keeps the original 24 lines byte for byte. Root and independent Codex review
accepted the source and complete diff. Implementation
`b5e5ce0c0b48e23f4668fb512f3eab2094f4fc31` passes all 20 steps of exact hosted
run 42622185103. The record revision receives separate hosted inspection.
See the [focused review](docs/verification/COMMAND_SESSION_REVIEW.md#retained-ingress-follow-up).
