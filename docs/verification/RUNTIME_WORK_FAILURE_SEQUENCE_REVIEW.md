# Finite runtime ordinary-work failure sequence review

Date: **2026-10-12**
Scope: **Returned ordinary-work failure, terminal availability and peer retention**

## Contract and reviewed input

Input `8705408ee9810470b19c7eef796557b5d2b7ee38` starts clean on codex/nightly,
equal to refreshed origin. Exact input hosted run 38054936905 is freshly
successful; the initial locked local baseline passes 143 workspace tests.
The accepted [host review](HOST_V0_1_REVIEW.md) and successful-callback
[runtime lifecycle model](RUNTIME_MESSAGE_SEQUENCE_REVIEW.md) remain separate.

RFF-REQ-002/003, the peer-operability aspect of RFF-REQ-008 and ADR-0003/0009/
0011 define this checkpoint. Running work invokes one callback; success retains
state and inboxes. A returned concrete work error commits only the selected
application to terminal Failed, preserves its error and clears exactly its
pending inbox. Peers retain their state, queued deliveries and work eligibility.
Non-running work and rejected stop/restart invoke no callback and clear nothing.
Only Running endpoints accept publication; unavailable destinations remain in
ordered reports. This is verification of existing contracts, with no production,
API, requirement, dependency, licence, policy or workflow change. No external
research, upstream behavior claim or source reuse is needed.

## Frozen domain and alternatives

Before implementation, root Codex and parallel Codex scope_review freeze:

- Two initially Running applications, one shared Command topic, capacities
  [1,2]. Run both fault roles, so each position/capacity is the failing endpoint
  in a separate traversal. Only faulting ordinary work returns an error; healthy
  work and start/stop/restart/message callbacks succeed.
- Five generated operations: Publish with its position as a one-byte payload,
  Work(fault), Work(peer), Stop(fault), Restart(fault).
- Every sequence length zero through six, independently initialized for each
  fault role: 39,062 traces and 224,610 generated operations. Assert the counts.
- Final passive dispatches and one healthy-peer work attempt are supplemental
  observations, excluded from generated counts. Every generated prefix is also
  independently enumerated, making its retained FIFO and peer progress observable.

One fault role would miss slot/capacity asymmetry. Adding peer stop/restart
would duplicate accepted successful-lifecycle evidence and broaden the alphabet.
Generating dispatch or adding events/schedules/configuration would introduce
different composition responsibilities. Length six retains stop/restart then
failure followed by publication and rejected-operation witnesses. No claim is
made for longer histories or arbitrary fault policies.

## Oracle and observability

The independent oracle uses Active/Paused/Terminal states, hard-coded routes
[0,1], capacities [1,2], append-only accepted payload histories and discard
watermarks. Successful stop or returned work failure advances only the selected
watermark; restart changes only Paused to Active. Rejection preserves all
histories and counters. Oracle decisions do not query runtime state or reuse
production transition, queue or report-classification helpers.

At the public comparison boundary, translate the model into expected states,
ordered destination statuses, classifications and exact typed error values.
Compare both capacities, pending counts, lifecycle/work counters and states
initially and after each generated operation. Distinct position bytes expose
replacement and reordering. Final passive callbacks record complete retained
topic/payload FIFO; empty and unavailable dispatches must suppress callbacks.
A supplemental healthy work attempt directly checks retained peer operability.
Final state/counter and receipt equality checks cover these observations.

Test-only Rc/RefCell/Vec histories and counters are finite observation fixtures,
not operational storage or whole-process bounds. Ordinary work never publishes,
dispatch callbacks neither publish nor fail, and final observations do not
expand the generated operation domain.

## Verification and review

Root Codex and parallel Codex scope_review review the model and source form;
scope_review implements the test. Parallel Codex sensitivity independently
checks the oracle and selected isolated source faults; final_review reviews
the complete diff, source form and durable/document evidence. Initial/final
locked local baselines pass 143/144 workspace tests and the focused model
passes one test over all asserted counts. Commands actually completed:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_work_failure_sequences
git diff --check
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Scope review also runs focused Clippy
and rustfmt checks. Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable
and Clippy 0.1.98. Logs stay in target/review-2026-10-12-failure. The planning
count typo 225,610 is caught by the aggregate assertion and corrected to
224,610 before acceptance; the exploration domain is unchanged. Root source
audit catches one 81-column comment, shortened to 72 columns. Final test blob
is `caf375400917899de01ff7615a411b4555ca1b11`, with no new expectation.
Conditional host sample/adapter commands, ADR-0018/0019 probes and actionlint
are unchanged and not triggered locally. The initial committed-tip whitespace
command `git show --format= --check --diff-merges=first-parent HEAD` also passes;
the new committed tip is checked separately before publication.

The complete source audit covers 36 Rust files/13,935 physical lines, zero
physical/comment-width findings and three unchanged fulfilled expectations.
All 336 relative links across 65 Markdown files and 108 exact traceability
function references resolve. Changed documents are rendered to HTML and
inspected for headings, full content, links, tables, code blocks and nesting;
structure/content passes. This is separate local review, not pixel-level or
human acceptance. Root and parallel Codex final_review accept the source/oracle
without a blocking finding; complete diff and durable-record review also pass.

## Selected isolated sensitivity

Parallel Codex sensitivity runs copied packages under the ignored
target/review-2026-10-12-failure/mutations/final-copy directory. Each narrow
production substitution compiles with the focused `cargo test --locked
--test runtime_work_failure_sequences --no-run` command before the focused
test fails a behavioral assertion. All first detections use fault role zero:

| Isolated fault | First detected trace | Assertion observation |
| --- | --- | --- |
| Work failure retains selected inbox | Publish, WorkFault | Selected pending 1, expected 0 |
| Work failure clears peer | Publish, WorkFault | Peer pending 0, expected 1 |
| Work discard count adds one | WorkFault | Discard 1, expected 0 |
| Work error retains Running | WorkFault | Running, expected Failed |
| Publication admits Failed | WorkFault, Publish | Delivered, expected Unavailable |
| Rejected work invokes callback | WorkFault, WorkFault | Two work callbacks, expected one |

Exact substitutions, commands, original production/test blobs and byte hashes,
successful no-run logs and assertion-failure logs remain in
mutations/final-copy/mutation_results.json under that review directory. The
temporary harness command is `python
target/review-2026-10-12-failure/mutations/mutation_trials.py final-copy`; a
rerun requires a new output subdirectory because existing evidence is preserved.
All trials use final test blob `caf375400917899de01ff7615a411b4555ca1b11`.
Production source is never faulted in the working tree. These six selected
detections establish assertion sensitivity only, not general mutation adequacy
or coverage of all possible defects. Root and final_review inspect actual logs
and provenance separately from the sensitivity agent's summary.

## Limitations and disposition

This model excludes generated start/dispatch, Registered, start/stop/restart/
message callback errors, callback publication, event/schedule/configuration
composition, arbitrary topologies/topics/capacities, allocation failures,
concurrency, timing and whole-process bounds. Failed remains terminal; this
checkpoint does not establish recovery, application/external cleanup, panic or
hang containment, or the event aspect of RFF-REQ-008. Existing event evidence
remains the authority for that aspect. Autonomous Codex review does not establish
human review, flight qualification, compatibility or operational suitability.
