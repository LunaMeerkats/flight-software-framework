//! Evidence for the same finite scenario executed by the host-echo binary.

use std::error::Error;
use std::time::Duration;

use rust_flight_framework::{
    ApplicationState, ConfigurationError, Event, EventEmitOutcome, EventSeverity, EventSource,
    EventTimestamp, FrameworkInstant, RuntimeConfigurationError, RuntimeWorkError, ScheduledWork,
    ScheduledWorkOutcome,
};

#[path = "../examples/host-echo/mission.rs"]
mod mission;
#[path = "../examples/host-echo/sample.rs"]
mod sample;

use mission::IngestError;
use mission::codec::ValidationError;
use mission::work::{MissionConfigurationError, MissionWorkError, WorkObservation};
use sample::{SampleEvent, run};

#[test]
fn combined_sample_runs_both_lifecycles_and_preserves_exact_adapter_records() {
    let report = run().unwrap();
    assert_ne!(report.applications[0], report.applications[1]);
    assert_eq!(
        report.lifecycle,
        [
            [ApplicationState::Registered; 2],
            [ApplicationState::Running; 2],
            [ApplicationState::Stopped; 2],
            [ApplicationState::Running; 2],
        ]
    );
    assert_eq!(
        report.malformed_command,
        IngestError::Validation(ValidationError::UnknownCommand { actual: 0xff })
    );
    assert_eq!(report.telemetry, [[0x81, 0], [0x81, 42], [0x81, 100]]);
}

#[test]
fn combined_sample_waits_then_runs_equal_time_work_with_observed_configuration() {
    let report = run().unwrap();
    let due = FrameworkInstant::from_elapsed(Duration::from_millis(10));
    let [echo, telemetry] = report.applications.map(|id| ScheduledWork::new(id, due));
    assert_eq!(
        report.schedule,
        [
            ScheduledWorkOutcome::Waiting {
                next: echo,
                observed_at: FrameworkInstant::from_elapsed(Duration::ZERO),
            },
            ScheduledWorkOutcome::Completed {
                scheduled_work: echo,
                observed_at: due,
                state: ApplicationState::Running,
            },
            ScheduledWorkOutcome::Completed {
                scheduled_work: telemetry,
                observed_at: due,
                state: ApplicationState::Running,
            },
            ScheduledWorkOutcome::Complete,
        ]
    );
    assert_eq!(
        report.scheduled_observations,
        [WorkObservation {
            revision: 1,
            value: 10
        }; 2]
    );
}

#[test]
fn combined_sample_observes_rejection_rollback_revision_non_reuse_and_restart_retention() {
    let report = run().unwrap();
    assert_eq!(report.configuration.revisions, [2, 1, 3]);
    assert_eq!(
        report.configuration.observations,
        [
            WorkObservation {
                revision: 2,
                value: 20
            },
            WorkObservation {
                revision: 2,
                value: 20
            },
            WorkObservation {
                revision: 1,
                value: 10
            },
            WorkObservation {
                revision: 3,
                value: 30
            },
        ]
    );
    assert_eq!(
        report.configuration.rejection,
        RuntimeConfigurationError::Table(ConfigurationError::Rejected(
            MissionConfigurationError::ValueOutOfRange { actual: 101 }
        ))
    );
    assert_eq!(
        report.configuration.consumed_rollback,
        RuntimeConfigurationError::Table(ConfigurationError::NoRollbackAvailable)
    );
    assert_eq!(
        report.restarted_observations,
        [WorkObservation {
            revision: 3,
            value: 30
        }; 2]
    );
}

#[test]
fn combined_sample_records_exact_failure_and_preserves_peer_work_and_queued_telemetry() {
    let report = run().unwrap();
    let fault = &report.fault;
    assert_eq!(fault.failure.discarded_deliveries(), 1);
    assert_eq!(
        fault.failure.operation_error().work_error(),
        &RuntimeWorkError::Application {
            application_id: report.applications[0],
            source: MissionWorkError::InjectedEchoFailure,
        }
    );
    let original = fault
        .failure
        .source()
        .unwrap()
        .source()
        .unwrap()
        .source()
        .unwrap();
    assert_eq!(
        original.downcast_ref::<MissionWorkError>(),
        Some(&MissionWorkError::InjectedEchoFailure)
    );
    assert_eq!(fault.pending_after_failure, [0, 1]);
    assert_eq!(
        fault.observations,
        [WorkObservation {
            revision: 3,
            value: 30
        }; 2]
    );
    assert_eq!(fault.peer_work, ApplicationState::Running);
    assert_eq!(fault.retained_telemetry, [0x81, 7]);
    assert_eq!(
        fault.event,
        Event::new(
            EventSource::Application(report.applications[0]),
            EventSeverity::Error,
            SampleEvent::EchoWorkFailed,
            EventTimestamp::from_elapsed(Duration::from_millis(20)),
        )
    );
    let attempt = fault
        .failure
        .operation_error()
        .failure_event_attempt()
        .unwrap();
    assert_eq!(attempt.event(), &fault.event);
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(fault.next_event, None);
    assert_eq!(
        fault.final_states,
        [ApplicationState::Failed, ApplicationState::Running]
    );
}

#[test]
fn combined_sample_replays_the_complete_fixed_observable_report() {
    assert_eq!(run().unwrap(), run().unwrap());
}
