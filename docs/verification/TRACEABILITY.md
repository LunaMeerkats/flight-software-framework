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
Implementation and verification state remain separate so planned evidence is
not represented as completed behavior.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Implemented: bounded logical LC1 transitions plus two statically composed applications completing owned registration, start, work, stop, restart, and work | Preserve exhaustive lifecycle and complete two-application scenario regression coverage | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `configured_capacity_bounds_registration_without_mutating_existing_records`; `approved_lc1_sequence_preserves_identity_and_registry_bound`; `invalid_transition_is_typed_and_leaves_state_unchanged`; `two_lifecycle_records_transition_independently`; `two_applications_complete_lifecycle_with_running_work`; `unknown_identity_is_rejected_before_application_code_runs`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-003 | Implemented: inline payload bound, immutable positive-capacity topology, registration-order routing, cross-topic FIFO, reject-newest saturation, unaffected fan-out, ordered outcomes, lifecycle unavailability, exact stop/failure clearing, empty restart reconnection, and running-only one-message application dispatch through a publish-only context | Preserve exact-boundary routing, lifecycle ownership, oldest-only dispatch, self-publication, refreshed availability, and callback-error regressions | `inline_payload_accepts_its_exact_limit_and_rejects_one_more_byte`; `topology_validation_rejects_invalid_inboxes_without_a_partial_bus`; `reject_newest_saturation_preserves_older_entries_and_healthy_fan_out`; `publication_distinguishes_all_full_from_no_subscribers`; `one_inbox_preserves_cross_topic_publish_order`; `selective_routing_skips_nonmatching_endpoints_and_preserves_route_order`; `registered_subscribers_are_known_but_unavailable`; `stop_clears_one_inbox_and_restart_reconnects_it_empty`; `lifecycle_gate_and_empty_inbox_never_invoke_application_code`; `dispatch_presents_one_oldest_message_per_call`; `self_publication_uses_the_freed_slot_and_preserves_peer_outcomes`; `dispatch_refreshes_peer_availability_after_lifecycle_change`; `callback_error_clears_selected_queue_but_retains_peer_publication`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-004 | Implemented at the finite one-shot boundary: ordered `FrameworkInstant`, injected `Clock`, explicit `ManualClock` advancement, typed non-mutating overflow, fixed nondecreasing `WorkSchedule`, stable equal-time order, one due or overdue attempt per call, exact lifecycle/error outcomes, and identical replayed work/lifecycle/event-timestamp traces | Preserve manual-clock, schedule-order, one-item, lifecycle/error, and complete replay regressions; evaluate recurrence only through a separate decision | `manual_clock_starts_at_origin_and_reads_do_not_advance`; `manual_clock_advances_only_by_explicit_durations`; `advance_overflow_is_typed_and_preserves_current_time`; `schedule_rejects_descending_instants_and_accepts_an_empty_agenda`; `complete_reads_no_clock_and_each_pending_item_decision_reads_once`; `waiting_preserves_the_item_and_manual_clock_until_its_inclusive_instant`; `equal_time_order_is_stable_and_overdue_work_runs_one_item_per_call`; `lifecycle_rejection_consumes_one_item_without_blocking_a_due_peer`; `returned_work_error_consumes_one_item_without_blocking_a_due_peer`; `identical_manual_scenarios_replay_the_same_work_and_event_timestamp_trace`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-005 | Implemented at the direct cooperative returned-work boundary: structured source, error severity, mission-defined copied identifier, one injected timestamp read, positive-capacity FIFO queueing, caller-visible reject-newest saturation, and the exact rejected event retained for explicit handling | Preserve structured-field, exact-capacity, single-attempt, no-event-on-success/rejection, saturation, retry, and timestamp regressions; add broader event paths only through separate decisions | `event_exposes_structured_fields_without_text_parsing`; `queue_requires_positive_capacity`; `queue_accepts_its_exact_limit_and_dequeues_fifo`; `reject_newest_preserves_older_events_and_allows_explicit_retry`; `returned_work_error_records_clock_captured_event_and_preserves_peer_progress`; `saturated_failure_event_retains_error_and_exact_event_for_retry`; `lifecycle_rejections_do_not_read_clock_or_emit`; `successful_work_does_not_read_clock_or_emit`; `cargo test --workspace --all-features` (58 passed) | `a04c5bd3f929934b7578b14f181138ae0be56e9b` | Verified |
| RFF-REQ-006 | Not implemented | Configuration state-machine and activation/rejection/rollback integration tests | None | - | Not verified |
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
