# v0.1 requirements

Status: **Provisional; verified and reviewed within the recorded host boundaries**
Last reviewed: **2026-10-09**

These requirements define host-observable behavior. They do not specify cFS
compatibility, flight readiness, real-time performance, or certification.

This version records the reviewed v0.1 baseline, not the complete scope of the
practical library. [ADR-0025](adr/0025-practical-library-development.md) authorizes
later requirements and milestones. Define and trace new behavior before treating
it as implemented or verified; preserve the exact scope of existing evidence.

## Terms

- **Explicit error:** a typed or otherwise programmatically distinguishable
  result; expected failure does not require parsing text and does not panic.
- **Bounded:** a finite configured maximum enforced by behavior, not merely an
  initial allocation capacity.
- **Repeatable:** identical initial state, simulated time, and ordered inputs
  produce the same defined observable trace in a test. A trace includes ordered
  application work, lifecycle outcomes, and structured event timestamps, but
  excludes unspecified host diagnostics. This is not a real-time determinism
  claim.

## Behavioral requirements

### RFF-REQ-001 — Scope truth

Top-level, sample, and generated project documentation shall identify the
framework as experimental host software and shall not describe it as
flight-qualified, safety-certified, NASA-endorsed, operationally suitable,
TRL-proven, or automatically compatible with cFS, cFE, OSAL, PSP, CCSDS, or an
RTOS.

Acceptance: a documented entry-point review checks every user-facing entry point
and records the reviewed commit, actual reviewer, findings, and disposition.
[ADR-0024](adr/0024-autonomous-host-review-authority.md) permits autonomous
review; delegation alone does not complete it.

### RFF-REQ-002 — Explicit application lifecycle

A host scenario shall register at least two independently defined applications
and expose ordered registration, start, stop, and restart transitions. Expected
invalid transitions shall return explicit errors and leave the affected
application in a documented state.

Acceptance: state-machine tests cover every allowed and rejected transition;
an integration test observes both applications completing the scenario.

### RFF-REQ-003 — Bounded publish/subscribe

Publish/subscribe storage shall enforce configured finite capacity. Routing,
FIFO ordering, successful delivery, saturation, and known-but-unavailable
subscriber outcomes shall be explicit. The chosen backpressure or overflow
policy shall not silently depend on a library default. One caller-selected
dispatch shall present at most the oldest queued delivery to a running
application. The framework-supplied callback context shall expose message
publication only through the same bounded, lifecycle-aware mechanism.

Acceptance: tests fill each relevant queue to its exact limit and assert the
publisher-visible result and unaffected-subscriber behavior. Dispatch tests
assert oldest-only consumption, capacity-one self-publication, refreshed peer
availability, and explicit empty, lifecycle-rejected, and returned-error
outcomes.

### RFF-REQ-004 — Injected time and repeatable scheduling

Framework time shall be supplied through an injected clock. Under a simulated
clock, identical ordered inputs shall produce repeatable scheduled-work and
timestamp ordering without wall-clock sleeps.

Acceptance: tests advance time manually, cover equal-deadline ordering, and
replay the same scenario with an identical defined observable trace.

### RFF-REQ-005 — Structured events

The host scenario shall emit machine-inspectable events containing source,
severity, event identifier, and framework timestamp. Tests shall not need to
parse free-form log text to assert those fields.

Acceptance: tests assert structured values and the finite storage or delivery
policy for events.

### RFF-REQ-006 — Validated configuration lifecycle

At least one versioned runtime configuration shall support validation and
activation. Rejecting an invalid replacement shall leave the active value
unchanged. Rollback shall restore the previous accepted value.

Acceptance: state-machine and integration tests cover accepted activation,
rejection without mutation, version behavior, and rollback.

### RFF-REQ-007 — Command and telemetry boundary

The sample mission shall accept at least one validated host command and emit
observable telemetry through explicit boundary adapters. Malformed input shall
be rejected explicitly before reaching application business logic.

Acceptance: integration tests cover one valid command/telemetry path and
representative malformed inputs defined by the chosen adapter, including
length, identifier, or payload constraints when that boundary exposes them.

### RFF-REQ-008 — Defined application-error containment

When one application returns a defined runtime error, the runtime shall produce
a documented lifecycle outcome while another application remains operable.
This requirement does not claim containment of arbitrary panics, hangs, process
termination, memory exhaustion, or hardware faults.

Acceptance: a fault-injection integration test observes the failed
application's state/event and successful subsequent work by the other
application.

## Approved parameters and remaining refinement

These provisional requirements define behavior without freezing unnecessary
public APIs. Accepted project decisions now settle:

- RFF-REQ-002 lifecycle and identity in
  [ADR-0003](adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md);
- RFF-REQ-003 capacity, FIFO, saturation, fan-out, unavailable endpoints, and
  caller-selected one-message dispatch in
  [ADR-0004](adr/0004-bounded-application-inboxes.md) and
  [ADR-0012](adr/0012-application-message-dispatch.md);
- the RFF-REQ-004 elapsed-instant, injected-read, and manual-advance boundary in
  [ADR-0014](adr/0014-injected-manual-framework-clock.md), plus the finite
  one-shot scheduled-work, equal-time ordering, and replay boundary in
  [ADR-0015](adr/0015-caller-driven-scheduled-work.md), extended through the
  messaging owner with exact inbox cleanup in
  [ADR-0021](adr/0021-messaging-owned-scheduled-work.md);
- the RFF-REQ-005 and RFF-REQ-008 direct cooperative returned-work failure-event
  boundary in [ADR-0016](adr/0016-returned-work-failure-events.md), extended to
  messaging-owned ordinary work with exact inbox cleanup in
  [ADR-0020](adr/0020-messaging-work-failure-events.md); and
- RFF-REQ-006 revision and rollback behavior in
  [ADR-0005](adr/0005-configuration-revisions-and-rollback.md), with its
  standalone bounded snapshot core in
  [ADR-0017](adr/0017-bounded-configuration-snapshots.md) and runtime ownership,
  safe-point application visibility, restart retention, and no automatic error
  rollback implemented by
  [ADR-0018](adr/0018-configuration-aware-work-context.md).

[ADR-0019](adr/0019-host-command-telemetry-boundary.md) now selects the
RFF-REQ-007 demonstration grammar: exactly two command bytes, identifier `0x01`
and percentage `0..=100`, checked in length/identifier/value order before
publication or business logic. Matching telemetry uses identifier `0x81` and
the same percentage. A capacity-one host mailbox separates application
dispatch from host drain/encoding. Its returned full error uses existing
terminal failure semantics. The private `host-echo` example now implements
these adapters with shared-source integration evidence for RFF-REQ-007 at the
selected local slice/returned-array boundary. The full v0.1 sample gate remains
unchanged. [ADR-0022](adr/0022-combined-host-sample.md) now demonstrates the
required services in one executable with a shared, tested scenario driver.
The [recorded hosted CI run](verification/CI_BASELINE.md) now passes. The
documented [autonomous host review](verification/HOST_V0_1_REVIEW.md) now
accepts the current entry points and architecture under
[ADR-0024](adr/0024-autonomous-host-review-authority.md). This closes the host
milestone gates with actual Codex provenance; requirements and APIs remain
provisional. Routine human acceptance is not required for continued host work.
The original ownership probe alone does not establish adapter correctness.

Do not use an implementation's accidental behavior to settle these parameters
after the fact.

## v0.1 release gates

These gates support the requirements but are not additional system behaviors:

- a documented executable host sample demonstrates RFF-REQ-002 through
  RFF-REQ-008 and carries the RFF-REQ-001 safety notice;
- RFF-REQ-001 is reviewed separately across all user-facing entry points;
- formatting, selected source-quality lints, warning-free checks, unit and
  integration tests, and documentation checks pass in CI;
- important requirements map to exact evidence in the traceability register;
- dependency licences and enabled features are reviewed;
- unsupported claims and known limitations receive a recorded architecture
  review, performed autonomously under ADR-0024 with actual reviewer provenance.

These are host milestone evidence gates, not authorization to tag, release,
publish a crate, or deploy. The separate consequential gates remain unchanged.

Changes to requirement meaning or scope must update this file, the roadmap,
traceability, and project state in the same coherent increment.

## Stage 5 integration requirements — revision 1 (2026-10-20)

These requirements extend the practical-library programme without changing the
v0.1 contracts above.

### RFF-LIB-REQ-001 — Public-API getting started

A documented host consumer shall build and run as an independent Cargo package
using the unpublished library as a path dependency. Its source shall use only
public library APIs and the standard library, without loading private sample
modules or modifying framework source. The recipe shall explain setup, explicit
lifecycle and dispatch, typed failure handling, and configured resource bounds.

Acceptance: the exact runnable quickstart source shall also compile and run in
an independent consumer package. Tests shall observe successful command handling,
reject-newest ingress saturation retaining the older command, and explicit
stop/restart. The workspace baseline shall execute its regression tests. Review
shall distinguish this controlled composition from external adoption, API
stability, flight suitability, and release or publication readiness.

Borrowed setup acceptance supplement (2026-11-07). The quickstart shall document
and test how a caller recovers a still-registered runtime from a rejected inbox
attachment. The test shall check the typed rejection for the original ID, then
attach a corrected inbox and observe a dispatched command in the same borrowed
host slot using that ID. Run the same test in the independent package and retain
the original runnable scenario and transcript.

### RFF-LIB-REQ-002 — Independently implemented application composition

Revision 1 (2026-10-21). A documented public-API consumer shall compose two
distinct application types that each independently implement `Application` and
`MessagingApplication`. A mission-owned enum shall delegate lifecycle, ordinary
work and message callbacks, preserving each application's concrete message
failure through a typed mission error and its `Error::source` chain.

Acceptance: a bounded command-to-peer-output scenario shall run both as a tested
workspace example and as the identical source in an independent path-dependent
Cargo package. Tests shall observe delegated lifecycle/work behavior, distinct
application error attribution, selected failure/queued-inbox cleanup and retained
healthy peer progress. Guidance shall explain error unification, ownership,
resource limits and report handling. This verifies this composition only; it
does not establish external adoption, dynamic registration, API stability or
general application interoperability.

### RFF-LIB-REQ-003 — Ordinary-work output with explicit host transfer

Revision 1 (2026-10-22). A public-only consumer shall produce an output value
from caller-selected and finite scheduled ordinary work using immutable validated
configuration. It shall stage at most one pending value without silently
overwriting it. Ordinary work shall not publish; the host shall explicitly
transfer the value to exactly one bounded subscriber and clear the pending slot
only after verifying that destination accepted it.

Acceptance: the identical source shall pass as a tested workspace example and
an independent path-dependent Cargo package. Tests shall observe staging before
publication, full/unavailable destination retention and explicit retry without
failing the producer, lifecycle-rejected work without callback effects,
stop/restart retention, and typed full-slot work failure preserving the older
value and healthy peer progress. Scheduled attempts shall retain the existing
one-shot consumption contract. Guidance shall distinguish this frozen
single-subscriber retry policy from general fan-out, transactional delivery,
automatic recovery, external adoption and physical I/O.

### RFF-LIB-REQ-004 — Runtime-issued ordinary-work identity

Revision 1 (2026-10-23). Every invoked ordinary-work callback shall receive the
selected runtime-local `ApplicationId` through `ApplicationWorkContext`, alongside
the existing optional immutable configuration view. Applications shall not need
post-registration identity injection to attribute their own structured events.
Direct work, messaging-owned work, their failure-event wrappers and both finite
scheduled-work paths shall expose the same selected identity. Rejected work shall
invoke no callback. In-place stop/restart shall retain that identity.

Acceptance: public-API tests shall use the context identity as an event source,
distinguish two applications, retain configuration visibility, and cover each work
entry path. A producer shall observe caller-owned event-queue saturation inside
its callback before committing its demonstrated effect; typed failure shall
retain older events and allow peer progress. Verify the identical consumer tests
in an independent path-dependent Cargo package. Identity remains runtime-local,
not authenticated or globally unique; event ownership, clock origin and saturation
policy remain explicit mission responsibilities.

### RFF-LIB-REQ-005 — Bounded recurring work releases

Revision 1 (2026-10-25). A host-owned periodic release cursor shall retain one
application, a positive interval and at most one next release without agenda
allocation or growth. Each caller poll shall observe the injected clock once
when active, return Waiting without mutation before the release, or consume one
due release and re-arm at observed time plus interval. Delayed polls shall
coalesce missed intervals rather than accumulate a catch-up backlog. Overflow
shall still release the valid due item and explicitly exhaust the cursor.
Exhausted polls shall not read the clock.

Acceptance: public-API tests shall cover zero-interval rejection, exact and
overdue release, subsecond arithmetic, repeated same-time polls, final release
at arithmetic exhaustion and repeatable manual-time replay. A public-only
consumer shall explicitly delegate released work through the existing direct
and messaging owners, retaining identity/configuration, lifecycle rejection,
typed work failure, exact selected-inbox clearing and healthy peer progress.
The identical tests shall pass in an independent path-dependent Cargo package.
Guidance shall distinguish release from callback execution, host-selected error
and timer-disposal policy, observation-based delay from phase/completion timing,
and logical storage from process-memory or real-time guarantees. Existing
one-shot scheduling requirements and APIs retain their meaning.

### RFF-LIB-REQ-006 — Explicit event severity filtering

Revision 1 (2026-10-27). A caller-selected immutable EventFilter shall allow
events at or above its minimum in the local order Informational < Warning <
Error, without adding a general ordering contract to EventSeverity. The filter
shall retain one copied severity with no allocation, history or growth. It shall
expose its minimum and allow callers to inspect the severity decision directly.

EventQueue::emit_filtered shall apply the supplied filter before capacity
checking. An excluded event shall return FilteredOut without queue mutation,
including when the queue is full. An allowed event shall delegate to existing
unfiltered emission and return Emitted with its exact Recorded or QueueFull
outcome. The caller shall retain the borrowed event after every outcome. Existing
unfiltered emission, FIFO retention and runtime failure-event behavior shall
retain their meaning; the queue shall not retain a filter between attempts.

Acceptance: public-API tests shall cover all nine minimum/severity combinations,
suppression in empty, partially occupied and full queues, allowed saturation,
unchanged complete records/FIFO and explicit retry after dequeue. A public-only
ordinary-work consumer shall distinguish optional diagnostic suppression from
actual recording and capacity rejection, preserve selected runtime identity and
show that suppression leaves capacity for a later allowed error. It shall also
show that a queue full of allowed records can still reject an error. The identical
tests shall pass in an independent path-dependent Cargo package. Guidance shall
explain per-attempt policy, host drain and the difference between suppression,
acceptance and physical delivery. This adds no reserved capacity, priority,
automatic retry, source authentication, external adoption or stability claim.

### RFF-LIB-REQ-007 — Bounded host event presentation

Revision 1 (2026-10-28). A public-only consumer shall transfer structured events
from EventQueue to an owned standard Write sink through explicit host turns.
It shall retain at most one dequeued event, one fixed-capacity rendered line and
one byte cursor. Each turn shall attempt at most one sink write; positive short
writes shall advance only the accepted prefix. Write errors and zero progress
shall preserve the event and unwritten suffix. The next queued event shall not
be removed until the pending line is completely accepted by that writer.

Acceptance: the identical source shall pass as a tested workspace example and
an independent path-dependent Cargo package. Tests shall observe exact FIFO bytes
across partial writes, Interrupted/WouldBlock and other errors; zero progress;
idle turns without sink calls; retained events on rendering rejection; maximum
timestamp precision and fixed storage; and explicit filtering before presentation.
Guidance shall explain queue-plus-pending storage, caller-scoped source naming,
writer continuity, explicit flush/failure policy and the absence of physical,
durable or record-atomic delivery guarantees. This is mission composition, not
a new library sink service, queue policy or wire-format commitment.

### RFF-LIB-REQ-008 — Backpressure-aware recurring host coordination

Revision 1 (2026-10-29). The public-only work-output consumer shall compose
recurring work, sole-subscriber transfer and deferred host observation through
bounded host turns. A turn shall dispatch at most one sink message only when
its output slot is empty and the sink is Running, attempt at most one pending
transfer, then poll at most one periodic release and invoke at most one work
callback only if the pending slot is empty. Newly produced output shall remain
pending until a later turn. Host observation/draining shall remain separate.

Acceptance: tests shall fill output, inbox and pending storage without failing
either application; blocked turns shall retain FIFO values, pending revision,
next release and zero clock reads. Draining shall permit bounded progress;
resumed overdue work shall coalesce and re-arm from observed time. Configuration
replacement during blockage shall affect resumed work without rewriting older
output. Unavailable sinks shall preserve pending output; work errors shall retain
their concrete source and disable the selected timer. Verify the identical
source in an independent Cargo package. Existing direct-work/full-output failure
contracts remain unchanged. This mission policy adds no framework runner,
automatic drain/retry, fairness, timing or physical-delivery guarantee.

### RFF-LIB-REQ-009 — Subscription-respecting selective publication

Revision 1 (2026-10-31). MessageBus and the host-facing MessagingRuntime shall
offer publish_to for one caller-selected destination and a borrowed message.
Unknown destinations and known destinations not subscribed to the message topic
shall return distinct typed errors before availability or capacity is inspected,
without changing any inbox. A subscribed destination shall reuse existing
Delivered/InboxFull/Unavailable behavior and touch no peer inbox. The standalone
bus treats configured endpoints as available; the runtime shall use current
lifecycle state. The operation shall allocate no publication report or retry
history. Ordinary publication and callback contexts shall retain their contracts.

Acceptance: public-API tests shall cover selected delivery, peer exclusion,
unknown and unsubscribed rejection, full-inbox retention/FIFO, current lifecycle
availability, selected stop clearing and restart. A public-only two-subscriber
consumer shall retain one message and a fixed acceptance ledger, retry only
rejected destinations after partial fan-out and preserve older FIFO without
duplicating an accepted copy. Repeated rejection shall retain pending ownership;
new records shall wait for completion. Verify identical tests in an independent
Cargo package. Guidance shall distinguish inbox acceptance from processing,
later lifecycle clearing, physical delivery, authentication and exactly-once
guarantees. Retry order, storage limits and termination remain caller policy.

### RFF-LIB-REQ-010 — Callback-owned selective reply retry

Revision 1 (2026-11-01). ApplicationMessageContext shall offer publish_to for
one selected destination and borrowed message, returning the existing
DeliveryStatus or PublishToError. It shall reuse membership/subscription checks
and bounded enqueue, with the same pre-callback lifecycle snapshot as publish.
It shall allocate no report or retry history and expose no lifecycle mutation,
dequeue or nested dispatch. Existing host publication and failure cleanup retain
their contracts; accepted peer copies are not rolled back on callback failure.

Acceptance: a public-only message-driven consumer shall retain one reply and
two acceptance flags inside its application. Explicit later commands shall retry
only rejected subscribers, retain pending ownership on repeated full/unavailable
outcomes and gate new replies until completion. Tests shall observe exact FIFO,
peer exclusion, refreshed lifecycle availability between callbacks, unknown and
unsubscribed errors, capacity-one self-publication and selected clearing after
a returned callback error while accepted peer copies survive. Verify identical
tests in an independent Cargo package. Guidance shall distinguish acceptance
from processing, host-triggered retry from automatic retry, and committed effects
from transactions. Existing origin, resource and qualification limits remain.

### RFF-LIB-REQ-011 — Explicit callback reply cancellation

Revision 1 (2026-11-02). The public-only callback consumer shall accept an
explicit Cancel command through its ordinary bounded inbox. When dispatched,
it shall clear the current application-owned pending reply and acceptance flags,
if any, without publishing or changing destination queues. Repeated or idle
cancellation shall succeed. A later retry with no pending reply shall do nothing;
a new reply may then be admitted. Accepted copies remain accepted and cannot be
retracted by cancellation. Existing selective publication and lifecycle contracts
retain their meanings; stop/restart alone shall retain this consumer's reply.

Acceptance: public-only tests shall verify cancellation after partial acceptance,
unchanged accepted copies and rejected destination FIFO, retry after cancellation,
new-reply progress, repeated/idle cancellation, a permanently Failed subscriber,
ordinary command inbox saturation/FIFO and producer stop/restart retention.
Verify identical tests in an independent path-dependent Cargo package. Guidance
shall explain dispatch-time targeting, absence of operation IDs, host command
ordering, command rejection/clearing and the distinction between cancelling
future attempts and retracting accepted effects. No automatic cancellation,
priority command lane, acknowledgement, authentication or library retry service
is introduced. RFF-LIB-REQ-010 revision 1 evidence retains its original domain.

### RFF-LIB-REQ-012 — Correlated cancellation of one pending operation

Revision 1 (2026-11-03). A separate public-only consumer shall retain one reply
with a mission-owned operation ID and two destination acceptance flags. Cancel
and Retry commands shall carry that ID and affect only a matching pending
operation. Stale or idle controls, including repeated cancellation after clearing,
shall leave pending state and destination queues unchanged. New while busy shall be
consumed without replacing the retained operation and require host resubmission.
The host shall issue nonreused IDs within one session using a finite checked
allocator that explicitly rejects exhaustion without wrapping. Retransmissions
of the same operation shall retain its ID; creating another issuer while old
commands or replies survive is outside this consumer's session contract.

Acceptance: public-only tests shall observe delayed cancellation queued after a
newer operation, matching/idle/repeated cancellation, unchanged accepted copies
and older rejected FIFO, correlated rejected-only retry, busy-command retention,
ordinary command saturation and producer stop/restart retaining pending state
while clearing queued commands. Exercise the entire finite ID domain and repeated
exhaustion. Verify identical source in an independent path-dependent Cargo
package. Guidance shall distinguish correlation from acknowledgement,
authentication, duplicate suppression and durable identity across sessions.
Existing RFF-LIB-REQ-010/011 consumers and requirement meanings remain unchanged.

### RFF-LIB-REQ-013 — Bounded host command-result feedback

Revision 1 (2026-11-04). The correlated-command consumer shall expose one
capacity-one typed result slot shared by the serial host and producer. Each
successfully handled command shall record the exact command and distinguish
reply acceptance flags, busy rejection with the retained operation ID, matching
cancellation and no matching pending operation. Reply flags shall describe inbox
acceptance after this command, including completion; they shall not acknowledge
destination processing. The host shall explicitly take each result before
dispatching another producer command. An occupied slot shall cause a host turn to
skip dispatch without changing the slot, producer state or queued commands.

Acceptance: preserve the seven RFF-LIB-REQ-012 test histories. Public-only tests
shall observe busy feedback followed by same-ID resubmission, matching/stale/idle
controls, partial and completed reply flags, blocked host turns with exact FIFO
and inbox saturation, and explicit result draining. A direct dispatch bypass
shall return a typed occupied-slot callback error before command or publication
effects, preserving the old result and ordinary selected-inbox failure cleanup.
Malformed or routing failures shall return the existing runtime error without a
new result; previously accepted peer effects shall survive. Stop/restart shall
retain a recorded result and pending operation while queued undispatched commands
are cleared without results. Verify identical source in an independent
path-dependent Cargo package. Guidance shall state that results are local
callback observations, remain snapshots after later lifecycle clearing, and
provide neither reliable transport nor duplicate-command suppression. This adds
no library result service, concurrency, persistence or wire-format promise.

### RFF-LIB-REQ-014 — Runnable bounded command session

Revision 1 (2026-11-05). A standalone public-only executable consumer shall join
validated command ingress, a finite nonwrapping session issuer, one producer's
correlated pending reply/two acceptance flags, typed local command feedback and
two subscriber output slots. One explicit producer turn shall dispatch at most
one command and pause while its capacity-one result is occupied. One selected
subscriber turn shall dispatch at most one reply and pause while its output is
occupied. The serial host shall drain each slot explicitly and retain rejected
ingress for its own resubmission policy. Printing shall occur outside callbacks.

Acceptance: the identical source shall run and pass behavioral tests as a Cargo
example and independent path-dependent binary. Observe a finite session with
partial acceptance, busy feedback, gated results, rejected-only retry and same-ID
resubmission. Verify ingress and output saturation/FIFO, malformed input without
false success results, matching/stale controls, stop/restart retention and checked
issuer exhaustion. Preserve existing callback/correlated consumers unchanged.
Document setup, errors, finite storage, local-result versus subscriber-processing
boundaries, explicit host policy and independent verification. This demonstrates
one controlled composition, not a new library service, protocol, acknowledgement
transport, persistence or delivery guarantee.

Revision 2 supplement (2026-11-08). The host shall retain the typed command
result in its existing slot until a single writer accepts the complete bounded
diagnostic line. One result write turn shall attempt at most one write. Short
writes shall advance only the accepted prefix. An I/O error or invalid progress
shall preserve the record and its retry position. The producer gate shall stay
occupied until full acceptance, then permit the next queued command to dispatch.
The host shall resume with the same presenter and writer without resubmitting
the command. Subscriber output and other script diagnostics retain their
one-shot policy.

Acceptance: inject a short write, a writer error, zero progress and an
overreported byte count before a successful suffix write. Verify exact
accepted bytes, the retained result and queued command, and dispatch after
acceptance. Preserve the existing successful transcript and verify the
identical independent binary. Writer acceptance is a local observation.

Revision 3 supplement (2026-11-09). Apply the revision 2 write-turn and retry
policy to each subscriber's diagnostic line. Keep the selected typed reply in
its original slot until full acceptance, and release only its dispatch gate
on completion. Use one active bounded line and the same exclusively borrowed
writer across turns. The peer may dispatch into its own slot while the first
line is pending. Its line may start after the active line completes. Other
script diagnostics retain their one-shot policy.

Acceptance: inject short progress and a returned writer error, observe peer
dispatch and a queued selected reply during recovery, then resume the exact
suffix. Verify that only the selected gate opens, the peer retains its record,
and both lines appear in order on the same writer without interleaving.
Preserve the result retry regression and successful independent transcript.

Revision 4 supplement (2026-11-10). A serial host may retain one validated
command outside the inbox for explicit admission. Each turn shall attempt
admission at most once and clear the host slot only after `Delivered`.
`InboxFull`, `Unavailable` and typed errors shall retain the same command
and ID. An empty turn shall leave the inbox alone. Admission shall neither
dispatch nor issue an ID. The host chooses when to attempt another turn.

Acceptance: extend the executable with a full command inbox while a result
gates dispatch. After one older command dispatches, admit the retained command
behind the remaining older entry. Observe both FIFO order and the exact
retained command's separate callback result. Test retention when stopped and
an empty turn after admission. Preserve the previous transcript as an exact
prefix and verify the extended transcript in the identical independent binary.

Revision 5 supplement (2026-11-11). The example shall stage fixed two-byte
New requests from one caller-owned reader. A read turn shall call
that reader at most once, request only the unfilled suffix and retain a
partial prefix across returned I/O errors. A valid record shall use the
session issuer to fill the existing command slot or report exhaustion.
A malformed record shall consume no ID. The occupied slot shall suppress
all reads until admission clears it or the host abandons it. Admission
and dispatch stay separate.
Zero bytes shall end this finite input, reporting a truncation error if
a prefix was pending. An overreported count shall end input with an error.

Acceptance: use the reader in the full-inbox recovery scenario and keep its
exact transcript. Test a short read and returned error, rejected framing
and issuer exhaustion, clean and truncated end of input, invalid progress,
and a later record that stays unread through inbox rejection. Verify the
identical independent binary.

Revision 6 supplement (2026-11-12). The serial example shall distinguish a
command with a recorded callback result from known admissions cleared by a
successful producer stop. Start from an empty command inbox and result slot.
After one successful dispatch, keep its result staged while the host admits
exactly two commands and confirms that the producer turn is gated. Identify
those commands as cleared only after the stop count matches both admissions.
Restart the producer, observe the earlier result and confirm that dispatch
reports an empty inbox.

Acceptance: extend the exact transcript, preserve the previous output as a
byte-identical prefix and verify the identical independent binary. Reject
unexpected starting backlog before attributing any commands. Attribution
depends on the host knowing every admission to the selected inbox.

### RFF-LIB-REQ-015 — Read-only subscription inspection

Revision 1 (2026-11-06). A host shall be able to query whether a configured
inbox subscribes to a borrowed topic through either messaging owner. A known
endpoint shall return its configured membership independently of lifecycle
state, queued deliveries and available capacity. An unknown endpoint shall
return the existing typed inbox-access error. The owner shall query the copied
topology without allocating storage, enqueueing or invoking application callbacks.

Acceptance: verify membership, absence and unknown identities through public
APIs, including an occupied inbox and lifecycle changes. The command-session
consumer shall check all required routes before starting any application. A
structurally valid swap of producer and subscriber inboxes shall report the
missing required subscription while all applications remain registered. The
identical independent binary shall preserve the existing successful transcript
and behavioral cases. Application IDs remain scoped to their issuing runtime.

### RFF-LIB-REQ-016 — Observe application progress on host turns

Revision 1 (2026-11-13). A serial public-API consumer shall check for a fresh
application sample after each successful selected work callback. The host
shall start one elapsed-time window after successful startup and reset it
when it receives a fresh sample. The host shall attempt one warning on the
first completed callback without a sample observed at or after the threshold.
A fresh sample at that instant shall reset the window. Each window permits
one warning attempt, including a queue rejection. The host shall retain the
attempted record and queue outcome in its turn result. The application shall
remain Running after a warning.

A returned callback error shall preserve the runtime's typed error, Failed
state and failure-event attempt. That turn shall skip the progress check,
including when the threshold is due. A lifecycle rejection shall produce
neither diagnostic. The host shall use one injected clock and a finite
event queue. This observes completed serial turns.

Acceptance: verify the exact threshold, warning suppression and rearming,
fresh progress at the threshold, queue saturation, typed callback failure
at the threshold, and a later lifecycle rejection. Run the identical
public-API test source in an independent package.

Revision 2 (2026-11-14) retains revision 1 for directly selected work and adds
a periodic consumer with one downstream sample slot. A turn with that slot
occupied shall report deferral before polling or invoking work. It shall
retain the staged sample and the next release. The progress window shall
exclude the interval from the first turn that observes the occupied slot to
the first later turn that observes space. Pre-deferral elapsed time and the
warning-attempt state shall be preserved.

On resumption the host shall poll once using the same clock and invoke only
a released item. A waiting or exhausted cursor shall produce no progress
observation. The first successful empty work turn at the adjusted threshold
shall follow the revision 1 warning rule. A returned work error shall retain
the typed failure path and disable this host's cursor.

Acceptance: verify retained output, release and input across repeated deferred
turns, one late release with observation-based re-arming, no warning on an
early or waiting turn, a warning at the adjusted threshold, and returned
failure at that threshold with no further release. Re-run revision 1 and
the periodic cases in the same independent public-API package.
