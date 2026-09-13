# Stage 4 dependency and scope review

Date: **2026-09-13**
Status: **Local review checkpoint complete; publication verification pending**

## Objective and context

Record one bounded autonomous review of dependency features, licence metadata,
and user-facing scope claims. The combined sample and hosted CI already pass;
this is the next recorded Stage 4 task. It prepares evidence for human v0.1
architecture review without claiming that review or freezing the public API.

Started clean on `codex/nightly` at
`c472c57c1aa241d821b891674ba25f42b25ae723`, equal to the refreshed origin.
The initial locked Cargo baseline passes 115 tests on local Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. The existing 32-file source audit has
zero width findings and three fulfilled expectations. No policy adoption or
runtime feature is needed.

## Acceptance criteria

- Reconcile manifest, lockfile, full metadata, all-target dependency/feature
  tree, tracked build inputs, approved licence files, and CI tooling boundary.
- Inventory top-level, sample, and generated documentation entry points;
  correct concrete scope/status ambiguity while preserving requirement meaning.
- Record reviewed source revision, primary sources, commands, findings, limits,
  revisit triggers, and remaining human acceptance in one verification record.
- Pass the required baseline, source/relative-link/traceability audits,
  rendered-document inspection, and independent complete-diff review.
- Commit the coherent checkpoint and publish by ordinary fast-forward, then
  inspect the exact remote revision and completed hosted CI results.

## Components and approach

Add `docs/verification/DEPENDENCY_SCOPE_REVIEW.md`; update its source register,
README, crate notice, the sample ADR's CI status, roadmap, project state,
traceability, and the CI baseline's immutable evidence link. Leave dependencies, licences, runtime behavior, tests, workflow,
lint policy, and requirement meanings unchanged.

Use official Cargo metadata/tree documentation for graph semantics and the
existing licence decision with current primary licence texts for content review.
Compare direct evidence review with adding dependency tooling: retain direct
review for the empty external graph; a new policy tool needs its own plan.

## Verification approach

Run the six AGENTS.md baseline commands, using the supported `--locked` Cargo
forms and `RUSTDOCFLAGS=-D warnings`. Run the executable to inspect its actual
notice and controlled report. Host source behavior and ADR-0018/0019 probes
are unchanged; focused host reruns and standalone probes are conditional.
Temporary review aids under `target/review-2026-09-13` render Markdown and
check relative links, exact traceability references, source widths, and rendered
content. Inspect rendered pages separately; these aids are not new policy gates.

## Risks and safe stopping point

An empty Cargo graph does not cover the standard library, operating system,
compiler/linker, CI runner/action internals, or local review tools. Text and
metadata inspection is not security, legal, redistribution, or operational
clearance. Human entry-point and architecture acceptance remain pending.
The safe stopping point is a verified review record and narrow documentation
corrections. Revert only this run's own changes if they cannot be verified;
preserve all prior source and generated review artifacts.

## Completed local evidence

The final locked baseline passes 115 tests, warnings-denied Clippy/rustdoc,
formatting, checking, and whitespace review. The executable notice/report,
32-file source width and expectation audit, 79 exact test references, relative
links, generated Markdown content, and browser layouts pass inspection.
Independent complete-diff review identified a historical CI-evidence link
that would follow the rolling plan; it now targets immutable commit c472c57.
No behavior, dependency, licence, workflow, test, or lint policy changed.

Review aids/logs: `target/review-2026-09-13`. Render with
`./target/review-2026-09-13/render-documents.ps1 <phase>`, then run the local
`audit-documents.py <phase>` and `review-rendered-content.py <phase>` with
Python. The second aid compares the current diff with rendered text, code,
headings, tables, and source hashes; browser inspection is separate.
These temporary aids do not introduce a new checker or dependency gate.
