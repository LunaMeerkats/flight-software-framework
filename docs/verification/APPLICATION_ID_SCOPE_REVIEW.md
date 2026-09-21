# Application identity scope review

Date: **2026-09-22**
Scope: **Existing ADR-0003 caller-scoped `ApplicationId` contract**

## Decision and inspected boundary

Retain the current opaque slot-key representation for this checkpoint and make
its issuer boundary directly observable. No production defect was found in
`src/lifecycle.rs` or `src/runtime.rs` at starting revision
`a46ad83078a52609b4a75eabcdbec68ff69d0351`. This review supplements
RFF-REQ-002 and
[ADR-0003](../adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md)
without changing their behavior.

`ApplicationId` stores only a private record index. Successful registration in
each fresh registry or runtime starts at index zero. Equality therefore compares
positions, not issuer provenance. Every owner resolves a supplied identity
against its own record vector. A same-position key from another live owner can
address the corresponding local record; only an out-of-range position produces
`LifecycleError::UnknownApplication`.

This is an explicit caller contract, not authorization to mix identities. A
caller must retain the association between an identity and the registry,
runtime, schedule, event source, or detached topology that uses it. The type is
not a persistent mission identifier, protocol field, or globally unique value.

## Executed public observations

The two regressions are:

- `equal_position_id_from_another_registry_addresses_the_local_record`: two
  fresh single-record registries issue equal keys. Supplying the second key to
  the first registry starts the first registry's record while the second remains
  `Registered`.
- `equal_position_id_from_another_runtime_invokes_the_local_application`: two
  fresh single-record owned runtimes issue equal keys. Supplying the second key
  to the first runtime invokes only the first runtime's application and changes
  only its lifecycle state.

Together they execute equality, lookup, state mutation, and owned callback
selection through public APIs. Existing out-of-range tests remain evidence for
`UnknownApplication`; they do not prove origin validation.

## Alternatives and limits

An origin-bearing key could reject cross-owner mixing, but it needs a separately
reviewed source of bounded issuer identity plus defined equality, exhaustion,
copying, debug, and persistence behavior. A generation counter alone addresses
slot reuse, not separate live owners. Caller-supplied mission identifiers would
couple this local lifecycle key to configuration or protocol namespaces. None
of those designs is introduced implicitly during this evidence review.

The tests use two live owners and their first positions. They do not establish
behavior across processes, persistence, serialization, deregistration, slot
reuse, recovery from `Failed`, or a future API freeze. Human v0.1 architecture
acceptance must decide whether the documented caller discipline is sufficient
for the current host-only target before public API stabilization.

No new external source was needed. The observations follow the accepted local
ADR and the executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 124 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. After formatting, the focused command
`cargo test --locked --test lifecycle_registry --test application_runtime`
passes six and twelve tests respectively. The final locked baseline and the
source, link, traceability, rendered-document, and complete-diff reviews pass:
126 workspace tests, 32 Rust files with zero width findings and three unchanged
fulfilled expectations, 45 Markdown files, 194 resolving relative links, and 90
exact traceability test references. Rendered structure and exact source-content
comparisons pass; pixel-level visual acceptance is not claimed.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. This checkpoint does
not establish human acceptance.

## Published checkpoint and continuation

Commit `9ea7f48f827031fcbeb63f712c97dfbe14cc629c` contains both
regressions and the reviewed checkpoint. Ordinary fast-forward publication
succeeded and the remote head matched. On 2026-09-21 UTC (2026-09-22 Sydney),
[hosted run 35658016416](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35658016416)
passed that exact push and checkout: one Windows job and all configured steps.
Logs confirm 126 workspace tests including both new regression names, 14
focused adapter tests, five focused sample tests, the sample executable, and
every configured baseline command.

Actual runner: 2.337.0; image: `windows-2025-vs2026` version
`20260907.229.1` (requested label `windows-2025`). Actual Rust/Cargo: 1.98.1;
rustfmt: 1.9.0-stable; Clippy: 0.1.98. This matches the previously reviewed
hosted environment. The all-target hosted baseline and companion whole-tree
source review preserve existing policy without a new waiver. Local Rust/Cargo
remain 1.98.0.

This documentation-only follow-up records the completed result with unchanged
Rust, Cargo, workflow, and lint inputs. It does not establish a hosted pass for
its own later revision or human v0.1 acceptance. The next bounded task should be
selected from a reconciled service or public-API contract gap; broader scope
remains unapproved.
