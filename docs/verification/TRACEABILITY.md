# Verification traceability

The implemented Rust slices cover bounded logical lifecycle records and owned
synchronous start/stop boundaries. They do not execute application work or
restart callbacks and do not satisfy the two-application lifecycle or
error-containment scenarios in full. Implementation and verification state are
tracked separately so neither planned evidence nor existing code is
misrepresented.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Partial: bounded logical LC1 transitions plus owned registration, start, and stop for statically composed applications; no owned restart or complete host scenario | Exhaustive lifecycle tests plus complete two-application start/stop/restart scenario | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `lifecycle::tests::unknown_identity_is_rejected_without_mutating_existing_records`; four tests in `tests/lifecycle_registry.rs`; `valid_stop_commits_stopped_and_rejected_stops_suppress_the_callback`; `unknown_identity_is_rejected_before_application_code_runs`; `cargo test --workspace --all-features` (11 passed) | `8899cf986ad35ea32b8aff7950a958ca80365d7d` | Partially verified |
| RFF-REQ-003 | Not implemented | Queue boundary/FIFO/overflow/disconnect unit and fan-out integration tests | None | - | Not verified |
| RFF-REQ-004 | Not implemented | Manual-clock unit tests and replayed scheduling integration scenario | None | - | Not verified |
| RFF-REQ-005 | Not implemented | Structured-field and bounded-event-path tests | None | - | Not verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | - | Not verified |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | - | Not verified |
| RFF-REQ-008 | Partial: returned start and stop errors commit only the selected application to terminal `Failed`, preserve the concrete error, and do not prevent an eligible peer lifecycle operation; no structured event or subsequent peer work | Returned-error fault-injection integration scenario including event and subsequent peer work | `returned_start_error_fails_only_the_selected_application`; `returned_stop_error_fails_only_the_selected_application`; `cargo test --workspace --all-features` (11 passed) | `8899cf986ad35ea32b8aff7950a958ca80365d7d` | Partially verified |

## Evidence policy

"Verified" requires all of the following:

- an exact test, review artefact, or other observable evidence;
- the successful command that exercised it when applicable, or the documented
  procedure and result for a human review;
- the commit hash containing the evidence; and
- requirement wording that matches what was actually demonstrated.

A compiled interface, an ignored test, an unexecuted test, or a planned command
is not verification. Environment-only failures must be recorded as failures or
blocked checks, not silently treated as passes.
