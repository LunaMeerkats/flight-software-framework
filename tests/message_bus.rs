//! Public-API evidence for bounded available-endpoint message fan-out.

use rust_flight_framework::{
    ApplicationId, ApplicationInboxConfig, DeliveryStatus, InboxAccessError, LifecycleRegistry,
    Message, MessageBus, MessageBusCreateError, MessageCreateError, PublishClassification,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Command,
    Telemetry,
    Unrouted,
}

const COMMAND_ONLY: [MissionTopic; 1] = [MissionTopic::Command];

fn configured_application_ids<const COUNT: usize>() -> [ApplicationId; COUNT] {
    let mut registry = LifecycleRegistry::new(COUNT).expect("test registry capacity is positive");
    std::array::from_fn(|_| registry.register().expect("test application fits"))
}

fn two_command_subscriber_bus<const MAX_PAYLOAD_BYTES: usize>(
    application_ids: [ApplicationId; 2],
    capacities: [usize; 2],
) -> MessageBus<MissionTopic, MAX_PAYLOAD_BYTES> {
    let configurations = [
        ApplicationInboxConfig::new(application_ids[0], capacities[0], &COMMAND_ONLY),
        ApplicationInboxConfig::new(application_ids[1], capacities[1], &COMMAND_ONLY),
    ];
    MessageBus::new(&configurations).expect("test topology is valid")
}

#[test]
fn inline_payload_accepts_its_exact_limit_and_rejects_one_more_byte() {
    let at_limit = Message::<MissionTopic, 4>::try_new(MissionTopic::Command, &[1, 2, 3, 4])
        .expect("payload at the limit fits");
    assert_eq!(at_limit.topic(), &MissionTopic::Command);
    assert_eq!(at_limit.payload(), &[1, 2, 3, 4]);
    assert_eq!(at_limit.payload_capacity(), 4);

    assert_eq!(
        Message::<MissionTopic, 4>::try_new(MissionTopic::Command, &[1, 2, 3, 4, 5]),
        Err(MessageCreateError::PayloadTooLong {
            length: 5,
            maximum: 4,
        })
    );

    let empty = Message::<MissionTopic, 0>::try_new(MissionTopic::Telemetry, &[])
        .expect("a zero-capacity message accepts an empty payload");
    assert!(empty.payload().is_empty());
    assert_eq!(empty.payload_capacity(), 0);
}

#[test]
fn topology_validation_rejects_invalid_inboxes_without_a_partial_bus() {
    let application_ids = configured_application_ids::<2>();
    assert_eq!(
        MessageBus::<MissionTopic, 4>::new(&[]).expect_err("an empty topology is invalid"),
        MessageBusCreateError::NoInboxes
    );

    let zero_capacity = [ApplicationInboxConfig::new(
        application_ids[0],
        0,
        &COMMAND_ONLY,
    )];
    assert_eq!(
        MessageBus::<MissionTopic, 4>::new(&zero_capacity)
            .expect_err("an inbox must have at least one slot"),
        MessageBusCreateError::ZeroInboxCapacity {
            application_id: application_ids[0],
        }
    );

    let out_of_order = [ApplicationInboxConfig::new(
        application_ids[1],
        1,
        &COMMAND_ONLY,
    )];
    assert_eq!(
        MessageBus::<MissionTopic, 4>::new(&out_of_order)
            .expect_err("the first configuration must use the first identity"),
        MessageBusCreateError::ApplicationOutOfOrder {
            application_id: application_ids[1],
            expected_index: 0,
        }
    );

    let repeated_topics = [
        MissionTopic::Command,
        MissionTopic::Telemetry,
        MissionTopic::Command,
    ];
    let duplicate = [ApplicationInboxConfig::new(
        application_ids[0],
        1,
        &repeated_topics,
    )];
    assert_eq!(
        MessageBus::<MissionTopic, 4>::new(&duplicate)
            .expect_err("one application cannot repeat a topic"),
        MessageBusCreateError::DuplicateTopic {
            application_id: application_ids[0],
            first_index: 0,
            duplicate_index: 2,
        }
    );
}

#[test]
fn oversized_later_inbox_returns_exact_reservation_error() {
    let application_ids = configured_application_ids::<2>();
    let configurations = [
        ApplicationInboxConfig::new(application_ids[0], 1, &COMMAND_ONLY),
        ApplicationInboxConfig::new(application_ids[1], usize::MAX, &COMMAND_ONLY),
    ];

    // A nonzero-sized message cannot fit usize::MAX slots. This exercises
    // capacity overflow after an earlier inbox was built, not simulated OOM.
    assert_eq!(
        MessageBus::<MissionTopic, 4>::new(&configurations)
            .expect_err("the later inbox requests an unrepresentable capacity"),
        MessageBusCreateError::InboxStorageAllocationFailed {
            application_id: application_ids[1],
            requested: usize::MAX,
        }
    );
}

#[test]
fn reject_newest_saturation_preserves_older_entries_and_healthy_fan_out() {
    let application_ids = configured_application_ids::<2>();
    let mut bus = two_command_subscriber_bus::<4>(application_ids, [1, 2]);
    let first = Message::try_new(MissionTopic::Command, &[1]).expect("first payload fits");
    let second = Message::try_new(MissionTopic::Command, &[2]).expect("second payload fits");

    let complete = bus.publish(&first).expect("report storage can be reserved");
    assert_eq!(complete.classification(), PublishClassification::Complete);

    let partial = bus
        .publish(&second)
        .expect("report storage can be reserved");
    assert_eq!(partial.classification(), PublishClassification::Partial);
    assert_eq!(partial.outcomes().len(), 2);
    assert_eq!(partial.outcomes()[0].application_id(), application_ids[0]);
    assert_eq!(partial.outcomes()[0].status(), DeliveryStatus::InboxFull);
    assert_eq!(partial.outcomes()[1].application_id(), application_ids[1]);
    assert_eq!(partial.outcomes()[1].status(), DeliveryStatus::Delivered);
    assert_eq!(bus.inbox_capacity(application_ids[0]), Ok(1));
    assert_eq!(bus.inbox_capacity(application_ids[1]), Ok(2));
    assert_eq!(bus.pending(application_ids[0]), Ok(1));
    assert_eq!(bus.pending(application_ids[1]), Ok(2));

    assert_eq!(bus.dequeue(application_ids[0]), Ok(Some(first)));
    assert_eq!(bus.dequeue(application_ids[0]), Ok(None));
    assert_eq!(bus.dequeue(application_ids[1]), Ok(Some(first)));
    assert_eq!(bus.dequeue(application_ids[1]), Ok(Some(second)));
    assert_eq!(bus.dequeue(application_ids[1]), Ok(None));
}

#[test]
fn publication_distinguishes_all_full_from_no_subscribers() {
    let application_ids = configured_application_ids::<2>();
    let mut bus = two_command_subscriber_bus::<4>(application_ids, [1, 1]);
    let retained = Message::try_new(MissionTopic::Command, &[7]).expect("payload fits");
    let rejected = Message::try_new(MissionTopic::Command, &[8]).expect("payload fits");
    let unrouted = Message::try_new(MissionTopic::Unrouted, &[9]).expect("payload fits");

    assert_eq!(
        bus.publish(&retained)
            .expect("report storage can be reserved")
            .classification(),
        PublishClassification::Complete
    );
    let all_full = bus
        .publish(&rejected)
        .expect("report storage can be reserved");
    assert_eq!(
        all_full.classification(),
        PublishClassification::WhollyUndelivered
    );
    assert_eq!(all_full.outcomes().len(), 2);
    assert_eq!(all_full.outcomes()[0].application_id(), application_ids[0]);
    assert_eq!(all_full.outcomes()[1].application_id(), application_ids[1]);
    assert!(
        all_full
            .outcomes()
            .iter()
            .all(|outcome| outcome.status() == DeliveryStatus::InboxFull)
    );

    let no_subscribers = bus
        .publish(&unrouted)
        .expect("an empty report needs no storage");
    assert_eq!(
        no_subscribers.classification(),
        PublishClassification::NoSubscribers
    );
    assert!(no_subscribers.outcomes().is_empty());
    assert_eq!(bus.pending(application_ids[0]), Ok(1));
    assert_eq!(bus.pending(application_ids[1]), Ok(1));
    assert_eq!(bus.dequeue(application_ids[0]), Ok(Some(retained)));
    assert_eq!(bus.dequeue(application_ids[1]), Ok(Some(retained)));
    assert_eq!(bus.dequeue(application_ids[0]), Ok(None));
    assert_eq!(bus.dequeue(application_ids[1]), Ok(None));
}

#[test]
fn one_inbox_preserves_cross_topic_publish_order() {
    let [application_id] = configured_application_ids::<1>();
    let topics = [MissionTopic::Telemetry, MissionTopic::Command];
    let configurations = [ApplicationInboxConfig::new(application_id, 2, &topics)];
    let mut bus =
        MessageBus::<MissionTopic, 4>::new(&configurations).expect("cross-topic topology is valid");
    let telemetry = Message::try_new(MissionTopic::Telemetry, &[3]).expect("payload fits");
    let command = Message::try_new(MissionTopic::Command, &[4]).expect("payload fits");

    assert_eq!(
        bus.publish(&telemetry)
            .expect("report storage can be reserved")
            .classification(),
        PublishClassification::Complete
    );
    assert_eq!(
        bus.publish(&command)
            .expect("report storage can be reserved")
            .classification(),
        PublishClassification::Complete
    );
    assert_eq!(bus.dequeue(application_id), Ok(Some(telemetry)));
    assert_eq!(bus.dequeue(application_id), Ok(Some(command)));
    assert_eq!(bus.dequeue(application_id), Ok(None));
}

#[test]
fn selective_routing_skips_nonmatching_endpoints_and_preserves_route_order() {
    let application_ids = configured_application_ids::<3>();
    let telemetry_only = [MissionTopic::Telemetry];
    let both_topics = [MissionTopic::Telemetry, MissionTopic::Command];
    let configurations = [
        ApplicationInboxConfig::new(application_ids[0], 1, &COMMAND_ONLY),
        ApplicationInboxConfig::new(application_ids[1], 1, &telemetry_only),
        ApplicationInboxConfig::new(application_ids[2], 1, &both_topics),
    ];
    let mut bus =
        MessageBus::<MissionTopic, 4>::new(&configurations).expect("selective topology is valid");
    let command = Message::try_new(MissionTopic::Command, &[5]).expect("payload fits");

    let report = bus
        .publish(&command)
        .expect("report storage can be reserved");
    assert_eq!(report.classification(), PublishClassification::Complete);
    assert_eq!(report.outcomes().len(), 2);
    assert_eq!(report.outcomes()[0].application_id(), application_ids[0]);
    assert_eq!(report.outcomes()[1].application_id(), application_ids[2]);
    assert_eq!(bus.pending(application_ids[0]), Ok(1));
    assert_eq!(bus.pending(application_ids[1]), Ok(0));
    assert_eq!(bus.pending(application_ids[2]), Ok(1));
    assert_eq!(bus.dequeue(application_ids[0]), Ok(Some(command)));
    assert_eq!(bus.dequeue(application_ids[1]), Ok(None));
    assert_eq!(bus.dequeue(application_ids[2]), Ok(Some(command)));
}

#[test]
fn inbox_access_rejects_an_identity_outside_the_topology() {
    let application_ids = configured_application_ids::<2>();
    let configurations = [ApplicationInboxConfig::new(
        application_ids[0],
        1,
        &COMMAND_ONLY,
    )];
    let mut bus = MessageBus::<MissionTopic, 4>::new(&configurations)
        .expect("single-inbox topology is valid");
    let unknown_error = InboxAccessError::UnknownApplication {
        application_id: application_ids[1],
    };

    assert_eq!(bus.inbox_capacity(application_ids[1]), Err(unknown_error));
    assert_eq!(bus.pending(application_ids[1]), Err(unknown_error));
    assert_eq!(bus.dequeue(application_ids[1]), Err(unknown_error));
    assert_eq!(bus.pending(application_ids[0]), Ok(0));
}
