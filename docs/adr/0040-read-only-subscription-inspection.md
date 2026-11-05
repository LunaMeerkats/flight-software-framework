# ADR-0040: Read-only subscription inspection

- Status: Selected for RFF-LIB-REQ-015
- Date: 2026-11-06

## Context

ADR-0011 attaches immutable inbox configurations in registration order. The
command session registers subscribers first to obtain their IDs, then builds a
producer with those destinations. Exchanging a reply inbox and the command
inbox is structurally valid and can defer discovery until the first routing
attempt, after startup callbacks have run.

## Decision

Expose `subscribes_to(application_id, &topic)` on `MessageBus` and
`MessagingRuntime` beside the existing inbox queries. Reuse the same copied
topology and topic comparison as publication. Return `Ok(false)` for a known
endpoint missing the topic and the existing `InboxAccessError` for an unknown
endpoint. Inspect one endpoint and scan its finite topic list. Lifecycle,
queue contents and capacity have no bearing on membership.

The command session checks required command and reply subscriptions before
starting its applications. It owns the route expectations and reports the
first mismatch with the application ID and topic. Verify a swapped topology
and retain the existing successful transcript.

## Alternatives and consequences

- Probe `publish_to` while every application is registered. `Unavailable`
  implies a configured subscription in that state, but the probe requires a
  message and becomes a possible enqueue once the selected application runs.
- Add a paired registration and inbox builder. It could avoid separate
  positional lists, but would need to return IDs during assembly and define
  storage and ownership recovery for partially staged setup. The demonstrated
  need is checking required routes before startup.
- Return a complete topic slice. That exposes ordering and storage shape a
  membership check does not require.

Rust's standard `Result` and slice membership support the existing typed
lookup and comparison (SRC-RUST-SUBSCRIPTION-INSPECTION). Existing construction,
publication and dispatch semantics remain. The query can also be used while
running without queue effects. The host still supplies the correct issuing
IDs and expectations. A valid membership result cannot establish callback
behavior, available capacity or acceptance of a later publication. Revisit a
paired builder when checking expectations leaves demonstrated assembly errors.
