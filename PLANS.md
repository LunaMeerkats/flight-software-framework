# Stage 4 event-queue boundary review

Date: **2026-09-20**
Status: **Local checks and review passed; publication pending**

## Objective and context

Verify the existing event queue's fallible construction and repeated bounded
reuse. The starting revision is `4bfbda5907b92b3b71cded0422c25dfca8d987a7`,
clean and equal to refreshed `origin/codex/nightly`. Its hosted run 35018337047
passed; the initial locked local baseline passes 120 tests on Rust/Cargo 1.98.0.

This is the recorded next Stage 4 gap. Independent source/test inspection
found no production defect: the four existing standalone tests cover zero
capacity, metadata, capacity-two FIFO, and one saturation/retry cycle. Exact
reservation-overflow diagnostics and repeated reuse remain untested.

## Acceptance criteria and files

- Add two public-API tests in `tests/event_queue.rs`: exact `usize::MAX`
  reservation rejection and repeated capacity-one/capacity-three saturation,
  explicit retry, complete FIFO drain, and refill.
- Observe pending counts and unchanged logical capacity, retain distinct
  rejected records, and prove that only an explicitly retried record appears.
- Preserve production code, API, requirements, dependencies, and lint policy.
- Record source-inspected resource properties separately from executed evidence
  in `docs/verification/EVENT_QUEUE_REVIEW.md`; update README, source register,
  project state, roadmap, and traceability.

## Verification

Run `cargo test --locked --test event_queue`, then the required locked Cargo
baseline with warnings-denied Clippy and rustdoc. Audit Rust physical/comment
widths, relative links, traceability names, rendered changed documents, and the
complete diff. Obtain independent Codex review. Unchanged host adapters,
workflow, and ADR-0018/0019 probes do not trigger separate local commands.

After local acceptance, commit on `codex/nightly`, refresh origin, publish by
ordinary fast-forward, and inspect exact-revision hosted CI. Record completed
publication evidence separately without projecting a pass onto a later commit.

## Risks and safe stopping point

The impossible nonzero-sized record count tests capacity overflow, not actual
allocator exhaustion. Public observations do not measure allocation bytes,
internal deque wrapping, timing, concurrency, or arbitrary fault containment.
Human v0.1 acceptance remains open. If verification fails, repair or remove
only this run's changes. Stop after this one verified checkpoint and its
publication evidence.

## Completed local checks

Both new regressions pass: six focused and 122 workspace tests. Formatting,
all-target check, warnings-denied Clippy/rustdoc, and whitespace checks pass.
No production change, dependency, or lint exception was needed. Author and
independent Codex complete-diff/source reviews found no blocking defect.
The final audit passes 43 Markdown files, 187 relative links, 86 exact test
references, and 32 Rust files with zero width findings and three unchanged
fulfilled expectations. Seven rendered changed documents match source content
and pass browser DOM/layout review at 1280px with no page overflow or console
warnings/errors. Browser inventory recovered after an initial timeout.
Screenshot capture timed out; pixel inspection is unavailable. The completed
rendered-content/DOM review is the documented adaptation, not a screenshot
pass. Temporary aids/logs are under `target/review-2026-09-20`; no checker is
adopted. Exact-revision hosted results remain pending.
