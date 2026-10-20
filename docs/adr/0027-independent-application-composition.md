# ADR-0027: Mission-owned composition of independent applications

- Status: Accepted for this bounded consumer checkpoint
- Date: 2026-10-21
- Scope: Two independently implemented host applications through public APIs

## Context

ADR-0025 prioritizes downstream usability. ADR-0026's quickstart verifies one
application type. Runtime documents static enum composition, but the private
combined sample implements framework traits on its mission wrapper rather than
on each contained application. It therefore does not demonstrate reuse of two
independent framework application implementations or their distinct error types.
Root Codex and independent consumer_assessment identify this concrete gap at
f539755e50b0f15aaa4361cba2e2eae4e7b3dca9; this is source assessment, not a user study.

## Decision

Establish RFF-LIB-REQ-002 before implementation. Add one public-only tested Cargo
example with independently implemented relay and output applications. A
mission-owned enum delegates every lifecycle/work/message callback. A typed
mission error distinguishes the two message error types and retains concrete
causes through `Error::source`. The callback publication report determines the
relay's explicit saturation/failure policy. The host controls all lifecycle,
dispatch and output draining.

Verify the identical source in an independent Cargo package as well as the
workspace, including successful delegation, distinct errors, selected cleanup
and healthy peer progress. Document the composition and finite resource policy.
Keep the small first quickstart intact. Add no public framework adapter yet.

## Alternatives

- A public generic sum adapter could centralize forwarding, but one concrete
  consumer has not yet established reusable error mapping or acceptable nesting.
  First expose and assess actual mission code; revisit with repeated consumer
  duplication or a demonstrated composition limit.
- Trait objects still require a shared associated-error vocabulary or an
  additional erasure layer. They add ownership/allocation choices without
  resolving this selected consumer's need. Defer dynamic application sets.
- Ordinary-work messaging/event access could enable periodic producers. It
  changes context, owner and schedule contracts and does not resolve typed
  heterogeneous composition. Assess that capability in a separate consumer.
- Publishing private sample modules would expose mission policy as library API.
  Reject that shortcut; use existing supported public entry points.

## Evidence and consequences

The Rust Reference defines enums as heterogeneous disjoint unions. Standard
library Error documentation describes retaining lower-level causes through
`source`. Official Cargo documentation supports test-enabled examples. Exact
sources and local adoption are in SRC-RUST-TYPED-COMPOSITION and the existing
SRC-CARGO-CONSUMER-RECIPE entry. No upstream implementation is copied.

The [composition guide](../COMPOSING_APPLICATIONS.md) explains the recipe;
the [focused record](../verification/COMPOSITION_REVIEW.md) records actual source,
commands, reviews and limits. Static enum variants occupy inline storage sized
for their largest variant plus representation overhead; this does not bound
allocations inside applications, errors or publication reports. Existing library
behavior, old requirements and tests are retained.

## Risks and revisit conditions

Applications share a mission-selected topic and payload bound. Forwarding and
error conversion remain mission code; the evidence covers this fixed pair only.
Infallible lifecycle methods do not demonstrate conversion of distinct lifecycle
errors. Callback failures are terminal; earlier publication/output effects are
not transactional. No dynamic registration, external adoption, stable API or
physical-output claim follows. Revisit adapters after repeated independently
implemented consumers expose common policy or unacceptable integration cost.
Release, platform, runtime and other consequential gates remain separate.
