# Bounded plan: ordinary-work output consumer

Date: **2026-10-22**
Status: **Complete: consumer verified, reviewed, published and exact CI passed**

## Objective and context

Close the concrete consumer-evidence gap after the two message-driven examples:
produce a configured value during ordinary work, then explicitly transfer it
through a host-owned capacity-one slot to one bounded subscriber. Clean input is
0848e32821046b91048ad607a7aba267a8850fb4 on codex/nightly. Initial required locked
baseline passes 158 tests; source-quality configuration and local tools are
unchanged. Root and independent consumer_need assess the alternatives.

RFF-LIB-REQ-003 and ADR-0028 precede source implementation. Existing APIs can
express this use; immediate in-callback publication feedback is not required.

## Acceptance and files

- Public-only examples/work_output.rs with a test-enabled standard Cargo target.
- Immutable validated configuration, ordinary and one-shot scheduled work,
  explicit staging/transfer, retained retry, lifecycle and typed failure tests.
- One fixed subscriber so retry cannot duplicate an earlier fan-out delivery.
- Runnable guide and exact independent Cargo source/hash/metadata verification.
- Requirement, ADR, source register, traceability, architecture and state agree.
- Full locked baseline, width/link/rendered-document and complete diff review;
  independent Codex review before commit; ordinary push and exact hosted result.

## Verification and risks

Run AGENTS.md baseline plus focused example test/run and independent package
check/Clippy/test/run/metadata/hash inspection. Keep local and hosted evidence
separate. Audit handwritten Rust widths and existing waivers, inspect changed
rendered documents and all relative links. Old tests and requirement meanings
remain intact. No workflow or conditional experiment source changes are planned.

The pending slot is mission policy. Full-slot work error is terminal; host
publication rejection is a retryable caller outcome. Logical queue counts do
not bound process bytes or callback duration. Do not generalize to fan-out,
transactional delivery, arbitrary failures, real I/O or API stability.

## Safe stopping point

Stop after this one verified, reviewed consumer checkpoint and publication
evidence. If implementation cannot pass, remove only current-run source edits
while retaining the useful decision/plan and exact blocker. Do not begin a
broader service-context design during this run.

## Results

The implementation and independent Cargo consumer pass all checks: 168 workspace
tests, ten focused and ten external tests, identical source/expected output,
sole path dependency and independent metadata. Whole-tree width/link/test-name
and thirteen rendered Markdown inspections pass. Root and independent review
inspect actual source, errors, bounds and raw command evidence. Final staged
review accepts the checkpoint with no blocking findings. Ordinary fast-forward
publishes f354ba2ad0061ce7687855cb8f166f0c543edd08. Exact hosted run 39605633954,
job 119240975638 passes all 20 steps, 168 workspace tests, separate 14 adapter/
five sample tests, rustdoc, both whitespace checks and unchanged 222-line sample.
Hosted tools/image match input; runner changes to 2.338.0. Source, manifest and
workflow are unchanged by the documentation-only evidence follow-up, which
receives its own document/diff and exact hosted review. Stop after this checkpoint.
