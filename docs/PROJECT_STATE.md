# Project state

Last updated: **2026-08-05**

## Current milestone

Stage 1 is in progress: one bounded application-lifecycle vertical slice. The
logical registry portion is implemented; an application execution boundary is
not.

## Verified baseline

- Commit `35b1a4f` is the verified documentation-only architecture checkpoint.
- The current Rust slice has successfully run formatting, build, lint, six
  tests, and documentation generation on rustc/cargo 1.96.1. Its exact evidence
  commit still needs to be recorded in traceability.
- Relative Markdown links, requirement identifiers, and referenced source
  identifiers resolve in the current working slice.

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0004 defines one bounded inbox per application with non-blocking
  reject-newest partial fan-out; it is not implemented.
- ADR-0005 defines immutable configuration snapshots, monotonic high-water
  revision allocation, and consume-once rollback; it is not implemented.

The implementation is one unpublished, dependency-free library containing a
finite-capacity `LifecycleRegistry`. It owns logical records only and has no
threads, executor, application callback, message bus, or configuration service.

## Work in progress

Tie the successful lifecycle checks to a committed evidence revision, then stop
this increment.

## Highest risks and uncertainties

- The future application execution boundary must preserve original returned
  errors while committing the affected record to terminal `Failed`.
- IDs are valid only with their origin registry, but the current opaque index
  cannot detect a numerically aliased ID from another registry.
- Queue-slot limits will not constitute memory bounds until message payloads
  receive a separate enforced bound.
- The public registry API is intentionally early and may need compatible
  extension once it owns real application objects.

## Important unresolved decisions

- The exact copyright-holder text is required before adding the approved MIT
  and Apache-2.0 licence files and Cargo licence expression.
- The minimal owned application interface and returned-error representation are
  not selected.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.

## Most likely next tasks

1. Add the smallest synchronous application execution boundary needed to prove
   `Registered -> Running` and terminal `Failed` behavior on returned start
   errors, without a factory, thread, or async runtime.
2. Extend that boundary through work, stop, and restart only as each operation
   can be tested without speculative services.
3. Apply the approved dual licence after the holder text is supplied.

## Latest run

2026-08-05: Recorded the approved project, lifecycle, inbox, and configuration
decisions and implemented the bounded logical LC1 registry. Final evidence and
repository-state recording are in progress.
