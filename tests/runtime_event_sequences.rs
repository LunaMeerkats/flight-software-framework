//! Finite direct-runtime failure-event histories through the public API.
//!
//! RUNTIME_EVENT_SEQUENCE_REVIEW.md freezes the domain. Symbolic accepted
//! records and a consumed ordinal predict retained events independently of
//! queue storage. Failure and invocation histories predict runtime effects.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, Clock, Event,
    EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp, FrameworkInstant,
    LifecycleError, ManualClock, Runtime, RuntimeWorkError,
};

const MAX_SEQUENCE_LENGTH: usize = 5;
const EXPECTED_TRACES: usize = 39_216;
const EXPECTED_OPERATIONS: usize = 189_546;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    OlderDiagnostic,
    AlphaFailure,
    BetaFailure,
    PeerFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Record {
    Older,
    Failure { application: usize, seconds: u64 },
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Work(usize),
    Advance,
    Dequeue,
    Retry(usize),
}

const OPERATIONS: [Operation; 7] = [
    Operation::Work(0),
    Operation::Work(1),
    Operation::Work(2),
    Operation::Advance,
    Operation::Dequeue,
    Operation::Retry(0),
    Operation::Retry(1),
];

struct TestApplication {
    calls: Rc<Cell<usize>>,
    failure: Option<WorkFailure>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkFailure(u16);

struct CountingClock {
    manual: ManualClock,
    reads: Cell<usize>,
}

struct Reference {
    failures: [Option<u64>; 2],
    calls: [usize; 3],
    seconds: u64,
    retry: [Option<Record>; 2],
    accepted: Vec<Record>,
    consumed: usize,
}

struct SequenceComparison {
    runtime: Runtime<TestApplication>,
    ids: [ApplicationId; 3],
    calls: [Rc<Cell<usize>>; 3],
    clock: CountingClock,
    queue: EventQueue<MissionEventId>,
    retry: [Option<Event<MissionEventId>>; 2],
    reference: Reference,
    capacity: usize,
}

#[test]
fn all_bounded_runtime_event_sequences_match_reference_history() {
    let mut traces = 0;
    let mut operations = 0;
    for capacity in [1, 2] {
        let mut sequences_at_length = 1;
        for length in 0..=MAX_SEQUENCE_LENGTH {
            for ordinal in 0..sequences_at_length {
                let sequence = decode_sequence(ordinal, length);
                let mut comparison = SequenceComparison::new(capacity);
                comparison.assert_state(&sequence, 0);
                for (position, operation) in sequence.iter().copied().enumerate() {
                    comparison.perform(operation, &sequence, position + 1);
                }
                comparison.observe_remaining(&sequence);
                traces += 1;
                operations += length;
            }
            sequences_at_length *= OPERATIONS.len();
        }
    }
    assert_eq!(traces, EXPECTED_TRACES);
    assert_eq!(operations, EXPECTED_OPERATIONS);
}

impl Application for TestApplication {
    type StartError = Infallible;
    type WorkError = WorkFailure;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.calls.set(self.calls.get() + 1);
        self.failure.map_or(Ok(()), Err)
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
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
        write!(formatter, "requested work failure {}", self.0)
    }
}

impl Error for WorkFailure {}

impl Record {
    fn event(self, ids: &[ApplicationId; 3]) -> Event<MissionEventId> {
        let (source, severity, identifier, seconds) = match self {
            Self::Older => (
                EventSource::Framework,
                EventSeverity::Warning,
                MissionEventId::OlderDiagnostic,
                99,
            ),
            Self::Failure {
                application,
                seconds,
            } => (
                EventSource::Application(ids[application]),
                EventSeverity::Error,
                failure_identifier(application),
                seconds,
            ),
        };
        Event::new(
            source,
            severity,
            identifier,
            EventTimestamp::from_elapsed(Duration::from_secs(seconds)),
        )
    }
}

impl Reference {
    fn pending(&self) -> usize {
        self.accepted.len() - self.consumed
    }

    fn emit(&mut self, record: Record, capacity: usize) -> EventEmitOutcome {
        if self.pending() == capacity {
            EventEmitOutcome::QueueFull { capacity }
        } else {
            self.accepted.push(record);
            EventEmitOutcome::Recorded
        }
    }

    fn dequeue(&mut self) -> Option<Record> {
        let record = self.accepted.get(self.consumed).copied()?;
        self.consumed += 1;
        Some(record)
    }
}

impl SequenceComparison {
    fn new(capacity: usize) -> Self {
        let calls = std::array::from_fn(|_| Rc::new(Cell::new(0)));
        let mut runtime = Runtime::new(3).expect("three application records fit");
        let ids = std::array::from_fn(|application| {
            let id = runtime
                .register(TestApplication {
                    calls: Rc::clone(&calls[application]),
                    failure: [Some(WorkFailure(11)), Some(WorkFailure(22)), None][application],
                })
                .expect("test application fits");
            runtime.start(id).expect("test application starts");
            id
        });
        let mut queue = EventQueue::new(capacity).expect("positive test capacity fits");
        assert_eq!(
            queue.emit(&Record::Older.event(&ids)),
            EventEmitOutcome::Recorded
        );
        Self {
            runtime,
            ids,
            calls,
            clock: CountingClock {
                manual: ManualClock::new(),
                reads: Cell::new(0),
            },
            queue,
            retry: [None; 2],
            reference: Reference {
                failures: [None; 2],
                calls: [0; 3],
                seconds: 0,
                retry: [None; 2],
                accepted: vec![Record::Older],
                consumed: 0,
            },
            capacity,
        }
    }

    fn perform(&mut self, operation: Operation, sequence: &[Operation], position: usize) {
        let context = format!(
            "capacity {}, trace {sequence:?}, step {position}",
            self.capacity
        );
        match operation {
            Operation::Work(application) => self.work(application, &context),
            Operation::Advance => {
                self.reference.seconds += 1;
                assert_eq!(
                    self.clock.manual.advance(Duration::from_secs(1)),
                    Ok(FrameworkInstant::from_elapsed(Duration::from_secs(
                        self.reference.seconds
                    ))),
                    "{context}"
                );
            }
            Operation::Dequeue => {
                let expected = self
                    .reference
                    .dequeue()
                    .map(|record| record.event(&self.ids));
                assert_eq!(self.queue.dequeue(), expected, "{context}");
            }
            Operation::Retry(application) => self.retry(application, &context),
        }
        self.assert_state(sequence, position);
    }

    fn work(&mut self, application: usize, context: &str) {
        let id = self.ids[application];
        let result = self.runtime.work_with_failure_event(
            id,
            failure_identifier(application),
            &self.clock,
            &mut self.queue,
        );
        if application == 2 {
            self.reference.calls[application] += 1;
            assert_eq!(result, Ok(ApplicationState::Running), "{context}");
            return;
        }
        let error = result.expect_err(context);
        if self.reference.failures[application].is_some() {
            assert_eq!(
                error.work_error(),
                &RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                    application_id: id,
                    state: ApplicationState::Failed,
                }),
                "{context}"
            );
            assert_eq!(error.failure_event_attempt(), None, "{context}");
            return;
        }
        self.reference.calls[application] += 1;
        self.reference.failures[application] = Some(self.reference.seconds);
        assert_eq!(
            error.work_error(),
            &RuntimeWorkError::Application {
                application_id: id,
                source: WorkFailure([11, 22][application]),
            },
            "{context}"
        );
        let record = Record::Failure {
            application,
            seconds: self.reference.seconds,
        };
        let expected = self.reference.emit(record, self.capacity);
        let attempt = error.failure_event_attempt().expect(context);
        assert_eq!(attempt.event(), &record.event(&self.ids), "{context}");
        assert_eq!(attempt.outcome(), expected, "{context}");
        if matches!(expected, EventEmitOutcome::QueueFull { .. }) {
            self.reference.retry[application] = Some(record);
            self.retry[application] = Some(*attempt.event());
        }
    }

    fn retry(&mut self, application: usize, context: &str) {
        // These separate slots model a caller's explicit handling policy.
        // They are not runtime-owned retry machinery or queue storage.
        let Some(record) = self.reference.retry[application] else {
            assert_eq!(self.retry[application], None, "{context}");
            return;
        };
        let event = self.retry[application].expect(context);
        assert_eq!(event, record.event(&self.ids), "{context}");
        let expected = self.reference.emit(record, self.capacity);
        assert_eq!(self.queue.emit(&event), expected, "{context}");
        if expected == EventEmitOutcome::Recorded {
            self.reference.retry[application] = None;
            self.retry[application] = None;
        }
    }

    fn assert_state(&self, sequence: &[Operation], position: usize) {
        let context = format!(
            "capacity {}, trace {sequence:?}, step {position}",
            self.capacity
        );
        for application in 0..3 {
            let failed = application < 2 && self.reference.failures[application].is_some();
            let expected = if failed {
                ApplicationState::Failed
            } else {
                ApplicationState::Running
            };
            assert_eq!(
                self.runtime.state(self.ids[application]),
                Ok(expected),
                "{context}"
            );
            assert_eq!(
                self.calls[application].get(),
                self.reference.calls[application],
                "{context}"
            );
        }
        let failures = self.reference.failures.iter().flatten().count();
        assert_eq!(self.clock.reads.get(), failures, "{context}");
        assert_eq!(self.queue.capacity(), self.capacity, "{context}");
        assert_eq!(self.queue.pending(), self.reference.pending(), "{context}");
        assert_eq!(
            self.queue.is_empty(),
            self.reference.pending() == 0,
            "{context}"
        );
        for application in 0..2 {
            let expected = self.reference.retry[application].map(|record| record.event(&self.ids));
            assert_eq!(self.retry[application], expected, "{context}");
        }
    }

    fn observe_remaining(&mut self, sequence: &[Operation]) {
        // Probe peer progress, then expose the exact retained suffix. These
        // observations do not extend the generated action counts.
        self.perform(Operation::Work(2), sequence, sequence.len() + 1);
        let remaining = self.reference.pending();
        for offset in 0..remaining + 2 {
            self.perform(Operation::Dequeue, sequence, sequence.len() + offset + 2);
        }
    }
}

fn failure_identifier(application: usize) -> MissionEventId {
    [
        MissionEventId::AlphaFailure,
        MissionEventId::BetaFailure,
        MissionEventId::PeerFailure,
    ][application]
}

fn decode_sequence(mut ordinal: usize, length: usize) -> Vec<Operation> {
    let mut sequence = Vec::with_capacity(length);
    for _ in 0..length {
        sequence.push(OPERATIONS[ordinal % OPERATIONS.len()]);
        ordinal /= OPERATIONS.len();
    }
    sequence
}
