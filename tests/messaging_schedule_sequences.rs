//! Finite sequence evidence for messaging-owned one-shot schedule consumption.
//!
//! The frozen domain is recorded in MESSAGING_SCHEDULE_SEQUENCE_REVIEW.md.
//! Numeric elapsed time, consumed-item and accepted-message histories predict
//! results independently. Final healthy work and passive peer dispatch observe
//! retained progress without completing an unattempted fault schedule item.

use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, DeliveryStatus, FrameworkInstant, LifecycleError, ManualClock,
    Message, MessageDispatchOutcome, MessagingApplication, MessagingOperationError,
    MessagingRuntime, PublishClassification, Runtime, RuntimeInboxConfig, RuntimeWorkError,
    ScheduledWork, ScheduledWorkError, ScheduledWorkOutcome, WorkSchedule,
};

const CAPACITIES: [usize; 2] = [1, 2];
const MAX_SEQUENCE_LENGTH: usize = 8;
const EXPECTED_TRACES: usize = 19_682;
const EXPECTED_OPERATIONS: usize = 147_624;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Publish,
    Advance,
    Attempt,
}

const OPERATIONS: [Operation; 3] = [Operation::Publish, Operation::Advance, Operation::Attempt];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AgendaItem {
    application_position: usize,
    release_seconds: u64,
}

const AGENDA: [AgendaItem; 4] = [
    AgendaItem {
        application_position: 0,
        release_seconds: 1,
    },
    AgendaItem {
        application_position: 1,
        release_seconds: 1,
    },
    AgendaItem {
        application_position: 0,
        release_seconds: 2,
    },
    AgendaItem {
        application_position: 1,
        release_seconds: 2,
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Active,
    Terminal,
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Accepted,
    Full,
    Offline,
}

#[derive(Clone, Copy, Debug)]
enum AttemptResult {
    Complete,
    Waiting(AgendaItem),
    Succeeded(AgendaItem),
    Failed { item: AgendaItem, discarded: usize },
    Rejected(AgendaItem),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkFailure {
    application_position: usize,
}

struct Reference {
    faulting: usize,
    elapsed_seconds: u64,
    clock_reads: usize,
    consumed: Vec<AgendaItem>,
    states: [State; 2],
    accepted: [Vec<u8>; 2],
    discarded_through: [usize; 2],
    work_order: Vec<usize>,
}

#[derive(Debug, Default)]
struct Observations {
    starts: [usize; 2],
    stops: [usize; 2],
    restarts: [usize; 2],
    work_order: Vec<usize>,
    received: [Vec<(Topic, Vec<u8>)>; 2],
}

#[derive(Debug)]
struct FaultApplication {
    position: usize,
    work_failure: Option<WorkFailure>,
    observations: Rc<RefCell<Observations>>,
}

struct CountingClock {
    manual: ManualClock,
    reads: Cell<usize>,
}

type ScheduleError = MessagingOperationError<ScheduledWorkError<WorkFailure>>;

struct SequenceComparison {
    application_ids: [ApplicationId; 2],
    owner: MessagingRuntime<FaultApplication, Topic, 1>,
    schedule: WorkSchedule,
    clock: CountingClock,
    observations: Rc<RefCell<Observations>>,
    reference: Reference,
}

#[test]
fn all_bounded_messaging_schedule_sequences_match_reference_history() {
    let mut trace_count = 0;
    let mut operation_count = 0;
    for faulting in 0..CAPACITIES.len() {
        let mut sequences_at_length = 1;
        for length in 0..=MAX_SEQUENCE_LENGTH {
            for ordinal in 0..sequences_at_length {
                let sequence = decode_sequence(ordinal, length);
                let mut comparison = SequenceComparison::new(faulting);
                comparison.assert_state(&sequence);
                for (position, operation) in sequence.iter().copied().enumerate() {
                    comparison.execute(operation, position, &sequence);
                    comparison.assert_state(&sequence);
                }
                comparison.observe_peer_progress(&sequence);
                trace_count += 1;
                operation_count += sequence.len();
            }
            sequences_at_length *= OPERATIONS.len();
        }
    }
    assert_eq!(trace_count, EXPECTED_TRACES);
    assert_eq!(operation_count, EXPECTED_OPERATIONS);
}

impl Reference {
    fn new(faulting: usize) -> Self {
        Self {
            faulting,
            elapsed_seconds: 0,
            clock_reads: 0,
            consumed: Vec::new(),
            states: [State::Active; 2],
            accepted: std::array::from_fn(|_| Vec::new()),
            discarded_through: [0; 2],
            work_order: Vec::new(),
        }
    }

    fn outstanding(&self, inbox: usize) -> usize {
        self.accepted[inbox].len() - self.discarded_through[inbox]
    }

    fn publish(&mut self, payload: u8) -> [Destination; 2] {
        [0, 1].map(|inbox| {
            if self.states[inbox] != State::Active {
                return Destination::Offline;
            }
            if self.outstanding(inbox) == CAPACITIES[inbox] {
                return Destination::Full;
            }
            self.accepted[inbox].push(payload);
            Destination::Accepted
        })
    }

    fn attempt(&mut self) -> AttemptResult {
        let Some(item) = AGENDA.get(self.consumed.len()).copied() else {
            return AttemptResult::Complete;
        };
        self.clock_reads += 1;
        if self.elapsed_seconds < item.release_seconds {
            return AttemptResult::Waiting(item);
        }
        self.consumed.push(item);
        let inbox = item.application_position;
        if self.states[inbox] == State::Terminal {
            return AttemptResult::Rejected(item);
        }
        self.work_order.push(inbox);
        if inbox != self.faulting {
            return AttemptResult::Succeeded(item);
        }
        let discarded = self.outstanding(inbox);
        self.discarded_through[inbox] = self.accepted[inbox].len();
        self.states[inbox] = State::Terminal;
        AttemptResult::Failed { item, discarded }
    }
}

impl SequenceComparison {
    fn new(faulting: usize) -> Self {
        let observations = Rc::new(RefCell::new(Observations::default()));
        let mut runtime = Runtime::new(2).expect("two records fit");
        let application_ids = [0, 1].map(|position| {
            runtime
                .register(FaultApplication {
                    position,
                    work_failure: (position == faulting).then_some(WorkFailure {
                        application_position: position,
                    }),
                    observations: Rc::clone(&observations),
                })
                .expect("application fits")
        });
        let configurations =
            CAPACITIES.map(|capacity| RuntimeInboxConfig::new(capacity, &[Topic::Command]));
        let mut owner = MessagingRuntime::new(runtime, &configurations).expect("topology is valid");
        for application_id in application_ids {
            assert_eq!(owner.start(application_id), Ok(ApplicationState::Running));
        }
        let agenda = AGENDA.map(|item| public_item(item, application_ids));
        Self {
            application_ids,
            owner,
            schedule: WorkSchedule::new(&agenda).expect("fixed agenda is ordered"),
            clock: CountingClock {
                manual: ManualClock::new(),
                reads: Cell::new(0),
            },
            observations,
            reference: Reference::new(faulting),
        }
    }

    fn execute(&mut self, operation: Operation, position: usize, sequence: &[Operation]) {
        match operation {
            Operation::Publish => {
                let payload = u8::try_from(position).expect("eight positions fit");
                self.publish(payload, sequence);
            }
            Operation::Advance => {
                self.reference.elapsed_seconds += 1;
                let instant = self
                    .clock
                    .manual
                    .advance(Duration::from_secs(1))
                    .expect("eight advances fit");
                assert_eq!(instant, elapsed(self.reference.elapsed_seconds));
            }
            Operation::Attempt => self.attempt(sequence),
        }
    }

    fn publish(&mut self, payload: u8, sequence: &[Operation]) {
        let expected = self.reference.publish(payload);
        let message = Message::try_new(Topic::Command, &[payload]).expect("one byte fits");
        let report = self
            .owner
            .publish(&message)
            .expect("report allocation succeeds");
        let actual: Vec<_> = report
            .outcomes()
            .iter()
            .map(|outcome| (outcome.application_id(), outcome.status()))
            .collect();
        let expected_outcomes = [0, 1].map(|inbox| {
            let status = match expected[inbox] {
                Destination::Accepted => DeliveryStatus::Delivered,
                Destination::Full => DeliveryStatus::InboxFull,
                Destination::Offline => DeliveryStatus::Unavailable,
            };
            (self.application_ids[inbox], status)
        });
        let accepted = expected
            .iter()
            .filter(|destination| matches!(destination, Destination::Accepted))
            .count();
        let classification = match accepted {
            0 => PublishClassification::WhollyUndelivered,
            2 => PublishClassification::Complete,
            _ => PublishClassification::Partial,
        };
        assert_eq!(
            actual, expected_outcomes,
            "fault {}, payload {payload}, trace {sequence:?}",
            self.reference.faulting
        );
        assert_eq!(
            report.classification(),
            classification,
            "trace {sequence:?}"
        );
    }

    fn attempt(&mut self, sequence: &[Operation]) {
        let expected = self.reference.attempt();
        let actual = self
            .owner
            .run_next_scheduled_work(&mut self.schedule, &self.clock);
        let observed_at = elapsed(self.reference.elapsed_seconds);
        let expected_outcome = match expected {
            AttemptResult::Complete => ScheduledWorkOutcome::Complete,
            AttemptResult::Waiting(item) => ScheduledWorkOutcome::Waiting {
                next: public_item(item, self.application_ids),
                observed_at,
            },
            AttemptResult::Succeeded(item) => ScheduledWorkOutcome::Completed {
                scheduled_work: public_item(item, self.application_ids),
                observed_at,
                state: ApplicationState::Running,
            },
            AttemptResult::Failed { item, discarded } => {
                let expected_error = RuntimeWorkError::Application {
                    application_id: self.application_ids[item.application_position],
                    source: WorkFailure {
                        application_position: item.application_position,
                    },
                };
                self.assert_schedule_error(
                    actual.unwrap_err(),
                    item,
                    discarded,
                    expected_error,
                    sequence,
                );
                return;
            }
            AttemptResult::Rejected(item) => {
                let expected_error = RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                    application_id: self.application_ids[item.application_position],
                    state: ApplicationState::Failed,
                });
                self.assert_schedule_error(actual.unwrap_err(), item, 0, expected_error, sequence);
                return;
            }
        };
        assert_eq!(actual, Ok(expected_outcome), "trace {sequence:?}");
    }

    fn assert_schedule_error(
        &self,
        error: ScheduleError,
        item: AgendaItem,
        discarded: usize,
        expected: RuntimeWorkError<WorkFailure>,
        sequence: &[Operation],
    ) {
        assert_eq!(
            error.discarded_deliveries(),
            discarded,
            "trace {sequence:?}"
        );
        let scheduled_error = error.operation_error();
        assert_eq!(
            scheduled_error.scheduled_work(),
            public_item(item, self.application_ids)
        );
        assert_eq!(
            scheduled_error.observed_at(),
            elapsed(self.reference.elapsed_seconds)
        );
        assert_eq!(
            scheduled_error.work_error(),
            &expected,
            "trace {sequence:?}"
        );
        assert_error_sources(&error, &expected);
        assert_eq!(error.into_operation_error().into_work_error(), expected);
    }

    fn assert_state(&self, sequence: &[Operation]) {
        self.assert_control_state(sequence);
        for (inbox, capacity) in CAPACITIES.into_iter().enumerate() {
            let application_id = self.application_ids[inbox];
            let outstanding = self.reference.outstanding(inbox);
            assert!(outstanding <= capacity, "inbox {inbox}, trace {sequence:?}");
            assert_eq!(self.owner.inbox_capacity(application_id), Ok(capacity));
            assert_eq!(
                self.owner.pending(application_id),
                Ok(outstanding),
                "fault {}, inbox {inbox}, trace {sequence:?}",
                self.reference.faulting
            );
            if self.reference.states[inbox] == State::Terminal {
                assert_eq!(outstanding, 0, "trace {sequence:?}");
            }
            assert!(self.observations.borrow().received[inbox].is_empty());
        }
    }

    fn assert_control_state(&self, sequence: &[Operation]) {
        let observations = self.observations.borrow();
        assert_eq!(observations.starts, [1; 2], "trace {sequence:?}");
        assert_eq!(observations.stops, [0; 2], "trace {sequence:?}");
        assert_eq!(observations.restarts, [0; 2], "trace {sequence:?}");
        assert_eq!(
            observations.work_order, self.reference.work_order,
            "trace {sequence:?}"
        );
        for inbox in 0..CAPACITIES.len() {
            assert_eq!(
                self.owner.state(self.application_ids[inbox]),
                Ok(public_state(self.reference.states[inbox])),
                "fault {}, inbox {inbox}, trace {sequence:?}",
                self.reference.faulting
            );
        }
        let remaining = AGENDA.len() - self.reference.consumed.len();
        assert_eq!(self.schedule.remaining(), remaining, "trace {sequence:?}");
        assert_eq!(
            self.schedule.is_complete(),
            remaining == 0,
            "trace {sequence:?}"
        );
        assert_eq!(
            self.clock.reads.get(),
            self.reference.clock_reads,
            "trace {sequence:?}"
        );
        assert_eq!(
            self.clock.manual.now(),
            elapsed(self.reference.elapsed_seconds)
        );
    }

    fn observe_peer_progress(&mut self, sequence: &[Operation]) {
        // The healthy peer is observable even when failure ends the trace.
        let peer = 1 - self.reference.faulting;
        self.reference.work_order.push(peer);
        assert_eq!(
            self.owner.work(self.application_ids[peer]),
            Ok(ApplicationState::Running),
            "trace {sequence:?}"
        );
        self.assert_state(sequence);
        let suffix = &self.reference.accepted[peer][self.reference.discarded_through[peer]..];
        let expected: Vec<_> = suffix
            .iter()
            .map(|&byte| (Topic::Command, vec![byte]))
            .collect();
        for consumed in 0..suffix.len() {
            assert_eq!(
                self.owner.dispatch_one(self.application_ids[peer]),
                Ok(MessageDispatchOutcome::Dispatched),
                "trace {sequence:?}"
            );
            assert_eq!(
                self.observations.borrow().received[peer],
                expected[..=consumed],
                "trace {sequence:?}"
            );
            assert_eq!(
                self.owner.pending(self.application_ids[peer]),
                Ok(suffix.len() - consumed - 1)
            );
        }
        assert_eq!(
            self.owner.dispatch_one(self.application_ids[peer]),
            Ok(MessageDispatchOutcome::InboxEmpty),
            "trace {sequence:?}"
        );
        assert_eq!(self.observations.borrow().received[peer], expected);
        assert!(self.observations.borrow().received[self.reference.faulting].is_empty());
        assert_eq!(self.owner.pending(self.application_ids[peer]), Ok(0));
        self.assert_control_state(sequence);
    }
}

impl Application for FaultApplication {
    type StartError = Infallible;
    type WorkError = WorkFailure;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.observations.borrow_mut().starts[self.position] += 1;
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.observations
            .borrow_mut()
            .work_order
            .push(self.position);
        match self.work_failure {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.observations.borrow_mut().stops[self.position] += 1;
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.observations.borrow_mut().restarts[self.position] += 1;
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for FaultApplication {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        _context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        self.observations.borrow_mut().received[self.position]
            .push((*message.topic(), message.payload().to_vec()));
        Ok(())
    }
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        self.manual.now()
    }
}

impl fmt::Display for WorkFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "requested work failure at position {}",
            self.application_position
        )
    }
}

impl Error for WorkFailure {}

fn assert_error_sources(error: &ScheduleError, expected: &RuntimeWorkError<WorkFailure>) {
    let scheduled_error = Error::source(error)
        .and_then(|source| source.downcast_ref::<ScheduledWorkError<WorkFailure>>())
        .expect("messaging source retains scheduled error");
    assert_eq!(scheduled_error, error.operation_error());
    let work_error = Error::source(scheduled_error)
        .and_then(|source| source.downcast_ref::<RuntimeWorkError<WorkFailure>>())
        .expect("scheduled source retains runtime work error");
    assert_eq!(work_error, expected);
    match expected {
        RuntimeWorkError::Application { source, .. } => {
            assert_eq!(
                Error::source(work_error).and_then(|error| error.downcast_ref()),
                Some(source)
            );
        }
        RuntimeWorkError::Lifecycle(source) => {
            assert_eq!(
                Error::source(work_error).and_then(|error| error.downcast_ref()),
                Some(source)
            );
        }
    }
}

fn elapsed(seconds: u64) -> FrameworkInstant {
    FrameworkInstant::from_elapsed(Duration::from_secs(seconds))
}

fn public_item(item: AgendaItem, application_ids: [ApplicationId; 2]) -> ScheduledWork {
    ScheduledWork::new(
        application_ids[item.application_position],
        elapsed(item.release_seconds),
    )
}

fn public_state(state: State) -> ApplicationState {
    match state {
        State::Active => ApplicationState::Running,
        State::Terminal => ApplicationState::Failed,
    }
}

fn decode_sequence(mut ordinal: usize, length: usize) -> Vec<Operation> {
    (0..length)
        .map(|_| {
            let operation = OPERATIONS[ordinal % OPERATIONS.len()];
            ordinal /= OPERATIONS.len();
            operation
        })
        .collect()
}
