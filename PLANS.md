# Configuration-aware work decision checkpoint

Date: **2026-09-01**
Status: **Complete**

## Objective and context

Resolve how runtime-owned configuration reaches ordinary application work
without creating a second work path that bypasses scheduling, failure events,
or messaging lifecycle cleanup. The clean starting commit is `5b3a677`; all
required baseline checks passed with 73 tests on the unchanged Rust 1.98.0
toolchain. The source-quality policy is already encoded and audited.

ADR-0005 selects immutable runtime-owned snapshots and safe-point updates.
ADR-0017 implements only storage. Configuration now triggers ADR-0012's
second-service context revisit. This run selects a decision checkpoint before
the cross-cutting callback migration; no production Rust API changes are planned.

## Acceptance criteria

- Compare a separate configured wrapper/callback, a generic configuration
  provider, and a concrete optional table with one ordinary work context.
- Record constructor ownership, absent configuration, immutable callback
  access, history retention, and the required migration of existing work paths.
- Exercise the proposed split-borrow shape with the real configuration table
  in runnable Markdown probes; inspect negative compiler diagnostics for
  attempted mutation, conflicting replacement, and escaping borrowed bytes.
- Keep prototype evidence distinct from runtime integration. RFF-REQ-006 stays
  partial, and the 73 production tests retain their existing scope.
- Rerun the full Cargo baseline, Markdown link/reference and rendered-content
  review, source widths, and complete diff review before a local commit.
- Leave a concrete, testable next implementation increment in durable docs.

## Files and verification approach

Add ADR-0018 and a focused experiment under `docs/verification/`; update the
source register, project state, roadmap, requirements and traceability, plus
README and architecture wording where needed. Document the experiment command
in AGENTS.md. No crate, dependency, runtime wrapper, lint, or checker is added.

Required commands are the six existing Cargo/Git baseline commands from
AGENTS.md, with `RUSTDOCFLAGS=-D warnings`. The experiment uses a freshly built
library and stable `rustdoc --test`; the exact reproducible commands and
diagnostic expectations belong in its document. Temporary extracted snippets
also receive rustfmt, Clippy where applicable, and physical/comment-width
review; negative cases are checked for their intended errors, not merely any
compiler failure. Ad hoc document review scripts remain under the ignored
`target/nightly-2026-09-01` directory and are not new repository gates.

## Risks and safe stopping point

The selected representation may expose generic parameters through existing
messaging owner/error types. A compiler probe cannot establish lifecycle,
scheduled-work, event, or queue-cleanup integration. The next implementation
must exercise those paths together and revisit the decision if concrete
composition becomes unwieldy. No API freeze or shared-service registry is
authorized by this checkpoint.

The stopping point is one reviewed decision with reproducible narrow evidence,
truthful partial requirement status, a passing unchanged production baseline,
and a clean local commit. If a probe cannot support a claim, narrow the claim;
do not implement speculative runtime code to rescue the proposal.

## Outcome

ADR-0018 selects a concrete optional table owned from construction and one
ordinary work context. Constructor-only ownership avoids reattachment and
revision reset, including the valid zero-byte default-type case. Direct,
scheduled, failure-event, and messaging-owned work must migrate through the
same callback in the next implementation. Message and lifecycle access remain
deferred. No production behavior changed and RFF-REQ-006 remains partial.

Official Rust field-borrowing and rustdoc test/command documentation informed
two source-register entries. The reproducible experiment passed one executable
probe and three intended compiler rejections. Independent diagnostic review
confirmed E0594, E0502, and the expected lifetime escape error. The positive
snippet compiled and ran directly and passed warnings-denied Clippy with the
existing function threshold. All four snippets pass rustfmt and width review.

The following commands completed successfully before the local commit:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo build --workspace --all-features
git diff --check
```

Rustdoc ran with `RUSTDOCFLAGS=-D warnings`; all 73 production tests passed.
The separate `rustdoc --test` command is recorded in the experiment and passed
all four probes. It is not included in the production test count.

Ad hoc document commands also passed:

```text
./target/nightly-2026-09-01/render-documents.ps1 -Phase final-review
python ./target/nightly-2026-09-01/audit-documents.py final-review
python ./target/nightly-2026-09-01/review-rendered-content.py final-review
```

Results: 30 Markdown documents, 87 resolving relative links, 8 matched
requirements/traceability rows, 23 source identifiers, and 54 exact production
test references. All 21 Rust files meet width limits; the three existing
reasoned function expectations are unchanged. All 11 changed documents pass
generated-HTML source-hash, normalized-text, heading, list, code, and table
review. Independent complete-diff/design review found no remaining issue.

Browser setup returned `js execution timed out; kernel reset` before any page
inspection. The narrow adaptation is generated-HTML structure/content review;
visual QA is not claimed. No alternate browser-control mechanism was used.
This is an environment limitation, not a reason to weaken source checks.

The stopping point is reached: reviewed decision, reproducible scoped evidence,
unchanged passing production baseline, and a concrete next integration plan.
Only a local commit is authorized; no push or publication is performed.
