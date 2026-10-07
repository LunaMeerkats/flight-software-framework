# ADR-0024: Autonomous host milestone review authority

- Status: Accepted; routine review authority delegated by the user
- Date: 2026-10-08
- Scope: Host v0.1 entry-point and architecture acceptance

## Context and authority

The user directed on 2026-10-08:

> I like this, feel free to not ask for Human acceptance and just keep moving
> this project forward

The host framework already has executable behavior, local verification, and
exact-revision hosted CI. Routine human entry-point and architecture acceptance
had remained outstanding in Stage 4. The new direction delegates that routine
acceptance work to autonomous review; it is not evidence that a human reviewed
the source or completed the review.

## Decision

Codex may perform and record the host v0.1 entry-point and architecture reviews
without soliciting routine human acceptance. Continue one bounded, verified
increment at a time. A missing human review must not stop this engineering work
or become a repeated request in the nightly report.

Review the actual entry points, requirement evidence, resource/failure contracts,
API shape, provenance, dependency scope, and unsupported claims. Record the
exact reviewed revision, reviewer identity, evidence, findings, limitations, and
disposition. Autonomous review may satisfy these host milestone gates once it
actually completes; this authority checkpoint does not complete them by itself.
Keep source publication on the existing verified `codex/nightly` path.

The separate consequential authorization gates remain: project name/branding
and licensing, compatibility claims, major public API freeze, significant unsafe
code, major async/concurrency runtime, mandatory code generation, large or
security-sensitive dependencies, hardware/RTOS/no_std commitments, security
architecture, destructive migration, and publication outside authorized source
pushes, including tags, releases, crate publication, and deployment.

## Alternatives considered

- Retain routine human acceptance as a progress gate: rejected by the user's
  explicit delegation.
- Treat delegation as completed review or unrestricted authorization: rejected
  because permission is not verification or authority for separate actions.
- Document autonomous review and continue bounded work: selected; it removes
  the routine blocker while retaining evidence and scope controls.

## Evidence, consequences, and risks

The direct user instruction is the authority; no external source or reused
design is needed. [Requirements](../REQUIREMENTS.md),
[roadmap](../ROADMAP.md), [traceability](../verification/TRACEABILITY.md), and
[AGENTS.md](../../AGENTS.md) adopt this review policy. Dated records of earlier
pending human reviews remain historical evidence.

Stage 4 remains incomplete until its recorded autonomous reviews and other
applicable gates pass. Codex reviewers are not independent human reviewers.
The project remains experimental host software; review does not establish
flight qualification, operational suitability, compatibility, or API stability.
Revisit this delegation if the user changes it, a consequential decision is
needed, or review exposes a limitation that invalidates the host milestone.
