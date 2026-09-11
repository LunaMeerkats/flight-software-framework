# Project state

Last updated: **2026-09-12**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
ADR-0022 now combines lifecycle, messaging, manual time, finite scheduling,
configuration, host adapters, and cooperative work-failure events in one
executable sample. CI and human v0.1 release reviews remain outstanding.

## Verified baseline

- Started clean on `codex/nightly` at
  `c8696f041d05a19aabb22c37871d1381425c06f9`. Initial required baseline
  passed 109 tests on unchanged rustc/cargo 1.98.0, rustfmt 1.9.0-stable,
  and Clippy 0.1.98. These are evidence, not an MSRV or pin.
- Final format, all-target/all-feature check, warnings-denied Clippy,
  115 tests, warnings-denied rustdoc, and Git whitespace checks pass.
- Focused `host_sample` passes five shared-driver tests; `host_adapters`
  passes 14 tests. The executed `host-echo` prints the scope notice, all three
  documented command/telemetry records, and the fixed structured report.
- A bounded two-slot monitor copies configuration from actual callbacks.
  Review replaced non-consuming reads with consume-once reads to prevent
  stale values from establishing later callback/restart evidence.
- No library API, dependency, unsafe code, global policy change, or lint waiver
  was added. Independent diff review also corrected one stale sample claim.
- All 32 Rust files meet physical/comment widths; three existing expectations
  remain fulfilled. Audits pass for 36 Markdown files, 128 relative links,
  eight requirement rows, 26 sources, 79 test references, and 11 changed HTML
  content comparisons. Rendered browser DOM/layout inspection at 1,280 pixels
  found no overflow. Screenshot capture timed out; the plan records the narrow
  review adaptation. Screenshot visual QA is not claimed.
- ADR-0018/0019 experiments are unchanged and not separately rerun.
- Implementation and exact test evidence are committed at
  `4ef42bdd66997aeeefbbfe0ea95c647cdc6e6fa2`. The follow-up evidence commit
  changes documentation only; the verified Rust source is unchanged.

## Current architecture

One unpublished, dependency-free safe-Rust library provides finite LC1
lifecycle, synchronous work, bounded lifecycle-owned inbox dispatch, manual
injected time and one-shot scheduling, bounded events, and constructor-owned
configuration with immutable ordinary-work visibility and consume-once rollback.

The private `host-echo` driver composes those APIs without a new service owner.
The runtime owns two applications, their inboxes, and a one-byte table. Host
values own one mailbox, two consumed observation slots, a manual clock, two
agenda items, and one event slot. The fixed report copies observed outcomes.
The separate ordinary-work fault path clears only echo's queued command and
records one event at 20 ms; the telemetry peer then works and dispatches its
retained record. The command grammar is unchanged.

## Work in progress

No unfinished implementation remains. The combined sample has reached its
tested, documented, reviewed stopping point.

## Highest risks and uncertainties

- CI does not yet execute the verified local baseline or combined sample.
- Human scope, dependency, and architecture review remain release gates;
  the sample does not establish flight readiness or v0.1 release readiness.
- Callback/clock panics and hangs remain outside containment. No recovery,
  real-time guarantee, recurrence, implicit retry, or event delivery is added.
- Host output saturation remains terminal with retained older output; drain
  does not recover it. Physical delivery and stream framing remain absent.
- IDs, revisions, and instants remain caller-scoped. Validators/callbacks need
  not terminate; large inline bounds can exhaust stack resources.

## Important unresolved decisions

CI platform and reproducible baseline execution are next. Message/lifecycle
configuration access, application-authored or scheduled events, external I/O,
MSRV, hardware, RTOS, and no_std remain open. Local grammar and pre-v0.1 APIs
remain unfrozen; the sample's observation byte is not a physical setting.

## Most likely next tasks

1. Establish CI for the required baseline and combined sample commands.
2. Review dependency features/licences and user-facing scope claims.
3. Record human v0.1 architecture review before broadening scope.

## Latest run

2026-09-12: Built the combined host sample using existing service APIs. Five
sample tests and one additional adapter fixture test pass; the full suite has
115 passing tests. Initial/final Cargo baselines and executable inspection pass.
Source, document-content/layout, and independent complete-diff reviews are
complete, with the screenshot limitation recorded above.
No push is authorized or performed.
