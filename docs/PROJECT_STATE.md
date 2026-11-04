# Project state

Last updated: **2026-11-05**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-014/ADR-0039 join
validated ingress, correlated replies, local command feedback and subscriber
output in a standalone runnable public-only example. Local and independent
binary verification, complete source/document/diff review and exact implementation
hosted verification pass after ordinary main publication. Reviewed host v0.1
remains the regression baseline. Package 0.0.0 stays unpublished; APIs remain provisional.

## Verified baseline

- Clean input main faab18be2fb06a4bec43f0889338409c5b71b26a matches fetched
  origin/main. Initial locked format/check/Clippy/243 workspace tests,
  warnings-denied rustdoc and working/committed whitespace checks pass.
- Independent verification_support accepts exact input run 41642653583/job
  125939797733: all 20 steps, 243 workspace/14 adapter/five sample tests,
  rustdoc/whitespace and unchanged 222-line sample versus run 41451380499.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner
  2.338.2 and windows-2025-vs2026 image 20261030.294.1 are separate evidence.
- Final locked baseline passes 253 workspace tests and warnings-denied rustdoc;
  ten focused tests and the 24-line runnable session pass.
- Identical independent binary check/Clippy/ten tests/run pass. Metadata,
  exact source bytes/SHA256 and runnable stdout equality are verified.
- Root and independent consumer_assessment accept the complete new source and
  tests without findings. Both historical callback/correlated test sources
  remain unchanged. See [the focused record](verification/COMMAND_SESSION_REVIEW.md).
- Published implementation 3d171dc22d9c2661baf4d077871ebc60cad83003 passes exact
  run 41855764215/job 126663830711: all 20 steps, 253 workspace/14 adapter/five
  sample tests, rustdoc/whitespace and unchanged 222-line sample. Hosted tools,
  runner and image match input; root/independent raw-log review accepts it.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded routing,
manual time, finite schedules, events and optional validated configuration.
Existing work owners, periodic releases and callback publication retain their
contracts. The command-session consumer uses host-owned borrowed Cell slots,
one producer with known fixed subscribers, one pending operation/two flags and
one finite session issuer. Explicit host turns gate producer/subscriber dispatch
on occupied result/output slots. No framework service/API, dependency, workflow
or source-quality policy change is selected.

## Work in progress

Implementation and local/independent package checks pass. Root and independent
verification_support accept the complete source/document diff without findings.
Whole-tree source-form/link/test-reference and all 12 changed rendered-document
content/structure reviews pass. Implementation is published and exact hosted
acceptance passes; no unfinished source work remains. Record-only completion
receives separate review/publication/hosted inspection in automation memory/report.

## Highest risks and uncertainties

- Serial host policy requires explicit draining. Bypassing a full slot gate
  causes callback failure; accepted peer effects remain independent.
- Results describe local handling and enqueue snapshots; subscriber output is a
  separate observation. Taken output is not restored after writer failure.
- Session issuer reset/bypass can alias IDs. No persistence, authentication,
  duplicate suppression, reliable transport or durable/physical delivery.
- Framework identities/clocks remain caller-paired. Logical limits do not bound
  process memory or callback time; panics/hangs/process failures are uncontained.
  Floating hosted tools/images require exact-revision evidence.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV and external I/O protocols remain undecided.
Codex review is not human acceptance or qualification.

## Most likely next task

Assess concrete API usability or integration friction exposed by the runnable
command session, particularly host setup and error handling. A general runner,
result transport or persistence requires demonstrated need. Preserve one bounded
increment and the reviewed host regression baseline.

## Latest run

2026-11-05: initial/final 243/253-test locked baselines, ten focused and identical
independent binary tests/run pass. Source/output/metadata identities and complete
root/independent source/document/diff review and exact implementation hosted
verification pass. Record-only completion receives separate review/publication/CI;
stop after this one increment.
