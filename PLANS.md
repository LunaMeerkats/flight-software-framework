# Recover borrowed quickstart setup

## Objective and acceptance

Verify the ownership-preserving attachment error in a downstream host whose
application borrows a local output slot. A rejected zero-capacity inbox must
report the original application ID and return its still-registered runtime.
Attach a corrected inbox, start and dispatch using that same ID, and observe
the handled command in the original slot. Preserve the executable transcript.

The getting-started guide explains how to inspect the typed cause and consume
the wrapper before retrying setup. Keep the policy local to the caller under
[ADR-0011](docs/adr/0011-runtime-owned-message-availability.md) and
[ADR-0026](docs/adr/0026-public-api-quickstart.md). The quickstart's normal
one-shot driver continues to propagate its diagnostic and exit.

## Verification and stopping point

Input `e100be6e06439e394c665466808f758de96f6eb1` matches fetched origin.
The locked local baseline passes 256 tests. Independent Codex inspection of
input hosted run 42033160214 confirms all 20 steps and the 222-line sample.

Run the focused example and full locked baseline, and build, lint, test and
run its exact source in an independent Cargo binary. Compare bytes and stdout,
inspect Cargo metadata, and review source, Markdown and the complete diff.
Record the results, publish the reviewed increment on `main`, and inspect
hosted CI for that revision.

Stop after one corrected pre-start attachment. This experiment covers a known
configuration error with an explicit correction. If verification fails,
preserve the evidence and withdraw only this run's incomplete changes.

## Result

The locked local baseline passes 257 tests. The workspace and identical
independent quickstart each pass four tests with matching stdout. Complete
source and rendered-content review passes. Exact implementation
`3a7c87a905fd33ee256c7d4f692c83d054153227` passes hosted run 42222349329, all
20 steps. See the [focused review](docs/verification/QUICKSTART_REVIEW.md#borrowed-setup-recovery-follow-up).
The record revision receives its own hosted inspection.
