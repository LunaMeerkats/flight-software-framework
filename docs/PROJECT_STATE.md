# Project state

Last updated: **2026-09-02**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Stage 3
now has a verified bounded in-memory configuration lifecycle: a standalone
table plus optional constructor ownership and immutable ordinary-work visibility
through every existing work entry path. RFF-REQ-006 is verified at that boundary.
No command/telemetry adapter or sample mission exists yet.

## Verified baseline

- The clean starting commit `5251f65784e0676f242158752894c9728ffebe6d`
  passed formatting, all-target check, warnings-denied Clippy, all 73 tests,
  warnings-denied rustdoc, and Git whitespace before this increment.
- Implementation commit `9afc85686de192d66e36af950b1b63a29ca541ca`
  adds runtime-owned optional configuration, the work context, forwarding
  generics, ten focused integration tests, and the production lifetime probe.
- The post-implementation baseline passes formatting, all-target/all-feature
  check, warnings-denied Clippy, all 83 tests, warnings-denied rustdoc, and Git
  whitespace on rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
  These versions are evidence, not an MSRV or toolchain pin.
- The separate configuration-context experiment passes one executable and four
  intended compiler-rejection probes. Direct diagnostic inspection confirms
  E0594, E0502, table-view lifetime escape, and production work-context lifetime
  escape. The new production probe fails only because its callback borrow cannot
  outlive `'static`.
- All 22 handwritten Rust files and 8,208 lines satisfy the 100-column physical
  and 80-column comment-only review limits. Three unchanged reasoned function
  expectations remain; there are no `allow` attributes or unsafe-code tokens.
- The Markdown audit covers all 30 documents, 87 relative links, eight
  requirement definitions and trace rows, and 53 exact test references without
  findings. All 11 changed pages render without horizontal overflow, skipped
  heading levels, table overflow, or browser console warnings at the inspected
  1,265-pixel viewport. Browser screenshot capture timed out, so no
  screenshot-based visual QA is claimed.

## Current architecture

One unpublished, dependency-free safe-Rust package provides:

- A bounded LC1 registry and finite-capacity `Runtime` owning statically
  composed applications with synchronous start, ordinary work, stop, and
  in-place restart. Returned errors fail only the selected record.
- An optional `ConfigurationTable` moved into `Runtime` only at construction.
  `Application::work` receives one `ApplicationWorkContext` with an optional
  immutable revision/bytes view. Narrow replacement and consume-once rollback
  operations delegate to the table; unconfigured operations return a typed
  absence error. Start, stop, restart, and message callbacks remain context-free.
- Bounded FIFO publish/subscribe with explicit partial fan-out and reject-newest
  saturation. `MessagingRuntime` owns one inbox per application, couples
  availability and clearing to lifecycle, delegates ordinary work through the
  configured runtime, and exposes only narrow configuration operations.
- A bounded structured-event queue and injected manual clock. Opt-in direct
  work failure reporting borrows them and preserves the original work error and
  exact event attempt, including queue saturation.
- A finite one-shot schedule with stable equal-time order and one due/overdue
  attempt per caller request. Scheduled and failure-event work both delegate
  through the same configuration-aware `Runtime::work` callback.

The runtime still does not permanently own a clock or event queue. Configuration
has no schema, wire format, persistence, host loader, lifecycle/message access,
automatic error rollback, or attachment after runtime construction.

## Work in progress

The ADR-0018 production increment is complete. No next implementation has begun.
The next run should reorient before selecting the command/telemetry boundary;
repository evidence, not this ordering alone, remains authoritative.

## Highest risks and uncertainties

- Large inline configuration capacities can exhaust stack resources. Caller
  copies, compiler temporaries, and validator effects/errors need separate
  budgets.
- Validators need not be pure or terminating; rollback does not revalidate.
  Revisions, application IDs, and instants carry no owner origin.
- Optional configuration expands concrete runtime and messaging-owner generic
  parameters. These APIs remain pre-v0.1 and are not frozen.
- A returned error can follow partial app mutation or peer publication. No
  cleanup, arbitrary panic/hang containment, or general fault tolerance exists.
- Scheduling has no recurrence, fairness, or deadline guarantee. Failure-event
  reporting is opt-in and saturation offers no guaranteed delivery or retry.
- No CI currently executes the local baseline.

## Important unresolved decisions

- No RFF-REQ-007 host command/telemetry grammar or validation boundary is
  selected.
- Message/lifecycle configuration access, application-authored events, event
  filtering/drain, persistent clock/event owners, messaging-aware scheduling,
  and a broader service context remain outside the implemented boundary.
- Mission payload limits, external identifiers, wire representations, and MSRV
  remain open. No hardware, RTOS, or `no_std` commitment is made.

## Most likely next tasks

1. Select the smallest host command-ingest and telemetry-output grammar and
   validation boundary for RFF-REQ-007.
2. Implement one bounded adapter pair after the decision is recorded.
3. Compose a small sample mission and establish CI without widening claims.

## Latest run

2026-09-02: Implemented ADR-0018 as one cross-cutting ordinary-work increment.
`Runtime` now owns an optional complete table from construction and passes a
validator- and capacity-independent immutable active view to
`Application::work`. Construction failure returns the unchanged table;
unconfigured mutation is typed; replacement and rollback retain ADR-0005
semantics. Direct, scheduled, failure-event, and messaging-owned work share the
same callback. Returned errors retain the exact error and `Failed` state without
automatic rollback; existing event and inbox behavior remains intact and a peer
can continue.

Ten focused tests cover absence, the valid zero-byte/default-type distinction,
used-prefix visibility, construction recovery, activation/rejection/rollback,
revision non-reuse, lifecycle suppression and restart, returned errors and peer
progress, schedule/event behavior, and messaging cleanup. Independent code and
acceptance reviews found no remaining defect after stale source wording,
comment widths, used-prefix evidence, and one overstrong documentation phrase
were corrected. The implementation is local-only on `codex/nightly`; nothing
was pushed.
