# Account for known command admissions when stopping the producer

## Objective and acceptance

Use the existing messaging lifecycle and typed command-result APIs to
distinguish one handled command from two commands cleared at stop. Follow
RFF-LIB-REQ-014 revision 6 and the
[ADR-0039 decision](docs/adr/0039-runnable-command-session.md#command-clearing-follow-up).

Start the fixed scenario with an empty command inbox and result slot.
Dispatch one command, retain its result and admit two known commands while
that result gates further dispatch. Check the exact stop count before
attributing the clearing. Restart, drain the earlier result and verify
that producer dispatch reports an empty inbox.

## Verification and stopping point

Preserve the preceding 40 transcript lines byte for byte. Exercise the
complete new transcript and reject unexpected starting backlog. Run the
locked baseline and the identical independent binary recipe. Audit source
form, rendered docs and the complete diff. Inspect exact hosted CI after
publication.

Stop at the private serial consumer. If verification fails, retain the
evidence and withdraw only this run's incomplete changes.

## Result

The locked local baseline passes 265 tests, including 19 focused tests.
The independent binary produces the identical 51-line transcript.
Implementation `1b746222d535eb467236c85c69170f7888576f58` passes all 20
steps of hosted run 43045074920.
See the [focused review](docs/verification/COMMAND_SESSION_REVIEW.md#command-clearing-follow-up)
for the exact source, transcript and publication evidence.
