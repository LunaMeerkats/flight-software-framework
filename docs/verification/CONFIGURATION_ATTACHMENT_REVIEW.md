# Configured messaging-attachment recovery review

Date: **2026-09-26**
Scope: **Existing runtime ownership across failed messaging attachment**

## Decision and inspected boundary

Retain the current configured-runtime and messaging-construction ownership
contracts, and execute their composition at one failed inbox reservation. No
production mismatch was found at starting revision
`6a60f381736d2b7c7d069088d494000017ceaa75`. This review supplements
RFF-REQ-003, RFF-REQ-006, [ADR-0011](../adr/0011-runtime-owned-message-availability.md),
and [ADR-0018](../adr/0018-configuration-aware-work-context.md) without changing
behavior.

`MessagingRuntime::new` consumes a still-registered `Runtime`, validates the
inbox count and lifecycle states, and attempts to build fresh messaging
storage. Every construction error returns the original runtime. The constructor
reads lifecycle state but does not call a configuration operation or application
callback. The runtime continues to own its active configuration, rollback
snapshot, revision high-water mark, and registered application values.

The earlier [messaging construction review](MESSAGING_CONSTRUCTION_REVIEW.md)
proved returned-runtime reuse without a configuration table and explicitly left
configuration lineage at failed attachment unobserved. This checkpoint closes
only that composition gap.

## Executed public observation

`failed_messaging_attachment_preserves_configuration_lineage` creates a table
at revision 1, replaces it with revision 2, moves it into a one-application
runtime, and requests an inbox with `usize::MAX` slots. Attachment returns the
exact `InboxStorageAllocationFailed` application identity and requested count.
The returned runtime still reports the application as `Registered`.

After corrected capacity-one attachment and start, ordinary work observes
revision 2 and its bytes. Explicit rollback then restores revision 1 once, and
the next accepted replacement receives revision 3. Later work observes each
state in order. This establishes preservation of active content, consume-once
history, and the never-reused revision high-water mark across this failed
attachment path.

## Limits

The test uses a deterministic capacity overflow for a nonzero-sized message;
it does not exhaust available memory or inject an allocator failure. It does not
instrument temporary endpoint destruction, prove secure erasure, or execute
every constructor error. It adds no recovery policy: the caller explicitly
consumes the returned runtime, supplies corrected topology, and continues.

No concurrency, panic or hang containment, post-start topology mutation,
message/lifecycle callback configuration access, or human API acceptance is
established. No external source was needed; the observation follows accepted
local decisions and public interfaces.

## Verification

The initial locked baseline passes 131 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. After formatting,
`cargo test --locked --test configuration_runtime` passes all eleven focused
tests including the new regression. The final locked baseline is recorded
below; publication evidence remains pending.

The final locked workspace baseline passes 132 tests. Formatting, all-target
checking, warnings-denied Clippy/rustdoc, and whitespace checks pass without a
new lint exception. Host adapters, the combined sample, CI workflow, and
ADR-0018/0019 experiments are unchanged, so their separate local commands are
not triggered; the workspace suite still exercises both host test targets.

The whole-tree audit covers 32 Rust files with zero physical/comment width
findings, no block comments, and three unchanged fulfilled expectations. It
also covers 49 Markdown files, 222 resolving relative links, 35 source
definitions, and 96 exact traceability test references. Generated HTML for all
documents passes structural and source-content comparison. The six changed
documents preserve exact content, and their inspected 1280-pixel browser views
have no visible layout defect.

## Publication and continuation

Commit `b21b59bff68fe6c16b0d153661865a67a28f8df2` contains the regression
and reviewed checkpoint. Ordinary fast-forward publication succeeded, and the
remote head matched. On 2026-09-25 UTC (2026-09-26 Sydney),
[hosted run 36184310315](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/36184310315)
passed that exact push and checkout: one Windows job and every configured step.
Logs confirm the new regression, 132 workspace tests in aggregate, 14 focused
adapter tests, five focused sample tests, the sample executable, and the
warnings-denied/whitespace baseline.

Actual runner: 2.337.0; image: `windows-2025-vs2026` version
`20260922.246.2` (requested label `windows-2025`). Actual Rust/Cargo: 1.98.1;
rustfmt: 1.9.0-stable; Clippy: 0.1.98. The all-target hosted baseline and
companion whole-tree source review preserve existing policy without a new
waiver. Local Rust/Cargo remain 1.98.0.

This documentation-only follow-up records the completed result with unchanged
Rust, Cargo, workflow, and lint inputs. It does not establish a hosted pass for
its own later revision or human v0.1 acceptance. The next bounded task should
be selected from a reconciled service, resource, failure, or public-API gap;
broader scope remains unapproved.
