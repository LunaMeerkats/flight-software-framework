# Project state

Last updated: **2026-08-23**

## Current milestone

Stage 1 is complete: the bounded logical LC1 model and owned synchronous
start/work/stop/in-place-restart boundary are implemented. The first
source-quality policy checkpoint is also complete. Stage 2 bounded interaction
policies have recorded decisions, but implementation has not started.

## Verified baseline

- Implementation commit `43ef56b92da18302eee0c0a0a1f071a29aae0ade`
  contains the caller-driven work boundary and complete RFF-REQ-002 integration
  evidence.
- Source-quality commit `c8f2f7af9232aa370a3ea9ded211eac1581dd375`
  adds tracked stable rustfmt and Clippy configuration, denies
  `clippy::too_many_lines` at 60 for all targets, reflows the eight initial
  comment-width findings, and records five narrow reasoned test expectations.
- Formatting, all-target checking, warnings-denied Clippy, all 15 tests, and
  warnings-denied all-feature documentation generation pass with rustc/cargo
  1.96.1, rustfmt 1.9.0-stable, and Clippy 0.1.96.
- No handwritten Rust physical line exceeds 100 columns and no comment-only Rust
  line exceeds the 80-column review default.
- All 37 relative links resolve across 20 Markdown files; all eight requirement
  identifiers match traceability; all eight referenced source identifiers are
  defined in the 21-entry register; both Markdown tables have consistent row
  shapes; and all 20 documents render structurally with one top-level heading.

The tool versions record this baseline but do not set a minimum supported Rust
version or a repository toolchain pin. The detailed initial audit is in
[the source-quality baseline](verification/SOURCE_QUALITY_BASELINE.md).

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 through ADR-0009 select finite-capacity static application ownership
  and distinct synchronous start, work, stop, and in-place restart callbacks
  with concrete returned errors.
- ADR-0004 defines bounded application inbox behavior, and ADR-0005 defines
  configuration revision/rollback behavior; neither service is implemented.

The unpublished, dependency-free library retains a standalone
`LifecycleRegistry` for logical transition evidence. `Runtime<A>` owns bounded
application records and invokes one caller-selected application callback only
after identity and lifecycle validation. Lifecycle success commits `Running`,
`Stopped`, or `Running`; work success retains `Running`. Any returned concrete
operation error is preserved in an operation-specific caller result while only
the selected record enters terminal `Failed`.

The runtime creates no threads or executor. Work has no automatic dispatch,
service context, schedule, fairness rule, message bus, clock, events, or
configuration access. Public integration tests run two independently defined
applications through registration, start, work, stop, restart, and work, so
RFF-REQ-002 is verified. RFF-REQ-008 remains partial because no structured
failure event exists.

## Source-quality policy

Stable rustfmt owns normal formatting at 100 columns. Clippy enforces a
normally-60-line function threshold for every target. The five current
expectations preserve complete chronological state/error traces and have
item-level reasons; production functions need no exception.

Comment prose, exceptional physical lines, progressive source ordering, module
cohesion, abstraction level, and naming remain review responsibilities. A
strict physical-line checker, additional selected lints, validated complexity
metric, dependency-policy tool, CI workflow, and toolchain pin are deliberately
deferred to separate measured increments. Clippy's cognitive-complexity lint is
not accepted as evidence of cognitive or cyclomatic complexity.

## Work in progress

No implementation work is in progress. The source-quality checkpoint reached
its intended stopping point before Stage 2 feature growth.

## Highest risks and uncertainties

- The pre-v0.1 context-free work callback may need a recorded revision when the
  first message, time, event, or configuration service crosses the application
  boundary.
- A returned work, stop, or restart error may follow partial application-
  internal mutation. The runtime records `Failed` but proves no rollback,
  cleanup, reinitialisation, or isolation.
- Four operation-specific error wrappers duplicate a small amount of display
  and source plumbing; current evidence favors clarity over a generic marker.
- Static mission composition sizes each runtime record to its concrete
  representation's largest variant; record capacity does not bound allocation
  inside application or error values.
- IDs are valid only with their origin registry/runtime, but the opaque index
  cannot detect a numerically aliased ID from another issuer.
- One blocking or non-returning callback prevents caller progress. Returned
  application errors do not contain panics, hangs, process failure, memory
  exhaustion, or hardware faults.
- Queue-slot limits will not constitute memory bounds until message payloads
  receive a separate enforced bound.
- The function-size gate can detect lines, not semantic complexity. Reviewers
  must reject count-gaming and reassess each reasoned expectation when its test
  changes.
- No CI currently executes the local baseline.

## Important unresolved decisions

- The exact copyright-holder text is required before adding the approved MIT
  and Apache-2.0 licence files and Cargo licence expression.
- The first bounded message representation, payload limit, and service-context
  borrowing shape are not selected.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.
- Additional source lints, a physical-line checker, a complexity metric,
  dependency-policy tooling, and toolchain pinning remain unselected.

## Most likely next tasks

1. Reorient from ADR-0004 and implement the smallest coherent bounded inbox and
   publish/subscribe behavior with exact capacity, reject-newest saturation,
   and publisher-visible results.
2. Introduce a work service context only when that inbox slice has a concrete
   message-delivery need, keeping borrowing and ownership explicit.
3. Follow bounded messaging with structured finite event delivery, then inject
   simulated time before scheduling work.

## Latest run

2026-08-23: Audited the complete Rust tree and policy configuration before
unrelated feature growth. Added stable `rustfmt.toml` and `clippy.toml`,
activated the individual 60-line Clippy gate, reflowed all eight comment-width
findings, and retained five low-complexity chronological integration tests under
narrow reasoned expectations. Expanded contributor guidance, exact public-source
provenance, tool-version evidence, release gates, roadmap state, and the
warnings-denied baseline. No runtime behavior, dependency, crate, thread,
executor, checker, CI workflow, licence, remote, or push was added.
