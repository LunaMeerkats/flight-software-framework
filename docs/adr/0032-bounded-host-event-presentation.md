# ADR-0032: Bounded host event presentation

- Status: Accepted and implemented; final local/independent verification passes
- Date: 2026-10-28
- Scope: Public-only example; library event APIs retain their contracts

## Context

Event filtering permits selective admission, but the public consumers only
dequeue records into test observations. The private combined sample prints a
fixed report. Neither gives a downstream developer a tested fallible byte-output
boundary. Retrying an entire formatted record after write_all fails may duplicate
an already accepted prefix. This source-based assessment is by root Codex and
independent consumer_assessment, not an adoption study. RFF-LIB-REQ-007 revision 1
is established before source changes.

## Decision

Add a test-enabled event_output example using public APIs and std only. A
mission-owned presenter owns its writer, one pending Event, a 96-byte line and
an offset. Each host turn removes at most one oldest event and invokes Write::write
at most once. Rendering completes into a separate fixed Cursor before output.
Rendering rejection retains the original event. A successful short write advances
only its accepted prefix. Zero progress becomes WriteZero; all write errors retain
the pending event and cursor and return their concrete io::Error. The host chooses
whether and when to retry. A completed line clears the pending slot but does not
dequeue another event in that turn.

The format uses mission-selected labels for one known application and Framework,
explicit severity and event names, and elapsed seconds with nine fractional
digits. Unknown sources are typed rendering errors. IDs remain caller-scoped;
equal-position foreign keys cannot be distinguished. The format is example
policy, not a protocol or stable framework serialization. Owning the writer
preserves stream continuity across ordinary retries.

## Alternatives considered

- Dequeue and print/write_all, retrying a whole record after failure: simpler,
  but loses progress after a successful prefix followed by an error.
- Add library peek/ack or an event-sink trait: could support record acceptance,
  but neither is needed to compose this concrete byte-output boundary.
- Own a fixed line and byte cursor in a public-only consumer: selected; exercises
  real standard writer behavior without extending the library API or queue.

## Evidence

Official Rust std 1.99.1 documentation for Write, ErrorKind and Cursor, accessed
2026-10-28, specifies short writes, no consumed bytes on Err, separate flush and
fixed array cursors. These are language/library contracts, not agency guidance.
See [the source register](../research/SOURCES.md). Acceptance requires exact
byte/error tests, a matching independent Cargo consumer, the full baseline,
source/diff review and [the focused record](../verification/EVENT_OUTPUT_REVIEW.md).

## Consequences, risks and revisit conditions

Queue capacity plus one pending event is the consumer's event storage bound.
Fixed rendering/cursor storage and turn count do not bound writer blocking,
allocation, callback time or process memory. Writer acceptance is not flush,
durability or physical delivery. Explicit flush after completion can fail after
records leave the presenter; automatic replay is unsafe. No retry limit,
reconnection, crash recovery or cross-stream resume is supplied. Revisit for a
demonstrated multi-source mapping, output protocol, persistence or reusable sink
need. APIs remain provisional and separate consequential gates remain unchanged.
