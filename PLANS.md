# Plan: apply the approved dual licence

Status: **Complete**
Date: **2026-08-26**

## Objective

Apply the human-approved `MIT OR Apache-2.0` licence using the confirmed exact
notice `Copyright 2026 Daniel Smith`, while preserving `publish = false` and
all project safety, non-affiliation, namespace, and trademark limitations.

## Context

ADR-0002 already selected recipient-choice dual licensing but deliberately
withheld the licence files and Cargo expression until the copyright holder was
confirmed. Daniel Smith has now confirmed both the exact notice and his authority
to license all current repository content.

Licensing the repository grants permissions under the selected terms. It does
not publish the crate, register the project name, clear a trademark, authorize a
push or release, or change any technical behavior.

## Acceptance criteria

- Root `LICENSE-MIT` contains the confirmed notice exactly once plus the
  canonical MIT terms.
- Root `LICENSE-APACHE` matches the unmodified Apache License 2.0 text from the
  Apache Software Foundation.
- Narrow Git attributes preserve both canonical licence files as UTF-8/LF on
  checkout without changing the repository's broader line-ending policy.
- Cargo exposes `license = "MIT OR Apache-2.0"`, no `license-file`, and the
  existing non-publication setting remains explicit.
- Cargo package inventory and the actual crate archive contain both licence
  files and the normalized manifest retains the expression and
  `publish = false`.
- README, AGENTS, the charter, ADR-0002, source provenance, and project state
  describe the applied licence without implying publication, trademark
  clearance, NASA affiliation, certification, or compatibility.
- No unnecessary NOTICE file, mass source headers, dependency, toolchain change,
  technical API change, release, push, or publication is added.
- The complete documented baseline and repository document audits pass.

## Files and components

- `LICENSE-MIT`, `LICENSE-APACHE`, and `.gitattributes`: canonical
  recipient-choice licence terms with reproducible LF checkout bytes.
- `Cargo.toml`: SPDX licence metadata while retaining `publish = false`.
- `AGENTS.md`, `README.md`, and `docs/CHARTER.md`: contributor and
  user-facing applied-licence status.
- `docs/adr/0002-project-name-and-licensing-intent.md`: confirmed holder,
  exact notice, completed application, and retained restrictions.
- `docs/research/SOURCES.md`: rechecked primary-source provenance and applied
  local treatment.
- `docs/PROJECT_STATE.md`: verified licence state, resolved blocker, and latest
  run.
- `PLANS.md`: this bounded plan and final verification result.

## Verification approach

- Compare `LICENSE-APACHE` to the current canonical ASF text and
  `LICENSE-MIT` to the OSI terms after substituting the confirmed notice.
- Assert the exact MIT notice occurs once.
- Inspect `cargo metadata --no-deps --format-version 1`.
- Run `cargo package --allow-dirty --locked --list` and
  `cargo package --allow-dirty --locked`; inspect the resulting archive.
- Run the repository's full format, check, Clippy, test, warnings-denied rustdoc,
  and Git whitespace baseline.
- Re-audit handwritten Rust widths and expectations even though Rust source is
  unchanged.
- Verify relative Markdown links, changed rendered documents, source identifiers,
  tables, headings, and stale pending-licence wording.
- Review the complete diff and confirm no unrelated file changed.

## Risks and safe stopping point

The principal risks are altering canonical legal text, changing the confirmed
notice, conflating licensing with publication, or overstating namespace and
trademark clearance. Exact source comparison, Cargo metadata/package inspection,
and durable-document review address those risks.

Stop after the approved terms, metadata, documentation, and verification are
coherent. Do not publish, push, create a release, add NOTICE or per-file headers,
or proceed into structured events or any other technical objective.

## Result

The confirmed notice and approved dual licence are applied. `LICENSE-MIT` is
1,052 UTF-8/LF bytes with SHA-256
`ead0c047b719d7911306382a6a203a2bc0a7261f73f4fbc22b1fbbcca6bf328e`;
`LICENSE-APACHE` is 11,357 bytes with SHA-256
`c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`.
The exact MIT notice occurs once, and narrow Git attributes preserve both files
with LF endings across checkouts.

Cargo metadata reports `MIT OR Apache-2.0`, no `license-file`, and an empty
publication allow-list representing the retained `publish = false`. Both files
appear in the 44-entry predicted package and actual archive; the normalized
manifest retains the expression and disabled publication, and the packaged
crate verifies successfully. The complete 38-test Rust baseline, warnings-denied
Clippy and rustdoc, document links and rendering, provenance identifiers, source
form, and whitespace checks pass. No Rust source, dependency, NOTICE, per-file
header, package name, namespace claim, trademark claim, publication setting,
release, push, or unrelated technical behavior changed.
