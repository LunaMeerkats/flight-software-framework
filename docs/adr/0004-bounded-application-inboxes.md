# ADR-0004: Bounded application inboxes and partial fan-out

- Status: Accepted; routing and lifecycle-availability slices partially implemented
- Date: 2026-08-05
- Scope: In-process v0.1 publish/subscribe delivery

## Context

The serial caller-driven runtime cannot make progress if publication blocks
waiting for a subscriber to consume. Each application also needs an independent,
finite and observable delivery bound. NASA cFE bounded FIFO pipes provide useful
problem context, but the rules below are an independent local design and make no
compatibility claim. (`SRC-NASA-CFE`)

## Decision

- Each registered application has one runtime-owned inbox shared by all topics
  to which it subscribes.
- Mission composition supplies a positive capacity measured in queued delivery
  slots. The one item currently being dispatched is outside the inbox bound, so
  a serial runtime holds at most the sum of inbox capacities plus one in-flight
  delivery record.
- Each `(application, topic)` subscription is unique and route iteration uses
  stable application-registration order.
- Successfully accepted deliveries are FIFO within one inbox across all topics,
  ordered by serial publish-call order.
- Publication never waits for inbox capacity or subscriber progress, retries,
  spills, or creates hidden work. This queue policy is not a claim that
  arbitrary mission topic comparisons or host allocation cannot block.
- An available inbox with space accepts one delivery. A full inbox retains its
  older entries and rejects the incoming delivery. An unavailable endpoint does
  not enqueue. Every other destination is still processed.
- The publisher receives a structured report distinguishing complete, partial,
  wholly undelivered, and `NoSubscribers` outcomes, including full and
  unavailable destinations.
- Only a `Running` application is available. Subscriptions remain mission
  topology while an application is `Registered`, stopped, or failed, and
  publication reports those matching endpoints unavailable. Stop or returned
  callback error clears queued entries with an observable discarded count.
  Successful restart from `Stopped` reconnects an empty inbox; terminal
  `Failed` does not restart under LC1.
- Overflow diagnostics are returned directly and are not recursively published
  onto the same potentially saturated bus.
- Slot capacity alone is not a byte bound. Messages must use statically bounded
  content or an enforced maximum payload length before bounded-memory claims are
  made.

## Alternatives considered

- Blocking backpressure: rejected because the serial runtime could deadlock.
- Atomic all-or-nothing fan-out: rejected because one slow subscriber would deny
  healthy subscribers.
- Drop-oldest or coalescing: deferred as possible topic-specific policies rather
  than a generic default.
- Per-subscription queues: rejected initially because they require merge and
  fairness rules and weaken the per-application total bound.
- A global pool, reliable delivery, priorities, and dynamic subscriptions are
  deferred until requirements justify them.

## Current implementation boundary

[ADR-0010](0010-bounded-message-routing-core.md) implements the first
available-endpoint routing core with inline bounded payloads, immutable topic
sets, pre-reserved positive-capacity inboxes, FIFO dequeue, reject-newest
fan-out, stable per-destination outcomes, and publisher-visible classification.

[ADR-0011](0011-runtime-owned-message-availability.md) adds an owning
integration that constructs one fresh inbox per still-registered runtime
record, derives availability from lifecycle state, clears stopped and failed
endpoints with exact discarded-delivery counts, and reconnects an empty inbox
only after successful restart from `Stopped`.

Applications still cannot consume or publish messages through their work
callback, and no one-in-flight dispatch rule exists. The standalone
`MessageBus` deliberately continues to model direct construction as
available-endpoint routing. This ADR and RFF-REQ-003 therefore remain only
partially implemented.

## Required verification and revisit conditions

Tests must preserve the exact capacity boundary, cross-topic FIFO, partial
fan-out, all-full delivery, no subscribers, unavailable endpoints,
clearing/reconnect, duplicate subscriptions, stable report ordering, and
payload limits. Application self-publication and one-in-flight dispatch remain
required evidence.

Revisit when an application needs independent traffic classes, lossless or
priority delivery, runtime subscription mutation, concurrency, or a stricter
whole-bus byte bound.
