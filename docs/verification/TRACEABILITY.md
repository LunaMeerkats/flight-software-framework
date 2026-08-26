# Verification traceability

The implemented Rust slices cover bounded logical lifecycle records, owned
synchronous start/work/stop/in-place-restart, bounded application messaging, and
a standalone bounded structured-event queue. Public integration tests satisfy
the complete two-application RFF-REQ-002 scenario. The routing core,
lifecycle-owned availability, and caller-selected one-message dispatch together
verify RFF-REQ-003. RFF-REQ-005 remains partial because event timestamps are
caller-supplied and no runtime/application or host integration exists.
Returned-error containment remains partial because no failure path emits through
the event queue. Implementation and verification state remain separate so
planned evidence is not represented as completed behavior.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Implemented: bounded logical LC1 transitions plus two statically composed applications completing owned registration, start, work, stop, restart, and work | Preserve exhaustive lifecycle and complete two-application scenario regression coverage | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `configured_capacity_bounds_registration_without_mutating_existing_records`; `approved_lc1_sequence_preserves_identity_and_registry_bound`; `invalid_transition_is_typed_and_leaves_state_unchanged`; `two_lifecycle_records_transition_independently`; `two_applications_complete_lifecycle_with_running_work`; `unknown_identity_is_rejected_before_application_code_runs`; `cargo test --workspace --all-features` (42 passed) | `cc0e453431588a1f39e147249af701c21c1e1b1d` | Verified |
| RFF-REQ-003 | Implemented: inline payload bound, immutable positive-capacity topology, registration-order routing, cross-topic FIFO, reject-newest saturation, unaffected fan-out, ordered outcomes, lifecycle unavailability, exact stop/failure clearing, empty restart reconnection, and running-only one-message application dispatch through a publish-only context | Preserve exact-boundary routing, lifecycle ownership, oldest-only dispatch, self-publication, refreshed availability, and callback-error regressions | `inline_payload_accepts_its_exact_limit_and_rejects_one_more_byte`; `topology_validation_rejects_invalid_inboxes_without_a_partial_bus`; `reject_newest_saturation_preserves_older_entries_and_healthy_fan_out`; `publication_distinguishes_all_full_from_no_subscribers`; `one_inbox_preserves_cross_topic_publish_order`; `selective_routing_skips_nonmatching_endpoints_and_preserves_route_order`; `registered_subscribers_are_known_but_unavailable`; `stop_clears_one_inbox_and_restart_reconnects_it_empty`; `lifecycle_gate_and_empty_inbox_never_invoke_application_code`; `dispatch_presents_one_oldest_message_per_call`; `self_publication_uses_the_freed_slot_and_preserves_peer_outcomes`; `dispatch_refreshes_peer_availability_after_lifecycle_change`; `callback_error_clears_selected_queue_but_retains_peer_publication`; `cargo test --workspace --all-features` (42 passed) | `cc0e453431588a1f39e147249af701c21c1e1b1d` | Verified |
| RFF-REQ-004 | Not implemented | Manual-clock unit tests and replayed scheduling integration scenario | None | - | Not verified |
| RFF-REQ-005 | Partial: typed framework/application source, severity, mission-defined copied identifier, explicit elapsed timestamp, and a standalone positive-capacity FIFO queue with caller-visible reject-newest saturation; no framework clock, runtime/application emission, or host scenario | Preserve structured-field, exact-capacity, FIFO, and saturation regressions; add framework-owned timestamp generation and host/runtime integration | `event_exposes_structured_fields_without_text_parsing`; `queue_requires_positive_capacity`; `queue_accepts_its_exact_limit_and_dequeues_fifo`; `reject_newest_preserves_older_events_and_allows_explicit_retry`; `cargo test --workspace --all-features` (42 passed) | `cc0e453431588a1f39e147249af701c21c1e1b1d` | Partially verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | - | Not verified |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | - | Not verified |
| RFF-REQ-008 | Partial: returned application errors from start, work, stop, restart, and message callbacks commit only the selected application to terminal `Failed`, preserve the concrete source, and leave a peer operable; messaging clears only the failed endpoint while already accepted peer deliveries remain; a standalone event queue exists, but no returned-error path emits into it | Add a framework-timestamped returned-error event to a fault-injection scenario; assert failed state/event, successful subsequent peer work, original-error preservation, and explicit saturated-event-path behavior | `returned_start_error_fails_only_the_selected_application`; `returned_work_error_fails_only_the_selected_application`; `returned_stop_error_fails_only_the_selected_application`; `returned_restart_error_fails_only_the_selected_application`; `returned_work_error_clears_only_the_failed_endpoint`; `callback_error_clears_selected_queue_but_retains_peer_publication`; `cargo test --workspace --all-features` (42 passed) | `cc0e453431588a1f39e147249af701c21c1e1b1d` | Partially verified |

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
