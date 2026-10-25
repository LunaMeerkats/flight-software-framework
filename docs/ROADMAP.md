# Roadmap

The roadmap is evidence-driven. A later stage may be reordered when repository
evidence changes, but scope changes must be recorded rather than implied.

## Stage 5 — Practical library use and continued development

Status: **Active; host-owned periodic release checkpoint verified**

[ADR-0025](adr/0025-practical-library-development.md) records the user's direction
to build a practical library and progress beyond v0.1 autonomously. Stages 0–4
below retain their historical evidence; they do not cap the product programme.

First assess a representative downstream consumer against the public API,
getting-started documentation and current sample. Identify the most consequential
integration friction or missing capability, then select one executable vertical
slice. Establish traceable requirements before extending behavior. Use real
consumer evidence to balance capability development, API simplification,
documentation, fault testing and portability research.

Entry evidence is the reviewed host baseline. Subsequent checkpoints must show
the selected consumer use working through supported public APIs, with explicit
errors/bounds and runnable guidance, or resolve a concrete research uncertainty
blocking that use. Do not report adoption or practical readiness before those
observations exist. Release, publication and consequential decisions keep their
separate gates; routine work beyond v0.1 requires no additional permission.

The first selected increment is the [public-API quickstart](GETTING_STARTED.md):
one bounded command handler, explicit lifecycle and dispatch, ingress saturation
and host observation. [ADR-0026](adr/0026-public-api-quickstart.md) selects a
standard tested example plus actual independent Cargo path-dependency verification,
avoiding a second maintained crate or unsupported service-context expansion.
RFF-LIB-REQ-001 defines the new integration evidence without changing v0.1 meanings.
The [verification record](verification/QUICKSTART_REVIEW.md) carries actual results
and limits. Checkpoint c772dd960fe45b8cbc440dd1101587e101d1244d passes exact
hosted run 39258022861, all 20 steps and 154 workspace tests. Later candidates
should follow concrete consumer friction observed
here, including heterogeneous composition and ordinary-work service access.

The second selected checkpoint is
[independent application composition](COMPOSING_APPLICATIONS.md), under
RFF-LIB-REQ-002 and [ADR-0027](adr/0027-independent-application-composition.md).
It tests two independently implemented applications, explicit mission enum
delegation, typed error-source retention and bounded peer progress after failure,
including actual independent Cargo package execution. The
[focused record](verification/COMPOSITION_REVIEW.md) carries results and limits.
Checkpoint d63d4f88d23a70cfd0fd4badf3efeba88ca3bb8d passes exact hosted
run 39429032917, all 20 steps and 158 workspace tests including its four new
regressions. Complete autonomous source/document/diff and consumer review passes.
Assess further adapter or service capability from concrete consumer friction;
do not infer a generic adapter need from this pair alone.

The third selected checkpoint is
[ordinary-work output](ORDINARY_WORK_OUTPUT.md), under RFF-LIB-REQ-003 and
[ADR-0028](adr/0028-host-mediated-work-output.md). A configured producer stages
one value during ordinary or scheduled work. The host explicitly transfers it
to one subscriber, retaining it on full/unavailable outcomes. This tests the
existing public API before adding a work publication context. The
[focused record](verification/WORK_OUTPUT_REVIEW.md) separates actual consumer,
review and hosted evidence from planned verification. Checkpoint
f354ba2ad0061ce7687855cb8f166f0c543edd08 passes exact hosted run 39605633954,
all 20 steps and 168 workspace tests, including all ten new consumer regressions.
Root and independent Codex source/document/diff and consumer review pass.
Revisit service access
when a consumer needs acceptance feedback inside its work callback or ordering
that explicit host mediation cannot express coherently.

The fourth selected increment supplies
[runtime-issued ordinary-work identity](APPLICATION_WORK_IDENTITY.md), under
RFF-LIB-REQ-004 and [ADR-0029](adr/0029-ordinary-work-identity.md). A public-only
diagnostic consumer uses checked mission-owned services and the selected ID to
attribute events before committing its effect. All work owners retain one
context-construction path. The [focused record](verification/WORK_IDENTITY_REVIEW.md)
separates local and independent-package evidence, autonomous review and hosted
results. Checkpoint b3ebba331e73d2d4b46189accfc75e9ef9a6281b passes exact hosted
run 39959273438, all 20 steps and 176 workspace tests including eight diagnostic
consumer regressions. Local identical-source consumer tests and complete
root/independent Codex review pass. No new framework service, queue owner or
maintained example is needed.

The fifth selected increment is [host-owned periodic releases](PERIODIC_WORK.md)
under RFF-LIB-REQ-005 and [ADR-0030](adr/0030-host-owned-periodic-releases.md).
One inline cursor supplies recurring work without rebuilding agendas. It
coalesces late polls, re-arms from observed time and exposes arithmetic
exhaustion after the final valid release. The host explicitly delegates chosen
items to existing work owners. Public-only consumer tests, independent Cargo
verification and actual autonomous/hosted results are recorded in
[the focused review](verification/PERIODIC_WORK_REVIEW.md). The full locked
baseline passes 186 tests and the identical independent consumer passes all ten
new regressions. Root and independent Codex complete source/document/diff review
accepts with no findings. Checkpoint 53ac9cb7a41af36a6f123be884399562db2ee245
passes exact hosted run 40097187490, all 20 steps and the configured host/sample
evidence. No one-shot contract, work owner or workflow changes. The documentation
evidence follow-up receives separate review and exact hosted inspection.

## Stage 0 — Foundation and responsibility checkpoint

Status: **Complete (2026-08-05)**

- Record charter, limitations, provenance, v0.1 behavior, and traceability.
- Decide the first execution model and its revisit conditions.
- Defer Cargo structure until the first vertical slice defines a real boundary.

Exit evidence: internally consistent documents, resolving relative links, clean
diff checks, and a local commit on `codex/nightly`.

## Stage 1 — One lifecycle vertical slice

Status: **Complete (2026-08-23)**

- Create the smallest stable-Rust crate structure needed for one runtime and
  independently defined test applications.
- Implement explicit identity and lifecycle states for registration and start.
- Extend through stop/restart only when transition semantics are tested.
- Use no external runtime or framework dependency unless a recorded need arises.

Completed slices: the single unpublished library provides a bounded logical
lifecycle registry with exhaustive LC1 transition evidence, plus a
finite-capacity generic runtime that owns statically composed applications. The
runtime executes start, caller-selected work, stop, and in-place restart
synchronously, preserves distinct concrete operation errors, commits terminal
`Failed` on any returned error, suppresses callbacks for rejected operations,
and leaves peer records unchanged. Tests observe retained application state,
successful subsequent peer work after a returned work error, and two
independently defined applications completing registration, start, work, stop,
restart, and work.

Exit evidence: applicable Cargo checks and complete transition tests for the
implemented slice plus the complete two-application host scenario. RFF-REQ-002
is verified by the exhaustive logical transition matrix and public integration
evidence. No dependency, thread, executor, service context, or automatic
dispatch was added.

### Source-quality checkpoint before Stage 2

Status: **Complete (2026-08-23)**

The first whole-tree source-form audit found no handwritten Rust line over 100
columns, eight comment-only lines over the 80-column review default, and five
cohesive chronological integration tests over the proposed 60-line Clippy
threshold. Tracked stable rustfmt and Clippy configuration now enforce the
100-column format and individual function-size lint. The comments were
reflowed, and the five initial test findings received narrow reasoned
expectations instead of count-driven fragmentation. A follow-up source-order
cleanup split two independently meaningful behaviors and simplified one
identity fixture, retiring two expectations; three cohesive chronological
scenarios remain explicitly justified.

The contributor policy now records progressive source ordering, module and
naming conventions, waiver retirement, manual review boundaries, exact tool
versions, and a warning-free baseline. Official Rust, JPL, NASA, ECSS, JAXA,
and Australian public-source decisions are recorded without claiming agency
applicability or compliance. Other lints, a strict physical-line checker,
complexity tooling, dependency policy, CI, and toolchain pinning remain
separate measured increments.

## Stage 2 — Bounded interaction under controlled time

Status: **Complete (2026-08-30)**

- Add publish/subscribe fan-out with an explicit finite capacity and overflow
  contract.
- Add structured events without creating an unbounded side channel.
- Add injected and simulated time, then caller-driven scheduled work.

Completed messaging slices: the standalone routing core uses inline const-bounded
payloads, immutable unique topic sets, pre-reserved positive-capacity inboxes,
cross-topic FIFO, reject-newest saturation, continued partial fan-out, and
ordered publisher-visible classifications. The owning integration constructs
one fresh inbox per still-registered runtime application, makes only `Running`
endpoints available, returns exact clearing counts after stop or returned
callback error, and reconnects an empty inbox after successful restart from
`Stopped`. Caller-selected dispatch now removes at most one oldest message for
a running application and supplies a publish-only context. Tests prove bounded
self-publication, refreshed lifecycle availability, explicit empty and rejected
dispatch, and non-transactional peer delivery when a callback later fails.
RFF-REQ-003 is verified by the combined evidence.

Completed event slices: machine-inspectable records enter a standalone positive-
capacity FIFO queue with direct reject-newest saturation reporting. An opt-in
direct runtime work operation now constructs one clock-captured application
error event after a cooperative returned work error, preserves the complete
original error and exact event attempt under saturation, and leaves a peer able
to complete later work. Ordinary work, lifecycle rejection, and other callback
paths emit no event.

Completed time and scheduling slices: a general `FrameworkInstant` and injected
`Clock` boundary have one manually advanced implementation. Reads never move
time; checked overflow is typed and non-mutating; and `EventTimestamp` can
capture the injected reading. A finite `WorkSchedule` now preserves configured
order for equal elapsed instants, and one caller request attempts at most one
due or overdue item through the running-only work boundary. Identical manual
scenarios reproduce scheduled outcomes, application work order, lifecycle
states, and clock-captured event timestamps. RFF-REQ-004 is verified. RFF-REQ-005
is verified at the direct cooperative returned-work failure boundary. The same
failed-state, event, original-error, saturation, and peer-progress scenario
verifies RFF-REQ-008 at that narrow boundary.

Exit evidence: exact-boundary, ordering, saturation, and unavailable/clearing
tests for RFF-REQ-003; structured-event and simulated-time integration tests
for RFF-REQ-004 and RFF-REQ-005; and direct cooperative fault-injection evidence
for RFF-REQ-008. No application-authored events, other callback event path,
panic or hang containment, guaranteed event delivery, thread, or executor was
added.

## Stage 3 — Configuration and mission boundaries

Status: **Complete (2026-09-12)**

- Add validated, versioned configuration activation and rollback.
- Add one command-ingest and telemetry-output host adapter pair.
- Integrate and re-demonstrate the already verified direct cooperative
  application-error behavior in the sample mission.

The configuration core validates const-bounded byte snapshots, preserves state
on rejection, assigns never-reused revisions, and restores one consume-once
rollback snapshot. ADR-0018 now integrates that table as optional
constructor-owned runtime state. One immutable ordinary-work context exposes
the active revision and bytes through direct, scheduled, failure-event, and
messaging-owned work. Tests cover construction ownership recovery, absence,
safe-point activation/rejection/rollback visibility, restart retention, no
automatic error rollback, peer progress, event behavior, and inbox cleanup.
Together with the standalone transition and exhaustion tests, this verifies
RFF-REQ-006. No schema, host loading adapter, or wire format has been selected.

ADR-0019 now implements the command-ingest and telemetry-output host adapter
pair in the private `host-echo` example. The codec and two applications share
source with integration tests covering malformed records, explicit dispatch,
publication diagnostics, bounded host output, terminal saturation failure,
lifecycle retention, and replay. The host-owned borrowed mailbox is drained
outside callbacks. RFF-REQ-007 has adapter-boundary evidence; the original
ownership probe remains separate. ADR-0022 now composes and documents the
complete service sample, including the narrow returned-work failure/event
scenario, using that same private mission source.

ADR-0020 adds the prerequisite opt-in failure-event operation through the
messaging owner. It completes ordinary-work lifecycle commitment and exact
selected-inbox clearing before reading the borrowed clock and attempting one
event, retaining the existing nested error types.

ADR-0021 adds messaging-owned one-shot scheduling through ordinary work. Both
owners share the private clock/consumption decision; messaging errors retain
the consumed item, observed instant, original error, and exact selected-inbox
discard count. Later due peers retain their queues and can progress. This
completes the recorded scheduling prerequisite without adding scheduled events.

ADR-0022 completes the combined host scenario: two-application lifecycle,
command/telemetry exchange, finite scheduling under manual time, fresh work
configuration observations through activation/rejection/rollback, and a
cooperative work failure with exact inbox cleanup, event, and peer progress.
Five shared-driver integration tests and the executed binary establish the
sample behavior. CI and recorded host milestone reviews remain in Stage 4.

Exit evidence: hostile-input and rollback tests for RFF-REQ-006,
command/telemetry adapter tests for RFF-REQ-007, and sample-mission
re-demonstration of the narrow RFF-REQ-008 behavior.

## Stage 4 — v0.1 integration and review

Status: **Complete for the experimental serial host milestone (2026-10-09)**

ADR-0023 configures one Windows GitHub Actions job for the existing baseline
and sample. Local command replay and workflow validation are a configuration
checkpoint; the first [hosted run](verification/CI_BASELINE.md) subsequently
passes the baseline and sample at `abb1293790136128d5f27d8c48c1e3d98a355540`.
The user's 2026-09-12 approval now authorizes verified source pushes without
per-push human review under AGENTS.md. Hosted results must be recorded
separately; source publication does not complete v0.1 release review.

- Run the complete sample mission and verification suite in CI.
- Audit all resource bounds, failure paths, dependency licences, public APIs,
  provenance, and unsupported claims.
- Record a v0.1 architecture review before widening scope.

The user's 2026-10-08 [delegation](adr/0024-autonomous-host-review-authority.md)
allows Codex to complete routine host entry-point and architecture acceptance
without asking for human acceptance. The
[completed review](verification/HOST_V0_1_REVIEW.md) now records its exact
revision, Codex reviewers, requirement/resource/failure/API evidence, findings,
and accepted limits. Delegation alone did not complete Stage 4; the actual
review and verification do. Consequential scope changes, tags, releases,
crate publication, and deployment retain their separate authorization gates.

The 2026-09-13 [dependency and scope checkpoint](verification/DEPENDENCY_SCOPE_REVIEW.md)
records an empty external Cargo dependency/feature graph, unchanged approved
licences, separate host/CI tool dependencies, and an autonomous entry-point
scope inventory. Crate notice omissions and stale sample CI wording are
corrected. This completes the bounded dependency/features review and prepares
scope evidence. The 2026-09-14
[messaging construction review](verification/MESSAGING_CONSTRUCTION_REVIEW.md)
adds exact later-inbox reservation-overflow and returned-runtime reuse evidence.
This closes one constructor boundary. The 2026-09-15
[returned-message failure review](verification/MESSAGE_FAILURE_REVIEW.md)
examines immediate peer publication, selected cleanup, complete retained FIFO,
and post-failure callback availability. The 2026-09-16
[schedule construction review](verification/SCHEDULE_CONSTRUCTION_REVIEW.md)
adds first-descent diagnostics and copied-agenda ownership observations.
The 2026-09-20 [event-queue review](verification/EVENT_QUEUE_REVIEW.md) adds
exact capacity-overflow rejection and repeated saturation/reuse observations
at logical capacities one and three without changing the queue policy.
The 2026-09-21
[lifecycle construction review](verification/LIFECYCLE_CONSTRUCTION_REVIEW.md)
adds exact reservation-overflow diagnostics through both the standalone
registry and unconfigured owned runtime constructors. Existing zero-capacity,
logical saturation, returned-application ownership, and configured-runtime
table-lineage evidence remains unchanged. The 2026-09-22
[application identity scope review](verification/APPLICATION_ID_SCOPE_REVIEW.md)
executes ADR-0003's caller-scoped key consequence through both owners:
equal-position foreign keys select local records because issuer provenance is
not encoded. The 2026-09-23
[detached-message identity review](verification/MESSAGE_IDENTITY_SCOPE_REVIEW.md)
executes the corresponding ADR-0010 service boundary: positional topology
validation accepts a same-position key from another issuer, and equal keys
address the same configured inbox. The 2026-09-24
[scheduled-work identity review](verification/SCHEDULE_IDENTITY_SCOPE_REVIEW.md)
executes the ADR-0015/0021 consequence through both scheduling owners: a due
foreign same-position key selects the receiving owner's local application,
while the foreign owner remains unchanged. The 2026-09-25
[clock-origin scope review](verification/CLOCK_IDENTITY_SCOPE_REVIEW.md)
executes the companion ADR-0014/0015/0021 limitation: both scheduling owners
accept an instant from one manual clock and release it using an unrelated
clock's equal elapsed value because no provenance is encoded. The 2026-09-26
[configured attachment review](verification/CONFIGURATION_ATTACHMENT_REVIEW.md)
executes the previously open composition boundary: a failed inbox reservation
returns a configured runtime with active revision, rollback history, revision
high-water, and application ownership intact. The 2026-09-27
[failure-event identity review](verification/FAILURE_EVENT_IDENTITY_SCOPE_REVIEW.md)
executes the ADR-0016/0020 attribution limitation through both event-producing
owners: a foreign same-position key fails the receiving local application and
is copied into its event while the foreign owner remains unchanged. The
2026-09-28
[scheduled-failure diagnostics review](verification/SCHEDULE_FAILURE_DIAGNOSTICS_REVIEW.md)
executes the ADR-0015 direct standard-error chains for lifecycle rejection and
a cooperative returned application error while preserving final item
consumption and later peer progress. The 2026-09-29
[messaging-construction diagnostics review](verification/MESSAGING_CONSTRUCTION_DIAGNOSTICS_REVIEW.md)
executes the existing attachment wrapper's standard source chain through the
exact message-bus capacity-overflow cause while retaining the corrected
attachment recovery path. The 2026-09-30
[configured-runtime construction diagnostics review](verification/CONFIGURATION_CONSTRUCTION_DIAGNOSTICS_REVIEW.md)
executes the ADR-0018 ownership wrapper's standard source chain for zero
capacity and deterministic record-capacity overflow while retaining the full
active, rollback, and revision-allocation lineage. The 2026-10-01
[configuration-operation diagnostics review](verification/CONFIGURATION_OPERATION_DIAGNOSTICS_REVIEW.md)
executes selected runtime-wrapper source shapes for unconfigured
operations, semantic validation rejection, and exhausted consume-once rollback
while retaining the same lineage behavior. The 2026-10-03
[returned-stop failure review](verification/STOP_FAILURE_REVIEW.md)
adds a full selected-inbox stop failure followed by exact retained peer FIFO
dispatch, later publication, and terminal callback suppression. It retains
the existing cooperative stop policy. The 2026-10-04
[copied message-topology review](verification/MESSAGE_TOPOLOGY_OWNERSHIP_REVIEW.md)
executes original capacity, routing, and standalone FIFO after caller
configuration/topic mutation and destruction, plus runtime availability,
saturation, stop clearing, and restart reconnection under the copied topology.
The 2026-10-05
[inline payload review](verification/MESSAGE_PAYLOAD_OWNERSHIP_REVIEW.md)
executes constructor-owned bytes after source-vector reuse/destruction and
standalone per-destination copies after publisher binding replacement, scope
exit, and independent consumption. It retains the existing ADR-0010 contract.
The 2026-10-06
[event-record ownership review](verification/EVENT_RECORD_OWNERSHIP_REVIEW.md)
adds explicit producer binding replacement and caller-retained full metadata
after slot reuse and destruction of a nonempty queue. Earlier scheduled replay
already covers loop-local producer scope exit and an emptied queue's
destruction; the new standalone observations retain ADR-0013's contract.
The 2026-10-07
[configuration-view refresh review](verification/CONFIGURATION_VIEW_REFRESH_REVIEW.md)
extends the existing ordinary-work prefix regression through exact-capacity,
shorter, empty, and rolled-back content, then checks the complete copied log
after runtime destruction. Earlier tests cover standalone length changes,
fixed-length runtime transitions, and retained host scalar observations;
borrowing probes already reject callback-byte escape. Focused and full locked
checks and source/document/diff review pass. Checkpoint `16fb457` has
[successful hosted verification](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37468611582)
for the exact checkout, 141 workspace tests, and configured host/sample evidence.
The 2026-10-08
[dispatch availability review](verification/DISPATCH_AVAILABILITY_REFRESH_REVIEW.md)
extends the existing callback-publication regression through peer restart
after an intervening stopped-peer dispatch. It checks delivered/unavailable/
delivered outcomes and exact new-only reply consumption without changing the
per-dispatch lifecycle snapshot contract. Focused/full locked checks and
source/document/diff review pass. Checkpoint `4470987` has
[successful hosted verification](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37626284281)
for the exact checkout, 141 workspace tests, and configured host/sample evidence.
The 2026-10-09 [host milestone review](verification/HOST_V0_1_REVIEW.md)
consolidates the existing lifecycle/service resource, failure, ownership,
diagnostic, dependency, and public-API evidence. Fresh locked checks pass
141 workspace tests, focused host targets and the executable; the reviewed
input has exact hosted CI. Codex accepts RFF-REQ-001 and the routine entry-point
and architecture gates with explicit limits. Stage 4 is complete for this host
target; the package remains unpublished and APIs remain provisional.

After v0.1, select hardening, new capabilities, public-API simplification,
integration guidance and measured experiments by their contribution to practical
consumer use. Property/fault tests and fuzzing should close identified risks;
feature growth should satisfy recorded needs. ADR-0025 supersedes any reading of
the original hardening preference as an indefinite restriction on useful growth.

The 2026-10-10 [finite message-sequence checkpoint](verification/MESSAGE_SEQUENCE_REVIEW.md)
adds independent accepted-history/cursor evidence for standalone publication
and dequeue. It explores every sequence of up to six operations over one fixed
three-inbox topology: 55,987 traces and 324,726 generated operations, plus final
drains. The finite-domain result supplements RFF-REQ-003 without changing its
contract, lifecycle integration, runtime behavior, or APIs.

The 2026-10-11 [runtime lifecycle sequence checkpoint](verification/RUNTIME_MESSAGE_SEQUENCE_REVIEW.md)
adds the separate reduced model: two initially running successful applications,
capacities [1,2], one shared topic and publication/stop/restart sequences of
length zero through six. It checks 19,531 traces and 112,305 generated operations
against independent lifecycle states, accepted histories and discard watermarks.
Passive final dispatch observes retained FIFO records separately from generated
operations. This supplements RFF-REQ-002/003; it changes no production behavior.

The 2026-10-12 [ordinary-work failure sequence checkpoint](verification/RUNTIME_WORK_FAILURE_SEQUENCE_REVIEW.md)
adds the separately frozen fault domain: both fault roles, two initially Running
applications, capacities [1,2], one topic, and publication/work/selected-stop/
selected-restart histories through length six. It checks 39,062 traces and
224,610 generated operations, plus final healthy work and passive FIFO observers.
Terminal availability, exact selected clearing, concrete errors, rejected callback
suppression and retained peer progress supplement RFF-REQ-002/003 and the
peer-operability aspect of RFF-REQ-008. Existing event evidence remains separate.

The 2026-10-13 [message-callback failure checkpoint](verification/RUNTIME_MESSAGE_FAILURE_SEQUENCE_REVIEW.md)
freezes a separate reduced model: two initially Running applications, capacities
[1,2], one shared topic, both fault roles and publication/selected-dispatch/peer-
dispatch through length six. The fixed fault callback publishes one tagged reply
before returning its error. The independent history/cursor oracle checks
2,186 traces and 12,030 generated operations, with passive final peer drains.
Focused/full locked checks pass one/145 tests. Exact queued discard counts
exclude in-flight input; peer replies, callback reports and terminal suppression
match the reference. Production and requirement meanings are unchanged.

The 2026-10-14 [schedule sequence checkpoint](verification/MESSAGING_SCHEDULE_SEQUENCE_REVIEW.md)
freezes a separate messaging-owned model: two initially Running applications,
capacities [1,2], both fault positions, agenda [(0,1s),(1,1s),(0,2s),(1,2s)] and
Publish/Advance1s/Attempt sequences through length eight. The 19,682 traces and
147,624 generated operations join timing, final item consumption and returned
work failure with publication saturation and retained peer queues. Supplemental
healthy work and passive FIFO drains remain distinct from generated operations.
Focused/full locked local checks pass one/146 workspace tests. Independent
source/oracle review accepts the model; selected compiling fault copies fail
behavioral assertions. Production and requirement meanings are unchanged.

The 2026-10-15 [configuration sequence checkpoint](verification/CONFIGURATION_SEQUENCE_REVIEW.md)
addresses the identified standalone history-composition gap. One table with
bound two and initial [1] explores ReplaceA=[1], ReplaceB=[2,3], Reject=[0],
Oversize=[0,0,0] and Rollback through length six. An independent append-only
acceptance/predecessor and movement history predicts original revisions,
active bytes, exact errors, history eviction and consume-once restoration.
Supplemental rollback/replacement/rollback probes expose retained history and
revision allocation after trailing rejected operations. Runtime composition
and revision exhaustion remain separate existing evidence.
Focused/full locked checks pass one/147 workspace tests; independent source/
oracle/document/diff review accepts the frozen scope. Six selected compiling
fault copies fail behavioral assertions. Published checkpoint
`56a68a0c7c860dee02ac42e441f9857421aad164` has
[successful hosted verification](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38569938802)
at its exact checkout, including all 20 steps and configured host/sample checks.
Production and requirement meanings are unchanged.

The 2026-10-16 [runtime configuration sequence checkpoint](verification/RUNTIME_CONFIGURATION_SEQUENCE_REVIEW.md)
addresses the integration gap with a separate direct-runtime model. It composes
accepted/rejected replacement and rollback with selected/peer work and selected
stop/restart through length five for both selected application indices. The
selected application fails on its second work callback, allowing the generated
work/stop/change/restart/work path to observe new configuration and retained
application state at failure. The frozen 74,898 traces/363,792 generated operations
and separate 524,286 terminal probes preserve the distinction between direct
runtime, standalone table and alternate service-owner evidence. Production and
requirement meanings remain unchanged. Focused/full locked checks pass one/148
workspace tests; independent source/oracle review and six selected compiling
fault trials accept their defined expectations. The focused review records limits.
Published checkpoint `283b07c280746dd6ac7c26b60bdaf0ab14e5bad3` has
[successful hosted verification](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38739984030)
for its exact logged checkout, all 20 steps and configured host/sample checks.
The next selected checkpoint assesses standalone event history composition.

The 2026-10-17 [event sequence checkpoint](verification/EVENT_QUEUE_SEQUENCE_REVIEW.md)
supplements the existing capacity, FIFO, saturation/retry, reuse and ownership
scenarios with every mixed Emit(A)/Emit(B)/Dequeue history through length eight
at capacities one, two and three. An independent accepted-record ledger and
consumed ordinal predicts 29,523 traces and 221,436 generated operations;
terminal FIFO drains remain separate observations. Repeated equal records and
opposing timestamp order exercise acceptance order without deduplication or
priority. Focused/full locked checks pass one/149 workspace tests; independent
oracle/source review accepts the frozen domain. Production and requirement
meanings remain unchanged. Whole-tree/rendered-document/complete-diff review and
six compiling fault trials pass their defined expectations. Publication and
exact-revision hosted evidence are recorded in the focused checkpoint.
Published checkpoint `7137e924819b8cc35f0c4f3464f80c3d8df1b149` has
[successful hosted verification](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38905783350)
at its exact logged checkout, all 20 steps and configured host/sample checks.
The 1.99.1 hosted toolchain and updated image receive the required configuration/
whole-tree policy re-audit. The following checkpoint assesses mixed direct-runtime
returned-work failure-event histories beyond existing saturation scenarios.

The 2026-10-18 [runtime event history checkpoint](verification/RUNTIME_EVENT_SEQUENCE_REVIEW.md)
selects direct Runtime composition: two distinct first-work failures and a healthy
peer, shared event capacities one/two with an older diagnostic, and generated
WorkA/WorkB/PeerWork/Advance1s/Dequeue/RetryA/RetryB histories through length five.
Independent symbolic acceptance, failure, invocation and time histories predict
39,216 traces/189,546 generated operations. Separate terminal peer work and FIFO
drains remain supplemental observations. Source/oracle review, full locked local
checks (150 workspace tests), whole-tree/rendered-document review and six selected
compiling fault trials accept their defined expectations. Complete source/document/
diff review accepts the checkpoint. Published revision
`9fc7e0240b36f7eca76f81c5b48030ff8904a338` has
[successful hosted verification](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39035017652)
at its exact logged checkout, all 20 steps and configured host/sample checks.
Production and requirement meanings remain unchanged. Next, assess messaging-owned
event histories only if they add
meaningful evidence beyond this model and existing cleanup/event scenarios.

The 2026-10-19 [messaging event history checkpoint](verification/MESSAGING_EVENT_SEQUENCE_REVIEW.md)
addresses that gap with two initially Running applications, inbox capacities
[1,2], both fault roles and event capacities one/two. Seven actions through
length five join publication, selected failure, peer work/dispatch, advancing
time, event dequeue and explicit rejected-event retry. Independent message and
event acceptance histories predict 78,432 traces/379,092 generated operations;
terminal peer work and FIFO drains remain separate observations. This checks
owner cleanup and peer retention amid event saturation, without changing
production behavior or requirement meanings. Focused/full locked checks pass
one/151 workspace tests; independent oracle/source review accepts the domain.
Six selected ownership faults compile then fail behavioral assertions. Whole-tree,
rendered-document and complete-diff review pass with root and independent Codex
records_review acceptance. Published checkpoint
`27194287b8670b75efac05f3a3f0a7de38a312d5` has
[successful hosted verification](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39130580285)
at its exact logged checkout, all 20 steps and configured host/sample evidence.
