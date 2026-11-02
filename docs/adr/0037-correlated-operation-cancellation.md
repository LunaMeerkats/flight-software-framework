# ADR-0037: Correlated operation cancellation

- Status: Implemented; local checks, Codex review and exact hosted checks pass
- Date: 2026-11-03
- Scope: One session-local public-only callback consumer

## Context

ADR-0036 deliberately cancels whichever reply exists at dispatch. Its guide and
project state identify that a delayed command can therefore cancel newer work.
Root Codex and independent consumer_assessment select this concrete risk over
adding a framework coordinator. RFF-LIB-REQ-012 precedes implementation; the
original consumer and its exact evidence remain unchanged.

## Decision

Independently compose a second public-only consumer. Store an operation ID with
one pending reply and two acceptance flags. Cancel(id) clears only matching
pending state; Retry(id) attempts only matching rejected destinations. Idle or
stale control commands do nothing. A busy New command is consumed without
replacing the retained operation and requires explicit host resubmission.

The host owns one finite issuer for the entire session, retained across producer
stop/restart. Allocate IDs 1..=255 with checked addition, then return a typed
exhaustion error on every later request. Keep the ID when retrying a rejected or
busy command. This small domain makes the no-wrap boundary fully exercisable;
it is a controlled consumer choice, not a library identity API or wire standard.
Do not reset the issuer while old commands, replies or saved host requests can
survive. Payload parsing validates exact command shape and nonzero identity.

## Alternatives considered

- Host ordering alone: retain as valid ADR-0036 policy, but it leaves stale
  cancellation vulnerable to a later operation.
- Correlation only: selected. Fixed ID storage and equality guards close this
  risk using existing public callbacks and bounded publication.
- Result acknowledgements: defer. They need another bounded output admission,
  loss/retry and retention contract; correlation does not require them.
- Automatic failed-subscriber abandonment: defer. Lifecycle state does not
  establish whether a previously accepted message was processed.
- Wrapping or random IDs: reject here; wrapping aliases surviving commands and
  randomness does not prove uniqueness. Persistent epochs remain a separate need.

## Evidence and consequences

Official Rust u8::checked_add and Option documentation inform explicit overflow
and optional matching state (SRC-RUST-CORRELATED-COMMANDS). Session scope,
correlation and command policy are local choices. No upstream implementation
or dependency is reused. The [guide](../CORRELATED_COMMANDS.md) and
[focused review](../verification/CORRELATED_COMMANDS_REVIEW.md) record seven
focused/identical independent tests and the 236-test locked baseline passing.
Root and independent verification_support accept the complete diff with no
findings. Implementation 34514bd98bb671fadb2863f5d12492b54297e37c is published;
exact hosted run 41450862997 passes all 20 steps. Later record revisions receive
separate checks.

## Risks and revisit conditions

Any caller bypassing the issuer or resetting it can reuse an ID; equality does
not authenticate a sender or detect duplicate New commands. Copies accepted by
subscribers remain independent of cancellation and may later be cleared by
lifecycle operations. Commands still require ordinary admission and dispatch;
stop may discard them. No timing, progress, processing, durability, adoption,
stability or qualification claim follows. Revisit for multi-session persistence,
multiple issuers, concurrent outstanding operations or actual feedback needs.
Separate consequential authorization gates remain unchanged.
