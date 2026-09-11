//! Public-runtime evidence using the executable example's private adapters.
//!
//! Alternate subscriptions expose otherwise unreachable wrong-topic and absent
//! subscriber cases. They do not replace the fixed mission topology.

use std::error::Error;

use rust_flight_framework::{
    ApplicationId, ApplicationState, DeliveryStatus, LifecycleError, Message, MessageCreateError,
    MessageDispatchError, MessageDispatchOutcome, MessagingOperationError, MessagingRuntime,
    PublishClassification, PublishReport, Runtime, RuntimeInboxConfig, RuntimeWorkError,
};

#[path = "../examples/host-echo/mission.rs"]
mod mission;

use mission::applications::{
    EchoApplication, MissionApplication, MissionMessageError, OutputMailbox, TelemetryApplication,
};
use mission::codec::ValidationError;
use mission::work::{
    MissionRole, MissionWorkError, WorkMonitor, WorkObservation, initial_configuration,
};
use mission::{ComposedMission, IngestError, MissionTopic, compose, ingest};

type DispatchFailure = MessagingOperationError<MessageDispatchError<MissionMessageError>>;

#[derive(Debug, Eq, PartialEq)]
struct ReplayTrace {
    publications: Vec<PublishClassification>,
    dispatches: Vec<MessageDispatchOutcome>,
    drains: Vec<Option<[u8; 2]>>,
    rejections: Vec<IngestError>,
    states: [ApplicationState; 2],
}

#[test]
fn composition_is_registered_and_enforces_the_selected_record_bounds() {
    let mailbox = OutputMailbox::default();
    let mission = compose(&mailbox, None).unwrap();
    assert_eq!(mission.runtime.capacity(), 2);
    for application_id in [mission.echo_id, mission.telemetry_id] {
        assert_eq!(
            mission.runtime.state(application_id),
            Ok(ApplicationState::Registered)
        );
        assert_eq!(mission.runtime.inbox_capacity(application_id), Ok(1));
        assert_eq!(mission.runtime.pending(application_id), Ok(0));
    }
    assert_eq!(mailbox.drain(), None);
    assert_eq!(
        Message::<MissionTopic, 1>::try_new(MissionTopic::EchoCommand, &[0, 1]),
        Err(MessageCreateError::PayloadTooLong {
            length: 2,
            maximum: 1
        })
    );
}

#[test]
fn observed_work_records_configuration_and_injects_only_one_echo_failure() {
    let mailbox = OutputMailbox::default();
    let monitor = WorkMonitor::default();
    let mut mission = compose(&mailbox, Some(&monitor)).unwrap();
    start_both(&mut mission);
    for role in [MissionRole::Echo, MissionRole::Telemetry] {
        assert_eq!(monitor.take_observation(role), None);
    }
    monitor.arm_echo_failure();
    assert_eq!(
        mission.runtime.work(mission.telemetry_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(monitor.take_observation(MissionRole::Echo), None);
    let failure = mission.runtime.work(mission.echo_id).unwrap_err();
    assert_eq!(
        failure.operation_error(),
        &RuntimeWorkError::Application {
            application_id: mission.echo_id,
            source: MissionWorkError::InjectedEchoFailure,
        }
    );
    for role in [MissionRole::Echo, MissionRole::Telemetry] {
        assert_eq!(
            monitor.take_observation(role),
            Some(WorkObservation {
                revision: 1,
                value: 10,
            })
        );
        assert_eq!(monitor.take_observation(role), None);
    }
    drop(mission);
    let mut fresh_mission = compose(&mailbox, Some(&monitor)).unwrap();
    start_both(&mut fresh_mission);
    assert_eq!(
        fresh_mission.runtime.work(fresh_mission.echo_id),
        Ok(ApplicationState::Running)
    );
}

#[test]
fn every_valid_percentage_requires_separate_dispatch_and_consume_once_drain() {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    for value in 0..=100 {
        let report = ingest(&mut mission.runtime, &[0x01, value]).unwrap();
        assert_report(
            &report,
            PublishClassification::Complete,
            &[(mission.echo_id, DeliveryStatus::Delivered)],
        );
        assert_eq!(mission.runtime.pending(mission.echo_id), Ok(1));
        assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(0));
        assert_eq!(mailbox.drain(), None);
        assert_eq!(
            mission.runtime.dispatch_one(mission.telemetry_id),
            Ok(MessageDispatchOutcome::InboxEmpty)
        );

        dispatch(mission.echo_id, &mut mission);
        assert_eq!(mission.runtime.pending(mission.echo_id), Ok(0));
        assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(1));
        assert_eq!(mailbox.drain(), None);
        assert_eq!(
            mission.runtime.dispatch_one(mission.echo_id),
            Ok(MessageDispatchOutcome::InboxEmpty)
        );
        dispatch(mission.telemetry_id, &mut mission);
        assert_eq!(mailbox.drain(), Some([0x81, value]));
        assert_eq!(mailbox.drain(), None);
        assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(0));
        assert_eq!(
            mission.runtime.dispatch_one(mission.telemetry_id),
            Ok(MessageDispatchOutcome::InboxEmpty)
        );
        for application_id in [mission.echo_id, mission.telemetry_id] {
            assert_eq!(
                mission.runtime.state(application_id),
                Ok(ApplicationState::Running)
            );
        }
    }
}

fn assert_host_rejected(
    mission: &mut ComposedMission<'_>,
    record: &[u8],
    expected: ValidationError,
) {
    let error = ingest(&mut mission.runtime, record).unwrap_err();
    assert_eq!(error, IngestError::Validation(expected));
    assert_eq!(
        Error::source(&error).and_then(|source| source.downcast_ref()),
        Some(&expected)
    );
    assert_eq!(mission.runtime.pending(mission.echo_id), Ok(1));
    assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(0));
    for application_id in [mission.echo_id, mission.telemetry_id] {
        assert_eq!(
            mission.runtime.state(application_id),
            Ok(ApplicationState::Running)
        );
    }
}

#[test]
fn malformed_host_records_preserve_occupied_inbox_and_output_with_exact_precedence() {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    fill_output(&mut mission, 7);
    ingest(&mut mission.runtime, &[0x01, 11]).unwrap();
    let oversized = [0xff; 4096];
    for record in [
        &[][..],
        &[0xff],
        &[0xff, 0xff, 0xff],
        &[1, 7, 1, 8],
        &oversized,
    ] {
        assert_host_rejected(
            &mut mission,
            record,
            ValidationError::HostLength {
                actual: record.len(),
            },
        );
    }
    for identifier in 0..=u8::MAX {
        if identifier != 0x01 {
            for value in [0, 101, 255] {
                assert_host_rejected(
                    &mut mission,
                    &[identifier, value],
                    ValidationError::UnknownCommand { actual: identifier },
                );
            }
        }
    }
    for value in 101..=u8::MAX {
        assert_host_rejected(
            &mut mission,
            &[0x01, value],
            ValidationError::PercentOutOfRange { actual: value },
        );
    }
    assert_eq!(mailbox.drain(), Some([0x81, 7]));
    dispatch(mission.echo_id, &mut mission);
    dispatch(mission.telemetry_id, &mut mission);
    assert_eq!(mailbox.drain(), Some([0x81, 11]));
    assert_eq!(mailbox.drain(), None);
}

#[test]
fn ingress_saturation_returns_full_report_and_preserves_older_command() {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    ingest(&mut mission.runtime, &[0x01, 41]).unwrap();
    let rejected = ingest(&mut mission.runtime, &[0x01, 42]).unwrap();
    assert_report(
        &rejected,
        PublishClassification::WhollyUndelivered,
        &[(mission.echo_id, DeliveryStatus::InboxFull)],
    );
    assert_eq!(mission.runtime.pending(mission.echo_id), Ok(1));
    assert_eq!(mailbox.drain(), None);
    dispatch(mission.echo_id, &mut mission);
    dispatch(mission.telemetry_id, &mut mission);
    assert_eq!(mailbox.drain(), Some([0x81, 41]));
    assert_eq!(
        mission.runtime.dispatch_one(mission.echo_id),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    assert_eq!(
        mission.runtime.dispatch_one(mission.telemetry_id),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    assert_eq!(mailbox.drain(), None);
}

#[test]
fn unavailable_ingress_and_absent_subscriber_keep_distinct_reports() {
    let mailbox = OutputMailbox::default();
    let mut mission = compose(&mailbox, None).unwrap();
    let registered = ingest(&mut mission.runtime, &[0x01, 42]).unwrap();
    assert_report(
        &registered,
        PublishClassification::WhollyUndelivered,
        &[(mission.echo_id, DeliveryStatus::Unavailable)],
    );
    start_both(&mut mission);
    mission.runtime.stop(mission.echo_id).unwrap();
    let stopped = ingest(&mut mission.runtime, &[0x01, 42]).unwrap();
    assert_eq!(stopped, registered);
    assert_eq!(mission.runtime.pending(mission.echo_id), Ok(0));
    assert_eq!(mailbox.drain(), None);

    let mut no_subscriber =
        alternative_subscriptions(&mailbox, &[], &[MissionTopic::EchoTelemetry]);
    let absent = ingest(&mut no_subscriber.runtime, &[0x01, 42]).unwrap();
    assert_report(&absent, PublishClassification::NoSubscribers, &[]);
    assert_eq!(no_subscriber.runtime.pending(no_subscriber.echo_id), Ok(0));
    assert_eq!(
        no_subscriber.runtime.pending(no_subscriber.telemetry_id),
        Ok(0)
    );
    assert_eq!(mailbox.drain(), None);
}

fn assert_internal_rejected(
    mission: &mut ComposedMission<'_>,
    application_id: ApplicationId,
    topic: MissionTopic,
    payload: &[u8],
    expected: ValidationError,
) {
    let message = Message::try_new(topic, payload).unwrap();
    let report = mission.runtime.publish(&message).unwrap();
    assert_report(
        &report,
        PublishClassification::Complete,
        &[(application_id, DeliveryStatus::Delivered)],
    );
    let error = mission.runtime.dispatch_one(application_id).unwrap_err();
    assert_application_failure(
        &error,
        application_id,
        MissionMessageError::Validation(expected),
    );
    assert_eq!(mission.runtime.pending(application_id), Ok(0));
    assert_eq!(
        mission.runtime.state(application_id),
        Ok(ApplicationState::Failed)
    );
    let peer_id = if application_id == mission.echo_id {
        mission.telemetry_id
    } else {
        mission.echo_id
    };
    assert_eq!(mission.runtime.pending(peer_id), Ok(0));
    assert_eq!(
        mission.runtime.state(peer_id),
        Ok(ApplicationState::Running)
    );
}

fn reject_internal_payload(topic: MissionTopic, payload: &[u8], expected: ValidationError) {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    fill_output(&mut mission, 7);
    let application_id = match topic {
        MissionTopic::EchoCommand => mission.echo_id,
        MissionTopic::EchoTelemetry => mission.telemetry_id,
    };
    assert_internal_rejected(&mut mission, application_id, topic, payload, expected);
    assert_eq!(mailbox.drain(), Some([0x81, 7]));
    assert_eq!(mailbox.drain(), None);
}

#[test]
fn internal_payload_validation_precedes_business_logic_and_full_mailbox() {
    for topic in [MissionTopic::EchoCommand, MissionTopic::EchoTelemetry] {
        reject_internal_payload(topic, &[], ValidationError::PayloadLength { actual: 0 });
        for value in 101..=u8::MAX {
            reject_internal_payload(
                topic,
                &[value],
                ValidationError::PercentOutOfRange { actual: value },
            );
        }
    }
}

#[test]
fn wrong_internal_topic_precedes_payload_length_value_and_full_mailbox() {
    for expected in [MissionTopic::EchoCommand, MissionTopic::EchoTelemetry] {
        for payload in [&[][..], &[101][..]] {
            let mailbox = OutputMailbox::default();
            fill_output(&mut running(&mailbox), 7);
            let (echo_topics, telemetry_topics) = match expected {
                MissionTopic::EchoCommand => (&[MissionTopic::EchoTelemetry][..], &[][..]),
                MissionTopic::EchoTelemetry => (&[][..], &[MissionTopic::EchoCommand][..]),
            };
            let mut mission = alternative_subscriptions(&mailbox, echo_topics, telemetry_topics);
            let (application_id, actual) = match expected {
                MissionTopic::EchoCommand => (mission.echo_id, MissionTopic::EchoTelemetry),
                MissionTopic::EchoTelemetry => (mission.telemetry_id, MissionTopic::EchoCommand),
            };
            assert_internal_rejected(
                &mut mission,
                application_id,
                actual,
                payload,
                ValidationError::UnexpectedTopic { expected, actual },
            );
            assert_eq!(mailbox.drain(), Some([0x81, 7]));
        }
    }
}

#[test]
fn full_telemetry_destination_preserves_report_and_older_peer_delivery() {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    queue_telemetry(&mut mission, 11);
    let telemetry = Message::try_new(MissionTopic::EchoTelemetry, &[22]).unwrap();
    let expected_report = mission.runtime.publish(&telemetry).unwrap();
    assert_report(
        &expected_report,
        PublishClassification::WhollyUndelivered,
        &[(mission.telemetry_id, DeliveryStatus::InboxFull)],
    );
    ingest(&mut mission.runtime, &[0x01, 22]).unwrap();
    let error = mission.runtime.dispatch_one(mission.echo_id).unwrap_err();
    assert_application_failure(
        &error,
        mission.echo_id,
        MissionMessageError::TelemetryDelivery(expected_report),
    );
    assert_eq!(
        mission.runtime.state(mission.echo_id),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(mission.runtime.pending(mission.echo_id), Ok(0));
    assert_eq!(
        mission.runtime.state(mission.telemetry_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(1));
    dispatch(mission.telemetry_id, &mut mission);
    assert_eq!(mailbox.drain(), Some([0x81, 11]));
    assert_eq!(mailbox.drain(), None);
}

#[test]
fn unavailable_telemetry_preserves_report_and_does_not_retry_after_restart() {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    mission.runtime.stop(mission.telemetry_id).unwrap();
    let telemetry = Message::try_new(MissionTopic::EchoTelemetry, &[22]).unwrap();
    let expected_report = mission.runtime.publish(&telemetry).unwrap();
    assert_report(
        &expected_report,
        PublishClassification::WhollyUndelivered,
        &[(mission.telemetry_id, DeliveryStatus::Unavailable)],
    );
    ingest(&mut mission.runtime, &[0x01, 22]).unwrap();
    let error = mission.runtime.dispatch_one(mission.echo_id).unwrap_err();
    assert_application_failure(
        &error,
        mission.echo_id,
        MissionMessageError::TelemetryDelivery(expected_report),
    );
    assert_eq!(
        mission.runtime.state(mission.echo_id),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(
        mission.runtime.state(mission.telemetry_id),
        Ok(ApplicationState::Stopped)
    );
    assert_eq!(
        mission.runtime.restart(mission.telemetry_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        mission.runtime.dispatch_one(mission.telemetry_id),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    assert_eq!(mailbox.drain(), None);
}

#[test]
fn absent_telemetry_subscription_is_a_concrete_terminal_delivery_failure() {
    let mailbox = OutputMailbox::default();
    let mut mission = alternative_subscriptions(&mailbox, &[MissionTopic::EchoCommand], &[]);
    let telemetry = Message::try_new(MissionTopic::EchoTelemetry, &[22]).unwrap();
    let expected_report = mission.runtime.publish(&telemetry).unwrap();
    assert_report(&expected_report, PublishClassification::NoSubscribers, &[]);
    ingest(&mut mission.runtime, &[0x01, 22]).unwrap();
    let error = mission.runtime.dispatch_one(mission.echo_id).unwrap_err();
    assert_application_failure(
        &error,
        mission.echo_id,
        MissionMessageError::TelemetryDelivery(expected_report),
    );
    assert_eq!(
        mission.runtime.state(mission.echo_id),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(
        mission.runtime.state(mission.telemetry_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(0));
    assert_eq!(mailbox.drain(), None);
}

#[test]
fn full_host_output_preserves_older_record_and_drain_does_not_recover_failure() {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    fill_output(&mut mission, 11);
    queue_telemetry(&mut mission, 22);
    let error = mission
        .runtime
        .dispatch_one(mission.telemetry_id)
        .unwrap_err();
    assert_application_failure(
        &error,
        mission.telemetry_id,
        MissionMessageError::OutputFull { rejected: 22 },
    );
    assert_eq!(
        mission.runtime.state(mission.telemetry_id),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(0));
    assert_eq!(
        mission.runtime.state(mission.echo_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        mission.runtime.work(mission.echo_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(mailbox.drain(), Some([0x81, 11]));
    assert_eq!(mailbox.drain(), None);
    let rejected = mission
        .runtime
        .dispatch_one(mission.telemetry_id)
        .unwrap_err();
    assert_eq!(
        rejected.operation_error(),
        &MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
            application_id: mission.telemetry_id,
            state: ApplicationState::Failed,
        })
    );
    assert_eq!(rejected.discarded_deliveries(), 0);
    assert_eq!(mailbox.drain(), None);
}

#[test]
fn host_output_survives_stop_restart_while_queued_telemetry_is_cleared() {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    fill_output(&mut mission, 7);
    queue_telemetry(&mut mission, 11);
    let stopped = mission.runtime.stop(mission.telemetry_id).unwrap();
    assert_eq!(stopped.discarded_deliveries(), 1);
    assert_eq!(mission.runtime.pending(mission.telemetry_id), Ok(0));
    assert_eq!(
        mission.runtime.state(mission.telemetry_id),
        Ok(ApplicationState::Stopped)
    );
    mission.runtime.stop(mission.echo_id).unwrap();
    for application_id in [mission.echo_id, mission.telemetry_id] {
        assert_eq!(
            mission.runtime.restart(application_id),
            Ok(ApplicationState::Running)
        );
    }
    assert_eq!(mailbox.drain(), Some([0x81, 7]));
    assert_eq!(mailbox.drain(), None);
    assert_eq!(
        mission.runtime.dispatch_one(mission.telemetry_id),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    fill_output(&mut mission, 22);
    assert_eq!(mailbox.drain(), Some([0x81, 22]));
}

#[test]
fn identical_ordered_inputs_dispatches_and_drains_replay_identically() {
    let first = replay();
    assert_eq!(first, replay());
    assert_eq!(first.publications, vec![PublishClassification::Complete; 4]);
    assert_eq!(
        first.dispatches,
        vec![MessageDispatchOutcome::Dispatched; 8]
    );
    assert_eq!(
        first.drains.iter().copied().flatten().collect::<Vec<_>>(),
        [[0x81, 0], [0x81, 42], [0x81, 100], [0x81, 42]]
    );
    assert_eq!(
        first.rejections,
        vec![IngestError::Validation(ValidationError::UnknownCommand { actual: 0xff }); 4]
    );
    assert_eq!(first.states, [ApplicationState::Running; 2]);
}

fn running(mailbox: &OutputMailbox) -> ComposedMission<'_> {
    let mut mission = compose(mailbox, None).expect("fixed mission topology is valid");
    start_both(&mut mission);
    mission
}

fn start_both(mission: &mut ComposedMission<'_>) {
    for application_id in [mission.echo_id, mission.telemetry_id] {
        assert_eq!(
            mission.runtime.start(application_id),
            Ok(ApplicationState::Running)
        );
    }
}

fn alternative_subscriptions<'a>(
    mailbox: &'a OutputMailbox,
    echo_topics: &[MissionTopic],
    telemetry_topics: &[MissionTopic],
) -> ComposedMission<'a> {
    let mut runtime = Runtime::with_configuration(2, initial_configuration().unwrap()).unwrap();
    let echo_id = runtime
        .register(MissionApplication::Echo(EchoApplication::new(None)))
        .unwrap();
    let telemetry_id = runtime
        .register(MissionApplication::Telemetry(TelemetryApplication::new(
            mailbox, None,
        )))
        .unwrap();
    let inboxes = [
        RuntimeInboxConfig::new(1, echo_topics),
        RuntimeInboxConfig::new(1, telemetry_topics),
    ];
    let mut mission = ComposedMission {
        runtime: MessagingRuntime::new(runtime, &inboxes).unwrap(),
        echo_id,
        telemetry_id,
    };
    start_both(&mut mission);
    mission
}

fn assert_report(
    report: &PublishReport,
    classification: PublishClassification,
    outcomes: &[(ApplicationId, DeliveryStatus)],
) {
    assert_eq!(report.classification(), classification);
    assert_eq!(
        report
            .outcomes()
            .iter()
            .map(|outcome| (outcome.application_id(), outcome.status()))
            .collect::<Vec<_>>(),
        outcomes
    );
}

fn dispatch(application_id: ApplicationId, mission: &mut ComposedMission<'_>) {
    assert_eq!(
        mission.runtime.dispatch_one(application_id),
        Ok(MessageDispatchOutcome::Dispatched)
    );
}

fn queue_telemetry(mission: &mut ComposedMission<'_>, value: u8) {
    let report = ingest(&mut mission.runtime, &[0x01, value]).unwrap();
    assert_report(
        &report,
        PublishClassification::Complete,
        &[(mission.echo_id, DeliveryStatus::Delivered)],
    );
    dispatch(mission.echo_id, mission);
}

fn fill_output(mission: &mut ComposedMission<'_>, value: u8) {
    queue_telemetry(mission, value);
    dispatch(mission.telemetry_id, mission);
}

fn assert_application_failure(
    error: &DispatchFailure,
    application_id: ApplicationId,
    source: MissionMessageError,
) {
    let expected = MessageDispatchError::Application {
        application_id,
        source,
    };
    assert_eq!(error.operation_error(), &expected);
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(
        Error::source(error).and_then(|source| source.downcast_ref()),
        Some(&expected)
    );
}

fn replay() -> ReplayTrace {
    let mailbox = OutputMailbox::default();
    let mut mission = running(&mailbox);
    let mut trace = ReplayTrace {
        publications: Vec::new(),
        dispatches: Vec::new(),
        drains: vec![mailbox.drain()],
        rejections: Vec::new(),
        states: [ApplicationState::Running; 2],
    };
    for value in [0, 42, 100, 42] {
        let report = ingest(&mut mission.runtime, &[0x01, value]).unwrap();
        trace.publications.push(report.classification());
        trace
            .rejections
            .push(ingest(&mut mission.runtime, &[0xff, 255]).unwrap_err());
        for application_id in [mission.echo_id, mission.telemetry_id] {
            trace
                .dispatches
                .push(mission.runtime.dispatch_one(application_id).unwrap());
            trace.drains.push(mailbox.drain());
        }
        trace.drains.push(mailbox.drain());
    }
    trace.states =
        [mission.echo_id, mission.telemetry_id].map(|id| mission.runtime.state(id).unwrap());
    trace
}
