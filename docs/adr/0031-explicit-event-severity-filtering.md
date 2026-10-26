# ADR-0031: Explicit event severity filtering

- Status: Accepted and implemented; local verification passes
- Date: 2026-10-27
- Scope: Caller-selected filtering before standalone event enqueue

## Context

The public diagnostic consumer emits an informational event for each ordinary
work call. Its deliberate policy commits its demonstrated effect only after
actual queue acceptance, returning a terminal work error when the queue is full.
The periodic-release checkpoint enables recurring producers, but the standalone
event queue still accepts every attempted severity until its finite capacity is
full. A consumer that considers routine diagnostics optional must implement its
own selection before enqueue to avoid spending those slots on unwanted records.
ADR-0013 already records that reject-newest saturation can omit a later error.

Root Codex and independent consumer_assessment compare a narrow severity policy
with further dispatch/recovery guidance. Existing composition guidance and
dispatch tests already cover typed callback errors, selected-inbox cleanup,
retained peer progress and restart from Stopped rather than Failed. This
source-based assessment selects an executable filtering consumer; it is not an
external adoption study. It does not change the diagnostic identity consumer's
stronger acceptance-before-effect contract.

RFF-LIB-REQ-006 revision 1 is established before implementation. Input is clean
codex/nightly at 1759cbe2139220126fe4030c9b1d2b39f7745883. The initial locked
local baseline passes all 186 workspace tests; local Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98 are unchanged.

## Decision

Add an immutable copied EventFilter with new(minimum_severity),
minimum_severity() and allows(severity). Its local selection order is
Informational < Warning < Error: an informational minimum allows all three,
a warning minimum allows Warning and Error, and an error minimum allows Error
only. Express this finite policy explicitly without adding Ord or PartialOrd to
EventSeverity. The order establishes this filter's selection, not event priority,
delivery order or compatibility with another event system.

Add EventQueue::emit_filtered(&event, filter), returning a separate must-use
FilteredEventOutcome. FilteredOut means the supplied severity was excluded and
the queue was not inspected for capacity or changed. Emitted(EventEmitOutcome)
means the allowed event was delegated to the existing emit operation, preserving
Recorded and exact QueueFull outcomes. Filtering therefore precedes capacity
checking, including when the queue is already full. The event remains borrowed,
so the caller retains its complete record after every result.

The filter contains one severity and allocates nothing. Each attempt supplies
its filter explicitly; the queue retains no policy, counters, history or new
storage. The existing emit operation, EventEmitOutcome and all runtime failure
reporting retain their contracts. Unfiltered callers can still enqueue any
severity. A new public-only consumer treats suppression of its optional routine
diagnostic as success, while distinguishing acceptance from saturation for an
allowed record. The host continues to own lifecycle, clocks and event drain.

## Alternatives considered

- Leave selection entirely in mission branches: already possible with the
  public severity accessor. Select one small reusable policy and explicit
  filtered outcome so consumers can share this defined order without duplicating
  the selection/queue-result distinction. Other mission policies remain possible.
- Add only allows without queue integration: useful for host policy, but every
  consumer would still have to repeat the suppression versus enqueue result
  composition. Keep allows available and provide the narrow combined operation.
- Add FilteredOut to EventEmitOutcome: reject because that changes the outcome
  vocabulary of existing unfiltered emission and runtime failure attempts. A
  separate wrapper preserves those contracts and exhaustive consumer matches.
- Store mutable policy in each queue: defer. That would affect unrelated
  producers and require activation, queued-record and runtime-failure semantics.
  Explicit per-attempt policy has no retained configuration transition.
- Filter after dequeue or remove previously queued low-severity records: reject
  for this need. Such records have already consumed capacity, and retroactive
  removal would change the existing FIFO/retention contract.
- Reserve error slots, evict records by priority, add an all-off state, or match
  source/identifier rules: defer until a concrete consumer establishes the
  additional resource and policy need. This increment does not guarantee that
  an allowed error can enter a queue already full of allowed records.

## Evidence and consequences

The official Rust Reference match-expression documentation informs explicit
variant branching. Official VecDeque documentation, identifying std 1.99.1 when
consulted on 2026-10-27, describes the existing back-enqueue/front-dequeue
primitive. Both are language/library evidence, not authority for the locally
chosen severity order or queue policy. Sources and adoption decisions are
recorded in SRC-RUST-EVENT-FILTERING; no upstream implementation is copied and
no dependency is added. Local execution remains on Rust 1.98.0.

The [consumer guide](../EVENT_FILTERING.md) and
[focused review](../verification/EVENT_FILTERING_REVIEW.md) record seven passing
truth-table, queue-preservation and consumer tests, seven identical independent
Cargo tests, metadata/source identity and the complete locked 193-test baseline.
The guide snippet passes one standalone doctest. Complete root and independent
Codex source/diff review passes. Implementation db994e776c68fb5432e646afb2ae5344cb2444e5
passes exact hosted run 40329281385, all 20 steps and 193 workspace tests. The
focused record separates these executed results from the design decision.

## Risks and revisit conditions

FilteredOut is deliberate omission, not queue acceptance, physical delivery or
an audit record. The caller must decide whether an omitted diagnostic permits
its effect. Allowed records can still saturate the queue and omit a later error;
there is no reservation, eviction, retry, reliability or priority guarantee.
Changing a later attempt's filter does not revisit records already queued.
Unfiltered emission and runtime failure-event reporting bypass this policy.

Application source identity and clock origins retain their existing caller
responsibilities. Logical record bounds do not bound process memory, callback
allocation or time. No service context, concurrency support, source authentication,
API stability, qualification or operational suitability is established. Revisit
for demonstrated per-source rules, rate suppression, reserved severity capacity,
runtime-integrated filtering or persistent policy needs. Separate consequential
authorization gates remain unchanged.
