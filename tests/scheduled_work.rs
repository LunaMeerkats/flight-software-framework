//! Public-API evidence for finite caller-driven scheduled application work.

use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, Clock, Event,
    EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp, FrameworkInstant,
    LifecycleError, ManualClock, Runtime, RuntimeWorkError, ScheduledWork, ScheduledWorkOutcome,
    WorkSchedule, WorkScheduleCreateError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ApplicationName {
    Alpha,
    Beta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TestApplicationError {
    WorkRejected(ApplicationName),
}

impl fmt::Display for TestApplicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkRejected(name) => write!(formatter, "{name:?} rejected work"),
        }
    }
}

impl Error for TestApplicationError {}

type WorkTrace = Rc<RefCell<Vec<ApplicationName>>>;

struct CountingClock {
    current: Cell<FrameworkInstant>,
    reads: Cell<usize>,
}

impl CountingClock {
    fn new(current: FrameworkInstant) -> Self {
        Self {
            current: Cell::new(current),
            reads: Cell::new(0),
        }
    }

    fn set(&self, current: FrameworkInstant) {
        self.current.set(current);
    }

    fn reads(&self) -> usize {
        self.reads.get()
    }
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        self.current.get()
    }
}

#[derive(Debug)]
struct RecordingApplication {
    name: ApplicationName,
    work_trace: WorkTrace,
    reject_work: bool,
}

impl Application for RecordingApplication {
    type StartError = TestApplicationError;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    type WorkError = TestApplicationError;

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.work_trace.borrow_mut().push(self.name);
        if self.reject_work {
            return Err(TestApplicationError::WorkRejected(self.name));
        }
        Ok(())
    }

    type StopError = TestApplicationError;

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    type RestartError = TestApplicationError;

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

fn instant(seconds: u64) -> FrameworkInstant {
    FrameworkInstant::from_elapsed(Duration::from_secs(seconds))
}

fn scheduled(application_id: ApplicationId, seconds: u64) -> ScheduledWork {
    ScheduledWork::new(application_id, instant(seconds))
}

fn running_runtime(
    alpha_rejects_work: bool,
) -> (
    Runtime<RecordingApplication>,
    ApplicationId,
    ApplicationId,
    WorkTrace,
) {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let mut runtime = Runtime::new(2).expect("two application records can be reserved");
    let alpha = runtime
        .register(RecordingApplication {
            name: ApplicationName::Alpha,
            work_trace: Rc::clone(&trace),
            reject_work: alpha_rejects_work,
        })
        .expect("alpha fits");
    let beta = runtime
        .register(RecordingApplication {
            name: ApplicationName::Beta,
            work_trace: Rc::clone(&trace),
            reject_work: false,
        })
        .expect("beta fits");
    runtime.start(alpha).expect("alpha starts");
    runtime.start(beta).expect("beta starts");
    (runtime, alpha, beta, trace)
}

#[test]
fn schedule_rejects_descending_instants_and_accepts_an_empty_agenda() {
    let (_, alpha, beta, _) = running_runtime(false);
    let descending = [scheduled(alpha, 7), scheduled(beta, 6)];

    assert_eq!(
        WorkSchedule::new(&descending).expect_err("descending instants are rejected"),
        WorkScheduleCreateError::ScheduledInstantsOutOfOrder {
            index: 1,
            previous: instant(7),
            scheduled_at: instant(6),
        }
    );

    let mut runtime: Runtime<RecordingApplication> =
        Runtime::new(1).expect("one application record can be reserved");
    let mut empty = WorkSchedule::new(&[]).expect("an empty agenda is valid");
    assert_eq!(
        runtime.run_next_scheduled_work(&mut empty, &ManualClock::new()),
        Ok(ScheduledWorkOutcome::Complete)
    );
    assert!(empty.is_complete());
    assert_eq!(empty.remaining(), 0);
}

#[test]
fn construction_reports_the_first_descending_pair_at_nanosecond_precision() {
    let (_, alpha, _, _) = running_runtime(false);
    let cases = [([0, 1, 1, 0], 3), ([1, 1, 0, 0], 2), ([0, 3, 2, 1], 2)];

    for (nanoseconds, index) in cases {
        let items = nanoseconds.map(|value| {
            ScheduledWork::new(
                alpha,
                FrameworkInstant::from_elapsed(Duration::from_nanos(value)),
            )
        });
        assert_eq!(
            WorkSchedule::new(&items).expect_err("the first descending pair is rejected"),
            WorkScheduleCreateError::ScheduledInstantsOutOfOrder {
                index,
                previous: items[index - 1].scheduled_at(),
                scheduled_at: items[index].scheduled_at(),
            }
        );
    }
}

#[test]
fn copied_agenda_preserves_items_after_caller_storage_is_changed_and_dropped() {
    let (mut runtime, alpha, beta, trace) = running_runtime(false);
    let items = [scheduled(beta, 5), scheduled(alpha, 5), scheduled(beta, 8)];
    let mut schedule = {
        let mut caller_items = items.to_vec();
        let schedule = WorkSchedule::new(&caller_items).expect("ordered agenda is copied");
        caller_items.fill(scheduled(alpha, 99));
        schedule
    };
    let clock = ManualClock::from_elapsed(Duration::from_secs(8));
    let expected_order = [
        ApplicationName::Beta,
        ApplicationName::Alpha,
        ApplicationName::Beta,
    ];
    assert_eq!(schedule.remaining(), items.len());

    for (index, item) in items.into_iter().enumerate() {
        assert_eq!(
            runtime.run_next_scheduled_work(&mut schedule, &clock),
            Ok(ScheduledWorkOutcome::Completed {
                scheduled_work: item,
                observed_at: instant(8),
                state: ApplicationState::Running,
            })
        );
        assert_eq!(schedule.remaining(), items.len() - index - 1);
        assert_eq!(*trace.borrow(), expected_order[..=index]);
    }
    assert!(schedule.is_complete());
    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Complete)
    );
    assert_eq!(*trace.borrow(), expected_order);
}

#[test]
fn equal_position_foreign_identity_schedules_the_receiving_runtime_application() {
    let (mut runtime, alpha, _, trace) = running_runtime(false);
    let (foreign_runtime, foreign_alpha, _, foreign_trace) = running_runtime(false);
    assert_eq!(foreign_alpha, alpha);
    let item = scheduled(foreign_alpha, 0);
    let mut schedule = WorkSchedule::new(&[item]).expect("one due item is ordered");

    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &ManualClock::new()),
        Ok(ScheduledWorkOutcome::Completed {
            scheduled_work: item,
            observed_at: FrameworkInstant::ZERO,
            state: ApplicationState::Running,
        })
    );
    assert_eq!(*trace.borrow(), [ApplicationName::Alpha]);
    assert!(foreign_trace.borrow().is_empty());
    assert_eq!(runtime.state(alpha), Ok(ApplicationState::Running));
    assert_eq!(
        foreign_runtime.state(foreign_alpha),
        Ok(ApplicationState::Running)
    );
    assert!(schedule.is_complete());
}

#[test]
fn complete_reads_no_clock_and_each_pending_item_decision_reads_once() {
    let (mut runtime, alpha, _, trace) = running_runtime(false);
    let clock = CountingClock::new(FrameworkInstant::ZERO);
    let mut empty = WorkSchedule::new(&[]).expect("an empty agenda is valid");

    assert_eq!(
        runtime.run_next_scheduled_work(&mut empty, &clock),
        Ok(ScheduledWorkOutcome::Complete)
    );
    assert_eq!(clock.reads(), 0);

    let mut pending = WorkSchedule::new(&[scheduled(alpha, 5)]).expect("one item is ordered");
    assert!(matches!(
        runtime.run_next_scheduled_work(&mut pending, &clock),
        Ok(ScheduledWorkOutcome::Waiting { .. })
    ));
    assert_eq!(clock.reads(), 1);

    clock.set(instant(5));
    assert!(matches!(
        runtime.run_next_scheduled_work(&mut pending, &clock),
        Ok(ScheduledWorkOutcome::Completed { .. })
    ));
    assert_eq!(clock.reads(), 2);
    assert_eq!(*trace.borrow(), [ApplicationName::Alpha]);
}

#[test]
fn waiting_preserves_the_item_and_manual_clock_until_its_inclusive_instant() {
    let (mut runtime, alpha, _, trace) = running_runtime(false);
    let item = scheduled(alpha, 5);
    let mut schedule = WorkSchedule::new(&[item]).expect("one item is ordered");
    let mut clock = ManualClock::new();

    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Waiting {
            next: item,
            observed_at: FrameworkInstant::ZERO,
        })
    );
    assert_eq!(schedule.remaining(), 1);
    assert!(trace.borrow().is_empty());
    assert_eq!(clock.now(), FrameworkInstant::ZERO);

    clock
        .advance(Duration::from_secs(5))
        .expect("test advance fits");
    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Completed {
            scheduled_work: item,
            observed_at: instant(5),
            state: ApplicationState::Running,
        })
    );
    assert_eq!(*trace.borrow(), [ApplicationName::Alpha]);
    assert!(schedule.is_complete());
    assert_eq!(clock.now(), instant(5));
}

#[test]
fn equal_time_order_is_stable_and_overdue_work_runs_one_item_per_call() {
    let (mut runtime, alpha, beta, trace) = running_runtime(false);
    let beta_at_five = scheduled(beta, 5);
    let alpha_at_five = scheduled(alpha, 5);
    let alpha_at_eight = scheduled(alpha, 8);
    let mut schedule =
        WorkSchedule::new(&[beta_at_five, alpha_at_five, alpha_at_eight]).expect("ordered agenda");
    let mut clock = ManualClock::from_elapsed(Duration::from_secs(5));

    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Completed {
            scheduled_work: beta_at_five,
            observed_at: instant(5),
            state: ApplicationState::Running,
        })
    );
    assert_eq!(*trace.borrow(), [ApplicationName::Beta]);
    assert_eq!(schedule.remaining(), 2);

    runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .expect("equal-time alpha work succeeds");
    assert_eq!(
        *trace.borrow(),
        [ApplicationName::Beta, ApplicationName::Alpha]
    );
    assert_eq!(schedule.remaining(), 1);

    clock
        .advance(Duration::from_secs(5))
        .expect("test advance fits");
    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Completed {
            scheduled_work: alpha_at_eight,
            observed_at: instant(10),
            state: ApplicationState::Running,
        })
    );
    assert_eq!(
        *trace.borrow(),
        [
            ApplicationName::Beta,
            ApplicationName::Alpha,
            ApplicationName::Alpha,
        ]
    );
}

#[test]
fn lifecycle_rejection_consumes_one_item_without_blocking_a_due_peer() {
    let (mut runtime, alpha, beta, trace) = running_runtime(false);
    runtime.stop(alpha).expect("alpha stops");
    let alpha_item = scheduled(alpha, 0);
    let beta_item = scheduled(beta, 0);
    let mut schedule = WorkSchedule::new(&[alpha_item, beta_item]).expect("equal-time agenda");
    let clock = ManualClock::new();

    let error = runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .expect_err("stopped alpha cannot work");
    assert_eq!(error.scheduled_work(), alpha_item);
    assert_eq!(error.observed_at(), FrameworkInstant::ZERO);
    assert_eq!(
        error.into_work_error(),
        RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
            application_id: alpha,
            state: ApplicationState::Stopped,
        })
    );
    assert!(trace.borrow().is_empty());
    assert_eq!(schedule.remaining(), 1);

    runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .expect("due peer work succeeds");
    assert_eq!(*trace.borrow(), [ApplicationName::Beta]);
    assert!(schedule.is_complete());
}

#[test]
fn returned_work_error_consumes_one_item_without_blocking_a_due_peer() {
    let (mut runtime, alpha, beta, trace) = running_runtime(true);
    let alpha_item = scheduled(alpha, 0);
    let beta_item = scheduled(beta, 0);
    let mut schedule = WorkSchedule::new(&[alpha_item, beta_item]).expect("equal-time agenda");
    let clock = ManualClock::new();

    let error = runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .expect_err("alpha rejects work");
    assert_eq!(error.scheduled_work(), alpha_item);
    assert_eq!(
        error.work_error(),
        &RuntimeWorkError::Application {
            application_id: alpha,
            source: TestApplicationError::WorkRejected(ApplicationName::Alpha),
        }
    );
    assert_eq!(runtime.state(alpha), Ok(ApplicationState::Failed));
    assert_eq!(*trace.borrow(), [ApplicationName::Alpha]);
    assert_eq!(schedule.remaining(), 1);

    runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .expect("due peer work succeeds");
    assert_eq!(runtime.state(beta), Ok(ApplicationState::Running));
    assert_eq!(
        *trace.borrow(),
        [ApplicationName::Alpha, ApplicationName::Beta]
    );
}

#[derive(Debug, Eq, PartialEq)]
struct ReplayTrace {
    work_order: Vec<ApplicationName>,
    outcomes: Vec<ScheduledWorkOutcome>,
    events: Vec<Event<usize>>,
}

fn replay_scenario() -> ReplayTrace {
    let (mut runtime, alpha, beta, work_trace) = running_runtime(false);
    let mut schedule =
        WorkSchedule::new(&[scheduled(beta, 2), scheduled(alpha, 2), scheduled(alpha, 6)])
            .expect("replay agenda is ordered");
    let mut clock = ManualClock::new();
    let mut queue = EventQueue::new(3).expect("three replay events can be reserved");
    let mut outcomes = Vec::new();

    for (identifier, advance) in [2, 0, 4].into_iter().enumerate() {
        clock
            .advance(Duration::from_secs(advance))
            .expect("replay advance fits");
        let outcome = runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .expect("replay work succeeds");
        let ScheduledWorkOutcome::Completed { scheduled_work, .. } = outcome else {
            panic!("each replay item must be due after its explicit advance");
        };
        let event = Event::new(
            EventSource::Application(scheduled_work.application_id()),
            EventSeverity::Informational,
            identifier,
            EventTimestamp::from_clock(&clock),
        );
        assert_eq!(queue.emit(&event), EventEmitOutcome::Recorded);
        outcomes.push(outcome);
    }

    let mut events = Vec::new();
    while let Some(event) = queue.dequeue() {
        events.push(event);
    }
    let work_order = work_trace.borrow().clone();
    ReplayTrace {
        work_order,
        outcomes,
        events,
    }
}

#[test]
fn identical_manual_scenarios_replay_the_same_work_and_event_timestamp_trace() {
    let first = replay_scenario();
    let second = replay_scenario();

    assert_eq!(first, second);
    assert_eq!(
        first.work_order,
        [
            ApplicationName::Beta,
            ApplicationName::Alpha,
            ApplicationName::Alpha,
        ]
    );
    assert_eq!(
        first
            .events
            .iter()
            .map(|event| event.timestamp().instant())
            .collect::<Vec<_>>(),
        [instant(2), instant(2), instant(6)]
    );
    assert_eq!(
        first
            .outcomes
            .iter()
            .map(|outcome| match *outcome {
                ScheduledWorkOutcome::Completed { state, .. } => state,
                _ => panic!("the replay contains only completed scheduled work"),
            })
            .collect::<Vec<_>>(),
        [ApplicationState::Running; 3]
    );
}
