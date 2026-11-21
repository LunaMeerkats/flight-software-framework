# ADR-0043: Shared inspection of owned applications

- Status: Accepted; locally verified and reviewed by Codex
- Date: 2026-11-22
- Scope: RFF-LIB-REQ-018 revision 1

## Context

Successful registration transfers application ownership into Runtime.
Neither it nor MessagingRuntime currently exposes the retained value.
The quickstart, periodic peer and progress consumers export observations
through borrowed Cell slots. Those policies remain useful for host-owned
effects, but a host cannot simply read an ordinary application-owned
counter or diagnostic field between callbacks. Inspection after terminal
failure is also useful without attempting recovery.

## Decision

Add `application(&self, ApplicationId) -> Result<&A, LifecycleError>` to
both owners. Runtime reuses its record lookup; MessagingRuntime delegates
without exposing its inner owner. Permit all known lifecycle states,
including Failed, and reuse UnknownApplication for an unrepresented slot.
Lookup invokes no callback, mutates no lifecycle state and touches no inbox.
Retain runtime-local key scoping, including equal-slot aliasing across owners.

A shared borrow prevents overlapping mutable owner use while the borrow
remains live and does not expose ordinary mutable application methods.
It does permit application-defined interior mutation and effects through
shared-reference APIs. This is cooperative inspection, not a security or
arbitrary-code containment boundary. No mutable accessor or output draining
is added. Existing host-owned output and retry consumers remain unchanged.

## Alternatives considered

- Shared-reference lookup: selected; closes the owned-observation gap using
  existing identity/error types without allocation or callback dispatch.
- Closure-based inspection: adds ceremony without preventing shared-reference
  effects or interior mutation; no demonstrated benefit for this need.
- Mutable application access: can bypass lifecycle callbacks and messaging
  cleanup policy. Defer pending a separate concrete use and contract.
- Continue requiring external Cell slots: retains needless observation
  lifetimes for plain application-owned fields; preserve as an option.

## Evidence and verification

Official Rust borrowing and interior-mutability guidance informs the boundary
in `SRC-RUST-SHARED-INSPECTION` in the [source register](../research/SOURCES.md).
No upstream code is reused. The [focused record](../verification/APPLICATION_INSPECTION_REVIEW.md)
records passing public consumer, compile-fail/positive probes, independent
package and complete root/independent Codex source/document/diff review.
Publication and exact hosted evidence are recorded separately when completed.

## Consequences, risks and revisit conditions

The API remains provisional. Application representation is selected by the
mission; enum consumers must match their own variants. Shared lookup neither
drains non-Copy output nor proves cleanup or recovery. No dependency, queue,
service context or runtime policy changes. Revisit if a downstream consumer
needs owned output transfer, restricted views or controlled mutation; do not
infer that such needs justify unrestricted mutable owner access.
