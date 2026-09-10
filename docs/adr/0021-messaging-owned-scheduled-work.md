# ADR-0021: Messaging-owned one-shot scheduled work

- Status: Accepted and implemented
- Date: 2026-09-11
- Scope: Finite scheduled ordinary work through the existing messaging owner

## Context

The combined host sample needs both finite scheduling and lifecycle-owned
inboxes. Moving a composed `Runtime` into `MessagingRuntime` deliberately
removes direct mutable access to the inner runtime. The direct scheduled-work
method is therefore unavailable to the host after that move.

[ADR-0015](0015-caller-driven-scheduled-work.md) already requires a future
messaging schedule path to delegate through `MessagingRuntime::work`.
[ADR-0011](0011-runtime-owned-message-availability.md) assigns inbox cleanup to
that owner, and [ADR-0018](0018-configuration-aware-work-context.md) keeps all
ordinary work on one configuration-aware callback. These local contracts
settle the required behavior without a new upstream assumption.

## Decision

Add `MessagingRuntime::run_next_scheduled_work`, borrowing the existing
`WorkSchedule` and injected clock. Reuse `ScheduledWorkOutcome` and return
`MessagingOperationError<ScheduledWorkError<A::WorkError>>` on failure. The
outer error retains exact discarded deliveries; the inner error retains the
consumed item, observed instant, and unchanged `RuntimeWorkError`.

Both owners use one crate-private `WorkSchedule::run_next` implementation:

1. Return `Complete` without reading the clock when no item remains.
2. Otherwise read the clock once and return `Waiting` without mutation before
   the next item's instant.
3. For an equal or overdue instant, consume exactly one item before calling
   the owner's ordinary-work operation.
4. Direct scheduling delegates to `Runtime::work`; messaging scheduling
   delegates to `MessagingRuntime::work`.
5. Preserve successful state or the complete owner-specific error. Consumption
   is final for success, lifecycle rejection, and a returned application error.

The private schedule implementation accepts a single-use closure with the
selected item and observed instant. It owns only the timing/cursor/outcome
decision. The owner constructs its error after ordinary work returns. A
crate-private `ScheduledWorkError::new` avoids exposing constructors or
changing the existing error's public generic meaning.

Successful messaging work leaves inboxes unchanged. Lifecycle rejection
invokes no callback and clears no inbox. A cooperative returned work error
commits terminal `Failed`, clears exactly the selected inbox, and preserves
peer queues. The same callback observes the active configuration and retains
configuration history after failure. Later explicit calls can attempt due
peers. Ordinary work does not dispatch a message or emit an event.

The messaging operation belongs beside its owning work method in
`messaging_runtime.rs`; timing and scheduled errors remain in `scheduling.rs`.
No public executor trait, new error type, runtime escape, service owner,
dependency, or mutable schedule configuration is introduced.

## Alternatives considered

- **Call the inner direct scheduled method, then clear:** would require a
  second interpretation of its error to reproduce messaging work cleanup.
  Reject this bypass of the existing authoritative owner operation.
- **Duplicate schedule selection in both owners:** initially small, but clock
  read counts, waiting, and final consumption would have two implementations.
  Share that exact policy privately without broadening the public surface.
- **Generalize the public scheduled error over arbitrary owner errors:** would
  change the meaning of existing `ScheduledWorkError<E>` and its accessors.
  Keep its runtime-error meaning and nest it in the established messaging
  wrapper, as other messaging operations already do.
- **Add a public work-executor trait or unified service runtime:** neither is
  needed for two concrete callers of one private decision. Defer until a
  demonstrated composition problem warrants the wider commitment.
- **Add scheduled failure events now:** adds timestamp and reporting-order
  choices beyond this missing operation. Keep the existing opt-in ordinary
  failure-event path separate, as in ADR-0020.

## Evidence and acceptance

The clean pre-change baseline passes all 102 tests. The focused public-API
target `tests/messaging_scheduled_work.rs` checks clock-read and consume
boundaries, equal-time order, lifecycle rejection, exact errors and cleanup,
peer FIFO and later work, configuration retention, and repeatable manually
controlled clock readings. All seven focused tests and the full 109-test Cargo
baseline pass. Existing direct-schedule tests protect the shared-policy
extraction. Source review verifies consumption before owner invocation; public
tests observe post-return state. Final document/source reviews and their limits
are recorded in the plan, project state, and traceability.

## Consequences, risks, and revisit conditions

This completes the recorded scheduling ownership prerequisite for the full
sample; it does not implement that sample. The agenda remains finite and
one-shot, with no recurrence, automatic retry/draining, fairness, execution-time
bound, or event reporting. Failure does not roll back application effects or
configuration. Callback/clock panics and hangs remain outside containment.

Application and clock identities do not encode origin. Callers must pair the
schedule with the intended owner and clock. The private cursor advances before
application invocation, but no panic-containment test or guarantee is added.

Revisit for a concrete recurring schedule, scheduled event requirement,
origin-aware identities, or an owner redesign supported by evidence. Preserve
finite bounds, owner cleanup, exact error context, explicit consumption, and
manual-time replay in any extension.
