# ADR-0026: Public-API quickstart and independent consumer evidence

- Status: Accepted for this bounded host integration checkpoint
- Date: 2026-10-20
- Scope: First Stage 5 consumer recipe and verification boundary

## Context

ADR-0025 prioritizes practical library use. The README describes substantial
historical evidence but lacks a dependency/setup recipe or small application
composition. The combined host sample intentionally joins many services across
private modules. A new developer should not have to extract those modules to
use the existing public lifecycle and message APIs.

Root Codex and independent consumer_assessment inspect the current entry points,
public APIs and sample at 61e3f0e54509ab4a2f45b94648c83d66e245b233. The
existing API can support one independently defined bounded command handler.
This is a documented assessment, not a user study or external adoption report.

## Decision

Add one standard Cargo quickstart example with public APIs and `std` only. Show
one application, one capacity-one inbox, bounded payload and output, explicit
start/publication/report inspection/dispatch/stop/restart. Validate the selected
command before changing the host observation. Keep diagnostics outside callbacks.
RFF-LIB-REQ-001 records acceptance before implementation.

Set the example's `test = true` so the existing workspace test command runs its
behavioral regressions. Document how to create an independent unpublished Cargo
binary, add a local path dependency, and use the exact example as its main source.
Actually compile, lint, test and run that package and inspect its dependency graph
and source identity. The generated consumer is verification output, not another
maintained framework crate or workspace member.

The one-shot recipe reports setup errors and tears down. Ownership-preserving
registration/attachment errors remain available to integrating callers for
explicit recovery. Explain this choice and the borrowing/lifetime implications
without adding framework recovery policy.

## Alternatives

- A permanent consumer crate gives a separate Cargo boundary but adds manifest,
  lockfile and workspace maintenance. Defer until multiple consumers or automation
  needs justify that maintained boundary.
- A root example alone is easy to run but does not verify independent dependency
  setup. Pair it with actual external-package verification.
- Expand ordinary-work publication/event contexts: potentially useful for future
  sensor workflows, but this consumer does not need them. Defer until a concrete
  consumer demonstrates the missing capability and its service/error contract.
- Publish private host-sample internals: exposes mission choices as library API
  without demonstrating a reusable responsibility. Reject for this checkpoint.

## Evidence and consequences

Official Cargo documentation supports path dependencies and `test = true` on
examples; exact URLs, access date and local adoption are in the source register
under SRC-CARGO-CONSUMER-RECIPE. Implementation is independently authored and
reuses no upstream source. Existing runtime behavior and v0.1 tests remain intact.

The [getting-started guide](../GETTING_STARTED.md) and
[verification record](../verification/QUICKSTART_REVIEW.md) record runnable steps,
observed results, actual Codex reviewer provenance and limits. A controlled
independent package demonstrates this composition only. It does not establish
external adoption, MSRV, API stability, release readiness, delivery over physical
I/O, real-time behavior or operational suitability.

## Risks and revisit conditions

The simple consumer uses one application type and a borrowed in-process output
slot. It does not settle heterogeneous application composition, input transports,
configuration loaders, broader service contexts or host scheduling policy.
Revisit example scope when a second concrete consumer exposes missing behavior;
consider a maintained external verification target when repeated setup warrants
it. No additional consequential gate is exercised by this recipe.
