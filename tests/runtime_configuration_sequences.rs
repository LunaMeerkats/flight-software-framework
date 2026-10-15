//! Finite public-API evidence for runtime configuration and work histories.
//!
//! The frozen domain is in RUNTIME_CONFIGURATION_SEQUENCE_REVIEW.md. An
//! acceptance/predecessor ledger and movement history predict configuration;
//! independent lifecycle rules and callback logs predict application behavior.

use std::cell::RefCell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, ConfigurationError,
    ConfigurationTable, LifecycleError, LifecycleOperation, Runtime, RuntimeConfigurationError,
    RuntimeRestartError, RuntimeStopError, RuntimeWorkError,
};

const MAX_BYTES: usize = 2;
const MAX_SEQUENCE_LENGTH: usize = 5;
const EXPECTED_TRACES: usize = 74_898;
const EXPECTED_OPERATIONS: usize = 363_792;
const EXPECTED_PROBES: usize = 524_286;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Content {
    Short,
    ExactBound,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Replace(Content),
    Reject,
    Rollback,
    WorkSelected,
    WorkPeer,
    StopSelected,
    RestartSelected,
}

const OPERATIONS: [Operation; 8] = [
    Operation::Replace(Content::Short),
    Operation::Replace(Content::ExactBound),
    Operation::Reject,
    Operation::Rollback,
    Operation::WorkSelected,
    Operation::WorkPeer,
    Operation::StopSelected,
    Operation::RestartSelected,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Active,
    Paused,
    Terminal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CopiedConfiguration {
    revision: u64,
    bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Callback {
    Start,
    Work(CopiedConfiguration),
    Stop,
    Restart,
}

#[derive(Debug, Eq, PartialEq)]
struct ValidationError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkFailure {
    application_position: usize,
    call: usize,
}

enum WorkResult {
    Succeeded,
    Failed(WorkFailure),
    Rejected(State),
}

struct Acceptance {
    content: Content,
    predecessor: Option<usize>,
}

enum Movement {
    Accept(usize),
    Restore(usize),
}

struct Reference {
    accepted: Vec<Acceptance>,
    movements: Vec<Movement>,
    selected: usize,
    states: [State; 2],
    works: [usize; 2],
    callbacks: [Vec<Callback>; 2],
}

#[derive(Debug)]
struct ObservingApplication {
    work_failure: Option<WorkFailure>,
    work_calls: usize,
    callbacks: Rc<RefCell<Vec<Callback>>>,
}

struct SequenceComparison {
    runtime: Runtime<ObservingApplication, ValidationError, MAX_BYTES>,
    application_ids: [ApplicationId; 2],
    callbacks: [Rc<RefCell<Vec<Callback>>>; 2],
    reference: Reference,
}

#[test]
fn all_bounded_runtime_configuration_sequences_match_reference_history() {
    let mut trace_count = 0;
    let mut operation_count = 0;
    let mut probe_count = 0;
    for selected in 0..2 {
        let mut sequences_at_length = 1;
        for length in 0..=MAX_SEQUENCE_LENGTH {
            for ordinal in 0..sequences_at_length {
                let sequence = decode_sequence(ordinal, length);
                let mut comparison = SequenceComparison::new(selected);
                comparison.assert_state(&sequence, 0);
                for (position, operation) in sequence.iter().copied().enumerate() {
                    comparison.perform(operation, &sequence, position + 1);
                }
                probe_count += comparison.probe_retained_history(&sequence);
                trace_count += 1;
                operation_count += sequence.len();
            }
            sequences_at_length *= OPERATIONS.len();
        }
    }
    assert_eq!(trace_count, EXPECTED_TRACES);
    assert_eq!(operation_count, EXPECTED_OPERATIONS);
    assert_eq!(probe_count, EXPECTED_PROBES);
}

impl Content {
    fn bytes(self) -> &'static [u8] {
        match self {
            Self::Short => &[1],
            Self::ExactBound => &[2, 3],
        }
    }
}

impl Reference {
    fn new(selected: usize) -> Self {
        Self {
            accepted: vec![Acceptance {
                content: Content::Short,
                predecessor: None,
            }],
            movements: vec![Movement::Accept(0)],
            selected,
            states: [State::Active; 2],
            works: [0; 2],
            callbacks: std::array::from_fn(|_| vec![Callback::Start]),
        }
    }

    fn current_index(&self) -> usize {
        match self.movements.last().expect("initial acceptance exists") {
            Movement::Accept(index) | Movement::Restore(index) => *index,
        }
    }

    fn configuration(&self) -> CopiedConfiguration {
        let index = self.current_index();
        CopiedConfiguration {
            revision: revision(index),
            bytes: self.accepted[index].content.bytes().to_vec(),
        }
    }

    fn replace(&mut self, content: Content) -> u64 {
        let index = self.accepted.len();
        self.accepted.push(Acceptance {
            content,
            predecessor: Some(self.current_index()),
        });
        self.movements.push(Movement::Accept(index));
        revision(index)
    }

    fn rollback(&mut self) -> Result<u64, RuntimeConfigurationError<ValidationError>> {
        let predecessor = match self.movements.last().expect("initial acceptance exists") {
            Movement::Accept(index) => self.accepted[*index].predecessor,
            Movement::Restore(_) => None,
        };
        let index = predecessor.ok_or(RuntimeConfigurationError::Table(
            ConfigurationError::NoRollbackAvailable,
        ))?;
        self.movements.push(Movement::Restore(index));
        Ok(revision(index))
    }

    fn work(&mut self, position: usize) -> WorkResult {
        if self.states[position] != State::Active {
            return WorkResult::Rejected(self.states[position]);
        }
        let configuration = self.configuration();
        self.callbacks[position].push(Callback::Work(configuration));
        self.works[position] += 1;
        if position == self.selected && self.works[position] == 2 {
            self.states[position] = State::Terminal;
            return WorkResult::Failed(WorkFailure {
                application_position: position,
                call: 2,
            });
        }
        WorkResult::Succeeded
    }

    fn stop_selected(&mut self) -> bool {
        if self.states[self.selected] != State::Active {
            return false;
        }
        self.callbacks[self.selected].push(Callback::Stop);
        self.states[self.selected] = State::Paused;
        true
    }

    fn restart_selected(&mut self) -> bool {
        if self.states[self.selected] != State::Paused {
            return false;
        }
        self.callbacks[self.selected].push(Callback::Restart);
        self.states[self.selected] = State::Active;
        true
    }
}

impl SequenceComparison {
    fn new(selected: usize) -> Self {
        let callbacks = std::array::from_fn(|_| Rc::new(RefCell::new(Vec::new())));
        let table = ConfigurationTable::new(Content::Short.bytes(), validate)
            .expect("the frozen initial content is accepted");
        let mut runtime = Runtime::with_configuration(2, table).expect("two records fit");
        let application_ids = [0, 1].map(|position| {
            runtime
                .register(ObservingApplication {
                    work_failure: (position == selected).then_some(WorkFailure {
                        application_position: position,
                        call: 2,
                    }),
                    work_calls: 0,
                    callbacks: Rc::clone(&callbacks[position]),
                })
                .expect("application fits")
        });
        for application_id in application_ids {
            assert_eq!(runtime.start(application_id), Ok(ApplicationState::Running));
        }
        Self {
            runtime,
            application_ids,
            callbacks,
            reference: Reference::new(selected),
        }
    }

    fn perform(&mut self, operation: Operation, sequence: &[Operation], step: usize) {
        let selected = self.reference.selected;
        match operation {
            Operation::Replace(content) => {
                let expected = Ok(self.reference.replace(content));
                let actual = self.runtime.replace_configuration(content.bytes());
                assert_eq!(actual, expected, "trace {sequence:?}, step {step}");
            }
            Operation::Reject => self.reject(sequence, step),
            Operation::Rollback => self.rollback(sequence, step),
            Operation::WorkSelected => self.work(selected, sequence, step),
            Operation::WorkPeer => self.work(1 - selected, sequence, step),
            Operation::StopSelected => self.stop_selected(sequence, step),
            Operation::RestartSelected => self.restart_selected(sequence, step),
        }
        self.assert_state(sequence, step);
    }

    fn reject(&mut self, sequence: &[Operation], step: usize) {
        let error = self
            .runtime
            .replace_configuration(&[0])
            .expect_err("the frozen mission rejects zero content");
        assert_eq!(
            error,
            RuntimeConfigurationError::Table(ConfigurationError::Rejected(ValidationError)),
            "trace {sequence:?}, step {step}"
        );
        assert_configuration_source(&error, sequence, step);
    }

    fn rollback(&mut self, sequence: &[Operation], step: usize) {
        let expected = self.reference.rollback();
        let actual = self.runtime.rollback_configuration();
        assert_eq!(actual, expected, "trace {sequence:?}, step {step}");
        if let Err(error) = &actual {
            assert_configuration_source(error, sequence, step);
        }
    }

    fn work(&mut self, position: usize, sequence: &[Operation], step: usize) {
        let application_id = self.application_ids[position];
        let expected = self.reference.work(position);
        let actual = self.runtime.work(application_id);
        match expected {
            WorkResult::Succeeded => {
                assert_eq!(
                    actual,
                    Ok(ApplicationState::Running),
                    "trace {sequence:?}, step {step}"
                );
            }
            WorkResult::Failed(source) => {
                let error = actual.expect_err("the second accepted selected work fails");
                assert_eq!(
                    error,
                    RuntimeWorkError::Application {
                        application_id,
                        source,
                    },
                    "trace {sequence:?}, step {step}"
                );
                assert_eq!(
                    Error::source(&error).and_then(|error| error.downcast_ref()),
                    Some(&source),
                    "trace {sequence:?}, step {step}"
                );
                assert!(Error::source(&source).is_none());
            }
            WorkResult::Rejected(state) => {
                let error = actual.expect_err("unavailable work is rejected");
                let source = LifecycleError::NotRunning {
                    application_id,
                    state: public_state(state),
                };
                assert_eq!(
                    error,
                    RuntimeWorkError::Lifecycle(source),
                    "trace {sequence:?}, step {step}"
                );
                assert_lifecycle_source(&error, &source, sequence, step);
            }
        }
    }

    fn stop_selected(&mut self, sequence: &[Operation], step: usize) {
        let application_id = self.application_ids[self.reference.selected];
        let expected = self.reference.stop_selected();
        let actual = self.runtime.stop(application_id);
        if expected {
            assert_eq!(
                actual,
                Ok(ApplicationState::Stopped),
                "trace {sequence:?}, step {step}"
            );
            return;
        }
        let error = actual.expect_err("paused or terminal stop is rejected");
        let source = LifecycleError::InvalidTransition {
            application_id,
            state: public_state(self.reference.states[self.reference.selected]),
            operation: LifecycleOperation::Stop,
        };
        assert_eq!(
            error,
            RuntimeStopError::Lifecycle(source),
            "trace {sequence:?}, step {step}"
        );
        assert_lifecycle_source(&error, &source, sequence, step);
    }

    fn restart_selected(&mut self, sequence: &[Operation], step: usize) {
        let application_id = self.application_ids[self.reference.selected];
        let expected = self.reference.restart_selected();
        let actual = self.runtime.restart(application_id);
        if expected {
            assert_eq!(
                actual,
                Ok(ApplicationState::Running),
                "trace {sequence:?}, step {step}"
            );
            return;
        }
        let error = actual.expect_err("active or terminal restart is rejected");
        let source = LifecycleError::InvalidTransition {
            application_id,
            state: public_state(self.reference.states[self.reference.selected]),
            operation: LifecycleOperation::Restart,
        };
        assert_eq!(
            error,
            RuntimeRestartError::Lifecycle(source),
            "trace {sequence:?}, step {step}"
        );
        assert_lifecycle_source(&error, &source, sequence, step);
    }

    fn assert_state(&self, sequence: &[Operation], step: usize) {
        for position in 0..2 {
            assert_eq!(
                self.runtime.state(self.application_ids[position]),
                Ok(public_state(self.reference.states[position])),
                "selected {}, position {position}, trace {sequence:?}, step {step}",
                self.reference.selected
            );
            assert_eq!(
                *self.callbacks[position].borrow(),
                self.reference.callbacks[position],
                "selected {}, position {position}, trace {sequence:?}, step {step}",
                self.reference.selected
            );
        }
    }

    fn probe_retained_history(&mut self, sequence: &[Operation]) -> usize {
        // Only the healthy peer observes suffixes, so probes cannot introduce
        // a selected failure absent from the generated history. Mutating
        // probes expose retained rollback and the next acceptance revision.
        let probes = [
            Operation::WorkPeer,
            Operation::Rollback,
            Operation::WorkPeer,
            Operation::Replace(Content::Short),
            Operation::WorkPeer,
            Operation::Rollback,
            Operation::WorkPeer,
        ];
        for (offset, operation) in probes.into_iter().enumerate() {
            self.perform(operation, sequence, sequence.len() + offset + 1);
        }
        probes.len()
    }
}

impl Application for ObservingApplication {
    type StartError = Infallible;
    type WorkError = WorkFailure;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.callbacks.borrow_mut().push(Callback::Start);
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        let configuration = context.configuration().expect("the runtime owns a table");
        self.work_calls += 1;
        self.callbacks
            .borrow_mut()
            .push(Callback::Work(CopiedConfiguration {
                revision: configuration.revision(),
                bytes: configuration.bytes().to_vec(),
            }));
        if let Some(failure) = self.work_failure
            && self.work_calls == failure.call
        {
            return Err(failure);
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.callbacks.borrow_mut().push(Callback::Stop);
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.callbacks.borrow_mut().push(Callback::Restart);
        Ok(())
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("the frozen mission rejects empty content or a first zero byte")
    }
}

impl Error for ValidationError {}

impl fmt::Display for WorkFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "application {} returned its error on work call {}",
            self.application_position, self.call
        )
    }
}

impl Error for WorkFailure {}

fn validate(bytes: &[u8]) -> Result<(), ValidationError> {
    match bytes.first() {
        Some(first) if *first != 0 => Ok(()),
        _ => Err(ValidationError),
    }
}

fn assert_configuration_source(
    error: &RuntimeConfigurationError<ValidationError>,
    sequence: &[Operation],
    step: usize,
) {
    let RuntimeConfigurationError::Table(expected) = error else {
        panic!("configured runtime unexpectedly lost its table");
    };
    let table_error = Error::source(error)
        .and_then(|source| source.downcast_ref::<ConfigurationError<ValidationError>>())
        .expect("runtime wrapper exposes its concrete configuration error");
    assert_eq!(table_error, expected, "trace {sequence:?}, step {step}");
    match expected {
        ConfigurationError::Rejected(validation) => {
            assert_eq!(
                Error::source(table_error).and_then(|source| source.downcast_ref()),
                Some(validation),
                "trace {sequence:?}, step {step}"
            );
            assert!(Error::source(validation).is_none());
        }
        ConfigurationError::NoRollbackAvailable => {
            assert!(
                Error::source(table_error).is_none(),
                "trace {sequence:?}, step {step}"
            );
        }
        _ => panic!("the frozen domain cannot produce another configuration error"),
    }
}

fn assert_lifecycle_source(
    error: &impl Error,
    expected: &LifecycleError,
    sequence: &[Operation],
    step: usize,
) {
    assert_eq!(
        Error::source(error).and_then(|source| source.downcast_ref()),
        Some(expected),
        "trace {sequence:?}, step {step}"
    );
    assert!(Error::source(expected).is_none());
}

fn public_state(state: State) -> ApplicationState {
    match state {
        State::Active => ApplicationState::Running,
        State::Paused => ApplicationState::Stopped,
        State::Terminal => ApplicationState::Failed,
    }
}

fn revision(index: usize) -> u64 {
    u64::try_from(index + 1).expect("the bounded acceptance history fits in u64")
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
