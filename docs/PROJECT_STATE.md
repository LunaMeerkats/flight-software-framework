# Project state

Last updated: **2026-09-08**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Stage 3
has verified in-memory configuration ownership and ordinary-work visibility.
ADR-0019 now selects the host command/telemetry boundary. Its borrowed output
mailbox has executable design evidence; no codec, adapter pair, or sample
mission exists yet. RFF-REQ-007 remains not verified.

## Verified baseline

- Starting commit `0ddf2b40e0bd4f7ae52e099ac9d9ec3be8c70428` is clean on
  `codex/nightly`; its format, all-target/all-feature check, warnings-denied
  Clippy, 83 tests, warnings-denied rustdoc, and Git whitespace checks pass.
- rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98 are unchanged.
  These versions record evidence, not an MSRV or toolchain pin.
- Runtime configuration integration remains at
  `9afc85686de192d66e36af950b1b63a29ca541ca`; exact requirement evidence is in
  the traceability register. The ADR-0018 borrowing probes are unchanged.
- The explicit host-mailbox build and standalone rustdoc probe pass: one
  executable containing three scenarios. It proves live borrowed-mailbox reuse,
  retained output under full storage, terminal selected-app failure with exact
  queue clearing, and internal validation before capacity handling. It does not
  prove the external codec, complete mission topology, or physical I/O.
- The extracted 185-line probe passes rustfmt and warnings-denied Clippy with
  the repository's 60-line threshold. All 22 handwritten Rust files and the
  probe have no physical/comment-only width findings. Three existing function
  expectations remain unchanged; no new waiver or dependency is introduced.
- The final Cargo/Git baseline passes with the same 83 tests. The document audit
  covers 32 Markdown files, 95 resolving links, eight requirement/trace rows,
  25 source entries, and 53 exact test references. All 11 changed pages match
  generated HTML content. Browser review at 1,280 pixels finds no page/table
  overflow or heading skips; decision and experiment screenshots were inspected.

## Current architecture

One unpublished, dependency-free safe-Rust library provides a finite LC1
runtime with synchronous owned lifecycle/work callbacks; bounded FIFO routing
and lifecycle-owned inbox dispatch; injected manual time and finite one-shot
scheduling; bounded structured events and opt-in direct returned-work failure
reporting; and constructor-owned optional configuration with immutable
ordinary-work visibility and consume-once rollback.

ADR-0019 selects mission-local two-byte EchoPercent input and matching output,
validated before business logic. Two applications use existing messaging APIs.
A telemetry subscriber borrows a capacity-one host mailbox, and host drain
returns encoded bytes outside callbacks. This design has not changed library
APIs or implemented the adapter pair.

## Work in progress

The host boundary decision and mailbox probe are complete with final baseline,
source/document, and independent diff review. No adapter implementation has
begun. The next run should reorient before implementing ADR-0019.

## Highest risks and uncertainties

- Full host output returned as a message error means terminal telemetry-app
  failure. Draining older output does not recover the app; no retry or delivery
  guarantee is selected. Physical I/O and caller framing need separate budgets.
- Large inline configuration capacities can exhaust stack resources; validator
  effects and caller copies are outside retained snapshot bounds.
- Application IDs, revisions, and instants carry no owner origin. Callbacks and
  validators need not terminate; returned failure can follow partial mutation
  or publication. Panic/hang containment and general fault tolerance are absent.
- Scheduling has no recurrence, fairness, or deadline guarantee. Failure-event
  delivery can saturate, and the runtime does not permanently own clock/events.
- No CI currently executes the local baseline.

## Important unresolved decisions

- The adapter implementation must select a small shared-source example/test
  arrangement. The local grammar and pre-v0.1 APIs remain unfrozen.
- Message/lifecycle configuration access, application-authored events, host
  event drain, persistent clock/event owners, and messaging-aware scheduling
  remain outside the implemented boundary.
- Physical input/output, protocol evolution, MSRV, and any hardware, RTOS, or
  `no_std` commitment remain open.

## Most likely next tasks

1. Implement the ADR-0019 mission-local codec and adapter pair with exact
   malformed-input, publication, output-saturation, and end-to-end evidence.
2. Compose a small sample mission demonstrating the existing service boundaries.
3. Establish CI and perform the v0.1 architecture review without widening claims.

## Latest run

2026-09-08: Selected the host boundary decision because it resolves grammar and
output ownership before the next Stage 3 implementation. Compared fixed byte
records with strict ASCII and borrowed mailbox ownership with other sinks.
The real-runtime probe validates borrowing and the full-output consequence;
RFF-REQ-007 remains unverified. Initial/final baselines, explicit experiment,
source-form, document, and independent complete-diff reviews pass. This is one
local decision checkpoint on `codex/nightly`; nothing was pushed.
