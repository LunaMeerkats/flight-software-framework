# Verification traceability

The implemented Rust slices cover bounded logical lifecycle records and one
owned start boundary. They do not execute application work or stop/restart
callbacks and do not satisfy the two-application lifecycle or error-containment
scenarios in full. Implementation and verification state are tracked separately
so neither planned evidence nor existing code is misrepresented.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Partial: bounded logical LC1 transitions plus owned registration and start for statically composed applications; no owned stop/restart or complete host scenario | Exhaustive lifecycle tests plus complete two-application start/stop/restart scenario | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `lifecycle::tests::unknown_identity_is_rejected_without_mutating_existing_records`; four tests in `tests/lifecycle_registry.rs`; three tests in `tests/application_runtime.rs`; `cargo test --workspace --all-features` (9 passed) | `1696f65a87f9a06ccb80894d2b07ffa211bdc991` | Partially verified |
| RFF-REQ-003 | Not implemented | Queue boundary/FIFO/overflow/disconnect unit and fan-out integration tests | None | - | Not verified |
| RFF-REQ-004 | Not implemented | Manual-clock unit tests and replayed scheduling integration scenario | None | - | Not verified |
| RFF-REQ-005 | Not implemented | Structured-field and bounded-event-path tests | None | - | Not verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | - | Not verified |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | - | Not verified |
| RFF-REQ-008 | Partial: a returned start error commits the selected application to terminal `Failed`, preserves the concrete error, and does not prevent a peer start; no structured event or subsequent peer work | Returned-error fault-injection integration scenario including event and subsequent peer work | `returned_start_error_fails_only_the_selected_application`; `cargo test --workspace --all-features` (9 passed) | `1696f65a87f9a06ccb80894d2b07ffa211bdc991` | Partially verified |

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
