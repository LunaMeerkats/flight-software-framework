# Configured-runtime construction diagnostics review

Date: **2026-09-30**
Scope: **Existing ADR-0018 construction-error source and ownership contract**

## Decision and inspected boundary

Retain the existing configured `Runtime` constructor error and ownership
recovery. Close one public diagnostic-evidence gap by executing the
`std::error::Error::source` chain for both deterministic runtime-storage
failures. No production mismatch was found in `src/runtime.rs` at starting
revision `9cad622987f47f0ab14e9922a8ab2b4e29b20f6a`. This review supplements
RFF-REQ-006 and
[ADR-0018](../adr/0018-configuration-aware-work-context.md) without changing
behavior.

`RuntimeConfigurationCreateError` preserves the unchanged validated
`ConfigurationTable`, reports the typed `RuntimeCreateError` through `kind()`,
and exposes that same error as its standard source. Typed access and standard
traversal therefore retain the configured-owner boundary and its exact storage
cause without flattening the wrapper or losing the table lineage.

## Executed public observation

The existing public regression
`configured_construction_failures_return_the_unchanged_table_lineage` now proves
the following chain for both explicit zero capacity and the deterministic
`usize::MAX` reservation-overflow case:

```text
RuntimeConfigurationCreateError
  -> RuntimeCreateError
```

For each failure, the traversed source exactly equals `kind()`. The same helper
then verifies active revision 2 and its bytes through the borrowed returned
table, consumes the wrapper to recover ownership, rolls back to revision 1,
and performs a successful replacement at revision 3. Constructor failure
therefore leaves the complete active, rollback, and revision-allocation lineage
available for explicit caller recovery.

## Alternatives and limits

Flattening `RuntimeCreateError` into the ownership wrapper would discard the
configured-constructor boundary. Adding a new conversion method is unnecessary
because `kind()`, `configuration()`, `into_configuration()`, and standard error
traversal already expose the required typed observations.

The zero-capacity case is an explicit logical rejection. The `usize::MAX` case
proves that reservation for nonzero-sized application records cannot represent
the requested capacity; it does not inject global allocator exhaustion or
measure allocations. This checkpoint does not execute unrelated constructor
wrappers, add automatic recovery, stabilize the public API, or establish human
v0.1 acceptance.

No new external source was needed. The observation follows accepted local
ADRs and executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 134 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. The focused command
`cargo test --locked --test configuration_runtime` passes 11 tests after the
assertions were added. The final locked workspace baseline passes 134 tests.
Formatting, all-target checking, warnings-denied Clippy/rustdoc, and whitespace
checks pass without a new exception.

Source, link, traceability, and generated-document audits pass: 32 Rust files
with zero physical or comment-width findings, no block comments, and three
unchanged fulfilled expectations; 53 Markdown files and 249 resolving relative
links; 35 source identifiers and 107 exact traceability test-name references
under the current audit method. PowerShell Markdown rendering for the six
changed controlling documents passes structural inspection. Pixel-level visual
acceptance is not claimed.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. This checkpoint does
not establish human v0.1 acceptance.

## Publication and continuation

The checkpoint commit, publication, and exact-revision hosted CI remain pending.
No earlier run is projected onto this revision. The next bounded task must be
selected from the repository state after this checkpoint; broader scope remains
unapproved.
