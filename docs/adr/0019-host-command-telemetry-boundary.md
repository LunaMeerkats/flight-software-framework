# ADR-0019: Mission-local host command and telemetry boundary

- Status: Accepted; private host adapter pair implemented 2026-09-09
- Date: 2026-09-08
- Scope: RFF-REQ-007 grammar, validation, composition, and output ownership

## Context

At decision time, Stage 3 had a verified configuration boundary but no command
adapter, telemetry adapter, or sample mission. RFF-REQ-007 requires a selected grammar
before implementation. The first pair should exercise existing messaging
without adding a protocol framework.

ADR-0001 keeps blocking I/O outside conforming callbacks. ADR-0011/0012 expose
neither host dequeue nor mutable application access through `MessagingRuntime`.
A telemetry subscriber needs a narrow host-owned output destination. Returning
a message error already means terminal `Failed` and selected-inbox clearing;
an adapter must not reinterpret this as recoverable backpressure.

## Decision

### One local demonstration grammar

Select a pure `EchoPercent` command. Its business operation returns the same
validated integer percentage; it changes no configuration, setpoint, actuator,
or lifecycle state. The semantic range supplies a meaningful malformed-payload
case without inventing a hardware responsibility.

| Boundary | Exact record | Meaning |
| --- | --- | --- |
| Host command | Two bytes: `0x01`, then one `u8` in `0..=100` | Echo the validated percentage |
| Host telemetry | Two bytes: `0x81`, then the same percentage | Observation produced after application dispatch |

The identifiers are local constants with no cFS, CCSDS, or other protocol
meaning. There is no version byte, checksum, sequence counter, timestamp,
padding, newline, or optional field. All other command identifiers and values
are rejected. A format extension needs a new decision; this is not a published
or frozen format.

Ingress borrows one complete caller-framed slice. Check length equals two
before inspecting fields, then command identifier, then percentage range.
Return distinct errors carrying the actual length, identifier, or value. Extra
bytes are rejected, not ignored or split into another command. The adapter
never accumulates input or scans for delimiters. Oversized caller buffers are
rejected by length without copying or traversing their content. Stream framing,
partial reads, files, terminals, and sockets remain outside this slice boundary.

### Validation before business logic

The mission codec constructs a private validated percentage type implementing
`Copy` only after all checks pass. Only that type reaches the echo business
function. A valid
command maps to a private mission `EchoCommand` topic with a one-byte payload;
the response uses `EchoTelemetry` with a one-byte payload. Internal topics are
Rust values, not external application IDs.

The command application's message boundary checks expected topic, exact
one-byte payload length, and range again before calling business logic. The
telemetry subscriber similarly validates before depositing a value. The bus
enforces maximum payload length, not mission semantics, so internal publication
must not bypass validation accidentally. Malformed host records cause no
publication, application invocation, lifecycle mutation, or mailbox change.

### Existing runtime composition

Use two independently defined mission applications, an echo processor and a
telemetry subscriber, composed through the existing static application enum.
Select two runtime records, one capacity-one inbox per application, one
subscription per inbox, and `MAX_PAYLOAD_BYTES = 1`. The host controls order:

1. Validate one host command and attempt one publication.
2. Inspect its complete report before selecting command dispatch.
3. Dispatch at most one echo message; its callback attempts one telemetry
   publication through the existing publish-only context.
4. Dispatch at most one telemetry message into the host mailbox.
5. Drain and encode at most one mailbox record outside both callbacks.

Ingress acceptance means queued delivery, not execution or telemetry success.
Return the original ordered `PublishReport` to the host; preserve full,
unavailable, and absent-subscriber distinctions. Report-allocation failure is
a distinct error before inbox mutation. The echo callback requires complete
delivery to its configured telemetry destination. Allocation failure or a
non-complete report becomes a concrete mission error retaining the original
error or report. Existing terminal message-failure semantics apply. No
automatic retry or re-publication occurs.

### Capacity-one host output

The host owns a private mailbox containing `Cell<Option<ValidatedPercent>>`.
The telemetry application borrows it for the runtime's lifetime. A narrow
deposit checks occupancy before setting the value; full output returns an
explicit error and preserves the older record. Host drain takes the optional
value and returns `None` or the exact two-byte encoded array. There is no
formatting buffer, mutable application escape, global state, reference-counted
owner, or generic sink trait.

Deposit and drain are serial operations with no user callback between the
occupancy check and store. This is not a concurrent mailbox or an atomic
check-and-set guarantee. Official `Cell` documentation describes copied reads,
interior mutation, and taking a value while replacing it with its default;
capacity and rejection remain local code obligations. (`SRC-RUST-CELL`)

Directly dispatching telemetry while the mailbox is full returns a concrete
full error: the telemetry application becomes terminal `Failed`, the attempted
message is consumed, and its remaining inbox is cleared. The old mailbox record
remains host-owned and drainable. Normally the host drains between telemetry
dispatches. There is no implicit recovery, mailbox clearing on stop/failure,
or guaranteed delivery. Restart from `Stopped` also retains the host mailbox.

Framework retention is two one-message inboxes plus at most one in-flight
message during a callback. The mailbox retains one validated percentage. Drain
returns one two-byte array whose later retention belongs to the caller.
Existing topology and ordered report allocations keep their documented bounds.
These are record bounds, not measured stack usage or allocation-free
publication. The mailbox itself needs no heap allocation. Caller archives and
future I/O buffering require separate budgets.

The first observable output boundary is the returned array, asserted in tests
and later displayed by the host sample. Physical writes, partial-write failure,
persistence, acknowledgements, retries, and flush policy need separate work.
An I/O error after drain cannot be called a rollback of execution or delivery.

### Source placement

Keep the codec, applications, and composition private to one Cargo example.
The implementation uses `examples/host-echo/main.rs` and `mission.rs`, with
`mission/codec.rs` and `mission/applications.rs` separating validation from
application/output behavior. `tests/host_adapters.rs` loads the same mission
through one relative `#[path]` attribute. Explicit child paths in `mission.rs`
keep ordinary example and attributed test loading on the same files; the first
test compilation identified this lookup difference. This follows Cargo's
multi-file target layout and Rust's module path rule. (`SRC-RUST-CARGO-LAYOUT`,
`SRC-RUST-MODULE-PATH`) A copied test adapter could drift; moving mission code
into the library would add an unsupported API commitment. Neither is needed.
Add no library exports or new crate solely for this example.

## Alternatives considered

- Strict ASCII `ECHO 042\n` and `PERCENT 042\n` improves manual input but adds
  decimal parsing, exact widths, and newline/whitespace policy. Two-byte records
  expose the required validation questions with less parsing.
- A version byte or generic envelope creates unused format commitments now.
  Revisit when another supported format or persistent/external consumer exists.
- Writing stdout or invoking an arbitrary `Write` sink inside a callback mixes
  potentially blocking I/O with dispatch, contrary to ADR-0001.
- Host dequeue, mutable application access, or a new output service/context
  widens framework ownership APIs for a single mission need.
- `Rc<Cell<Option<T>>>` shares one allocation across owners, but borrowing
  suffices while the host outlives its runtime. Reject that extra owner and
  allocation for this slice. (`SRC-RUST-RC`)
- Reporting full output as callback success with a second loss-status channel
  adds another observation policy. Preserve the explicit returned failure and
  its terminal consequence for this first experiment.

## Evidence and implementation acceptance

ADR-0001 defines callback I/O limits; ADR-0011/0012 and current publication and
dispatch tests define lifecycle, queue, error, and ownership behavior. No new
upstream cFS behavior is asserted. The
[mailbox experiment](../verification/HOST_MAILBOX_EXPERIMENT.md) tests borrowed
host output against the existing library. It does not implement the command
codec, two-application path, or sample mission.

The implementation acceptance criteria are:

- exact output for percentages 0, a middle value, and 100; one business
  invocation and telemetry observation per selected command;
- empty, truncated, oversized, and concatenated records; every unknown
  identifier and every value 101 through 255; precedence for overlapping faults;
- malformed host input leaves callbacks, inboxes, lifecycle, and output
  unchanged; internal invalid topic/length/value never reaches business logic;
- capacity-one ingress saturation preserves the older command and reports
  full; unavailable and no-subscriber reports remain distinguishable;
- acceptance is observable before execution; the caller selects each dispatch
  separately, with no hidden drain or retry;
- empty drain, exact bytes, consume-once behavior, slot reuse, and full-output
  preservation with terminal selected-app failure and exact inbox clearing;
  host mailbox retention across stop/restart/failure;
- telemetry publication failure retains its complete diagnostic and existing
  non-transactional semantics; no claimed undo of accepted peer publications;
- identical ordered input/dispatch/drain produces identical bytes and outcomes
  without wall-clock sleeps; and
- source-quality and Cargo checks, an executable example, and truthful
  requirement traceability.

Allocation-error preservation must be reviewed against the existing typed
publication path. No current allocator-injection seam can reliably force that
error in the fixed two-record mission; do not claim an injected allocation
failure test without a separate reproducible mechanism. Full and unavailable
destinations provide controlled publication-failure scenarios for this slice.

The real adapter evidence is recorded in the
[traceability register](../verification/TRACEABILITY.md). The pure business
function has one call site after internal validation; source review and exact
one-message/output tests establish its cardinality without an instrumented
business-call counter. The fixed mission's wrong-topic and no-subscriber tests
use explicitly different topology. Existing framework fan-out tests retain
the evidence for non-transactional partial publication; the fixed mission has
one telemetry destination and cannot produce partial delivery itself.

RFF-REQ-007 is covered at the selected caller-framed slice and returned-array
boundary. Adapter evidence does not prove the integrated v0.1 service sample,
CI, physical output delivery, or architecture review.

## Consequences, risks, and revisit conditions

This unblocks one host adapter pair while preserving framework APIs and serial
execution. It offers no authentication, corruption or duplicate detection, or
delivery guarantee. These bytes are for a controlled local demonstration.

Revisit for streaming or physical I/O, a stable-format consumer, continued
telemetry-app progress after output saturation, or ownership that requires the
runtime to outlive the mailbox. Revisit framework integration when multiple
concrete missions show a coherent shared responsibility. None of these needs
authorizes concurrency, hardware, compatibility, or publication by implication.
