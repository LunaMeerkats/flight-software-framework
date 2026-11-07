# Retain command-session results across short writes

## Objective and acceptance

Complete the result presentation follow-up to
[ADR-0039](docs/adr/0039-runnable-command-session.md) and RFF-LIB-REQ-014.
The host keeps the typed result in its existing slot until the writer accepts
the full line. Bind a fixed-size rendered line, its accepted-byte cursor, the
slot and the writer for one result. Each turn makes at most one write. An
error preserves the slot and accepted prefix. The next producer command stays
queued until the host completes the result. Keep the existing successful
transcript and the one-shot policy for subscriber and other diagnostics.

## Verification and stopping point

Input `fb2d728954980e042b7ca1d6af4278ed3b958767` matches fetched and live
origin. The locked local baseline passes 257 tests. Independent Codex
inspection confirms input hosted run 42222654426 passed all 20 steps and
reproduced the 222-line host sample.

Exercise short write, returned error, zero progress, invalid count, exact
suffix retry and a queued producer turn. Run the focused example and the
locked local baseline. Check, lint, test and run identical source in an
independent Cargo binary. Compare source and transcript bytes, inspect
metadata, and review source, rendered Markdown and the complete diff.
Publish the reviewed increment on `main` and inspect exact hosted CI.

Stop at command-result presentation. The fixed driver exits on writer error.
A serial host that keeps the presenter can choose another explicit turn.
If verification fails, preserve the evidence and withdraw only this run's
incomplete changes.

## Result

Local verification passes 258 workspace tests. The workspace and identical
independent binary each pass 12 focused tests and preserve the 24-line
transcript. Root and independent Codex source and document reviews pass.
See the [focused review](docs/verification/COMMAND_SESSION_REVIEW.md#retained-command-result-follow-up)
for exact checks and publication evidence.
