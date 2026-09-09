# Verification traceability

The implemented Rust slices cover bounded logical lifecycle records, owned
synchronous start/work/stop/in-place-restart, bounded application messaging, a
bounded structured-event queue, an injected manual clock, a finite one-shot
work agenda, direct returned-work failure-event reporting, and one bounded
runtime configuration lifecycle. Public integration tests satisfy the complete
two-application RFF-REQ-002 scenario.
The routing core, lifecycle-owned availability, and caller-selected one-message
dispatch together verify RFF-REQ-003. The clock, one-item due-work operation,
stable equal-time ordering, and replayed work/lifecycle/event-timestamp trace
verify RFF-REQ-004. The runtime-generated event, bounded queue, failed state,
preserved work error, saturation outcome, and later peer work verify RFF-REQ-005
and RFF-REQ-008 only at the direct cooperative returned-work boundary. The
standalone table and runtime work-context evidence together verify RFF-REQ-006
at the bounded in-memory ordinary-work boundary.
Implementation and verification state remain separate so planned evidence is
not represented as completed behavior.

The standalone configuration implementation is
`7dd237626f6a1207b6abffe3bee98830e8a9ada7`; runtime integration is
`9afc85686de192d66e36af950b1b63a29ca541ca`. The 2026-09-02 run passes all 83
production tests and the required Cargo baseline. Requirement rows retain exact
test names and the implementation commit containing their current evidence.

## Configuration integration evidence

[ADR-0018](../adr/0018-configuration-aware-work-context.md) implements one
ordinary work context and optional configuration owned at construction. Ten
public integration tests cover direct, scheduled, failure-event, and
messaging-owned work plus construction, lifecycle, revision, rollback, and
error boundaries. The separately invoked
[borrowing experiment](CONFIGURATION_CONTEXT_EXPERIMENT.md) now exercises one
positive owner/view probe and four intended compiler rejections. Its commands,
diagnostics, and limits are recorded there. Runtime tests establish behavior;
the probes establish only the selected borrowing constraints.

## Requirement evidence

The 2026-09-09 [host boundary implementation](../adr/0019-host-command-telemetry-boundary.md)
adds a private `host-echo` example and 13 integration tests loading its exact
mission source. `cargo test --test host_adapters` passes all 13; the full Cargo
baseline passes 96 tests, and `cargo run --example host-echo` prints the exact
documented output for 0, 42, and 100 percent. Implementation and test evidence
are in `2c337f311da7d62230d626bd10c7fbe1383b586e`. Existing evidence hashes and
their historical test counts below remain unchanged.

The adapter tests cover all 101 accepted percentages, all 255 unsupported
command identifiers, all 155 invalid percentage values, overlapping validation
faults, and rejection with occupied retained state. They assert staged dispatch,
original ordered reports, full/unavailable/absent routes, host-output slot
reuse, stop/restart retention, terminal full-output failure with zero remaining
queued discards, and replay. Wrong-topic and no-subscriber tests explicitly use
alternate subscriptions; oversized internal payloads fail message construction.
Source review confirms one business call after validation and original
allocation-error preservation; neither is an injected allocation-failure test.

The separately invoked [mailbox experiment](HOST_MAILBOX_EXPERIMENT.md) still
passes one executable with three scenarios, plus extracted rustfmt and Clippy.
Its two-slot clearing fixture remains distinct from the real mission's one-slot
inboxes. Adapter evidence does not complete the integrated v0.1 service sample,
CI, physical delivery, or architecture review.

| Requirement | Implementation state | Planned verification | Exact verification evidence | Last verified commit | Verification state |
| --- | --- | --- | --- | --- | --- |
| RFF-REQ-001 | Implemented for current entry points; future entry points remain in scope | Documented human checklist covering user-facing docs and sample output | None | - | Pending |
| RFF-REQ-002 | Implemented: bounded logical LC1 transitions plus two statically composed applications completing owned registration, start, work, stop, restart, and work | Preserve exhaustive lifecycle and complete two-application scenario regression coverage | `lifecycle::tests::registry_enforces_every_state_operation_pair`; `configured_capacity_bounds_registration_without_mutating_existing_records`; `approved_lc1_sequence_preserves_identity_and_registry_bound`; `invalid_transition_is_typed_and_leaves_state_unchanged`; `two_lifecycle_records_transition_independently`; `two_applications_complete_lifecycle_with_running_work`; `unknown_identity_is_rejected_before_application_code_runs`; `cargo test --workspace --all-features` (83 passed) | `9afc85686de192d66e36af950b1b63a29ca541ca` | Verified |
| RFF-REQ-003 | Implemented: inline payload bound, immutable positive-capacity topology, registration-order routing, cross-topic FIFO, reject-newest saturation, unaffected fan-out, ordered outcomes, lifecycle unavailability, exact stop/failure clearing, empty restart reconnection, and running-only one-message application dispatch through a publish-only context | Preserve exact-boundary routing, lifecycle ownership, oldest-only dispatch, self-publication, refreshed availability, and callback-error regressions | `inline_payload_accepts_its_exact_limit_and_rejects_one_more_byte`; `topology_validation_rejects_invalid_inboxes_without_a_partial_bus`; `reject_newest_saturation_preserves_older_entries_and_healthy_fan_out`; `publication_distinguishes_all_full_from_no_subscribers`; `one_inbox_preserves_cross_topic_publish_order`; `selective_routing_skips_nonmatching_endpoints_and_preserves_route_order`; `registered_subscribers_are_known_but_unavailable`; `stop_clears_one_inbox_and_restart_reconnects_it_empty`; `lifecycle_gate_and_empty_inbox_never_invoke_application_code`; `dispatch_presents_one_oldest_message_per_call`; `self_publication_uses_the_freed_slot_and_preserves_peer_outcomes`; `dispatch_refreshes_peer_availability_after_lifecycle_change`; `callback_error_clears_selected_queue_but_retains_peer_publication`; `cargo test --workspace --all-features` (83 passed) | `9afc85686de192d66e36af950b1b63a29ca541ca` | Verified |
| RFF-REQ-004 | Implemented at the finite one-shot boundary: ordered `FrameworkInstant`, injected `Clock`, explicit `ManualClock` advancement, typed non-mutating overflow, fixed nondecreasing `WorkSchedule`, stable equal-time order, one due or overdue attempt per call, exact lifecycle/error outcomes, and identical replayed work/lifecycle/event-timestamp traces | Preserve manual-clock, schedule-order, one-item, lifecycle/error, and complete replay regressions; evaluate recurrence only through a separate decision | `manual_clock_starts_at_origin_and_reads_do_not_advance`; `manual_clock_advances_only_by_explicit_durations`; `advance_overflow_is_typed_and_preserves_current_time`; `schedule_rejects_descending_instants_and_accepts_an_empty_agenda`; `complete_reads_no_clock_and_each_pending_item_decision_reads_once`; `waiting_preserves_the_item_and_manual_clock_until_its_inclusive_instant`; `equal_time_order_is_stable_and_overdue_work_runs_one_item_per_call`; `lifecycle_rejection_consumes_one_item_without_blocking_a_due_peer`; `returned_work_error_consumes_one_item_without_blocking_a_due_peer`; `identical_manual_scenarios_replay_the_same_work_and_event_timestamp_trace`; `cargo test --workspace --all-features` (83 passed) | `9afc85686de192d66e36af950b1b63a29ca541ca` | Verified |
| RFF-REQ-005 | Implemented at the direct cooperative returned-work boundary: structured source, error severity, mission-defined copied identifier, one injected timestamp read, positive-capacity FIFO queueing, caller-visible reject-newest saturation, and the exact rejected event retained for explicit handling | Preserve structured-field, exact-capacity, single-attempt, no-event-on-success/rejection, saturation, retry, and timestamp regressions; add broader event paths only through separate decisions | `event_exposes_structured_fields_without_text_parsing`; `queue_requires_positive_capacity`; `queue_accepts_its_exact_limit_and_dequeues_fifo`; `reject_newest_preserves_older_events_and_allows_explicit_retry`; `returned_work_error_records_clock_captured_event_and_preserves_peer_progress`; `saturated_failure_event_retains_error_and_exact_event_for_retry`; `lifecycle_rejections_do_not_read_clock_or_emit`; `successful_work_does_not_read_clock_or_emit`; `cargo test --workspace --all-features` (83 passed) | `9afc85686de192d66e36af950b1b63a29ca541ca` | Verified |
| RFF-REQ-006 | Implemented at the bounded in-memory runtime boundary: validated immutable snapshots, typed rejection without mutation, never-reused revisions, consume-once rollback, constructor ownership recovery, optional ordinary-work visibility, restart retention, and no automatic rollback after returned errors | Preserve standalone transition/exhaustion tests and configured direct, scheduled, failure-event, messaging, lifecycle, ownership, error, and borrowing regressions; select any host loader separately | `initial_configuration_is_validated_and_has_no_rollback_history`; `rejected_replacements_preserve_active_history_and_revision_allocation`; `rollback_is_consumed_once_and_does_not_reuse_issued_revisions`; `configuration::tests::final_revision_and_exhaustion_preserve_snapshots_and_high_water`; `unconfigured_work_reports_absence_and_rejects_table_operations`; `zero_byte_configured_runtime_is_distinct_from_absence`; `work_context_exposes_only_the_used_configuration_prefix`; `configured_construction_failures_return_the_unchanged_table_lineage`; `work_observes_activation_rejection_rollback_and_revision_non_reuse`; `lifecycle_rejection_suppresses_work_and_restart_retains_configuration_history`; `returned_work_error_retains_configuration_history_and_peer_progress`; `scheduled_work_observes_the_active_configuration_through_runtime_work`; `failure_event_work_observes_configuration_and_retains_history`; `messaging_work_preserves_configuration_and_failed_inbox_cleanup`; `cargo test --test configuration_table` (12 passed); `cargo test --test configuration_runtime` (10 passed); standalone rustdoc experiment (1 executable and 4 compile-fail probes passed); `cargo test --workspace --all-features` (83 passed) | `9afc85686de192d66e36af950b1b63a29ca541ca` | Verified |
| RFF-REQ-007 | Implemented at the local caller-framed slice/returned-array boundary: private validated codec, explicit ingress report, two message applications, separate dispatch, and capacity-one host output | Preserve exact command/telemetry, malformed-input, publication, bounded-output, lifecycle, and replay evidence; integrate into the full service sample separately | `every_valid_percentage_requires_separate_dispatch_and_consume_once_drain`; `malformed_host_records_preserve_occupied_inbox_and_output_with_exact_precedence`; `internal_payload_validation_precedes_business_logic_and_full_mailbox`; `full_telemetry_destination_preserves_report_and_older_peer_delivery`; `full_host_output_preserves_older_record_and_drain_does_not_recover_failure`; `host_output_survives_stop_restart_while_queued_telemetry_is_cleared`; `identical_ordered_inputs_dispatches_and_drains_replay_identically`; `cargo test --test host_adapters` (13 passed); `cargo test --workspace --all-features` (96 passed); `cargo run --example host-echo` (documented output matched) | `2c337f311da7d62230d626bd10c7fbe1383b586e` | Verified at selected adapter boundary |
| RFF-REQ-008 | Implemented at the selected cooperative returned-work fault boundary: the complete concrete error is preserved through its source chain, only the selected application enters terminal `Failed`, one clock-captured bounded event is attempted with explicit saturation, and a healthy peer completes later work | Preserve failed-state, original-error, event-attempt, saturation, no-duplicate, and peer-progress regressions; do not extend this verification to panics, hangs, cleanup, or other callback event paths without separate evidence | `returned_work_error_fails_only_the_selected_application`; `returned_work_error_records_clock_captured_event_and_preserves_peer_progress`; `saturated_failure_event_retains_error_and_exact_event_for_retry`; `cargo test --workspace --all-features` (83 passed) | `9afc85686de192d66e36af950b1b63a29ca541ca` | Verified |

## Messaging-owned work event evidence

The 2026-09-10 [ADR-0020](../adr/0020-messaging-work-failure-events.md)
increment adds `MessagingRuntime::work_with_failure_event(...)` and six focused
public tests. `cargo test --test messaging_work_events` passes all six;
`cargo test --workspace --all-features` passes 102 tests. The complete required
Cargo baseline also passes. The implementation commit is recorded after the
validated increment is committed locally; the table above retains its earlier
committed baselines.

Additional evidence for RFF-REQ-003, RFF-REQ-005, RFF-REQ-006, and RFF-REQ-008:

- `returned_work_error_clears_selected_inbox_and_preserves_peer_dispatch`
  proves two selected queued deliveries are discarded, the exact work error
  and recorded event are retained, and the peer later dispatches its unchanged
  FIFO contents and completes work.
- `saturated_event_preserves_original_error_and_exact_event_for_explicit_retry`
  proves a full event queue preserves the older event, original error/source
  chain, exact rejected event and timestamp, inbox cleanup, and peer work.
- `successful_work_preserves_inboxes_without_reading_clock_or_emitting`,
  `non_running_work_preserves_peer_inbox_and_suppresses_event_effects`, and
  `unknown_work_preserves_all_inboxes_without_clock_or_event_effects` prove
  success and lifecycle rejection do not cause reporting effects.
- `failed_work_observes_active_configuration_without_automatic_rollback`
  proves active configuration visibility during failure, retained peer
  visibility/history, rejection, consume-once rollback, and revision non-reuse.

Source review establishes cleanup-before-clock ordering. The tests assert
post-return state, counts, and effects; they do not inspect the exclusively
borrowed runtime during clock invocation. No message-callback or scheduled
event behavior, full-service sample, CI result, or arbitrary fault containment
is established by this increment.

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
