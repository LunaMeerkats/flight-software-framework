//! Public-only recurring producers with explicit host release and work policy.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, ConfigurationTable, FrameworkInstant, LifecycleError,
    LifecycleRegistry, ManualClock, Message, MessageDispatchOutcome, MessagingApplication,
    MessagingOperationError, MessagingRuntime, PeriodicWork, PeriodicWorkCreateError,
    PeriodicWorkOutcome, PublishClassification, Runtime, RuntimeInboxConfig, RuntimeWorkError,
    ScheduledWork,
};

struct CountingClock {
    current: Cell<FrameworkInstant>,
    reads: Cell<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Produced {
    application_id: ApplicationId,
    revision: u64,
    value: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkError {
    ConfigurationUnavailable,
    PendingFull {
        retained: Produced,
        rejected: Produced,
    },
}

#[derive(Default)]
struct ProducerState {
    pending: Cell<Option<Produced>>,
    attempted: Cell<Option<Produced>>,
    work_calls: Cell<usize>,
    delivered: Cell<Option<(ApplicationId, u8)>>,
}

struct Producer<'a> {
    state: &'a ProducerState,
}

type ProducerRuntime<'a> = Runtime<Producer<'a>, Infallible, 1>;
type MessageRuntime<'a> = MessagingRuntime<Producer<'a>, (), 1, Infallible, 1>;

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        self.current.get()
    }
}

impl fmt::Display for WorkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigurationUnavailable => formatter.write_str("producer configuration missing"),
            Self::PendingFull { retained, rejected } => {
                write!(formatter, "output {retained:?} blocks {rejected:?}")
            }
        }
    }
}

impl Error for WorkError {}

impl Application for Producer<'_> {
    type StartError = Infallible;
    type WorkError = WorkError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.state.work_calls.set(self.state.work_calls.get() + 1);
        let (revision, value) = context
            .configuration()
            .and_then(|view| view.bytes().first().map(|value| (view.revision(), *value)))
            .ok_or(WorkError::ConfigurationUnavailable)?;
        let produced = Produced {
            application_id: context.application_id(),
            revision,
            value,
        };
        self.state.attempted.set(Some(produced));
        if let Some(retained) = self.state.pending.get() {
            return Err(WorkError::PendingFull {
                retained,
                rejected: produced,
            });
        }
        // One pending output is committed only when the slot is available.
        self.state.pending.set(Some(produced));
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<(), 1> for Producer<'_> {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<(), 1>,
        context: &mut ApplicationMessageContext<'_, (), 1>,
    ) -> Result<(), Self::MessageError> {
        self.state.delivered.set(
            message
                .payload()
                .first()
                .map(|value| (context.application_id(), *value)),
        );
        Ok(())
    }
}

fn instant(seconds: u64) -> FrameworkInstant {
    FrameworkInstant::from_elapsed(Duration::from_secs(seconds))
}

fn identity() -> ApplicationId {
    LifecycleRegistry::new(1).unwrap().register().unwrap()
}

fn timer(application_id: ApplicationId, seconds: u64, interval: u64) -> PeriodicWork {
    PeriodicWork::new(
        ScheduledWork::new(application_id, instant(seconds)),
        Duration::from_secs(interval),
    )
    .unwrap()
}

fn counting_clock(current: FrameworkInstant) -> CountingClock {
    CountingClock {
        current: Cell::new(current),
        reads: Cell::new(0),
    }
}

fn released(timer: &mut PeriodicWork, clock: &dyn Clock) -> ScheduledWork {
    let PeriodicWorkOutcome::Released { scheduled_work, .. } = timer.poll(clock) else {
        panic!("host expected one released work item");
    };
    scheduled_work
}

fn configured_runtime(states: &[ProducerState; 2]) -> (ProducerRuntime<'_>, [ApplicationId; 2]) {
    let table = ConfigurationTable::new(&[7], |_| Ok::<_, Infallible>(())).unwrap();
    let mut runtime = ProducerRuntime::with_configuration(2, table).unwrap();
    let ids = states
        .each_ref()
        .map(|state| runtime.register(Producer { state }).unwrap());
    assert_ne!(ids[0], ids[1]);
    (runtime, ids)
}

fn message_runtime(states: &[ProducerState; 2]) -> (MessageRuntime<'_>, [ApplicationId; 2]) {
    let (runtime, ids) = configured_runtime(states);
    let inboxes = [RuntimeInboxConfig::new(2, &[()]); 2];
    (MessagingRuntime::new(runtime, &inboxes).unwrap(), ids)
}

fn produced(application_id: ApplicationId, revision: u64, value: u8) -> Produced {
    Produced {
        application_id,
        revision,
        value,
    }
}

fn assert_output(state: &ProducerState, expected: Produced, calls: usize) {
    assert_eq!(state.attempted.get(), Some(expected));
    assert_eq!(state.pending.take(), Some(expected));
    assert_eq!(state.work_calls.get(), calls);
}

fn fill_inboxes(runtime: &mut MessageRuntime<'_>, ids: [ApplicationId; 2]) {
    for value in [11, 22] {
        let report = runtime
            .publish(&Message::try_new((), &[value]).unwrap())
            .unwrap();
        assert_eq!(report.classification(), PublishClassification::Complete);
    }
    for id in ids {
        assert_eq!(runtime.pending(id), Ok(2));
    }
}

fn assert_failure(
    failure: &MessagingOperationError<RuntimeWorkError<WorkError>>,
    retained: Produced,
    rejected: Produced,
) {
    let source = WorkError::PendingFull { retained, rejected };
    assert_eq!(failure.discarded_deliveries(), 2);
    assert_eq!(
        failure.source().unwrap().downcast_ref(),
        Some(failure.operation_error())
    );
    assert_eq!(
        failure.operation_error(),
        &RuntimeWorkError::Application {
            application_id: rejected.application_id,
            source,
        }
    );
    assert_eq!(
        failure.operation_error().source().unwrap().downcast_ref(),
        Some(&source)
    );
}

fn assert_peer_fifo(runtime: &mut MessageRuntime<'_>, id: ApplicationId, state: &ProducerState) {
    for (value, remaining) in [(11, 1), (22, 0)] {
        assert_eq!(
            runtime.dispatch_one(id),
            Ok(MessageDispatchOutcome::Dispatched)
        );
        assert_eq!(state.delivered.take(), Some((id, value)));
        assert_eq!(runtime.pending(id), Ok(remaining));
    }
}

fn replay(id: ApplicationId) -> [PeriodicWorkOutcome; 5] {
    let mut timer = timer(id, 1, 2);
    let mut clock = ManualClock::new();
    [0, 1, 10, 0, 2].map(|seconds| {
        clock.advance(Duration::from_secs(seconds)).unwrap();
        timer.poll(&clock)
    })
}

#[test]
fn construction_rejects_only_zero_interval_and_retains_the_first_item() {
    let first = ScheduledWork::new(identity(), instant(5));
    let error = PeriodicWork::new(first, Duration::ZERO).unwrap_err();
    assert_eq!(error, PeriodicWorkCreateError::ZeroInterval);
    assert!(error.source().is_none());
    for interval in [
        Duration::from_nanos(1),
        Duration::from_secs(2),
        Duration::MAX,
    ] {
        let timer = PeriodicWork::new(first, interval).unwrap();
        assert_eq!(timer.next_scheduled_work(), Some(first));
        assert_eq!(timer.interval(), interval);
    }
}

#[test]
fn future_polls_read_once_without_mutation_and_equality_releases_once() {
    let id = identity();
    let first = ScheduledWork::new(id, instant(5));
    let mut timer = timer(id, 5, 2);
    let clock = counting_clock(instant(4));
    for reads in 1..=2 {
        assert_eq!(
            timer.poll(&clock),
            PeriodicWorkOutcome::Waiting {
                next: first,
                observed_at: instant(4)
            }
        );
        assert_eq!(timer.next_scheduled_work(), Some(first));
        assert_eq!(clock.reads.get(), reads);
    }
    clock.current.set(instant(5));
    assert_eq!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Released {
            scheduled_work: first,
            observed_at: instant(5),
            next_scheduled_at: Some(instant(7)),
        }
    );
    assert_eq!(clock.reads.get(), 3);
    assert_eq!(
        timer.next_scheduled_work(),
        Some(ScheduledWork::new(id, instant(7)))
    );
    assert_eq!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Waiting {
            next: ScheduledWork::new(id, instant(7)),
            observed_at: instant(5),
        }
    );
    assert_eq!(clock.reads.get(), 4);
}

#[test]
fn subsecond_release_adds_exactly_across_a_second_boundary() {
    let id = identity();
    let first = FrameworkInstant::from_elapsed(Duration::new(3, 999_999_999));
    let mut timer =
        PeriodicWork::new(ScheduledWork::new(id, first), Duration::from_nanos(2)).unwrap();
    let mut clock = ManualClock::from_elapsed(first.elapsed());
    let next = FrameworkInstant::from_elapsed(Duration::new(4, 1));
    assert_eq!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Released {
            scheduled_work: ScheduledWork::new(id, first),
            observed_at: first,
            next_scheduled_at: Some(next),
        }
    );
    clock.advance(Duration::from_nanos(1)).unwrap();
    assert!(matches!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Waiting { .. }
    ));
    assert_eq!(
        timer.next_scheduled_work(),
        Some(ScheduledWork::new(id, next))
    );
    clock.advance(Duration::from_nanos(1)).unwrap();
    assert_eq!(released(&mut timer, &clock).scheduled_at(), next);
    assert_eq!(
        timer.next_scheduled_work(),
        Some(ScheduledWork::new(
            id,
            FrameworkInstant::from_elapsed(Duration::new(4, 3))
        ))
    );
}

#[test]
fn final_representable_release_exhausts_without_later_clock_reads() {
    let id = identity();
    let first = FrameworkInstant::from_elapsed(Duration::MAX - Duration::from_nanos(1));
    let maximum = FrameworkInstant::from_elapsed(Duration::MAX);
    let mut timer =
        PeriodicWork::new(ScheduledWork::new(id, first), Duration::from_nanos(1)).unwrap();
    let clock = counting_clock(first);
    assert_eq!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Released {
            scheduled_work: ScheduledWork::new(id, first),
            observed_at: first,
            next_scheduled_at: Some(maximum),
        }
    );
    clock.current.set(maximum);
    assert_eq!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Released {
            scheduled_work: ScheduledWork::new(id, maximum),
            observed_at: maximum,
            next_scheduled_at: None,
        }
    );
    assert_eq!(timer.next_scheduled_work(), None);
    assert_eq!(clock.reads.get(), 2);
    for _ in 0..3 {
        assert_eq!(timer.poll(&clock), PeriodicWorkOutcome::Exhausted);
    }
    assert_eq!(clock.reads.get(), 2);
}

#[test]
fn late_release_coalesces_backlog_and_regression_keeps_the_future_release() {
    let id = identity();
    let mut timer = timer(id, 1, 2);
    let clock = counting_clock(instant(10));
    assert_eq!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Released {
            scheduled_work: ScheduledWork::new(id, instant(1)),
            observed_at: instant(10),
            next_scheduled_at: Some(instant(12)),
        }
    );
    for seconds in [10, 10, 3, 11] {
        clock.current.set(instant(seconds));
        assert_eq!(
            timer.poll(&clock),
            PeriodicWorkOutcome::Waiting {
                next: ScheduledWork::new(id, instant(12)),
                observed_at: instant(seconds),
            }
        );
        assert_eq!(
            timer.next_scheduled_work(),
            Some(ScheduledWork::new(id, instant(12)))
        );
    }
    clock.current.set(instant(12));
    assert_eq!(
        released(&mut timer, &clock),
        ScheduledWork::new(id, instant(12))
    );
    assert_eq!(
        timer.next_scheduled_work(),
        Some(ScheduledWork::new(id, instant(14)))
    );
    assert_eq!(clock.reads.get(), 6);
}

#[test]
fn manual_time_replay_repeats_the_same_release_history() {
    let id = identity();
    let expected = [
        PeriodicWorkOutcome::Waiting {
            next: ScheduledWork::new(id, instant(1)),
            observed_at: instant(0),
        },
        PeriodicWorkOutcome::Released {
            scheduled_work: ScheduledWork::new(id, instant(1)),
            observed_at: instant(1),
            next_scheduled_at: Some(instant(3)),
        },
        PeriodicWorkOutcome::Released {
            scheduled_work: ScheduledWork::new(id, instant(3)),
            observed_at: instant(11),
            next_scheduled_at: Some(instant(13)),
        },
        PeriodicWorkOutcome::Waiting {
            next: ScheduledWork::new(id, instant(13)),
            observed_at: instant(11),
        },
        PeriodicWorkOutcome::Released {
            scheduled_work: ScheduledWork::new(id, instant(13)),
            observed_at: instant(13),
            next_scheduled_at: Some(instant(15)),
        },
    ];
    assert_eq!(replay(id), expected);
    assert_eq!(replay(id), expected);
}

#[test]
fn direct_owner_produces_bounded_output_with_identity_and_current_configuration() {
    let states = [ProducerState::default(), ProducerState::default()];
    let (mut runtime, ids) = configured_runtime(&states);
    for id in ids {
        runtime.start(id).unwrap();
    }
    let mut timers = ids.map(|id| timer(id, 0, 1));
    let mut clock = ManualClock::new();
    for (revision, value) in [(1, 7), (2, 9)] {
        for (index, timer) in timers.iter_mut().enumerate() {
            let item = released(timer, &clock);
            assert_eq!(states[index].pending.get(), None);
            runtime.work(item.application_id()).unwrap();
            assert_output(
                &states[index],
                produced(ids[index], revision, value),
                revision as usize,
            );
        }
        if revision == 1 {
            assert_eq!(runtime.replace_configuration(&[9]), Ok(2));
            clock.advance(Duration::from_secs(1)).unwrap();
        }
    }
}

#[test]
fn messaging_owner_preserves_successful_work_configuration_and_queued_messages() {
    let states = [ProducerState::default(), ProducerState::default()];
    let (mut runtime, ids) = message_runtime(&states);
    for id in ids {
        runtime.start(id).unwrap();
    }
    fill_inboxes(&mut runtime, ids);
    let mut timers = ids.map(|id| timer(id, 0, 1));
    let mut clock = ManualClock::new();
    for (revision, value) in [(1, 7), (2, 9)] {
        for (index, timer) in timers.iter_mut().enumerate() {
            let item = released(timer, &clock);
            runtime.work(item.application_id()).unwrap();
            assert_output(
                &states[index],
                produced(ids[index], revision, value),
                revision as usize,
            );
            assert_eq!(runtime.pending(ids[index]), Ok(2));
            assert_eq!(states[index].delivered.get(), None);
        }
        if revision == 1 {
            assert_eq!(runtime.replace_configuration(&[9]), Ok(2));
            clock.advance(Duration::from_secs(1)).unwrap();
        }
    }
    for (index, id) in ids.into_iter().enumerate() {
        assert_peer_fifo(&mut runtime, id, &states[index]);
    }
}

#[test]
fn lifecycle_rejection_consumes_a_release_and_host_can_discard_the_next_one() {
    let states = [ProducerState::default(), ProducerState::default()];
    let (mut runtime, ids) = configured_runtime(&states);
    let mut timer = timer(ids[0], 0, 1);
    let mut clock = ManualClock::new();
    let item = released(&mut timer, &clock);
    assert_eq!(
        runtime.work(item.application_id()),
        Err(RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
            application_id: ids[0],
            state: ApplicationState::Registered,
        }))
    );
    assert_eq!(states[0].work_calls.get(), 0);
    assert_eq!(
        timer.next_scheduled_work(),
        Some(ScheduledWork::new(ids[0], instant(1)))
    );
    assert!(matches!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Waiting { .. }
    ));
    runtime.start(ids[0]).unwrap();
    clock.advance(Duration::from_secs(1)).unwrap();
    let discarded = released(&mut timer, &clock);
    assert_eq!(discarded, ScheduledWork::new(ids[0], instant(1)));
    // The host deliberately discards this release instead of invoking work.
    assert_eq!(states[0].work_calls.get(), 0);
    assert_eq!(states[0].pending.get(), None);
    assert_eq!(
        timer.next_scheduled_work(),
        Some(ScheduledWork::new(ids[0], instant(2)))
    );
    assert!(matches!(
        timer.poll(&clock),
        PeriodicWorkOutcome::Waiting { .. }
    ));
    clock.advance(Duration::from_secs(1)).unwrap();
    let item = released(&mut timer, &clock);
    runtime.work(item.application_id()).unwrap();
    assert_output(&states[0], produced(ids[0], 1, 7), 1);
}

#[test]
fn returned_work_error_clears_only_selected_inbox_and_host_disposes_its_timer() {
    for selected in [0, 1] {
        let states = [ProducerState::default(), ProducerState::default()];
        let (mut runtime, ids) = message_runtime(&states);
        for id in ids {
            runtime.start(id).unwrap();
        }
        let mut timers = ids.map(|id| Some(timer(id, 0, 1)));
        let mut clock = ManualClock::new();
        let item = released(timers[selected].as_mut().unwrap(), &clock);
        runtime.work(item.application_id()).unwrap();
        let retained = produced(ids[selected], 1, 7);
        assert_eq!(states[selected].pending.get(), Some(retained));
        fill_inboxes(&mut runtime, ids);
        assert_eq!(runtime.replace_configuration(&[9]), Ok(2));
        clock.advance(Duration::from_secs(1)).unwrap();
        let item = released(timers[selected].as_mut().unwrap(), &clock);
        let failure = runtime.work(item.application_id()).unwrap_err();
        let rejected = produced(ids[selected], 2, 9);
        assert_failure(&failure, retained, rejected);
        assert_eq!(states[selected].attempted.get(), Some(rejected));
        assert_eq!(states[selected].work_calls.get(), 2);
        assert_eq!(states[selected].pending.get(), Some(retained));
        assert_eq!(runtime.state(ids[selected]), Ok(ApplicationState::Failed));
        assert_eq!(runtime.pending(ids[selected]), Ok(0));
        assert_eq!(
            timers[selected].as_ref().unwrap().next_scheduled_work(),
            Some(ScheduledWork::new(ids[selected], instant(2)))
        );
        // The cursor survives failure; disabling it is explicit host policy.
        timers[selected] = None;
        assert!(timers[selected].is_none());
        let peer = 1 - selected;
        assert_eq!(runtime.pending(ids[peer]), Ok(2));
        assert_eq!(runtime.state(ids[peer]), Ok(ApplicationState::Running));
        let item = released(timers[peer].as_mut().unwrap(), &clock);
        assert_eq!(item, ScheduledWork::new(ids[peer], instant(0)));
        runtime.work(item.application_id()).unwrap();
        assert_output(&states[peer], produced(ids[peer], 2, 9), 1);
        assert_peer_fifo(&mut runtime, ids[peer], &states[peer]);
        assert_eq!(states[selected].pending.take(), Some(retained));
        assert!(timers[peer].is_some());
    }
}
