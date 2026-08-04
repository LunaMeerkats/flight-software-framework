# Verification traceability

No runtime behavior is implemented. RFF-REQ-001 notices exist in current entry
points, but the v0.1 review has not occurred. Implementation and verification
state are tracked separately so neither planned evidence nor existing prose is
misrepresented.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | — | Pending |
| RFF-REQ-002 | Not implemented | Exhaustive lifecycle unit tests plus two-application integration scenario | None | — | Not verified |
| RFF-REQ-003 | Not implemented | Queue boundary/FIFO/overflow/disconnect unit and fan-out integration tests | None | — | Not verified |
| RFF-REQ-004 | Not implemented | Manual-clock unit tests and replayed scheduling integration scenario | None | — | Not verified |
| RFF-REQ-005 | Not implemented | Structured-field and bounded-event-path tests | None | — | Not verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | — | Not verified |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | — | Not verified |
| RFF-REQ-008 | Not implemented | Returned-error fault-injection integration scenario | None | — | Not verified |

## Evidence policy

“Verified” requires all of the following:

- an exact test, review artefact, or other observable evidence;
- the successful command that exercised it when applicable, or the documented
  procedure and result for a human review;
- the commit hash containing the evidence; and
- requirement wording that matches what was actually demonstrated.

A compiled interface, an ignored test, an unexecuted test, or a planned command
is not verification. Environment-only failures must be recorded as failures or
blocked checks, not silently treated as passes.
