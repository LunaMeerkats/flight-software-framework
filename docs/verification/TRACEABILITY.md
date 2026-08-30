# Verification traceability

The implemented Rust slices cover bounded logical lifecycle records, owned
synchronous start/work/stop/in-place-restart, bounded application messaging, and
a standalone bounded structured-event queue, an injected manual clock, a finite
one-shot work agenda, and direct returned-work failure-event reporting. Public
integration tests satisfy the complete two-application RFF-REQ-002 scenario.
The routing core, lifecycle-owned availability, and caller-selected one-message
dispatch together verify RFF-REQ-003. The clock, one-item due-work operation,
stable equal-time ordering, and replayed work/lifecycle/event-timestamp trace
verify RFF-REQ-004. The runtime-generated event, bounded queue, failed state,
preserved work error, saturation outcome, and later peer work verify RFF-REQ-005
and RFF-REQ-008 only at the direct cooperative returned-work boundary.
The standalone configuration snapshot core partially implements RFF-REQ-006;
its revision, validation, and rollback tests do not establish runtime ownership
or application visibility at safe points.
Implementation and verification state remain separate so planned evidence is
not represented as completed behavior.

The 2026-08-31 full local baseline passes 73 tests. Earlier verified rows retain
their prior evidence commits until the new committed-state checkpoint is
recorded. Browser visual inspection is blocked by local-file URL policy;
generated HTML structure/content is reviewed under the documented adaptation
in `PLANS.md`, without claiming browser visual QA.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Implemented: bounded logical LC1 transitions plus two statically composed applications completing owned registration, start, work, stop, restart, and work | Preserve exhaustive lifecycle and complete two-application scenario regression coverage | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `configured_capacity_bounds_registration_without_mutating_existing_records`; `approved_lc1_sequence_preserves_identity_and_registry_bound`; `invalid_transition_is_typed_and_leaves_state_unchanged`; `two_lifecycle_records_transition_independently`; `two_applications_complete_lifecycle_with_running_work`; `unknown_identity_is_rejected_before_application_code_runs`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-003 | Implemented: inline payload bound, immutable positive-capacity topology, registration-order routing, cross-topic FIFO, reject-newest saturation, unaffected fan-out, ordered outcomes, lifecycle unavailability, exact stop/failure clearing, empty restart reconnection, and running-only one-message application dispatch through a publish-only context | Preserve exact-boundary routing, lifecycle ownership, oldest-only dispatch, self-publication, refreshed availability, and callback-error regressions | `inline_payload_accepts_its_exact_limit_and_rejects_one_more_byte`; `topology_validation_rejects_invalid_inboxes_without_a_partial_bus`; `reject_newest_saturation_preserves_older_entries_and_healthy_fan_out`; `publication_distinguishes_all_full_from_no_subscribers`; `one_inbox_preserves_cross_topic_publish_order`; `selective_routing_skips_nonmatching_endpoints_and_preserves_route_order`; `registered_subscribers_are_known_but_unavailable`; `stop_clears_one_inbox_and_restart_reconnects_it_empty`; `lifecycle_gate_and_empty_inbox_never_invoke_application_code`; `dispatch_presents_one_oldest_message_per_call`; `self_publication_uses_the_freed_slot_and_preserves_peer_outcomes`; `dispatch_refreshes_peer_availability_after_lifecycle_change`; `callback_error_clears_selected_queue_but_retains_peer_publication`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-004 | Implemented at the finite one-shot boundary: ordered `FrameworkInstant`, injected `Clock`, explicit `ManualClock` advancement, typed non-mutating overflow, fixed nondecreasing `WorkSchedule`, stable equal-time order, one due or overdue attempt per call, exact lifecycle/error outcomes, and identical replayed work/lifecycle/event-timestamp traces | Preserve manual-clock, schedule-order, one-item, lifecycle/error, and complete replay regressions; evaluate recurrence only through a separate decision | `manual_clock_starts_at_origin_and_reads_do_not_advance`; `manual_clock_advances_only_by_explicit_durations`; `advance_overflow_is_typed_and_preserves_current_time`; `schedule_rejects_descending_instants_and_accepts_an_empty_agenda`; `complete_reads_no_clock_and_each_pending_item_decision_reads_once`; `waiting_preserves_the_item_and_manual_clock_until_its_inclusive_instant`; `equal_time_order_is_stable_and_overdue_work_runs_one_item_per_call`; `lifecycle_rejection_consumes_one_item_without_blocking_a_due_peer`; `returned_work_error_consumes_one_item_without_blocking_a_due_peer`; `identical_manual_scenarios_replay_the_same_work_and_event_timestamp_trace`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-005 | Implemented at the direct cooperative returned-work boundary: structured source, error severity, mission-defined copied identifier, one injected timestamp read, positive-capacity FIFO queueing, caller-visible reject-newest saturation, and the exact rejected event retained for explicit handling | Preserve structured-field, exact-capacity, single-attempt, no-event-on-success/rejection, saturation, retry, and timestamp regressions; add broader event paths only through separate decisions | `event_exposes_structured_fields_without_text_parsing`; `queue_requires_positive_capacity`; `queue_accepts_its_exact_limit_and_dequeues_fifo`; `reject_newest_preserves_older_events_and_allows_explicit_retry`; `returned_work_error_records_clock_captured_event_and_preserves_peer_progress`; `saturated_failure_event_retains_error_and_exact_event_for_retry`; `lifecycle_rejections_do_not_read_clock_or_emit`; `successful_work_does_not_read_clock_or_emit`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-006 | Partial: standalone byte-bounded immutable snapshots, retained validation function, rejection without mutation, never-reused revisions, and consume-once rollback; runtime ownership and application access are pending | Integrate runtime ownership, safe-point visibility, restart retention, and no automatic rollback after application errors | `initial_configuration_is_validated_and_has_no_rollback_history`; `oversized_initial_configuration_is_rejected_before_validation`; `oversized_replacement_is_rejected_before_validation`; `exact_bound_is_accepted_and_shorter_snapshots_expose_only_used_bytes`; `zero_capacity_accepts_valid_empty_content_and_rejects_nonempty_content`; `initial_and_replacement_bytes_are_owned_copies`; `cloned_snapshot_retains_its_bytes_and_revision_after_replacement`; `rejected_replacements_preserve_active_history_and_revision_allocation`; `equal_content_replacements_still_allocate_distinct_revisions`; `rollback_is_consumed_once_and_does_not_reuse_issued_revisions`; `each_replacement_retains_only_the_immediately_previous_active_snapshot`; `rejected_error_preserves_concrete_mission_error_as_its_source`; `configuration::tests::final_revision_and_exhaustion_preserve_snapshots_and_high_water`; `configuration::tests::validation_precedes_exhaustion_without_changing_retained_state`; `configuration::tests::oversize_replacement_and_rollback_do_not_invoke_validator`; `cargo test --test configuration_table` (12 passed); `cargo test --workspace --all-features configuration` (three private configuration tests passed) | Pending local commit | Partial; standalone core tested |
| RFF-REQ-007 | Not implemented | Valid command/telemetry and adapter-defined malformed-input integration tests | None | - | Not verified |
| RFF-REQ-008 | Implemented at the selected cooperative returned-work fault boundary: the complete concrete error is preserved through its source chain, only the selected application enters terminal `Failed`, one clock-captured bounded event is attempted with explicit saturation, and a healthy peer completes later work | Preserve failed-state, original-error, event-attempt, saturation, no-duplicate, and peer-progress regressions; do not extend this verification to panics, hangs, cleanup, or other callback event paths without separate evidence | `returned_work_error_fails_only_the_selected_application`; `returned_work_error_records_clock_captured_event_and_preserves_peer_progress`; `saturated_failure_event_retains_error_and_exact_event_for_retry`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |

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
