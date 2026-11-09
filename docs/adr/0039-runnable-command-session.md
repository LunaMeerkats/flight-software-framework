# ADR-0039: Runnable bounded command session

- Status: Implemented; local/independent checks, complete Codex review and exact hosted pass
- Date: 2026-11-05
- Scope: Public-only executable consumer under RFF-LIB-REQ-014

## Context

ADR-0037/0038 demonstrate correlated commands and local results in integration
tests. Their fixture uses asserting setup/drain helpers and copied observations.
A developer still lacks an executable session connecting validated ingress,
result feedback and reply output without importing that fixture. Root Codex and
independent consumer_assessment select this specific integration gap.

## Decision

Independently author one standalone test-enabled Cargo example using public
framework APIs and std. One serial host owns the issuer and explicitly selects
every ingress, dispatch and drain. One producer retains a correlated reply with
two acceptance flags and emits one typed local result. Each of two subscribers
stages one host output. The host owns Cell slots borrowed by applications;
registering subscribers before the producer supplies destination IDs without
late-bound shared routing storage. Occupied host slots pause the corresponding
bounded turn before another dispatch. Retain full runtime errors and reject malformed
input explicitly. The fixed driver prints outside callbacks.

Keep the existing callback/correlated test sources unchanged. Verify the exact
new source as an independent path-dependent Cargo binary, including behavior,
execution, metadata and bytes/hash. One maintained package remains sufficient.

## Alternatives considered

- Extract the correlated fixture: defer; its test observations and asserting
  helpers are not operational policy, and extraction would churn frozen evidence.
- Import tests through path attributes: reject; developers should not depend on
  integration-test internals to run a public-API recipe.
- Independently author a standalone consumer: selected; it exposes setup/error
  handling and host decisions while preserving prior evidence.
- Add a generic runner, result API or bus acknowledgement service: defer; existing
  public composition has not demonstrated a missing library capability.
- Interactive/network command ingress: defer; external I/O framing, transport
  loss and retained-writer policy require their own concrete need and evidence.

## Evidence and consequences

The official Cargo targets reference informs example placement and test=true.
Official Rust Cell and checked_add documentation inform fixed copied slots and
explicit finite exhaustion (SRC-RUST-COMMAND-SESSION). Borrowed slots suffice
for this scoped session; Rc ownership, used by the earlier consumer, is not
needed here. Host turn order, payload grammar, capacities and scenario are local
choices. No upstream code is reused.
The [guide](../COMMAND_SESSION.md) and
[focused review](../verification/COMMAND_SESSION_REVIEW.md) record ten focused
and identical independent binary tests, matching source/output/metadata and the
253-test locked baseline. Root and independent consumer_assessment accept the
complete source without findings. Root and independent verification_support
also accept the complete document/diff evidence. Implementation
3d171dc22d9c2661baf4d077871ebc60cad83003 is published; exact hosted run
41855764215 passes all 20 steps with root/independent raw-log review. Later
record revisions receive separate checks.

## Risks and revisit conditions

The example duplicates mission policy intentionally rather than sharing test
internals; its own behavioral tests must protect the demonstrated composition.
Serial host gates depend on trusted access and explicit draining. Session IDs
supply correlation only; results are local snapshots of command handling and
inbox acceptance. Printing is fallible and does not establish durable or physical
delivery. Logical storage bounds do not establish process-memory or timing bounds.
No persistence, duplicate suppression, concurrency or public API freeze follows.
Revisit extraction for demonstrated shared production responsibility, or library
APIs when an independently authored consumer cannot compose the required use.
Separate release, platform, security and other consequential gates remain.

## Retained command result follow-up

Decision recorded on 2026-11-08.

The original driver took its result before writing it. A short write followed
by an error could leave an accepted prefix and no host result to resume. The
typed record matters to the host because rerunning a command may repeat its
effects. RFF-LIB-REQ-014 revision 2 adds a narrow output policy for command
results.

Use one scoped `ResultOutput` over the existing result slot and one writer.
Render the complete line once into a 128-byte buffer. Each explicit turn attempts
one write of its unwritten suffix. Keep the record and accepted-byte offset
across short writes and errors, and clear the slot only after the entire line
is accepted. The existing occupied-result gate keeps queued commands in place
while output is pending. The finite script lends the writer to this output,
advances after positive short writes and exits on error. A caller retaining the
output can choose a later retry on the same writer.

The original one-shot take and write is simpler for incidental diagnostics.
Keeping the result and retrying a whole `write_all` could duplicate the bytes
accepted before an error. Moving the result into a separate output slot would
permit another producer result to accumulate and change the current gate. A
shared sink service would add a library boundary for one host policy. The scoped
line applies the accepted-byte approach demonstrated by [ADR-0032](0032-bounded-host-event-presentation.md)
while leaving the typed result in its existing slot.

The standard writer contract is recorded in `SRC-RUST-EVENT-OUTPUT` in the
[source register](../research/SOURCES.md). Focused verification exercises short
progress, a returned writer error, zero progress, an invalid count, suffix
acceptance and queued producer dispatch. Writer acceptance, flush and physical
or durable delivery are separate effects. This revision originally retained
one-shot writes for subscriber output and other diagnostics.

## Retained subscriber output follow-up

Decision recorded on 2026-11-09 under RFF-LIB-REQ-014 revision 3. Taking a
subscriber output before writing can lose its typed observation on error.
Two separate byte cursors for one stream would require the host to arbitrate
between incomplete lines. A central pending queue would add storage and a
selection policy despite the existing independent slots.

Reuse the same private slot presenter for command results and replies. One
active line uses the existing 128-byte buffer and accepted-byte offset. The
presenter borrows the same writer mutably throughout the line, which prevents
other turns from writing through that object during its borrow. Peer dispatch
can still fill its own slot. The host completes the active line before lending
the writer to another presenter. Full acceptance clears only the selected
slot, so a queued selected delivery can then dispatch. The serial host keeps
one presenter for the active slot and does not drain it by another route.

The Rust writer and borrowing rules are recorded in `SRC-RUST-EVENT-OUTPUT`.
The fixed script exits on a writer error. A recovering caller can retain the
same presenter and writer and choose another turn. Writer acceptance remains
local progress. Other script diagnostics use one-shot writes.

## Retained ingress follow-up

Decision recorded on 2026-11-10 under RFF-LIB-REQ-014 revision 4. The
existing saturation test saves a rejected command in a local variable.
The executable's normal admission helper exits on rejection. Extend the
script to show how a host admits one saved command while draining existing
results and making space in the command inbox.

Keep a caller-owned `Option<Command>`. One explicit admission turn copies the
command into the existing validated enqueue path. Clear the slot on
`Delivered` before any fallible logging. A rejected turn keeps the original
value and ID. The caller controls intake, should keep later offers upstream
while this slot is occupied, and can choose to abandon the saved command.

Manual resubmission leaves the decision to clear the slot spread across
callers. An ingress queue or an automatic retry loop would add intake and
scheduling policy for this single saved command. The slot uses the existing
targeted publication and serial host turns. Admission, callback effects
(including `Busy`) and output remain separate. A later stop can clear an
accepted command. A repeated explicit turn has no effect once the slot is
empty. See `SRC-RUST-RETAINED-INGRESS` in the
[source register](../research/SOURCES.md).
