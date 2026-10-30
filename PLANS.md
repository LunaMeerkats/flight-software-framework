# Continue verified development on main

Status: Local verification passed; publication pending — 2026-10-31

## Objective and authority

The user directed: "Please don't use the nightly/codex branch anymore. Use main".
Use main for development, local commits and ordinary verified source pushes.
This supersedes earlier branch-specific instructions while retaining all other
verification/review, preservation and consequential authorization gates.

## Bounded change and acceptance

- Fetch origin and confirm clean checkout/ancestry. Fast-forward main to the
  latest verified 74b1b2398accb1435416f89bd38b4c1dfee762ce without losing history.
- Update active AGENTS/README guidance and add dated ADR-0023/0024 amendments;
  preserve historical branch/revision evidence. Change only the CI push filter.
- Update the existing automation through its tool, retaining its schedule,
  model, effort, notification policy, project, working directory and status.
- Run reviewed actionlint 1.7.12, inspect unchanged PowerShell run bodies and
  failure propagation, run the locked baseline, source/doc/link/diff checks.
- Commit on main after review; refresh origin and publish by ordinary
  fast-forward push. Inspect exact main-head CI and confirm clean remote match.

No library behavior, dependency, example or consumer-guidance change is selected.
Keep both existing branch histories; do not force-push or delete branches.
Stop after the requested branch-policy transition is verified.

## Actual local evidence

Clean main and origin/main were both 81ca906e97c28db53ca601d34c1b2d1b2e7926fb.
Fetched origin/main is an ancestor of the verified input; main fast-forwards
40 commits to 74b1b2398accb1435416f89bd38b4c1dfee762ce. No history is rewritten.
GitHub's default branch was already main; no repository setting changes.

The automation tool update changes only prompt/updated_at in the saved record;
all other fields are identical. Main direction supersedes the earlier branch
restriction. Locked format/check/Clippy/test/warnings-denied rustdoc pass with
217 workspace tests. Reviewed actionlint 1.7.12 passes with the retained archive
checksum verified. All 16 nonempty PowerShell run bodies parse and remain one
unchanged native command each; existing failure propagation is preserved.
Whole-tree width/waiver audit, links/traceability and changed rendered documents
pass; source and consumers are unchanged. Root and independent main_branch_audit
Codex review accept the complete eight-file diff with no findings. Exact hosted
main-head publication verification remains before completion. Raw aids/logs:
target/review-2026-10-31-main-branch. No new product behavior is selected.
