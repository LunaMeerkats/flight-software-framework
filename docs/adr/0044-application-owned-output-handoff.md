# ADR-0044: Application-owned output handoff

- Status: Accepted; verified and reviewed by Codex
- Date: 2026-11-23
- Scope: RFF-LIB-REQ-019 revision 1

## Context

ADR-0043 exposes a shared application reference through either runtime
owner. Existing ordinary-work output uses external borrowed Cell slots and
Copy records. A downstream application may instead own a fixed-size record
that the host must move out without cloning or exposing mutable callbacks.
The library supplies lookup; it need not dictate application output policy.

## Decision

Demonstrate one public-only runnable consumer with a private application
`Cell<Option<Output>>`. Output owns fixed bytes and implements neither Copy
nor Clone. Its shared `take_output` method uses Cell::take to leave None.
The work callback uses Cell::get_mut under its exclusive callback borrow
to reject a full slot before changing output. No output references escape.

The serial host owns at most one taken record. It gates production and
further taking while that slot is occupied. One separate sink turn moves
the record to a typed sink; definite rejection returns the whole record
for restoration to the host slot. Acceptance removes that local obligation.
Idle turns do not call the sink. Stop/restart and terminal failure retain
application-owned output according to this application's callbacks; shared
taking does not change framework lifecycle or inboxes.

## Alternatives considered

- Cell plus an application method: selected. Cell::take requires Default,
  supplied by Option even when Output is non-Copy and has no Default.
  The method states the narrow mission-specific mutation explicitly.
- RefCell: can support borrowed inspection, but dynamic borrow failure is
  unnecessary for this exclusive ownership move; no references are needed.
- External borrowed slot: remains valid and preserves host visibility, but
  imposes lifetime wiring for output the application can own itself.
- Mutable runtime accessor: exposes unrelated application mutation and
  callbacks outside the owner's lifecycle policy. Not justified here.
- Framework output trait/service: defer until multiple concrete consumers
  demonstrate a common policy that existing public composition cannot serve.

## Evidence and consequences

Official std Cell and Option documentation informs `SRC-RUST-OWNED-OUTPUT`
in the [source register](../research/SOURCES.md). No source code is reused.
The [focused review](../verification/OWNED_OUTPUT_REVIEW.md) records passing
304-test local and eight-test independent binary checks, exact source/output
identity and Codex source/document review. Implementation
`6fef2a7896aad016e585c6b08b4ce773d7e55343` reached origin/main and passes
exact hosted run 45134831252. The focused record preserves that evidence;
a later documentation head receives its own check.
Existing consumers, APIs, dependencies and runtime policies stay unchanged.

Cell provides single-threaded interior mutation; this is application policy,
not isolation, mutable owner access or a new framework output guarantee.
The finite slots bound record count, not execution time or arbitrary user
allocation. Typed sink rejection must mean no acceptance and return the
entire value. Local acceptance does not establish processing, durable or
physical delivery, recovery, concurrency, external adoption or API stability.

## Revisit conditions

Revisit for a concrete need for borrowed output views, multiple output kinds,
concurrent access, ambiguous transport outcomes or shared cross-application
handoff policy. Do not generalize this whole-record sink into byte-write
retry: existing event-output guidance owns partial-byte progress separately.
