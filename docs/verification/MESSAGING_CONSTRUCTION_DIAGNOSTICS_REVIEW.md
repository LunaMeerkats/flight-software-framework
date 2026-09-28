# Messaging-construction diagnostics review

Date: **2026-09-29**
Scope: **Existing ADR-0011 attachment-error source and ownership contract**

## Decision and inspected boundary

Retain the existing `MessagingRuntime` construction error types and ownership
recovery. Close one public diagnostic-evidence gap by executing the nested
`std::error::Error::source` chain for a later-inbox capacity-overflow failure.
No production mismatch was found in `src/messaging_runtime.rs` at starting
revision `f4ffabbef7787b3bbbdede28f1e4bcf50a044dcb`. This review supplements
RFF-REQ-003, [ADR-0010](../adr/0010-bounded-message-routing-core.md), and
[ADR-0011](../adr/0011-runtime-owned-message-availability.md) without changing
behavior.

`MessagingRuntimeCreateError` preserves the unchanged owned runtime and reports
`MessagingRuntimeCreateErrorKind` as its standard source. When that kind wraps
a message-bus construction failure, its source is the exact
`MessageBusCreateError`. Typed access and standard traversal therefore retain
both owner-level context and the service-level cause without flattening either
error.

## Executed public observation

The existing public regression
`oversized_later_inbox_preserves_runtime_for_corrected_attachment` now proves
the complete chain:

```text
MessagingRuntimeCreateError
  -> MessagingRuntimeCreateErrorKind::MessageBus
  -> MessageBusCreateError::InboxStorageAllocationFailed
```

The typed and traversed values agree on the second application's identity and
the requested `usize::MAX` capacity. The same test then consumes the returned
runtime, attaches corrected capacities, observes two fresh empty inboxes,
retains the application-specific start failure, publishes to the healthy peer,
and completes peer work.

## Alternatives and limits

Flattening the message-bus error into the owner wrapper would discard the
attachment boundary, while adding new conversion methods is unnecessary
because typed accessors and standard traversal already expose both layers.
Global allocator fault injection would require a separate isolated harness and
is not needed to verify this existing deterministic capacity-overflow path.

This observation does not inject allocator exhaustion, instrument temporary
endpoint destruction, execute every constructor variant, stabilize the public
API, or establish automatic recovery. Other construction errors retain their
existing typed and source-inspected behavior.

No new external source was needed. The observation follows accepted local
ADRs and executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 134 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. The focused command
`cargo test --locked --test runtime_messaging` passes 11 tests after the
assertions were added. The final locked workspace baseline passes 134 tests.
Formatting, all-target checking, warnings-denied Clippy/rustdoc, and whitespace
checks pass without a new exception.

Source, link, traceability, and generated-document audits pass: 32 Rust files
with zero physical or comment-width findings, no block comments, and three
unchanged fulfilled expectations; 52 Markdown files and 244 resolving relative
links; 35 source definitions and 104 exact traceability test-name references
under the current audit method. PowerShell Markdown rendering for the six
changed controlling documents passes structural inspection. Pixel-level visual
acceptance is not claimed.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. This checkpoint does
not establish human v0.1 acceptance.

## Published checkpoint and continuation

Commit `70beaedd8c315baeea3e399bb6a7cbf0a8ded9a9` contains the source-chain
assertions and reviewed checkpoint. Ordinary fast-forward publication
succeeded and the remote head matched. On 2026-09-28 UTC (2026-09-29 Sydney),
[hosted run 36477080053](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/36477080053)
passed that exact push and checkout: one Windows job and every configured step.
Logs confirm the strengthened regression name, 134 workspace tests in
aggregate, 14 focused adapter tests, five focused sample tests, the sample
executable, and the warnings-denied/whitespace baseline.

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
