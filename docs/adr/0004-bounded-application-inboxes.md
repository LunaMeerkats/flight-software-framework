# ADR-0004: Bounded application inboxes and partial fan-out

- Status: Accepted; not implemented
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
- Publication never blocks, waits, retries, spills, or creates hidden work.
- An available inbox with space accepts one delivery. A full inbox retains its
  older entries and rejects the incoming delivery. An unavailable endpoint does
  not enqueue. Every other destination is still processed.
- The publisher receives a structured report distinguishing complete, partial,
  wholly undelivered, and `NoSubscribers` outcomes, including full and
  unavailable destinations.
- Subscriptions remain mission topology while an application is stopped or
  failed. Its queued entries are cleared with an observable discarded count,
  publication reports it unavailable, and restart reconnects an empty inbox.
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

## Required verification and revisit conditions

Tests must cover the exact capacity boundary, cross-topic FIFO, self-publication,
partial fan-out, all-full delivery, no subscribers, unavailable endpoints,
clearing/reconnect, duplicate subscriptions, stable report ordering, and payload
limits.

Revisit when an application needs independent traffic classes, lossless or
priority delivery, runtime subscription mutation, concurrency, or a stricter
whole-bus byte bound.
