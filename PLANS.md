# Host CI baseline

Follow-up status: **Source published; first hosted CI run passed**

On 2026-09-12, after the completed local checkpoint below, the user approved
publication without human review. AGENTS.md and the existing nightly automation
now authorize ordinary fast-forward source pushes to the existing
`origin/codex/nightly`, including the 31 accumulated verified commits. This
supersedes this plan's historical publication hold, without claiming human
code review or waiving automated checks or separate release gates.

Acceptance for this follow-up: refresh remote ancestry, review the policy diff
and outgoing history, pass local baseline and document/source review, commit
and push without rewriting history, then record the exact hosted CI result.

The follow-up passed the full local baseline (115 tests), outgoing-history
whitespace checks, source/document audits, rendered inspection, and independent
policy-diff review. The automation prompt was updated and read back exactly;
schedule, model, reasoning, target, and active state were preserved.
Policy commit `abb1293790136128d5f27d8c48c1e3d98a355540` and the 31 prior
commits were published in one ordinary fast-forward push; remote HEAD matched.

[Hosted run 34697541024](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/34697541024)
then passed one job and all 20 steps at that exact commit, with 115 workspace,
14 adapter, and five sample tests. The CI baseline records the actual Windows
image and Rust 1.98.1 toolchain. No source or workflow repair was needed.
This evidence-only follow-up records that result; any later commit and CI run
require separate verification. No human code review is claimed, and no per-push approval
is required within the recorded source-publication scope.

Follow-up review aids are under `target/publication-2026-09-12`. The
`render-documents.ps1`, `audit-documents.py`, and `review-rendered-content.py`
commands there pass with `prepush` and then `final`; the content comparison
uses base `d1eb16b`. Required Cargo commands pass as originally listed in
AGENTS.md, with rustdoc warnings denied.
`git log origin/codex/nightly..HEAD --check --format= -- .` passed for the
outgoing historical commits before push.
Both `git diff --check` and staged whitespace checks pass before commits.

## Initial local configuration checkpoint

Date: **2026-09-12**
Status: **Complete: local configuration checkpoint; hosted CI unverified**

## Objective and context

Encode the existing Cargo baseline and combined host sample in one GitHub
Actions workflow. Stage 3 is complete; Stage 4 explicitly requires CI evidence.
This increment prepares and locally validates that execution boundary. A
workflow file does not satisfy the hosted CI release gate.

Started clean on `codex/nightly` at
`634941453be2503371d962f5d80ec865448a730a`. The initial required baseline
passes 115 tests on rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy
0.1.98. The initial audit passes for 32 Rust files with zero width findings,
three existing reasoned expectations, 36 Markdown files, 128 relative links,
eight requirement rows, and 79 exact test references. Existing source-quality
policy is already encoded. No Rust, dependency, or lint-policy change is needed.

## Acceptance criteria

- One finite Windows 2025 job runs the existing warning-free baseline,
  both focused host targets, and the executable sample using stable Rust.
- Use ordinary pull requests, pushes to the verified `codex/nightly` default
  branch, and manual dispatch. Do not introduce publication, secrets, write
  permissions, self-hosted execution, or a runtime dependency.
- Pin the checkout action to a verified upstream commit and disable persisted
  credentials. Record tool versions and checkout identity in the job log.
- Document `--locked`, committed-tip whitespace checking, floating stable/image
  limitations, fail-fast shell semantics, and remaining manual reviews.
- Validate workflow syntax with a versioned upstream validator, execute the
  applicable command bodies locally, audit source/documents, inspect rendered
  changes, and independently review the complete diff.
- Keep hosted CI unverified until an authorized push and successful run have
  an exact checkout SHA, job result, tool versions, and run URL recorded.

## Components and verification

Add `.github/workflows/ci.yml` and ADR-0023 with a focused CI verification
record. Update contributor guidance, development instructions, source
provenance, roadmap, project state, and traceability. Preserve all requirement
meanings and existing source evidence hashes.

Run the AGENTS.md baseline, then the workflow's locked Cargo forms and focused
host commands. Check workflow structure with checksum-verified actionlint
1.7.12 and parse its PowerShell bodies. Test native-command failure propagation
and committed whitespace rejection in temporary fixtures. Temporary tooling,
logs, and document review aids stay under `target/nightly-2026-09-12-ci`.
ADR-0018/0019 and their probes are unchanged and are not separately rerun.

## Risks and safe stopping point

The hosted image, checkout action, rustup installation, GitHub event behavior,
and account/repository policies cannot be established by local command replay.
Stable Rust and the runner image can change. No toolchain pin, MSRV, platform
support promise, or v0.1 release acceptance follows from this workflow.

Stop at a coherent, locally verified configuration and decision checkpoint.
Commit locally only. Preserve unexpected changes and leave external activation
and human scope/dependency/architecture review explicitly outstanding.

## Outcome and verification

One workflow and ADR-0023 configure the existing Windows baseline and sample.
No production or test Rust changed. The initial six AGENTS.md baseline
commands pass. All locked commands listed in the CI verification record pass
locally, including 115 workspace tests, 14 adapter tests, five sample tests,
warnings-denied rustdoc, and `cargo run --locked --example host-echo`.
The three expected command/telemetry records and structured report remain.
Version/checkout logging commands and both whitespace commands also pass.
Hosted rustup installation is help/syntax-reviewed only, not run locally.

Completed workflow checks:

```text
& ./target/nightly-2026-09-12-ci/actionlint-1.7.12/actionlint.exe -color .github/workflows/ci.yml
& ./target/nightly-2026-09-12-ci/verify-ci-syntax.ps1
python target/nightly-2026-09-12-ci/verify-ci-behavior.py
```

Actionlint's verified archive has the checksum in the CI baseline; both
`-oneline` and `-color` invocations pass without findings. All 16 workflow
command bodies parse as one native command. In isolated temporary repositories,
the whitespace command rejects dirty root, ordinary, first-parent merge, and
depth-two merge commits; a clean root passes. GitHub-style PowerShell wrapper
probes preserve success/failure. A deliberate two-command failure-then-success
control masks the failure, supporting the selected one-command-per-step form.

The first probe incorrectly expected process exit 7 instead of the observed
nonzero 1. Correcting that probe and two documentation sentences required no
workflow change. No warning, check, or policy was weakened. Local probe tools
are PowerShell 7.6.5 and Git 2.54.0.windows.1. General Python installations lacked
PyYAML; the dedicated actionlint parser supplied YAML validation without adding
a Python dependency.

Independent complete-diff review found no blocking defect. All 32 Rust files
still meet width policy, with three unchanged fulfilled expectations. Audits
pass for 38 Markdown files, 142 relative links, eight requirement rows,
32 source entries, 79 exact test references, and all nine changed rendered
document content comparisons. Review aids are run with `review`, then `final`
after these outcome edits:

```text
& ./target/nightly-2026-09-12-ci/render-documents.ps1 final
python target/nightly-2026-09-12-ci/audit-documents.py final
python target/nightly-2026-09-12-ci/review-rendered-content.py final 6349414
git diff --check
```

Browser DOM/layout inspection of all nine changed pages at 1,280 pixels finds
no page or block overflow and no console warnings/errors. Opening-viewport
screenshots of the CI record and ADR were inspected successfully; full-page
screenshot coverage is not claimed. Initial browser selection timed out;
selecting the enumerated in-app browser resolved access. Final changed outcome
pages are rendered and inspected again before commit. The temporary browser
tab and local review server are closed at completion.

The completed checkpoint remains local. No hosted run, publication, runtime
behavior, requirement-status change, or human release review is claimed. The
next independent increment can review dependencies and scope claims; CI
activation waits for explicit push authorization under AGENTS.md.
