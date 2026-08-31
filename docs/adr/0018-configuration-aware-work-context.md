# ADR-0018: Runtime-owned configuration and one ordinary work context

- Status: Accepted design checkpoint; production implementation pending
- Date: 2026-09-01
- Scope: Configuration ownership and read-only ordinary-work visibility

## Context

[ADR-0017](0017-bounded-configuration-snapshots.md) implements the standalone
table selected by [ADR-0005](0005-configuration-revisions-and-rollback.md).
Runtime ownership, application-visible safe points, restart retention, and
behavior after application errors remain unimplemented under RFF-REQ-006.

Ordinary work already feeds direct, scheduled, event-reporting, and messaging
runtime paths. A second configuration-only callback would create another work
meaning and require callers to choose which service guarantees apply.
[ADR-0012](0012-application-message-dispatch.md) explicitly calls for revisiting
the application boundary when a second service needs access. Configuration now
provides that concrete reason; it does not justify a general service framework.

## Decision

### One ordinary callback

The next implementation will change the single `Application::work` callback to
receive an `ApplicationWorkContext<'_>` by value. Its only service access will be
an optional immutable configuration view containing the accepted revision and
borrowed bytes. The view hides the storage capacity and validator error type
from the application trait. It exposes no mutable table, activation, rollback,
lifecycle control, nested work, or other runtime services.

`None` means that the runtime was constructed without configuration. Applications
that require configuration must handle absence explicitly through their ordinary
work-error contract. There will be no default callback that silently redirects
configuration-aware work into the former context-free callback.

All ordinary direct, scheduled, failure-event-reporting, and messaging-owned work
will invoke this same callback through `Runtime::work`. Existing application
implementations must migrate explicitly, including applications that ignore the
context. No parallel `work_with_configuration` operation will be introduced.

The separate `MessagingApplication::handle_message` callback remains unchanged
and receives no configuration view in this increment. Start, stop, and restart
callbacks also remain context-free. Configuration retention across restart means
that the next ordinary work callback sees the retained snapshot; it does not
imply that restart code can inspect it.

### Concrete ownership and construction

The selected representation is
`Runtime<A, E = Infallible, const MAX_CONFIGURATION_BYTES: usize = 0>` with a
private optional `ConfigurationTable<E, MAX_CONFIGURATION_BYTES>`. Defaults keep
the existing unconfigured runtime available. These parameters describe the one
implemented service, not an arbitrary service-provider extension point.
Keep the unconfigured constructor specialized to the default representation;
do not assume unconstrained generic constructor calls infer those defaults.

Configured construction accepts an already validated table and moves its entire
active snapshot, rollback slot, high-water mark, and validator into a fresh
runtime before application registration. It does not revalidate, recreate, or
renumber the table. Any construction failure, including storage reservation
failure, returns the unchanged table with the error to preserve caller ownership.

Configured construction is a separate constructor, not an attachment operation
on an existing runtime. No detach, replacement-of-whole-table, or mutable table
access is exposed. Subsequent changes use narrow runtime replacement and
rollback methods that delegate to the retained table. An unconfigured runtime
returns an explicit typed absence error for those operations.

Restricting attachment to the default runtime type would not establish that it
is unconfigured: a valid zero-byte table with an `Infallible` validator produces
the same type. Constructor-only ownership avoids that reattachment path and
prevents resetting the revision lineage by installing another table.

### Safe points and retained behavior

Replacement and rollback require exclusive mutable runtime access. Work borrows
the current accepted snapshot immutably for its synchronous callback, so the
owner cannot replace or roll back that snapshot during the call through this
API. A later work callback observes the then-active snapshot. The application
may copy bytes or revision values; such copies are not updated by activation
and remain outside the runtime's retention bound.

The private running-callback helper must split the configuration and application
record field borrows. Calling a helper that borrows the entire runtime mutably
while retaining an immutable configuration borrow does not provide that shape.
Keep lifecycle validation and state commitment in one shared private path; do
not duplicate them, remove an application temporarily, or use unsafe code to
obtain simultaneous access. (`SRC-RUST-FIELD-BORROWS`)

Unknown or non-running work requests invoke no application callback. Successful
work retains `Running`. A returned work error commits the selected record to
terminal `Failed` and preserves the exact concrete error before existing outer
operations perform inbox clearing or failure-event reporting. Configuration
content, rollback history, and revision high-water remain unchanged. There is
no automatic rollback after any application error.

Successful stop and in-place restart likewise leave the entire configuration
lineage unchanged. A replacement or explicit rollback between callbacks retains
ADR-0005's existing rules, including consume-once rollback and revision non-reuse.
No callback panic, hang, external effect, or partial application mutation is
contained by this design.

## Alternatives considered

- **Configuration wrapper and parallel callback:** locally additive, but creates
  another owner and work path. Schedule, event, and messaging composition would
  need duplicate forwarding or new service combinations. Reject for this slice.
- **`Runtime<A, C = ()>` with a sealed configuration-provider trait:** can encode
  configured presence in the owner type and avoid absence errors on table
  operations. Defer because it adds a provider abstraction and sealing machinery
  for exactly two known representations. The concrete optional table makes the
  current ownership and absence policy explicit without an extension framework.
- **Attachment after runtime construction:** preserves already registered setup,
  but needs ownership-returning rejection, lifecycle eligibility, and protection
  against repeated attachment, including the zero-byte default-type case.
  Choose constructor-only configuration while no existing consumer needs attach.
- **Caller-borrowed table for each work call:** offers a simple borrow boundary
  but does not meet ADR-0005 runtime ownership or retain one configuration lineage.
- **Broad shared service context now:** defer clocks, events, messaging access,
  and lifecycle callback changes until their specific borrowing need is proved.

## Evidence and implementation acceptance

The [configuration context experiment](../verification/CONFIGURATION_CONTEXT_EXPERIMENT.md)
records the Rust borrowing checks for this decision. Its isolated rustdoc probes
are language-shape evidence only, not production runtime integration tests or
RFF-REQ-006 completion evidence. (`SRC-RUSTDOC-TESTS`)

The first production implementation must migrate the concrete runtime parameters
through scheduling, direct failure-event reporting, and the messaging owner in
the same coherent increment. The messaging owner must expose the narrow
configuration operations without exposing its inner runtime mutably. Acceptance
requires:

- unconfigured and configured ordinary work using the one callback;
- runtime-construction failure preserving the caller's validated table;
- initial visibility, replacement visibility, and rejected replacement retaining
  both active content and rollback history as observed by later work;
- consume-once rollback and a later replacement that does not reuse revisions;
- stop/restart retaining configuration and history, followed by observable work;
- returned application errors preserving configuration/history, the original
  error, terminal `Failed`, peer progress, and existing inbox/event behavior;
- scheduled and opt-in failure-event work observing the same active snapshot;
- rejected lifecycle work suppressing callbacks without changing configuration;
- borrowed views unable to escape their runtime borrow, while explicit copies
  remain possible and are documented as application-owned;
- all existing verification, full-diff/source review, and traceability updates.

## Risks and revisit conditions

This changes the unpublished application work trait and expands concrete runtime
type parameters. It is not an API freeze. Optional configuration needs explicit
absence handling, and deferred message/lifecycle access must remain visible in
documentation. Inline capacities, validator purity/termination, table-local
revision identity, and caller-owned copies retain ADR-0017's limitations.

Revisit if a concrete mission requires typed decoding, message or lifecycle
configuration access, attachment of an already composed runtime, additional
owned services, or concurrency. RFF-REQ-006 remains partial until production
integration and its required verification actually complete.
