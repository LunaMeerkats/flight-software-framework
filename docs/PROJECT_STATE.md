# Project state

Last updated: **2026-09-09**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Stage 3
has verified in-memory configuration ownership and ordinary-work visibility,
plus a private host command/telemetry adapter pair implementing ADR-0019.
The full v0.1 service sample, CI, and architecture review remain outstanding.

## Verified baseline

- Started clean at `3674863865d68ced666a1fc0b3e478b672fc79c3` on `codex/nightly`.
  All six required pre-change commands passed with 83 tests.
- rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98 are unchanged.
  They record evidence, not an MSRV or toolchain pin.
- Final formatting, all-target/all-feature check, warnings-denied Clippy,
  96 tests, warnings-denied rustdoc, and Git whitespace checks pass.
- The focused adapter target passes 13 tests. The executable `host-echo`
  example prints exact command/telemetry pairs for 0, 42, and 100 percent.
- The explicit library build, standalone mailbox rustdoc probe (one executable,
  three scenarios), and extracted rustfmt/Clippy commands pass. Its 185-line
  source has no physical/comment-only width findings.
- All 27 handwritten Rust files have no width findings; three existing
  expectations remain fulfilled. No library API, dependency, unsafe code,
  or lint exception was added. Independent complete-diff review passed.
- Generated document content, link/source/test-name audits, and browser layout
  inspection pass. Screenshot capture timed out; the recorded narrow review
  adaptation uses content comparison and DOM/layout inspection, not visual QA.

## Current architecture

One unpublished, dependency-free safe-Rust library provides a finite LC1
runtime with synchronous owned lifecycle/work callbacks; bounded FIFO routing
and lifecycle-owned inbox dispatch; injected manual time and finite one-shot
scheduling; bounded events and direct returned-work failure reporting; and
constructor-owned optional configuration with immutable ordinary-work
visibility and consume-once rollback.

The private `examples/host-echo` target adds a validated two-byte command codec,
echo and telemetry applications, two one-slot inboxes, and a host-owned borrowed
one-record output mailbox. Ingress returns the original delivery report; the
host selects each dispatch and drains exact telemetry bytes separately.
`tests/host_adapters.rs` loads the same mission source. No callback performs I/O.

## Work in progress

No unfinished implementation remains. The adapter pair reached its tested,
documented, and reviewed stopping point. Local evidence references are recorded
with the commits; the next run should reorient before composing the full sample.

## Highest risks and uncertainties

- Full host output returns a message error and terminally fails telemetry.
  Older output remains drainable; draining does not recover the application.
  No retry, execution rollback, physical delivery, or stream framing is promised.
- Report-allocation failures are preserved by source-reviewed typed paths;
  no reproducible allocator-injection seam exists for the fixed mission.
- Callback cardinality combines one-call source review and exact runtime/output
  tests, not a separately instrumented business-call counter.
- Application IDs, revisions, and instants carry no owner origin. Callbacks and
  validators need not terminate; returned errors can follow partial effects.
  Panic/hang containment and general fault tolerance remain absent.
- Large inline configuration bounds can exhaust stack resources. Scheduling
  offers no fairness or deadline guarantee; failure-event storage can saturate.
- No CI currently executes the local baseline.

## Important unresolved decisions

The full sample must compose existing services with explicit owners and driver
order. Message/lifecycle configuration access, application-authored events,
host event drain, and messaging-aware scheduling remain outside this slice.
Physical I/O, format evolution, MSRV, hardware, RTOS, and no_std commitments
remain open; the local grammar and pre-v0.1 APIs are unfrozen.

## Most likely next tasks

1. Compose a documented sample mission with existing service boundaries and
   re-demonstrate the narrow returned-work failure/event and peer-progress path.
2. Establish CI for the baseline and executable sample.
3. Record the v0.1 architecture review without widening unsupported claims.

## Latest run

2026-09-09: Implemented one bounded ADR-0019 adapter pair. Tests cover exhaustive
percentage and identifier domains, malformed-input preservation, publication
reports, explicit dispatch/drain, output saturation, lifecycle retention, and
replay. Corrected shared-module child lookup after the first test build; both
targets now compile the same source. Initial/final baselines, explicit probe,
source-form, document-content/layout, and independent complete-diff review pass.
Screenshot visual QA was unavailable and is not claimed. This is one bounded
local increment; no push is authorized or performed.
