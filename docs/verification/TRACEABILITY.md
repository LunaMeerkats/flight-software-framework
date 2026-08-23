# Verification traceability

The implemented Rust slices cover bounded logical lifecycle records and owned
synchronous start/work/stop/in-place-restart boundaries. A public integration
test satisfies the complete two-application RFF-REQ-002 scenario. Returned-error
containment remains partial because no structured failure event exists.
Implementation and verification state are tracked separately so neither planned
evidence nor existing code is misrepresented.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Implemented: bounded logical LC1 transitions plus two statically composed applications completing owned registration, start, work, stop, restart, and work | Preserve exhaustive lifecycle and complete two-application scenario regression coverage | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `configured_capacity_bounds_registration_without_mutating_existing_records`; `approved_lc1_sequence_preserves_identity_and_registry_bound`; `invalid_transition_is_typed_and_leaves_state_unchanged`; `two_lifecycle_records_transition_independently`; `two_applications_complete_lifecycle_with_running_work`; `unknown_identity_is_rejected_before_application_code_runs`; `cargo test --workspace --all-features` (16 passed) | `c2aa32772c2fd32a0b87893f913e32ecacc5d051` | Verified |
| RFF-REQ-003 | Not implemented | Queue boundary/FIFO/overflow/disconnect unit and fan-out integration tests | None | - | Not verified |
| RFF-REQ-004 | Not implemented | Manual-clock unit tests and replayed scheduling integration scenario | None | - | Not verified |
| RFF-REQ-005 | Not implemented | Structured-field and bounded-event-path tests | None | - | Not verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | - | Not verified |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | - | Not verified |
| RFF-REQ-008 | Partial: returned start, work, stop, and restart errors commit only the selected application to terminal `Failed` and preserve the concrete error; a running peer completes subsequent work; no structured failure event | Add the structured event to the returned-error fault-injection integration scenario | `returned_start_error_fails_only_the_selected_application`; `returned_work_error_fails_only_the_selected_application`; `returned_stop_error_fails_only_the_selected_application`; `returned_restart_error_fails_only_the_selected_application`; `cargo test --workspace --all-features` (16 passed) | `c2aa32772c2fd32a0b87893f913e32ecacc5d051` | Partially verified |

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
