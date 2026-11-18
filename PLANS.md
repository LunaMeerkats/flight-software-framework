# Retain two independent queue-rejected work failures

## Objective and acceptance

Extend the runnable periodic peer consumer under RFF-LIB-REQ-017 revision 2.
Two independently registered samplers may fail while a capacity-one event
FIFO is occupied. Retain both rejected events in two inline slots, in host
observation order, and re-offer only the oldest on an explicit retry turn.
Keep the healthy heartbeat selectable throughout.

Retain original typed errors and timestamps. Full retries preserve both
slots; acceptance clears only the oldest. Verify both sampler selection
orders, repeated full retry, one admission per turn, no duplicates after
draining, and exclusion of recorded events and lifecycle rejections. Preserve
the two original histories and the original runnable trace as a prefix.

## Design and verification

Use the existing public event and runtime APIs; add no library service,
dependency or recovery policy. A fixed three-application mission bounds
terminal failures at two because each failing cursor is disabled. Retained
events retry in observation order; the shared FIFO orders acceptance only.
Fresh attempts may be admitted before older rejected events if the caller
frees space and selects work before retry.

Run the locked baseline, focused example and identical independent binary.
Inspect metadata, bytes, hashes, source form, links, rendered documents and
the complete diff. Record Codex review and exact hosted verification after
an ordinary fast-forward main push. Stop at this cooperative serial policy;
callbacks and output calls must return and the caller must select retry.

## Input

Clean main `c2da285952d99710adb61eb76ca872213e32edbd` matches refreshed
origin. The local locked 278-test baseline passes. Exact hosted run
44089559879 is completed successfully. Local toolchain remains Rust/Cargo
1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.

## Result

Implementation and independent review accepted source blob
`83886bee28bbcb7e36fd7ac3ef030ab24495ab5c`. The locked local baseline
passes 281 workspace tests and all required static checks. Five focused
and identical independent binary tests pass. Both binaries emit the same
14 lines with the exact six-line input prefix. Final source/document audit
and main publication evidence are recorded in the
[focused review](docs/verification/PERIODIC_PEER_REVIEW.md#two-failure-retry-follow-up).
Implementation `4e37bced92142092193b1bb334984c9d427d4ce6` reached
origin/main by ordinary fast-forward push and passed all 20 steps in exact
hosted run 44311260522. Codex accepts this fixed consumer checkpoint.
