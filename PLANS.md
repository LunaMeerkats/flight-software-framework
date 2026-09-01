# Configuration-aware ordinary work integration

Date: **2026-09-02**
Status: **Complete**

## Objective and context

Implement the production boundary selected by ADR-0018: one optional validated
configuration table owned by `Runtime` from construction and one immutable
configuration view passed through the existing ordinary-work callback. Migrate
direct, scheduled, failure-event-reporting, and messaging-owned work together so
none bypasses the existing lifecycle, event, or inbox-cleanup behavior.

The run starts clean on `codex/nightly` at `5251f657`. The complete documented
baseline passes with 73 tests on rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and
Clippy 0.1.98. ADR-0005, ADR-0017, and ADR-0018 already settle the behavior; no
new external research, dependency, concurrency model, schema, or wire format is
needed.

## Acceptance criteria

- Keep `Runtime::new` specialized for an unconfigured default runtime. Add a
  configured constructor that transfers a complete validated table and returns
  that table unchanged if runtime storage construction fails.
- Give `Application::work` one `ApplicationWorkContext` whose optional immutable
  view exposes only active bytes and the original acceptance revision.
- Provide narrow replacement and consume-once rollback operations. An
  unconfigured runtime returns a typed absence error; no table attachment,
  detachment, wholesale replacement, or mutable table access is exposed.
- Preserve initial, replacement, rejection, rollback, and non-reused revision
  behavior as observed by later work callbacks.
- Prove stop/restart retention, lifecycle-rejected callback suppression, and no
  automatic rollback after a returned application error. Preserve the original
  error, terminal `Failed` state, and later peer progress.
- Keep all direct, scheduled, failure-event, and messaging-owned ordinary work
  delegated through `Runtime::work`. Preserve exact schedule consumption,
  clock/event attempts, and messaging inbox clearing on returned errors.
- Expose the same narrow replacement and rollback operations through
  `MessagingRuntime` without exposing its inner runtime.
- Keep start, stop, restart, and `MessagingApplication::handle_message`
  context-free. Add no general service context, messaging-aware scheduler,
  persistent clock/event owner, parser, schema, or dependency.
- Verify that borrowed configuration cannot escape its callback lifetime while
  explicitly copied observations remain application-owned.
- Rerun the full Cargo/Git baseline, the standalone ADR-0018 rustdoc probes,
  source-width and Markdown audits, rendered-content review, and complete diff
  review before local commits.

## Proposed files and verification approach

Production changes are limited to `src/runtime.rs`, the generic forwarding
layers in `src/scheduling.rs`, `src/runtime_events.rs`, and
`src/messaging_runtime.rs`, public exports in `src/lib.rs`, and explicit
callback migrations in existing tests. Focused configuration-runtime tests may
use a new integration-test module when that keeps the service interaction
scenarios coherent.

Durable evidence updates include README, architecture, requirements, roadmap,
project state, ADR-0005, ADR-0017, ADR-0018, the borrowing experiment,
traceability, contributor layout descriptions, and this plan. The source
register changes only if new research becomes necessary.

Required commands are the six existing Cargo/Git checks from AGENTS.md, with
`RUSTDOCFLAGS=-D warnings`, plus the explicit build and standalone Markdown
`rustdoc --test` commands in the configuration-context experiment. Focused test
targets run before the final suite. Ad hoc source/document audit scripts remain
under the ignored `target/nightly-2026-09-01` review directory and are not
repository gates.

## Risks and safe stopping point

The concrete configuration error type and const byte bound propagate through
the runtime and messaging owner. Default generic parameters must preserve the
existing unconfigured call sites, including the distinct valid zero-byte
configured case. The split mutable-record/immutable-configuration borrow must
remain safe Rust and keep one lifecycle validation/state-commit path.

If generic composition fragments an existing work path or requires unsafe code,
application removal, duplicated lifecycle logic, a new dependency, or broad API
scope, stop with the existing decision and evidence intact. The successful
stopping point is one reviewed production integration with truthful RFF-REQ-006
traceability, a clean baseline, and local commits only. No push is authorized.

## Outcome

The production increment is complete in local commit `9afc8568`. Runtime-owned
optional configuration now reaches one immutable ordinary-work context through
direct, scheduled, failure-event, and messaging-owned work without changing
lifecycle gating, error identity, schedule consumption, event attempts, or
inbox cleanup. Ten focused integration tests cover the accepted behavior,
including used-prefix visibility and the distinct zero-byte configured case.

The full documented Cargo/Git baseline passes with 83 tests. The standalone
ADR-0018 experiment passes one executable and four intended compiler-rejection
probes, including production work-context lifetime escape. Source-form and
Markdown audits report no findings; all 11 changed Markdown pages render
without horizontal overflow, skipped heading levels, table overflow, or browser
console warnings at the inspected 1,265-pixel viewport. The browser screenshot
channel timed out, so this run does not claim screenshot-based visual QA.

No new external research, dependency, unsafe code, push, or next-feature work
was needed. The repository remains at the safe stopping point selected above.
