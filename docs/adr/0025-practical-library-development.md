# ADR-0025: Practical library development beyond v0.1

- Status: Accepted; direct user direction
- Date: 2026-10-19
- Scope: Product objective, work selection and routine autonomous progression

## Context and authority

The user directed:

> This should become a practical library people can use.

and:

> Also don't limit yourself to v0.1 scope, I trust you will research and progress
> this correctly

The initial host milestone has completed documented autonomous review. Recent
work strengthens its finite-domain verification, but continued development must
also make the library useful to downstream developers. Describing present
experimental maturity must not reduce the product goal to an experiment or
indefinitely confine work to the original sample and requirements.

## Decision

Build a practical, reusable Rust library. Treat v0.1 as a verified regression
baseline and historical milestone, not a scope ceiling or permanent API shape.
Codex may research, plan, implement and review later routine library milestones
autonomously. Work does not require renewed user permission merely because it
adds a justified capability or changes an ordinary API, service or adapter beyond
v0.1. Extend ADR-0024's documented routine-review process to these milestones;
retain exact evidence, limitations and actual Codex reviewer provenance.

Choose bounded increments by the developer use they enable, the concrete defect
risk they reduce or the research uncertainty they resolve. Evaluate public API
composition, setup, error/resource contracts, documentation and realistic
examples using a representative downstream consumer. A useful consumer should
not need to copy private sample internals or modify the library to compose the
supported behavior. That is an outcome to demonstrate, not a claim that current
APIs already satisfy it.

Research primary sources when technical questions need evidence. Compare
consequential alternatives, add or version requirements before extending behavior,
record architecture decisions and keep traceability/current state coherent.
Balance hardening with capability and usability development rather than selecting
further model machinery solely because the first milestone is complete. Preserve
the previous tests and their actual domains as regression evidence.

Current experimental status, qualification limits and caller responsibilities
remain truthful until evidence supports a change. This direction does not
establish API stability, adoption, release readiness, operational suitability or
any flight/safety/compatibility claim. Tags, releases, crate publication and
deployment remain separately authorized. Existing gates also remain for identity
and licensing, compatibility claims, major API freeze, significant unsafe code,
major runtime/code-generation systems, large or security-sensitive dependencies,
hardware/RTOS/no_std commitments, security architecture and destructive migration.
Research and reversible experiments can inform those decisions without committing
to the separately gated action. Ordinary verified source pushes remain authorized.

## Alternatives considered

- Continue treating the host demonstration and v0.1 surface as the final scope:
  rejected by the user's explicit product direction.
- Pursue feature breadth or remove maturity/authorization limits immediately:
  rejected because product ambition does not demonstrate correctness or grant
  approval for the separately consequential actions.
- Advance a practical library through researched consumer-facing milestones and
  bounded verification: selected. It permits useful growth while retaining
  evidence and explicit decision ownership.

## Evidence and consequences

The direct user instruction is the authority; no external design or source code
is reused. [The charter](../CHARTER.md), [roadmap](../ROADMAP.md),
[requirements](../REQUIREMENTS.md), [project state](../PROJECT_STATE.md),
[architecture](../ARCHITECTURE.md) and [AGENTS.md](../../AGENTS.md) carry this
direction. The nightly automation prompt must carry the same instruction.
Prior review records remain historical evidence at their exact scope.

The next candidate is a downstream-consumer/API usability assessment followed
by one useful executable integration increment. No API or runtime behavior is
changed by this direction checkpoint. Revisit priorities as consumer evidence,
research or user direction changes; do not manufacture a release date, adoption
claim, target commitment or major API freeze from this instruction.
