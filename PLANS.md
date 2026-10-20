# Independently implemented application composition

Date: **2026-10-21**
Status: **Complete: consumer verified, reviewed, published and exact CI passed**

## Objective and context

Demonstrate two independently implemented applications through existing public
APIs without extracting private host-sample internals. Clean input
f539755e50b0f15aaa4361cba2e2eae4e7b3dca9 matches refreshed origin and completed
hosted run 39258514095. Fresh locked format/check/Clippy/test and warnings-denied
rustdoc pass 154 workspace tests; both whitespace checks pass. Toolchain remains
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.

Root Codex and independent consumer_assessment find that Runtime documents enum
composition, but the private sample implements framework traits only on its
wrapper. The quickstart demonstrates one concrete type. RFF-LIB-REQ-002 revision
1 establishes acceptance before source implementation. ADR-0027 selects explicit
mission composition over a public sum adapter or erased application boundary.

## Acceptance and components

- Add `examples/composition.rs`: two types independently implementing the public
  traits, mission enum delegation, concrete message-error mapping, bounded
  command-to-peer-output flow and explicit caller orchestration.
- Include focused regressions for delegation, distinct error source chains,
  selected failure/cleanup and retained peer progress; test-enable the example.
- Provide a composition guide and actually check/lint/test/run identical source
  in an external Cargo package; inspect its source hash and metadata.
- Update requirements, traceability, roadmap, entry points, source register and
  project state with truthful evidence and autonomous review attribution.

## Verification and review

Run the complete locked baseline with RUSTDOCFLAGS=-D warnings, focused example
tests/run, external package check/Clippy/test/run and metadata/source comparison.
Inspect changed rendered Markdown and generated crate documentation, resolving
relative links, whole-tree handwritten Rust widths and lint expectations, full
source and complete diff. Obtain independent Codex review. Existing quickstart,
combined host adapters/sample, workflow and ADR-0018/0019 probes are unchanged;
their conditional additional commands do not apply. Existing CI retains its
host checks and executes the new example's tests through the workspace command.

## Risks and stopping point

This consumer fixes one mission topology and shared topic/payload vocabulary.
Forwarding boilerplate is explicit integration cost, not proof of a generic
adapter need. Application failures remain terminal and callback effects are not
rolled back. No new library API, dependency, platform or consequential gate is
selected. Preserve all previous contracts and exact-domain evidence.

Rollback only this run's source/manifest/document edits if needed. Stop after
one coherent verified and autonomously reviewed checkpoint, ordinary fast-forward
source publication and inspection of completed exact-revision hosted CI. Record
any remaining work explicitly; do not continue into ordinary-work service design.

## Local results

Required final locked baseline passes 158 workspace tests. Focused composition
passes four tests and quickstart three; both executables pass. Identical source
checks/lints/tests/runs in an external package whose metadata and SHA256 confirm
one independent workspace and sole framework path dependency. Root and independent
Codex composition_review accept the complete source/document/diff and raw consumer
evidence. Exact identities, commands and
limits are in [the focused record](docs/verification/COMPOSITION_REVIEW.md).

Ordinary fast-forward publishes d63d4f88d23a70cfd0fd4badf3efeba88ca3bb8d.
Hosted run 39429032917/job 118648900866 passes all 20 steps, 158 workspace tests
including all four new composition regressions, separate 14 adapter/five sample
tests, rustdoc, both whitespace checks and the unchanged 222-line sample. Hosted
tools/runner match input; the Windows image advances to 20261018.275.1. Source,
manifest and workflow are unchanged by the documentation-only evidence follow-up,
which receives its own review and exact hosted result.
