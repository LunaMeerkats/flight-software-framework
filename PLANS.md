# Prioritize retained failure admission without blocking healthy work

## Objective and acceptance

RFF-LIB-REQ-017 revision 3 adds an optional turn policy to the existing
public-only periodic peer consumer. Defer enabled sampler work before
polling while a rejected failure remains retained. Keep healthy heartbeat
turns selectable, disabled selections explicit, and retry/dequeue separate.
Resume fresh work after retained admission, even if the FIFO remains full.

Preserve earlier policies and the original 14-line executable trace as a
prefix. Verify both sampler orders, repeated deferral with full and empty
FIFO, unchanged release/lifecycle state, peer progress, retry rejection,
and a new failure retained behind the admitted older event. Verify the
empty-backlog waiting and lifecycle paths. Add a runnable priority trace.

## Design and verification

The gate belongs to the fixed mission host: only its two samplers can fail.
Compare explicit admission gating with automatic retry/work and global
gating in ADR-0042. Add no framework API, service, queue or dependency.
Keep the original unguarded policy for consumers choosing their own order.

Run the locked baseline before implementation and after formatting, focused
example tests/run and the identical-source independent binary recipe.
Inspect metadata, bytes/hashes, source widths, Markdown links/rendering and
the complete diff. Record actual Codex review and exact hosted evidence
after an ordinary fast-forward main push. Rollback is limited to this
run's consumer/docs changes; the reviewed input remains the fallback.

## Input

Clean main `dba77f744f9151d515d7110259f425a64254abb1` matches fetched
and live origin. Exact input hosted run 44311715530 completed all 20 steps
successfully. The saved pre-edit locked baseline passed 281 workspace tests
and all required static checks. The interrupted 2026-11-20 implementation
was preserved and completed on 2026-11-21; remote main had not moved.

## Result

Source blob `2a203431c3677b2216de329cb99c7df079a72c86` passes root
and independent Codex source review. Locked local checks pass 285 workspace
tests; the focused example and identical independent binary pass nine each.
Both binaries emit the same 23 lines with the exact 14-line input prefix.
The [focused review](docs/verification/PERIODIC_PEER_REVIEW.md#retained-admission-priority-follow-up)
records actual commands, bounds and evidence. Complete document review and
ordinary main publication, then record exact hosted results. Stop at this
fixed cooperative policy.
