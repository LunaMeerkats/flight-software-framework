# Plan: add an injected manual framework clock

Status: **In progress**
Date: **2026-08-28**

## Objective

Record and implement the smallest injected-time slice for Stage 2: a general
elapsed framework instant, an object-safe clock boundary, and a manually
advanced clock whose readings can produce event timestamps repeatably.

This is the highest-value next step because the bounded event queue now has a
timestamp consumer but still accepts caller-invented elapsed values. The clock
reduces that integrity gap and establishes the time seam needed by later
scheduling without combining either scheduling or runtime event emission into
this increment.

## Context and decision boundary

RFF-REQ-004 requires injected framework time and repeatable simulated-time
behavior. RFF-REQ-005 requires framework timestamps on structured events. The
current `EventTimestamp` wraps an explicit caller-supplied `Duration`, and no
clock exists.

The clock should not return event-specific vocabulary because scheduled work is
the next recorded time consumer. A distinct `FrameworkInstant` will represent
elapsed time from one clock origin. `EventTimestamp` remains the event-field
type and captures one instant explicitly. This keeps the provider independent
of the first consumer without adding a scheduler abstraction.

## Acceptance criteria

- `FrameworkInstant` is an ordered copied value containing elapsed `Duration`
  from one framework-clock origin; it carries no wall-clock or cross-clock
  comparison claim.
- `Clock::now` returns the current instant without advancing or causing a
  hidden effect, and remains usable as an injected trait boundary.
- `ManualClock` starts at zero by default, may start at an explicit elapsed
  instant for a controlled scenario, and advances only when the caller supplies
  a nonnegative `Duration`.
- Zero advancement and repeated reads preserve the current instant.
- Addition overflow returns a typed error containing the current instant and
  requested advance, and leaves the clock unchanged.
- `EventTimestamp` can capture an injected clock reading while retaining its
  existing explicit elapsed constructor for standalone and replayed records.
- Public tests prove zero and repeated reads, cumulative advancement, typed
  non-mutating overflow, identical replay traces, and exact event timestamps
  from an injected manual clock through the existing bounded FIFO queue.
- RFF-REQ-004 and RFF-REQ-005 remain partially verified. Equal-deadline work,
  scheduled dispatch, runtime failure events, and a complete host scenario are
  not claimed.
- No wall-clock implementation, sleep, thread, executor, scheduler, runtime
  integration, application context, dependency, protocol, or compatibility
  claim is added.
- The complete documented baseline and repository document/source-form audits
  pass, and the complete diff contains no unrelated implementation change.

## Files and components

- `src/clock.rs`: framework instant, clock boundary, manual implementation, and
  checked advancement error.
- `src/events.rs`: event timestamp conversion from an injected clock reading.
- `src/lib.rs`: narrow clock and timestamp re-exports.
- `tests/manual_clock.rs`: public API and injected event-timestamp evidence.
- `docs/adr/0014-injected-manual-framework-clock.md`: time semantics,
  alternatives, risks, and revisit conditions.
- `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, `docs/ROADMAP.md`,
  `docs/PROJECT_STATE.md`, `docs/verification/TRACEABILITY.md`: truthful
  current behavior, limits, evidence, and next step.
- `PLANS.md`: this bounded plan and final result.

No new external source is required. The slice implements the injected-time
direction already recorded in RFF-REQ-004 and ADR-0001 using stable standard
library `Duration` behavior on the repository's audited toolchain.

## Verification approach

- Run the focused manual-clock integration tests while implementing.
- Run `cargo fmt --all -- --check`.
- Run `cargo check --workspace --all-targets --all-features`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo doc --workspace --all-features --no-deps` with
  `RUSTDOCFLAGS=-D warnings`.
- Run `git diff --check`.
- Re-audit handwritten Rust physical and comment-only widths and every reasoned
  Clippy expectation.
- Verify relative Markdown links, headings, tables, requirement rows, ADR and
  source identifiers, and changed-document structure.
- Review the complete diff for accidental clock-domain comparisons, hidden time
  movement, unchecked overflow, public API commitments, stale claims, and
  changes outside the objective.

## Risks and safe stopping point

`FrameworkInstant` values do not encode clock identity, so values from different
clock instances can be compared accidentally. The manual clock also permits an
explicit nonzero starting point for controlled tests. Documentation must limit
ordering claims to readings from one clock origin.

Retaining `EventTimestamp::from_elapsed` means standalone callers can still
construct arbitrary event timestamps. Only the new capture path is
framework-clock-produced; RFF-REQ-005 therefore remains partial until a host or
runtime integration owns that path.

Stop after the clock vocabulary, checked manual implementation, injected event
timestamp evidence, decision record, and durable state are coherent. Do not add
scheduled work, equal-deadline policy, runtime error events, an application
service context, wall-clock access, automatic progression, or concurrency in
this run.

## Result

Pending implementation and verification.
