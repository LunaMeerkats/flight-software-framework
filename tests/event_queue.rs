//! Public-API evidence for bounded structured-event delivery.

use std::time::Duration;

use rust_flight_framework::{
    ApplicationId, Event, EventEmitOutcome, EventQueue, EventQueueCreateError, EventSeverity,
    EventSource, EventTimestamp, LifecycleRegistry,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    Started,
    Degraded,
    Failed,
}

fn one_application_id() -> ApplicationId {
    let mut registry = LifecycleRegistry::new(1).expect("test registry capacity is positive");
    registry.register().expect("test application fits")
}

fn timestamp(milliseconds: u64) -> EventTimestamp {
    EventTimestamp::from_elapsed(Duration::from_millis(milliseconds))
}

fn framework_event(
    identifier: MissionEventId,
    severity: EventSeverity,
    milliseconds: u64,
) -> Event<MissionEventId> {
    Event::new(
        EventSource::Framework,
        severity,
        identifier,
        timestamp(milliseconds),
    )
}

#[test]
fn event_exposes_structured_fields_without_text_parsing() {
    let application_id = one_application_id();
    let cases = [
        (
            EventSource::Framework,
            EventSeverity::Informational,
            MissionEventId::Started,
            10,
        ),
        (
            EventSource::Application(application_id),
            EventSeverity::Warning,
            MissionEventId::Degraded,
            20,
        ),
        (
            EventSource::Application(application_id),
            EventSeverity::Error,
            MissionEventId::Failed,
            30,
        ),
    ];

    for (source, severity, identifier, milliseconds) in cases {
        let event = Event::new(source, severity, identifier, timestamp(milliseconds));
        assert_eq!(event.source(), source);
        assert_eq!(event.severity(), severity);
        assert_eq!(event.identifier(), &identifier);
        assert_eq!(
            event.timestamp().elapsed(),
            Duration::from_millis(milliseconds)
        );
    }
}

#[test]
fn queue_requires_positive_capacity() {
    assert_eq!(
        EventQueue::<MissionEventId>::new(0).expect_err("zero records cannot form a queue"),
        EventQueueCreateError::ZeroCapacity
    );

    let queue = EventQueue::<MissionEventId>::new(2).expect("positive capacity can be reserved");
    assert_eq!(queue.capacity(), 2);
    assert_eq!(queue.pending(), 0);
    assert!(queue.is_empty());
}

#[test]
fn queue_accepts_its_exact_limit_and_dequeues_fifo() {
    let mut queue = EventQueue::new(2).expect("test event storage can be reserved");
    let first = framework_event(MissionEventId::Started, EventSeverity::Informational, 20);
    let second = framework_event(MissionEventId::Degraded, EventSeverity::Warning, 10);

    assert_eq!(queue.emit(&first), EventEmitOutcome::Recorded);
    assert_eq!(queue.emit(&second), EventEmitOutcome::Recorded);
    assert_eq!(queue.pending(), 2);
    assert_eq!(queue.dequeue(), Some(first));
    assert_eq!(queue.dequeue(), Some(second));
    assert_eq!(queue.dequeue(), None);
    assert!(queue.is_empty());
}

#[test]
fn reject_newest_preserves_older_events_and_allows_explicit_retry() {
    let mut queue = EventQueue::new(2).expect("test event storage can be reserved");
    let first = framework_event(MissionEventId::Started, EventSeverity::Informational, 10);
    let second = framework_event(MissionEventId::Degraded, EventSeverity::Warning, 20);
    let rejected = framework_event(MissionEventId::Failed, EventSeverity::Error, 30);

    assert_eq!(queue.emit(&first), EventEmitOutcome::Recorded);
    assert_eq!(queue.emit(&second), EventEmitOutcome::Recorded);
    assert_eq!(
        queue.emit(&rejected),
        EventEmitOutcome::QueueFull { capacity: 2 }
    );
    assert_eq!(queue.pending(), 2);
    assert_eq!(queue.dequeue(), Some(first));

    assert_eq!(queue.emit(&rejected), EventEmitOutcome::Recorded);
    assert_eq!(queue.pending(), 2);
    assert_eq!(queue.dequeue(), Some(second));
    assert_eq!(queue.dequeue(), Some(rejected));
    assert!(queue.is_empty());
}
