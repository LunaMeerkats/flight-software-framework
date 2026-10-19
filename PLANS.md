# Public-API quickstart checkpoint

Date: **2026-10-20**
Status: **Complete: consumer verified, reviewed, published and exact-revision CI passed**

## Objective and context

Enable a first downstream developer to compose one bounded command handler
without reverse-engineering the private combined mission. Clean input
61e3f0e54509ab4a2f45b94648c83d66e245b233 has successful exact-revision hosted
run 39248169214. Fresh local locked format/check/Clippy/test/rustdoc and both
whitespace checks pass 151 tests on the unchanged local 1.98.0 toolchain.

RFF-LIB-REQ-001 is established before implementation. ADR-0026 compares a
public-only example plus independent package verification, a permanent consumer
crate, and broader ordinary-work service contexts. Select the first: existing
APIs can satisfy this consumer; the adoption gap is setup and readable composition.

## Changes and acceptance

- Add one standalone `examples/quickstart.rs`, importing only the public crate
  and `std`, with bounded ingress/output and explicit lifecycle/dispatch.
- Opt its regression tests into the existing workspace test command.
- Add getting-started instructions for a separate Cargo binary with a local
  path dependency; compile, test and run that exact copied source independently.
- Explain expected errors, report-versus-delivery semantics, bounds, teardown
  policy, owner-local identifiers and scope limitations.
- Reconcile entry points, requirements, traceability, roadmap and project state;
  retain previous runtime APIs, tests, dependencies and CI workflow.

## Verification and review

Replay the required locked baseline with RUSTDOCFLAGS=-D warnings; run
`cargo test --locked --example quickstart` and
`cargo run --locked --example quickstart`. Separately build, lint, test and run
the independent consumer after generating its offline lockfile. Inspect metadata
and source hashes to confirm an independent workspace with only the library path
dependency and exactly the same consumer source. Record reproducible commands.

Review relative links, changed rendered Markdown, generated crate documentation,
all handwritten Rust widths and expectations, full source and complete diff.
Use independent Codex review for implementation, guide and evidence. Existing
host sample/adapters, workflow and ADR-0018/0019 experiments are unchanged, so
their conditional local probes are not triggered. Existing CI still exercises
its configured sample and adapter checks.

## Risks and stopping point

This is one selected consumer composition; it cannot establish general usability
or independent adoption. The demonstration may tear down on setup failure,
while an integrating host may recover ownership from the framework errors.
Keep callback failures and report saturation explicit. No new dependency, API
freeze, runtime, platform commitment or publication gate is selected.

Safe rollback is removal of this run's example/manifest/document edits only.
Stop after the coherent checkpoint passes applicable verification and reviews,
is committed and pushed by ordinary fast-forward on codex/nightly, and its exact
hosted CI is inspected. A later run chooses the next consumer-driven increment.

## Local results

Required final locked checks pass 154 workspace tests; three focused quickstart
tests and both executable forms pass. The independent package is outside this
checkout, has its own workspace and exactly one path dependency on the library.
Its main-source SHA256 matches the example byte for byte. Source/design review by
Codex root and independent quickstart_review has no blocking findings. Exact
commands, raw evidence locations, source identity and limits are in
[the focused review](docs/verification/QUICKSTART_REVIEW.md). Final rendered-document,
source-inventory and complete-diff review pass with independent Codex acceptance.
Checkpoint c772dd960fe45b8cbc440dd1101587e101d1244d is published by ordinary
fast-forward. Hosted run 39258022861/job 118078130174 passes all 20 steps,
154 workspace tests including the three quickstart regressions, separate 14
adapter/five sample tests, rustdoc, unchanged 222-line sample and both whitespace
checks. Root and independent quickstart_review inspect exact metadata/raw logs
separately from local acceptance. The evidence follow-up resolves 411 links/118
exact test references and all twelve run-changed rendered documents pass content/
structure review; source, manifest and workflow are unchanged from the checkpoint.
The documentation-only evidence follow-up receives its own review and exact CI.
