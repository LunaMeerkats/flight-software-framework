# Verification traceability

The implemented Rust slices cover bounded logical lifecycle records and owned
synchronous start/work/stop/in-place-restart boundaries. A public integration
test satisfies the complete two-application RFF-REQ-002 scenario. A separate
available-endpoint routing core partially implements RFF-REQ-003. Returned-error
containment remains partial because no structured failure event exists.
Implementation and verification state remain separate so planned evidence is
not represented as completed behavior.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Implemented: bounded logical LC1 transitions plus two statically composed applications completing owned registration, start, work, stop, restart, and work | Preserve exhaustive lifecycle and complete two-application scenario regression coverage | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `configured_capacity_bounds_registration_without_mutating_existing_records`; `approved_lc1_sequence_preserves_identity_and_registry_bound`; `invalid_transition_is_typed_and_leaves_state_unchanged`; `two_lifecycle_records_transition_independently`; `two_applications_complete_lifecycle_with_running_work`; `unknown_identity_is_rejected_before_application_code_runs`; `cargo test --workspace --all-features` (23 passed) | `6c6e0b0b31e7338414f28f330024c2bfbd3eec40` | Verified |
| RFF-REQ-003 | Partial: inline payload limit, caller-supplied positive-capacity inboxes, immutable unique topic sets, selective registration-order routing, cross-topic FIFO, reject-newest saturation, unaffected-subscriber fan-out, and ordered available-endpoint results; runtime ownership, lifecycle unavailability/clearing/reconnection, self-publication, and dispatch remain unimplemented | Add runtime-owned stopped/failed unavailable outcomes, discarded-count clearing, empty restart reconnection, self-publication, and one in-flight dispatch evidence | `inline_payload_accepts_its_exact_limit_and_rejects_one_more_byte`; `topology_validation_rejects_invalid_inboxes_without_a_partial_bus`; `reject_newest_saturation_preserves_older_entries_and_healthy_fan_out`; `publication_distinguishes_all_full_from_no_subscribers`; `one_inbox_preserves_cross_topic_publish_order`; `selective_routing_skips_nonmatching_endpoints_and_preserves_route_order`; `inbox_access_rejects_an_identity_outside_the_topology`; `cargo test --workspace --all-features` (23 passed) | `6c6e0b0b31e7338414f28f330024c2bfbd3eec40` | Partially verified |
| RFF-REQ-004 | Not implemented | Manual-clock unit tests and replayed scheduling integration scenario | None | - | Not verified |
| RFF-REQ-005 | Not implemented | Structured-field and bounded-event-path tests | None | - | Not verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | - | Not verified |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | - | Not verified |
| RFF-REQ-008 | Partial: returned start, work, stop, and restart errors commit only the selected application to terminal `Failed` and preserve the concrete error; a running peer completes subsequent work; no structured failure event | Add the structured event to the returned-error fault-injection integration scenario | `returned_start_error_fails_only_the_selected_application`; `returned_work_error_fails_only_the_selected_application`; `returned_stop_error_fails_only_the_selected_application`; `returned_restart_error_fails_only_the_selected_application`; `cargo test --workspace --all-features` (23 passed) | `6c6e0b0b31e7338414f28f330024c2bfbd3eec40` | Partially verified |

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
