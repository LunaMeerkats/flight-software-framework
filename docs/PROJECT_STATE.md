# Project state

Last updated: **2026-09-01**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Stage 3
has a standalone configuration snapshot core. ADR-0018 now selects how runtime
ownership and read-only ordinary work access will compose; that production
integration remains unimplemented and RFF-REQ-006 remains partial. No sample
mission exists yet.

## Verified baseline

- The clean starting commit `5b3a677c0e13a65387819b59490a0cd198d033fb`
  passed formatting, all-target check, warnings-denied Clippy, all 73 tests,
  warnings-denied rustdoc, and Git whitespace before this decision checkpoint.
- The source-quality policy remains stable rustfmt at 100 columns, the
  individual Clippy `too_many_lines` lint at 60, and forbidden unsafe code.
  Active rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98 are
  evidence versions, not an MSRV or toolchain pin.
- ADR-0017's configuration implementation remains `7dd2376`; the existing
  traceability register retains exact implementation evidence for all slices.
- The separate configuration-context experiment passes one executable probe
  and three intended compiler-rejection probes. Direct diagnostic inspection
  confirms immutable mutation, conflicting table replacement, and lifetime
  escape rejection. This proves the isolated borrow shape, not runtime access.
- No production Rust, Cargo configuration, dependency, or licence changes are
  part of this run. The full baseline passes again with 73 tests, and all four
  separate Markdown probes pass. All 21 Rust files and four snippets meet
  physical/comment width limits; the snippets also pass rustfmt review.
- All 87 relative links resolve across 30 Markdown documents; 8 requirement
  rows, 23 source identifiers, and 54 exact test references are consistent.
  All 11 changed documents pass generated-HTML content/structure review.
  Browser visual QA was not completed; see the precise adaptation below.

## Current architecture

One unpublished, dependency-free safe-Rust package provides:

- A bounded LC1 registry and finite-capacity `Runtime<A>` owning statically
  composed applications with synchronous start, ordinary work, stop, and
  in-place restart. Returned errors fail only the selected record.
- Bounded FIFO publish/subscribe with explicit partial fan-out and reject-newest
  saturation. `MessagingRuntime` owns one inbox per application, couples
  availability and clearing to lifecycle, and dispatches one oldest delivery
  through a separate publish-only application context.
- A bounded structured-event queue and injected manual clock. Opt-in direct
  work failure reporting borrows them and preserves the original work error
  and exact event attempt, including queue saturation.
- A finite one-shot schedule with stable equal-time order and one due/overdue
  attempt per caller request. Its existing evidence is on bare `Runtime`.
- A standalone `ConfigurationTable<E, MAX_BYTES>` with immutable bounded byte
  snapshots, one retained validator, fresh revisions, and consume-once rollback.

Neither runtime permanently owns a clock, event queue, or configuration table.
ADR-0018 is a selected future design, not implemented architecture. It chooses
configuration at construction and one context on ordinary work; it does not add
configuration access to message, start, stop, or restart callbacks.

## Work in progress

The configuration-context decision checkpoint is complete; no production
implementation has begun. Its next migration must preserve all existing
ordinary work paths through one callback without bypassing inbox
cleanup, event reporting, or lifecycle gates.

## Highest risks and uncertainties

- Optional configuration expands concrete runtime and messaging-owner type
  parameters. Constructor inference and composition require production tests;
  the small borrowing probe does not validate the entire proposed API.
- Construction must preserve the caller's table on failure and transfer its
  history unchanged on success. No mutable table exposure or reattachment may
  permit revision reset, including the valid zero-byte configuration case.
- Large inline byte capacities can exhaust stack resources. Caller copies,
  compiler temporaries, and validator effects/errors need separate budgets.
- Validators need not be pure or terminating; rollback does not revalidate.
  Revisions, application IDs, and instants carry no owner origin.
- A returned error can follow partial app mutation or peer publication. No
  cleanup, arbitrary panic/hang containment, or general fault tolerance exists.
- Scheduling has no recurrence, fairness, or deadline guarantee. Failure-event
  reporting is opt-in and saturation offers no guaranteed delivery or retry.
- No CI currently executes the local baseline. Browser review is subject to
  the precise environment limitation recorded in the latest-run evidence.

## Important unresolved decisions

- Production configuration integration must satisfy ADR-0018 before claiming
  RFF-REQ-006; concrete API names remain pre-v0.1 and unfrozen.
- No RFF-REQ-007 host command/telemetry grammar or input boundary is selected.
- Message/lifecycle configuration access, application-authored events, event
  filtering/drain, persistent clock/event owners, messaging-aware scheduling,
  and a broader service context remain outside the selected design.
- Mission payload limits, external identifiers, wire representations, and MSRV
  remain open. No hardware, RTOS, or no_std commitment is made.

## Most likely next tasks

1. Implement ADR-0018 runtime configuration ownership and ordinary work context,
   with direct/scheduled/event/messaging regression evidence in one increment.
2. Select the command/telemetry host grammar and validation boundary.
3. Compose a small sample mission and establish CI without widening claims.

## Latest run

2026-09-01: Selected the configuration ownership/work-context design checkpoint
because a separate configured wrapper would fragment existing work paths.
Compared concrete optional ownership, a provider abstraction, and wrapper/
parallel-callback alternatives. Recorded constructor-only ownership and the
zero-byte reattachment hazard. Added reproducible borrowing probes and primary
Rust/rustdoc source entries. Production behavior is unchanged; 73 production
tests and four separate probes pass, along with all required Cargo checks.
Independent design, source-form, and document reviews found no remaining issue.

Browser connection setup returned `js execution timed out; kernel reset` before
a page could be inspected. Generated-HTML source hashes, normalized text,
headings, lists, code, and tables were inspected as a narrow document-review
adaptation, consistent with the previous run's nonvisual review. Browser visual
QA is not claimed. No alternate browser-control mechanism was used.

The checkpoint is local-only on `codex/nightly`; its commit identity is
discoverable from Git history. Nothing has been pushed.
