# Verification traceability

The first Rust slice implements only bounded logical lifecycle records. It does
not execute application objects, admit returned application errors, or satisfy
the two-application scenario in full. Implementation and verification state are
tracked separately so neither planned evidence nor existing code is
misrepresented.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Partial: bounded logical registry, opaque runtime-local IDs, and successful LC1 transitions; no application objects, returned-error ingress, or host scenario | Exhaustive lifecycle unit tests plus two-application integration scenario | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `lifecycle::tests::unknown_identity_is_rejected_without_mutating_existing_records`; all four tests in `tests/lifecycle_registry.rs`; `cargo test --workspace --all-features` (6 passed) | `0844d7c21a715492a7754072c0d80fa2d7b812fe` | Partially verified |
| RFF-REQ-003 | Not implemented | Queue boundary/FIFO/overflow/disconnect unit and fan-out integration tests | None | - | Not verified |
| RFF-REQ-004 | Not implemented | Manual-clock unit tests and replayed scheduling integration scenario | None | - | Not verified |
| RFF-REQ-005 | Not implemented | Structured-field and bounded-event-path tests | None | - | Not verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | - | Not verified |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | - | Not verified |
| RFF-REQ-008 | Not implemented | Returned-error fault-injection integration scenario | None | - | Not verified |

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
