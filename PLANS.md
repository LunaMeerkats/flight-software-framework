# Bounded plan: runtime-issued ordinary-work identity

Date: **2026-10-24** (continuing the unfinished 2026-10-23 item)
Status: **Complete: verified, reviewed, published and exact checkpoint CI passed**

## Objective and context

Expose the selected runtime-local identity through ApplicationWorkContext so
ordinary-work diagnostic producers can attribute their own events without
post-registration identity wiring. Input HEAD a9afb328905eb2d413a868bc7e0371a06b5c6c20
matches refreshed origin/codex/nightly. Existing requirement, ADR, source and
guide edits are the prior run's unfinished work and are preserved. That run's
clean input baseline passed 168 tests; the current run repeats verification.
An initial formatting failure in the unfinished trait docs is corrected before
continuing. Local tools and policy are unchanged. Root Codex and independent
context_review reassess the gap and delegation paths.

RFF-LIB-REQ-004 and ADR-0029 precede implementation. Existing borrowed services
can already provide callback-time event feedback. Do not add another work trait,
framework queue owner or maintained example to close the narrower identity gap.

## Acceptance and files

- src/runtime.rs adds the copied selected ID to the existing private context
  constructor and a documented read-only accessor, retaining configuration.
- Public-only tests/work_identity.rs verifies two identities across direct and
  messaging-owned work, failure-event wrappers and finite scheduled paths.
- A checked mission queue borrow demonstrates immediate event acceptance before
  its bounded effect, explicit busy/full errors, retained older records, exact
  failure attribution, selected-inbox cleanup and healthy peer progress.
- In-place restart retains identity; rejected lifecycle work invokes no callback.
- The identical tests pass in an independent Cargo package with the sole library
  path dependency; metadata and source hashes establish that boundary.
- Guide, requirements, ADR, research, traceability, architecture and state agree.
- Full locked baseline, source widths, relative links, rendered Markdown and
  complete diff review pass; independent Codex review precedes publication.

## Verification and risks

Run AGENTS.md baseline plus cargo test --locked --test work_identity. Follow the
new guide's independent-package check/Clippy/test/metadata/hash recipe. Audit
all handwritten Rust widths and existing waivers, inspect changed rendered
Markdown and generated API docs. Preserve old tests and requirement meanings.
No workflow or ADR-0018/0019 experiment source is changed.

Identity has no issuer provenance across runtimes. Caller-owned queues and clock
origins remain mission duties. RefCell is serial interior mutability with an
explicit borrow-conflict error policy, not synchronization. Recording an event
before an effect does not make them atomic. No process-memory, timing, adoption,
API stability, operational or qualification claim follows from these tests.

## Safe stopping point

Stop after this one verified API increment, autonomous review, ordinary source
push and exact hosted inspection. If verification cannot pass, remove only this
run's implementation edits while preserving useful decisions and the blocker.
Do not start broader event-capability or service-context work in this run.

## Results

All acceptance behavior is implemented. Full locked local baseline passes 176
workspace tests; focused and identical-source independent tests each pass eight.
Metadata, hashes, full-tree source widths, relative links, exact test mappings,
thirteen rendered documents and generated context API documentation pass review.
The required composition-guide refresh passes its unchanged independent recipe.
Root and independent context_review accept the complete staged source/document/
diff and actual consumer/baseline/rendered evidence with no findings. Ordinary
fast-forward publishes b3ebba331e73d2d4b46189accfc75e9ef9a6281b. Exact hosted
run 39959273438/job 120432846468 passes all 20 steps, 176 workspace tests,
separate 14 adapter/five sample tests, rustdoc, both whitespace checks and
unchanged 222-line sample. Hosted tools/runner match input; Windows image is
20261021.278.1. This documentation-only evidence follow-up preserves source,
manifests and workflow and receives its own review and exact hosted inspection.
Stop after this completed identity checkpoint.
