# Bounded plan: host-owned periodic work releases

Date: **2026-10-25** (run started before local midnight)
Status: **Complete: verified, reviewed, published and checkpoint CI passed**

## Objective and context

Enable recurring ordinary-work producers without reconstructing finite agendas.
Input is clean codex/nightly at 86d82c9b336ba013f7e460fc309a6fb8fcfe08b6,
matching refreshed origin. Initial locked baseline passes all 176 workspace
tests, format/check/Clippy, warnings-denied rustdoc and both whitespace checks.
Exact input hosted run 39959863270 has 20 successful steps. Local Rust/Cargo
1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.

Root Codex and independent repeated_work_assessment compare explicit host
arithmetic, reconstructed one-shot agendas and periodic execution wrappers.
RFF-LIB-REQ-005 and ADR-0030 select one allocation-free release cursor, with
observed-time re-arming and explicit exhaustion, before implementation.

## Acceptance and files

- src/scheduling/periodic.rs owns positive-interval validation and one-item
  polling; scheduling.rs/lib.rs expose it without changing one-shot behavior.
- tests/periodic_work.rs exercises boundary arithmetic, clock reads, late polls,
  repeatable replay and public-only direct/messaging work delegation.
- A bounded configured producer records selected identity/output; host drains
  between releases. Typed work failure clears only its messaging inbox and does
  not prevent healthy peer delivery/work. Lifecycle rejection cannot retry the
  same release. Host timer disposal is explicit policy.
- The same test source passes check/Clippy/test as an independent Cargo package;
  metadata and hashes confirm the sole framework path dependency and exact bytes.
- docs/PERIODIC_WORK.md explains setup, release versus execution, errors, timing,
  resources and disposal; requirements/ADR/research/traceability/state agree.
- Full baseline, source widths/waivers, links, rendered Markdown/generated API
  review and complete independent Codex source/document/diff review pass.

## Verification and risks

Run AGENTS.md locked baseline plus cargo test --locked --test periodic_work and
its independent-package recipe. Old one-shot tests remain baseline regressions.
No workflow, host sample, existing consumer guide or borrowing probe changes
are planned. Conditional focused commands remain conditional on actual changes.

Release is consumed before host work and does not prove callback execution.
No fixed phase, catch-up backlog, automatic retry, fairness, synchronization,
deadline/real-time guarantee or origin authentication is introduced. Overflow
releases the final valid item once, then exposes exhaustion.

## Safe stopping point

Stop after this one coherent verified increment, autonomous review, ordinary
fast-forward source push and exact hosted inspection. If checks cannot pass,
remove only this run's implementation while retaining useful decisions and the
exact blocker. Do not start unrelated service or scheduling work.

## Results

The cursor and ten public-only consumer tests are implemented. Full locked
baseline passes 186 tests, focused and independent tests pass ten each, and the
independent package check/Clippy/metadata/source-hash inspection passes. Whole-tree
source widths/waivers, links and rendered-document content/structure review pass.
Root Codex and independent repeated_work_assessment accept the complete 16-file
source/document/diff and actual evidence with no remaining findings. The
capacity-two inbox documentation correction is incorporated. Ordinary
fast-forward publication of 53ac9cb7a41af36a6f123be884399562db2ee245 passes exact
hosted run 40097187490/job 120881785371: all 20 steps, 186 workspace tests,
separate 14 adapter/five sample tests, rustdoc, both whitespace checks and the
unchanged 222-line sample. Hosted tools/runner/image match input. This
six-document evidence follow-up changes no source/configuration and receives
its own complete review and exact hosted inspection. No further capability is
started; stop after this completed periodic-release checkpoint.

Completion resumed **2026-10-26** at the published implementation with these six
evidence documents preserved. A fresh locked baseline passes 186 tests; focused
and identical external-consumer checks pass ten each. Root Codex and independent
periodic_review recheck source identity, evidence, documents and the complete
follow-up diff before its authorized publication and exact hosted inspection.
