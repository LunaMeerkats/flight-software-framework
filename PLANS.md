# Stage 4 messaging construction resource review

Date: **2026-09-14**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Audit the existing message-topology construction boundary and prove that an
unrepresentable inbox capacity returns its typed error without losing the
owned runtime. This is one bounded part of Stage 4's resource/failure/public-API
review; the remaining services and human v0.1 acceptance stay open.

Started clean on `codex/nightly` at
`553fb50832a2aa4948944d051575cc5fb704ea26`, equal to refreshed origin.
The initial locked Cargo baseline passes 115 tests on local Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. Existing formatting and lint policy
remain encoded; no policy adoption or runtime feature is needed.

## Acceptance criteria

- Reconcile constructor validation/reservation order, logical retained bounds,
  typed errors, ownership return, and source-inspection-only failure paths.
- Exercise `usize::MAX` inbox capacity with a small nonzero-sized message after
  a valid first endpoint, asserting exact identity and requested capacity.
- Through `MessagingRuntime`, recover both unchanged registered applications,
  attach a corrected topology, and demonstrate preserved callback behavior and
  subsequent peer delivery/work. Do not claim simulated allocator exhaustion.
- Record exact tests, reviewed input, primary-source treatment, limits, and
  remaining audit work without changing requirement meaning or public APIs.
- Pass the applicable baseline, source/relative-link/traceability audit,
  rendered-document inspection, and independent complete-diff review.
- Commit and publish by ordinary fast-forward, verify the remote head, and
  inspect completed hosted CI at the exact published revision.

## Components and verification

Change only the two messaging integration test files and supporting plan,
review, source register, traceability, roadmap, and project-state records.
Compare a deterministic capacity-overflow input with global allocator fault
injection: use the public overflow boundary; defer allocator injection because
it needs an isolated harness and independent justification.

Run `cargo test --locked --test message_bus --test runtime_messaging` and all
six AGENTS.md baseline commands with supported locked Cargo forms and
`RUSTDOCFLAGS=-D warnings`. Host adapters, workflow, and ADR-0018/0019 probes
are unchanged, so their additional local checks are not triggered.
Reuse inspected temporary render/audit aids under `target/review-2026-09-14`;
these remain local review aids, not new repository policy tools.

## Risks and safe stopping point

Logical slots and inline payload bounds do not bound whole-process memory,
caller-retained values, allocator overhead, stack use, or callback/topic effects.
Capacity overflow does not establish allocator-exhaustion behavior. Report,
endpoint, topic, and dispatch-state allocation failures still require distinct
execution evidence. The safe stopping point is two public-contract regressions
and an honest review record. Preserve prior artifacts and unexpected changes;
revert only this run's own work if it cannot meet the baseline.

## Completed local evidence

All six required baseline commands pass; the workspace has 117 passing tests.
The focused command passes eight bus and 11 owner tests. No source-policy
waiver, production behavior, dependency, or API change was needed. Initial
Clippy review found 63 counted lines in the owner test; expressing the repeated
topology as a capacity array mapped to configurations restores the 60-line
baseline without dropping assertions. A trailing document blank line was
corrected before the final whitespace pass.

The whole-tree audit covers 32 Rust files with zero physical/comment width
findings and three unchanged fulfilled expectations, 40 Markdown files with
170 resolved relative links, and 81 exact traceability test references.
All six changed rendered documents match source text/structure and were
inspected through browser DOM and 1280px layout without page overflow.
Screenshot capture timed out twice; pixel-level screenshot inspection is
unavailable, not passed. This run's rendered inspection uses DOM content and
layout measurements. Independent complete-diff review found no defect.

Logs and reused temporary aids are under `target/review-2026-09-14`.
ADR-0018/0019, separate host, and actionlint local reruns are not triggered by
this change.

## Publication result

Published `4db1a8074b014abc1c9b5cf3df5daef3cfca2b55` by ordinary fast-forward;
the remote head matched. Hosted run 34780028767 passes at that exact revision:
one job/all 20 steps, 117 workspace tests including both new regressions,
14/five focused host tests, and the sample. The
[constructor review](docs/verification/MESSAGING_CONSTRUCTION_REVIEW.md)
records its URL and actual host/toolchain. This evidence-only follow-up changes
no Rust source and does not project the recorded CI pass onto later commits.
