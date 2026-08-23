# ADR-0010: Inline bounded messages and available-endpoint routing core

- Status: Accepted and implemented as a partial ADR-0004 slice
- Date: 2026-08-24
- Scope: In-process message representation and serial fan-out core

## Context

ADR-0004 defines one bounded FIFO inbox per application, immutable mission
subscriptions, stable registration-order fan-out, reject-newest saturation, and
publisher-visible partial delivery. It also requires a statically bounded
message or an enforced maximum payload before making a bounded-memory claim.
The repository had not selected that representation or demonstrated any
message delivery.

Adding the first routing evidence directly to `Application::work` would also
settle service-context borrowing and runtime lifecycle integration. Neither
choice is needed to prove the queue and fan-out policy, and both deserve
evidence from a working core first.

## Decision

The first message and routing slice uses these rules:

- `Message<Topic, MAX_PAYLOAD_BYTES>` stores its byte payload inline in a
  const-generic array plus a private logical length. Construction accepts a
  payload exactly at the limit and returns a typed error above it. A zero-byte
  limit remains valid for empty messages.
- The mission selects a `Copy + Eq` topic type. The framework does not assign a
  numeric topic width or imply that the topic or message is a wire format,
  cFS, CCSDS, or other protocol identifier.
- `MessageBus` copies the caller-supplied configured inbox topology at
  construction. Each configured application has a positive slot capacity and
  one copied set of unique topics. Applications appear in identity registration
  order, which also fixes route and report ordering. The detached core cannot
  prove that the issuer has no additional application records.
- Queue and copied-topic storage are reserved during construction. Logical
  inbox capacity is checked directly rather than inferred from allocator
  capacity.
- Publication is serial and does not wait for inbox capacity or subscriber
  progress. Each matching available endpoint is processed even when another is
  full. A full inbox keeps older entries and rejects the newest delivery. This
  does not claim that arbitrary mission topic equality or host allocation can
  never block.
- One ordered destination-outcome vector is the source of truth. Its derived
  classification is `NoSubscribers`, `Complete`, `Partial`, or
  `WhollyUndelivered`, so category counts cannot disagree with destination
  details.
- Storage for every matching destination outcome is reserved before delivery.
  If that reservation fails, publication returns a typed error and modifies no
  inbox.
- A provisional `dequeue` operation exposes FIFO evidence. It is not an
  application dispatch API and does not establish ADR-0004's future one
  in-flight delivery rule.

This core models every configured endpoint as available. It is deliberately
not owned by `Runtime` yet and does not consult lifecycle state. Stopped/failed
unavailability, discarded-count clearing, restart reconnection, true
self-publication from application work, and service-context borrowing remain
required integration work. RFF-REQ-003 therefore remains partially implemented
and not verified in full.

## Alternatives considered

- A generic caller-defined message value was rejected for this slice because
  the framework could not enforce its payload or allocation bound.
- A runtime-length-checked `Vec<u8>` payload was deferred because fan-out would
  allocate while cloning each delivery unless ownership and allocation failure
  received a larger policy.
- A fixed-width integer topic identifier was deferred because no protocol or
  mission evidence selects its width. A mission enum is a smaller Rust-native
  commitment.
- Mutable runtime subscription APIs were rejected because ADR-0004 fixes
  mission topology and dynamic mutation would need synchronization, capacity,
  and observability rules.
- Immediate integration into `Runtime<A>` was deferred because the present
  context-free callback cannot borrow a message service, and manual availability
  flags could contradict lifecycle state.
- Separate delivered, full, and unavailable vectors were rejected because
  their ordering and summary could diverge. One ordered outcome sequence is
  sufficient.

## Evidence

Public-API tests demonstrate:

- exact payload-limit acceptance and one-byte-over-limit rejection;
- atomic rejection of empty, zero-capacity, out-of-order, and duplicate-topic
  topologies;
- FIFO delivery across topics in one inbox;
- exact reject-newest saturation and continued healthy-subscriber delivery;
- stable registration-order outcomes for partial fan-out;
- distinct wholly-undelivered and no-subscriber classifications; and
- typed rejection of an identity outside the configured topology.

The slice adds no dependency, thread, executor, wait for inbox capacity or
subscriber progress, retry, spill path, hidden work, external protocol, or
runtime lifecycle mutation.

## Consequences and risks

- The bus bounds its queued message count and payload storage per slot, but
  short messages still occupy the full configured inline payload array.
- Topic values are fixed-size copied values, but a topic may still contain a
  reference to caller-owned static data. This decision does not claim a
  whole-process byte bound.
- A returned report and dequeued message become caller-owned; retaining them is
  outside the bus's storage bound.
- Topology construction remains separate from runtime registration. Failure
  cannot produce a partial bus, but the caller must decline to run a mission if
  either setup step fails. The detached core cannot prove that every registered
  application has a configured inbox.
- The opaque application identity still cannot detect a same-position key from
  another issuer. Construction checks ordering, not origin.
- Publication performs an inline copy of the full message representation for
  each accepted destination. A later measured payload size may justify a
  different ownership strategy.
- Configured endpoints must not be represented as lifecycle-integrated until
  availability is derived from runtime state.

## Revisit conditions

Revisit this decision when lifecycle integration supplies unavailable and
clearing behavior, `Application::work` needs a messaging context, measured
payloads make inline copies unsuitable, a protocol boundary requires a
validated external identifier, independent traffic classes are required, or
concurrency changes serial ordering and allocation assumptions.
