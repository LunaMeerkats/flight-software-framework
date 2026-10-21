# ADR-0028: Host-mediated ordinary-work output

- Status: Accepted for this bounded consumer checkpoint
- Date: 2026-10-22
- Scope: Configuration-driven ordinary work and one-subscriber host transfer

## Context

ADR-0025 prioritizes useful public-API integration. The first two consumers
demonstrate message-driven output; neither demonstrates ordinary work producing
an output. At input 0848e32821046b91048ad607a7aba267a8850fb4, ordinary work receives
configuration only, while message callbacks receive publication access. Root
Codex and independent consumer_need identify this gap by source assessment,
not a user study. RFF-LIB-REQ-003 is established before implementation.

## Decision

Demonstrate one bounded producer through existing public APIs. It borrows a
caller-owned capacity-one pending slot and reads validated runtime configuration
during ordinary work. The host publishes a copy after the callback, inspects
the sole destination result, and clears the slot only after acceptance. Full or
unavailable destinations retain the pending value for explicit retry. A full
pending slot is a typed work error under this consumer's strict policy; existing
terminal failure and selected-inbox clearing semantics apply.

Freeze one subscriber in mission construction. Retrying a rejected publication
is safe here because that subscriber accepted nothing. A general partial
fan-out retry could duplicate earlier deliveries and is outside this recipe.
An unexpected destination report is a separate host topology error detected
after publication; it may already contain delivery and must not be retried
blindly. The example stops for diagnosis on that error.
Host retry may transfer an already staged value after its producer stops or
fails; producer lifecycle controls callback invocation, not caller-owned output.
Document this explicit policy and verify the identical source in an independent
Cargo package. Add no new service owner, runtime callback or dependency.

## Alternatives

- A narrow ordinary-work publication context could expose immediate reports.
  The current Application trait has no topic/payload parameters; changing it
  or adding a second work trait must also reconcile direct work, scheduling,
  configuration and failure-event paths. Defer that change until a consumer
  needs in-callback acceptance decisions or ordering that this bridge cannot
  express coherently.
- Borrowing an event emitter would serve a diagnostic producer, but also needs
  explicit source identity, timestamp and saturation ownership. It does not
  answer this message-output transfer question. Assess it separately.
- Publishing from the message callback after a synthetic trigger adds an inbox
  delivery to represent ordinary work and obscures the work/schedule contract.
  Retain the existing distinction instead.

## Evidence and consequences

Official Rust Cell documentation supports copying an interior-mutable value
through a shared reference; it is not a synchronized concurrent queue. Official
Cargo documentation supports test-enabled examples. The source register records
their local adoption. Repository work, scheduling and routing contracts govern
the mission policy; no upstream implementation is copied.

The [guide](../ORDINARY_WORK_OUTPUT.md) explains ownership, transfer and errors;
the [focused record](../verification/WORK_OUTPUT_REVIEW.md) records actual
commands, source identity, independent review and limitations. Configuration is
observed when work runs; later configuration changes do not rewrite a pending
value. Delivery is delayed until the host explicitly publishes. Additional
callback/report allocations and duration remain outside logical slot bounds.

## Risks and revisit conditions

The slot is caller-owned mission policy, not framework-managed output or a
general reliable-delivery API. Do not extend the retry recipe to multiple
subscribers without per-destination duplicate handling. A work error remains
terminal; panics, hangs, process failure and allocator exhaustion are uncontained.
The example demonstrates controlled host composition, not external adoption,
API stability, physical delivery, real-time behavior or operational suitability.
Revisit a narrow service capability when immediate publication feedback or a
second concrete consumer makes this host policy inadequate. Existing separate
consequential gates remain unchanged.
