//! Public-API evidence for injected and manually controlled framework time.

use std::time::Duration;

use rust_flight_framework::{
    Clock, Event, EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp,
    FrameworkInstant, ManualClock, ManualClockAdvanceError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    First,
    Second,
    Third,
}

fn event_from_clock<C: Clock + ?Sized>(
    clock: &C,
    identifier: MissionEventId,
) -> Event<MissionEventId> {
    Event::new(
        EventSource::Framework,
        EventSeverity::Informational,
        identifier,
        EventTimestamp::from_clock(clock),
    )
}

fn replay_trace(clock: &mut ManualClock) -> [FrameworkInstant; 4] {
    let origin = clock.now();
    let first = clock
        .advance(Duration::from_millis(7))
        .expect("small test advance fits");
    let repeated = clock
        .advance(Duration::ZERO)
        .expect("zero advance always fits");
    let second = clock
        .advance(Duration::from_millis(5))
        .expect("cumulative test advance fits");
    [origin, first, repeated, second]
}

#[test]
fn manual_clock_starts_at_origin_and_reads_do_not_advance() {
    let clock = ManualClock::new();
    let injected_clock: &dyn Clock = &clock;

    assert_eq!(clock.now(), FrameworkInstant::ZERO);
    assert_eq!(clock.now(), FrameworkInstant::ZERO);
    assert_eq!(
        EventTimestamp::from_clock(injected_clock).instant(),
        FrameworkInstant::ZERO
    );
    assert_eq!(clock.now(), FrameworkInstant::ZERO);
}

#[test]
fn manual_clock_advances_only_by_explicit_durations() {
    let mut clock = ManualClock::new();

    assert_eq!(
        clock.advance(Duration::from_millis(7)),
        Ok(FrameworkInstant::from_elapsed(Duration::from_millis(7)))
    );
    assert_eq!(
        clock.advance(Duration::ZERO),
        Ok(FrameworkInstant::from_elapsed(Duration::from_millis(7)))
    );
    assert_eq!(
        clock.advance(Duration::from_millis(5)),
        Ok(FrameworkInstant::from_elapsed(Duration::from_millis(12)))
    );
}

#[test]
fn advance_overflow_is_typed_and_preserves_current_time() {
    let mut clock = ManualClock::from_elapsed(Duration::MAX);
    let current = FrameworkInstant::from_elapsed(Duration::MAX);
    let advance = Duration::from_nanos(1);

    assert_eq!(
        clock.advance(advance),
        Err(ManualClockAdvanceError::Overflow { current, advance })
    );
    assert_eq!(clock.now(), current);
}

#[test]
fn identical_manual_sequences_produce_identical_traces() {
    let first = replay_trace(&mut ManualClock::new());
    let second = replay_trace(&mut ManualClock::new());

    assert_eq!(first, second);
    assert_eq!(
        first,
        [
            FrameworkInstant::ZERO,
            FrameworkInstant::from_elapsed(Duration::from_millis(7)),
            FrameworkInstant::from_elapsed(Duration::from_millis(7)),
            FrameworkInstant::from_elapsed(Duration::from_millis(12)),
        ]
    );
}

#[test]
fn injected_clock_timestamps_events_without_changing_queue_policy() {
    let mut clock = ManualClock::new();
    let mut queue = EventQueue::new(3).expect("three event records can be reserved");
    let first = event_from_clock(&clock, MissionEventId::First);
    let second = event_from_clock(&clock, MissionEventId::Second);
    clock
        .advance(Duration::from_millis(9))
        .expect("test advance fits");
    let third = event_from_clock(&clock, MissionEventId::Third);

    assert_eq!(queue.emit(&first), EventEmitOutcome::Recorded);
    assert_eq!(queue.emit(&second), EventEmitOutcome::Recorded);
    assert_eq!(queue.emit(&third), EventEmitOutcome::Recorded);
    assert_eq!(queue.dequeue(), Some(first));
    assert_eq!(queue.dequeue(), Some(second));
    assert_eq!(queue.dequeue(), Some(third));
    assert_eq!(first.timestamp().instant(), FrameworkInstant::ZERO);
    assert_eq!(second.timestamp().instant(), FrameworkInstant::ZERO);
    assert_eq!(
        third.timestamp().instant(),
        FrameworkInstant::from_elapsed(Duration::from_millis(9))
    );
}
