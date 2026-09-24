# Clock-origin scope review

Date: **2026-09-25**
Scope: **Existing ADR-0014/0015/0021 caller-scoped clock contract**

## Decision and inspected boundary

Retain the current elapsed-time and finite one-shot scheduling contracts, and
execute their documented clock-origin limitation through both scheduling
owners. No production mismatch was found in `src/clock.rs` or
`src/scheduling.rs` at starting revision
`f5b6ab82f888b45ea122ab1fb84aed2e202187c2`. This review supplements
RFF-REQ-004, [ADR-0014](../adr/0014-injected-manual-framework-clock.md),
[ADR-0015](../adr/0015-caller-driven-scheduled-work.md), and
[ADR-0021](../adr/0021-messaging-owned-scheduled-work.md) without changing
behavior.

`FrameworkInstant` stores only elapsed `Duration`. It has no reference to the
`Clock` that produced it. `WorkSchedule` compares the next copied instant with
the value returned by the clock borrowed for that invocation. Equal elapsed
values therefore compare equal even when they came from unrelated clock
instances, and either public scheduling owner treats the item as due.

This remains caller discipline, not authorization to compare clock domains. A
caller must pair each configured schedule with the clock whose elapsed origin
gives those instants their intended meaning.

## Executed public observations

The two regressions are:

- `unrelated_clock_elapsed_value_drives_direct_schedule` obtains a five-second
  deadline from one `ManualClock`, then supplies another clock to the direct
  runtime. The second clock waits at zero and releases the item when it reaches
  the same elapsed value; the Alpha callback runs once.
- `unrelated_clock_elapsed_value_drives_messaging_schedule` repeats the
  observation through `MessagingRuntime`. Both full inboxes remain unchanged
  before and after successful ordinary work.

Each schedule clock remains at its configured reading. The independent
execution clock advances separately from zero to that same value. The public
outcomes expose only the copied item and elapsed observations; no origin check
or relationship between the clocks is observable.

## Alternatives and limits

An origin-bearing instant would require a bounded domain-identity source plus
explicit construction, equality, ordering, exhaustion, copying, persistence,
event-timestamp, and public-API decisions. Binding a schedule permanently to
one clock owner would change the existing borrowed host composition. Neither
redesign is justified by an evidence checkpoint.

These tests use two fresh manual clocks, one five-second deadline, successful
ordinary work, and no clock mutation during a callback. They do not establish
meaningful ordering across unrelated clocks, wall-clock mapping, clock drift,
deadline accuracy, execution-time bounds, real-time behavior, panic or hang
containment, or human API acceptance.

No new external source was needed. The observations follow accepted local ADRs
and executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 129 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. After formatting, the focused command
`cargo test --locked --test scheduled_work --test messaging_scheduled_work`
passes eleven direct and nine messaging-owned tests including both regressions.
The final locked baseline passes 131 workspace tests. Formatting, all-target
checking, warnings-denied Clippy/rustdoc, and whitespace checks pass without a
new exception.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. This checkpoint does
not establish cross-clock correctness or human acceptance.

## Continuation

Publication and exact-revision hosted CI evidence are pending. A later
documentation-only follow-up may record completed evidence without projecting
that result onto a later revision. The next bounded task should be selected
from a reconciled service, resource, failure, or public-API gap; broader scope
remains unapproved.
