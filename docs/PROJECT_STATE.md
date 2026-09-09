# Project state

Last updated: **2026-09-10**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Stage 3
has verified configuration and host adapters. ADR-0020 now adds cooperative
failure-event reporting through the messaging owner as one prerequisite for
the combined service sample. Messaging-aware scheduling, the full sample, CI,
and the v0.1 architecture review remain outstanding.

## Verified baseline

- Started clean on `codex/nightly` at
  `4cb621aa50708fbc823dd50f110db7404a461889`; all six initial checks passed
  with 96 tests. Toolchain unchanged: rustc/cargo 1.98.0, rustfmt
  1.9.0-stable, Clippy 0.1.98. These are evidence, not an MSRV or pin.
- Final formatting, all-target/all-feature check, warnings-denied Clippy,
  102 tests, warnings-denied rustdoc, and Git whitespace checks pass.
- Implementation and six-test evidence are committed locally at
  `0d1ddb81a8dd41ef8a96ffc5d7afefea3b1c12bc`.
- `cargo test --test messaging_work_events` passes six new tests for exact
  clearing/errors/events, lifecycle suppression, saturation, configuration
  history, and later peer work/dispatch. Existing direct-runtime event tests
  continue to pass after sharing the private reporting implementation.
- Independent complete-diff review found no actionable defect. All 28 Rust
  files meet physical/comment-only width limits; three existing expectations
  remain fulfilled. No dependency, unsafe code, public error type, or lint
  waiver was added. Unchanged host adapters and ADR-0018/0019 experiments were not
  separately rerun; workspace tests still include all 13 adapter tests.
- Document audits pass: 33 Markdown files, 107 links, eight requirement rows,
  26 sources, 66 exact test references, and ten changed content comparisons.
  Browser heading/overflow inspection passes at 1,280 pixels with no console
  warnings; the new decision's opening screenshot was inspected successfully.

## Current architecture

One unpublished, dependency-free safe-Rust library provides a finite LC1
runtime with synchronous lifecycle/work callbacks, bounded lifecycle-owned
inbox dispatch, injected manual time and finite one-shot scheduling, bounded
events, and constructor-owned optional configuration with immutable
ordinary-work visibility and consume-once rollback. The private `host-echo`
example provides the validated caller-framed command/telemetry adapter pair.

`MessagingRuntime::work_with_failure_event` delegates through its existing
work operation. Terminal failure and exact selected-inbox clearing complete
before one clock read and bounded event attempt. The nested existing errors
retain the work error, discard count, event, and emission outcome. Both work
owners share event construction; no mutable owner escape is exposed.

## Work in progress

No unfinished implementation remains. The messaging-owned work event operation
has reached its tested, documented, reviewed and locally committed stopping
point. The evidence update changes documentation only.

## Highest risks and uncertainties

- Events can saturate; the caller receives the exact rejected event. Reporting
  remains opt-in and ordinary-work-only. No callback panic/hang containment,
  execution rollback, recovery, or guaranteed diagnostic delivery is added.
- The full sample cannot yet use the existing finite schedule through the
  messaging owner. Preserve inbox clearing in that separate integration.
- Host output saturation terminally fails telemetry while retaining old output;
  drain cannot recover it. Physical delivery and stream framing remain absent.
- IDs, revisions, and instants do not encode owner origin; callbacks/validators
  need not terminate. Large inline bounds can exhaust stack resources.
- No CI currently executes the local baseline.

## Important unresolved decisions

Messaging-aware schedule consumption and error representation need a bounded
decision before implementing the full sample driver. Message/lifecycle
configuration access, application-authored events, scheduled event reporting,
host event drain, physical I/O, MSRV, hardware, RTOS, and no_std remain open.
The local grammar and pre-v0.1 APIs are unfrozen.

## Most likely next tasks

1. Integrate one-shot scheduled ordinary work through the messaging owner,
   preserving final item consumption, original errors, and exact inbox clearing.
2. Compose the documented combined sample with explicit ownership/driver order.
3. Establish CI and then record the v0.1 architecture review.

## Latest run

2026-09-10: Added one opt-in messaging-owned failure-event operation and six
public integration tests. Both initial and final Cargo baselines pass. The
increment resolves one concrete service-composition gap and leaves scheduling
separate. Source-form, document-content/layout, and independent complete-diff
reviews pass. No push is authorized or performed.
