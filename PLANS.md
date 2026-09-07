# Host command and telemetry boundary decision

Date: **2026-09-08**
Status: **Complete**

## Objective and context

Select the smallest concrete RFF-REQ-007 grammar, validation boundary, and
observable telemetry path before implementing the adapter pair. The clean
starting commit is `0ddf2b40e0bd4f7ae52e099ac9d9ec3be8c70428` on
`codex/nightly`. The documented baseline passes with 83 tests on unchanged
rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98. Source-quality
policy is already encoded; no adoption cleanup is needed.

The highest-value uncertainty is how telemetry leaves the owned messaging
runtime without exposing applications or performing blocking callback I/O.
ADR-0001 and the message callback/error contract constrain the answer. This
is a decision checkpoint, not an implementation of RFF-REQ-007.

## Acceptance criteria

- Compare fixed byte records with strict ASCII; specify grammar, bounds,
  validation precedence, and caller framing separately from stream acquisition.
- Trace validated ingress through publication and caller-selected dispatch to
  a telemetry subscriber and explicit host drain.
- Specify retained-record bounds, reject-newest outcomes, and the current
  terminal lifecycle consequence of returned message errors.
- Validate a borrowed capacity-one output mailbox against the real runtime in
  an executable probe; distinguish it from codec or sample integration tests.
- Record primary sources and decisions. Add no library API, production adapter,
  dependency, unsafe code, thread, or blocking callback I/O.
- Keep requirement status truthful and record future integration acceptance.
- Pass the Cargo/Git baseline, mailbox probe, source-form review, link audit,
  changed-document rendering, and complete diff review.

## Components and verification

ADR-0019 and the standalone mailbox experiment record the decision and
reproducible evidence. README, architecture, requirements, roadmap, source
register, traceability, project state, AGENTS.md, and this plan keep it usable
by the next run.

Use the six Cargo/Git commands in AGENTS.md with rustdoc warnings denied, plus
build and standalone rustdoc commands in the experiment. Review extracted Rust
with rustfmt and warnings-denied Clippy at the existing 60-line threshold.
Review aids stay under ignored `target/nightly-2026-09-08`; no new checker gate
is being adopted.

## Risks and safe stopping point

The grammar is a local demonstration choice, not an external protocol or API
freeze. Full output returned as a callback error fails the telemetry app under
current semantics. Caller framing and later I/O have separate resource costs.

Stop after this documented and verified decision checkpoint. RFF-REQ-007 stays
unverified until real adapters and integration evidence exist. Commit locally
only after applicable checks pass; no push is authorized.

## Outcome

ADR-0019 selects a two-byte EchoPercent command and matching telemetry with
length/identifier/value validation before publication or business logic. A
borrowed capacity-one mailbox lets host drain occur outside message callbacks.
The executable real-runtime probe confirms live reuse, full-output retention,
terminal failure with exact inbox clearing, and internal validation precedence.
It supplies ownership evidence only; RFF-REQ-007 remains not verified.

The initial and final Cargo/Git baselines pass with 83 tests. The explicit build,
standalone rustdoc probe (one executable, three scenarios), extracted rustfmt,
and warnings-denied Clippy checks pass. The 22 Rust files and 185-line snippet
have no width findings or new lint exceptions. The audit covers 32 Markdown
documents, 95 resolving links, eight requirement/traceability rows, 25 source
entries, and 53 exact test references. Changed documents match rendered HTML;
browser inspection at 1,280 pixels finds no page/table overflow or heading
skips. Screenshot spot checks cover the decision and experiment.

Independent decision and complete-diff reviews found no remaining actionable
defect. Allocation-failure injection remains explicitly unverified, and the
probe's two-slot clearing fixture is distinguished from the selected one-slot
mission inboxes. Library APIs, dependencies, and production behavior are
unchanged. The stopping point is this local decision checkpoint; no push.
