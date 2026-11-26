//! Public-API evidence for opt-in diagnostics after returned stop failures.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, DeliveryStatus, Event, EventEmitOutcome, EventQueue,
    EventSeverity, EventSource, EventTimestamp, FrameworkInstant, LifecycleError,
    LifecycleOperation, LifecycleRegistry, Message, MessageDispatchOutcome, MessagingApplication,
    MessagingOperationError, MessagingRuntime, Runtime, RuntimeInboxConfig, RuntimeStopError,
    RuntimeStopEventError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Behavior {
    Healthy,
    StopFails,
    WorkFails,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Diagnostic {
    Older,
    StopFailed,
}

#[derive(Debug, Eq, PartialEq)]
struct MissionFailure {
    code: u16,
}

struct MissionApplication {
    behavior: Behavior,
    stop_calls: usize,
    work_calls: usize,
    messages: Vec<u8>,
}

struct CountingClock {
    reads: Cell<usize>,
    elapsed: Cell<Duration>,
}

type DirectRuntime = Runtime<MissionApplication>;
type MissionRuntime = MessagingRuntime<MissionApplication, u8, 1>;
type StopEventError = RuntimeStopEventError<MissionFailure, Diagnostic>;
type MessagingStopEventError = MessagingOperationError<StopEventError>;

struct Mission {
    runtime: MissionRuntime,
    selected: ApplicationId,
    peer: ApplicationId,
}

impl fmt::Display for MissionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "requested failure {}", self.code)
    }
}

impl Error for MissionFailure {}

impl Application for MissionApplication {
    type StartError = Infallible;
    type WorkError = MissionFailure;
    type StopError = MissionFailure;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.work_calls += 1;
        if self.behavior == Behavior::WorkFails {
            return Err(MissionFailure { code: 37 });
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.stop_calls += 1;
        if self.behavior == Behavior::StopFails {
            return Err(MissionFailure { code: 29 });
        }
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<u8, 1> for MissionApplication {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<u8, 1>,
        _context: &mut ApplicationMessageContext<'_, u8, 1>,
    ) -> Result<(), Self::MessageError> {
        self.messages.extend_from_slice(message.payload());
        Ok(())
    }
}

impl Default for CountingClock {
    fn default() -> Self {
        Self {
            reads: Cell::new(0),
            elapsed: Cell::new(Duration::from_secs(9)),
        }
    }
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        FrameworkInstant::from_elapsed(self.elapsed.get())
    }
}

impl Mission {
    fn registered(behavior: Behavior) -> Self {
        let (runtime, selected, peer) = direct(behavior);
        let inboxes = [RuntimeInboxConfig::new(2, &[1]); 2];
        Self {
            runtime: MissionRuntime::new(runtime, &inboxes).unwrap(),
            selected,
            peer,
        }
    }

    fn started(behavior: Behavior) -> Self {
        let mut mission = Self::registered(behavior);
        mission.runtime.start(mission.selected).unwrap();
        mission.runtime.start(mission.peer).unwrap();
        mission
    }

    fn fill_inputs(&mut self) {
        for value in [11, 22] {
            let report = self.runtime.publish(&input(value)).unwrap();
            assert_eq!(report.outcomes().len(), 2);
            assert!(
                report
                    .outcomes()
                    .iter()
                    .all(|outcome| { outcome.status() == DeliveryStatus::Delivered })
            );
        }
        assert_eq!(self.runtime.pending(self.selected), Ok(2));
        assert_eq!(self.runtime.pending(self.peer), Ok(2));
    }

    fn stop_report(
        &mut self,
        clock: &CountingClock,
        events: &mut EventQueue<Diagnostic>,
    ) -> MessagingStopEventError {
        self.runtime
            .stop_with_failure_event(self.selected, Diagnostic::StopFailed, clock, events)
            .unwrap_err()
    }

    fn assert_peer_progress(&mut self) {
        assert_eq!(self.runtime.work(self.peer), Ok(ApplicationState::Running));
        for _ in 0..2 {
            assert_eq!(
                self.runtime.dispatch_one(self.peer),
                Ok(MessageDispatchOutcome::Dispatched)
            );
        }
        let peer = self.runtime.application(self.peer).unwrap();
        assert_eq!(peer.work_calls, 1);
        assert_eq!(peer.stop_calls, 0);
        assert_eq!(peer.messages, [11, 22]);
        assert_eq!(self.runtime.pending(self.peer), Ok(0));
    }
}

fn direct(behavior: Behavior) -> (DirectRuntime, ApplicationId, ApplicationId) {
    let mut runtime = Runtime::new(2).unwrap();
    let [selected, peer] = [behavior, Behavior::Healthy].map(|behavior| {
        runtime
            .register(MissionApplication {
                behavior,
                stop_calls: 0,
                work_calls: 0,
                messages: Vec::new(),
            })
            .unwrap()
    });
    (runtime, selected, peer)
}

fn input(value: u8) -> Message<u8, 1> {
    Message::try_new(1, &[value]).unwrap()
}

fn expected_error(application_id: ApplicationId) -> RuntimeStopError<MissionFailure> {
    RuntimeStopError::Application {
        application_id,
        source: MissionFailure { code: 29 },
    }
}

fn expected_event(application_id: ApplicationId) -> Event<Diagnostic> {
    Event::new(
        EventSource::Application(application_id),
        EventSeverity::Error,
        Diagnostic::StopFailed,
        EventTimestamp::from_elapsed(Duration::from_secs(9)),
    )
}

fn older_event() -> Event<Diagnostic> {
    Event::new(
        EventSource::Framework,
        EventSeverity::Warning,
        Diagnostic::Older,
        EventTimestamp::from_elapsed(Duration::from_secs(2)),
    )
}

fn unknown_id() -> ApplicationId {
    let mut registry = LifecycleRegistry::new(3).unwrap();
    registry.register().unwrap();
    registry.register().unwrap();
    registry.register().unwrap()
}

fn assert_source_chain(error: &StopEventError) {
    let stop = Error::source(error)
        .and_then(|source| source.downcast_ref::<RuntimeStopError<MissionFailure>>())
        .unwrap();
    assert_eq!(stop, error.stop_error());
    let cause = Error::source(stop)
        .and_then(|source| source.downcast_ref::<MissionFailure>())
        .unwrap();
    assert_eq!(cause, &MissionFailure { code: 29 });
    assert!(Error::source(cause).is_none());
}

fn rejected_error(application_id: ApplicationId, state: ApplicationState) -> LifecycleError {
    LifecycleError::InvalidTransition {
        application_id,
        state,
        operation: LifecycleOperation::Stop,
    }
}

fn prepare_direct_state(state: ApplicationState) -> (DirectRuntime, ApplicationId) {
    let (mut runtime, selected, _) = direct(Behavior::WorkFails);
    match state {
        ApplicationState::Registered => {}
        ApplicationState::Stopped => {
            runtime.start(selected).unwrap();
            runtime.stop(selected).unwrap();
        }
        ApplicationState::Failed => {
            runtime.start(selected).unwrap();
            runtime.work(selected).unwrap_err();
        }
        ApplicationState::Running => unreachable!("rejection test needs a non-running state"),
    }
    (runtime, selected)
}

fn prepare_messaging_state(state: ApplicationState) -> Mission {
    let mut mission = Mission::registered(Behavior::WorkFails);
    match state {
        ApplicationState::Registered => {}
        ApplicationState::Stopped => {
            mission.runtime.start(mission.selected).unwrap();
            mission.runtime.stop(mission.selected).unwrap();
        }
        ApplicationState::Failed => {
            mission.runtime.start(mission.selected).unwrap();
            mission.runtime.work(mission.selected).unwrap_err();
        }
        ApplicationState::Running => unreachable!("rejection test needs a non-running state"),
    }
    mission.runtime.start(mission.peer).unwrap();
    mission.runtime.publish(&input(77)).unwrap();
    mission
}

#[test]
fn direct_and_messaging_success_preserve_stop_results_without_reporting() {
    let (mut runtime, selected, _) = direct(Behavior::Healthy);
    runtime.start(selected).unwrap();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    assert_eq!(events.emit(&older_event()), EventEmitOutcome::Recorded);
    assert_eq!(
        runtime.stop_with_failure_event(selected, Diagnostic::StopFailed, &clock, &mut events),
        Ok(ApplicationState::Stopped)
    );
    assert_eq!(runtime.application(selected).unwrap().stop_calls, 1);
    let mut mission = Mission::started(Behavior::Healthy);
    mission.fill_inputs();
    let stopped = mission
        .runtime
        .stop_with_failure_event(
            mission.selected,
            Diagnostic::StopFailed,
            &clock,
            &mut events,
        )
        .unwrap();
    assert_eq!(stopped.state(), ApplicationState::Stopped);
    assert_eq!(stopped.discarded_deliveries(), 2);
    assert_eq!(mission.runtime.pending(mission.selected), Ok(0));
    assert_eq!(mission.runtime.pending(mission.peer), Ok(2));
    assert_eq!(
        mission
            .runtime
            .application(mission.selected)
            .unwrap()
            .stop_calls,
        1
    );
    mission.assert_peer_progress();
    assert_eq!(clock.reads.get(), 0);
    assert_eq!(events.dequeue(), Some(older_event()));
    assert!(events.is_empty());
}

#[test]
fn direct_and_messaging_reject_every_non_running_state_without_reporting() {
    for state in [
        ApplicationState::Registered,
        ApplicationState::Stopped,
        ApplicationState::Failed,
    ] {
        let (mut runtime, selected) = prepare_direct_state(state);
        let before = runtime.application(selected).unwrap().stop_calls;
        let clock = CountingClock::default();
        let mut events = EventQueue::new(1).unwrap();
        assert_eq!(events.emit(&older_event()), EventEmitOutcome::Recorded);
        let error = runtime
            .stop_with_failure_event(selected, Diagnostic::StopFailed, &clock, &mut events)
            .unwrap_err();
        assert_eq!(error.failure_event_attempt(), None);
        assert_eq!(
            error.into_stop_error(),
            RuntimeStopError::Lifecycle(rejected_error(selected, state))
        );
        assert_eq!(runtime.application(selected).unwrap().stop_calls, before);
        assert_eq!(runtime.state(selected), Ok(state));
        assert_messaging_rejection(state, &clock, &mut events);
        assert_eq!(clock.reads.get(), 0);
        assert_eq!(events.dequeue(), Some(older_event()));
        assert!(events.is_empty());
    }
}

fn assert_messaging_rejection(
    state: ApplicationState,
    clock: &CountingClock,
    events: &mut EventQueue<Diagnostic>,
) {
    let mut mission = prepare_messaging_state(state);
    let before = mission
        .runtime
        .application(mission.selected)
        .unwrap()
        .stop_calls;
    let error = mission.stop_report(clock, events);
    assert_eq!(error.discarded_deliveries(), 0);
    let (stop, attempt) = error.into_operation_error().into_parts();
    assert_eq!(
        stop,
        RuntimeStopError::Lifecycle(rejected_error(mission.selected, state))
    );
    assert_eq!(attempt, None);
    assert_eq!(
        mission
            .runtime
            .application(mission.selected)
            .unwrap()
            .stop_calls,
        before
    );
    assert_eq!(mission.runtime.state(mission.selected), Ok(state));
    assert_eq!(mission.runtime.pending(mission.selected), Ok(0));
    assert_eq!(mission.runtime.pending(mission.peer), Ok(1));
    assert_eq!(
        mission.runtime.state(mission.peer),
        Ok(ApplicationState::Running)
    );
}

#[test]
fn unknown_identity_rejects_both_owners_without_reporting() {
    let unknown = unknown_id();
    let (mut runtime, selected, peer) = direct(Behavior::StopFails);
    runtime.start(selected).unwrap();
    runtime.start(peer).unwrap();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    let error = runtime
        .stop_with_failure_event(unknown, Diagnostic::StopFailed, &clock, &mut events)
        .unwrap_err();
    assert_eq!(error.failure_event_attempt(), None);
    assert_eq!(
        error.into_stop_error(),
        RuntimeStopError::Lifecycle(LifecycleError::UnknownApplication {
            application_id: unknown
        })
    );
    assert_eq!(runtime.application(selected).unwrap().stop_calls, 0);
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Running));
    assert_eq!(runtime.state(peer), Ok(ApplicationState::Running));
    let mut mission = Mission::started(Behavior::StopFails);
    mission.fill_inputs();
    let error = mission
        .runtime
        .stop_with_failure_event(unknown, Diagnostic::StopFailed, &clock, &mut events)
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(error.operation_error().failure_event_attempt(), None);
    assert_eq!(
        error.into_operation_error().into_stop_error(),
        RuntimeStopError::Lifecycle(LifecycleError::UnknownApplication {
            application_id: unknown
        })
    );
    assert_eq!(mission.runtime.pending(mission.selected), Ok(2));
    assert_eq!(mission.runtime.pending(mission.peer), Ok(2));
    assert_eq!(
        mission
            .runtime
            .application(mission.selected)
            .unwrap()
            .stop_calls,
        0
    );
    assert_eq!(clock.reads.get(), 0);
    assert!(events.is_empty());
}

#[test]
fn direct_failure_preserves_typed_error_and_healthy_peer_work() {
    let (mut runtime, selected, peer) = direct(Behavior::StopFails);
    runtime.start(selected).unwrap();
    runtime.start(peer).unwrap();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    let error = runtime
        .stop_with_failure_event(selected, Diagnostic::StopFailed, &clock, &mut events)
        .unwrap_err();
    assert_eq!(error.stop_error(), &expected_error(selected));
    assert_source_chain(&error);
    assert!(error.to_string().contains("requested failure 29"));
    assert!(!error.to_string().contains("rejected at queue capacity"));
    let attempt = error.failure_event_attempt().unwrap();
    assert_eq!(attempt.event(), &expected_event(selected));
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(error.into_stop_error(), expected_error(selected));
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Failed));
    assert_eq!(runtime.application(selected).unwrap().stop_calls, 1);
    assert_eq!(runtime.work(peer), Ok(ApplicationState::Running));
    assert_eq!(runtime.application(peer).unwrap().work_calls, 1);
    assert_eq!(runtime.application(peer).unwrap().stop_calls, 0);
    assert_eq!(events.dequeue(), Some(expected_event(selected)));
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}

#[test]
fn messaging_failure_clears_selected_inbox_and_preserves_peer_fifo() {
    let mut mission = Mission::started(Behavior::StopFails);
    mission.fill_inputs();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    let error = mission.stop_report(&clock, &mut events);
    let nested = Error::source(&error)
        .and_then(|source| source.downcast_ref::<StopEventError>())
        .unwrap();
    assert_eq!(nested, error.operation_error());
    assert_source_chain(nested);
    assert_eq!(nested.stop_error(), &expected_error(mission.selected));
    assert_eq!(
        nested.failure_event_attempt().unwrap().event(),
        &expected_event(mission.selected)
    );
    assert_eq!(error.discarded_deliveries(), 2);
    assert!(error.to_string().contains("discarded 2 queued deliveries"));
    assert_eq!(mission.runtime.pending(mission.selected), Ok(0));
    assert_eq!(mission.runtime.pending(mission.peer), Ok(2));
    assert_eq!(
        mission.runtime.state(mission.selected),
        Ok(ApplicationState::Failed)
    );
    let selected = mission.runtime.application(mission.selected).unwrap();
    assert_eq!(selected.stop_calls, 1);
    assert!(selected.messages.is_empty());
    mission.assert_peer_progress();
    assert_eq!(events.dequeue(), Some(expected_event(mission.selected)));
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}

#[test]
fn saturated_stop_event_retains_original_record_for_explicit_retry() {
    let mut mission = Mission::started(Behavior::StopFails);
    mission.fill_inputs();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    assert_eq!(events.emit(&older_event()), EventEmitOutcome::Recorded);
    let error = mission.stop_report(&clock, &mut events);
    assert_source_chain(error.operation_error());
    assert_eq!(error.discarded_deliveries(), 2);
    assert!(error.to_string().contains("requested failure 29"));
    assert!(
        error
            .to_string()
            .contains("failure event was rejected at queue capacity 1")
    );
    let (stop, attempt) = error.into_operation_error().into_parts();
    assert_eq!(stop, expected_error(mission.selected));
    let (event, outcome) = attempt.unwrap().into_parts();
    assert_eq!(event, expected_event(mission.selected));
    assert_eq!(outcome, EventEmitOutcome::QueueFull { capacity: 1 });
    assert_eq!(events.emit(&event), outcome);
    assert_eq!(events.pending(), 1);
    clock.elapsed.set(Duration::from_secs(99));
    assert_eq!(events.dequeue(), Some(older_event()));
    assert_eq!(events.emit(&event), EventEmitOutcome::Recorded);
    assert_eq!(events.dequeue(), Some(expected_event(mission.selected)));
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
    mission.assert_peer_progress();
}

#[test]
fn later_failed_stop_cannot_duplicate_reporting() {
    let mut mission = Mission::started(Behavior::StopFails);
    mission.fill_inputs();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    mission.stop_report(&clock, &mut events);
    let error = mission.stop_report(&clock, &mut events);
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(error.operation_error().failure_event_attempt(), None);
    assert_eq!(
        error.into_operation_error().into_stop_error(),
        RuntimeStopError::Lifecycle(rejected_error(mission.selected, ApplicationState::Failed))
    );
    assert_eq!(
        mission
            .runtime
            .application(mission.selected)
            .unwrap()
            .stop_calls,
        1
    );
    assert_eq!(mission.runtime.pending(mission.selected), Ok(0));
    assert_eq!(mission.runtime.pending(mission.peer), Ok(2));
    assert_eq!(events.dequeue(), Some(expected_event(mission.selected)));
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}

#[test]
fn equal_slot_foreign_identity_affects_only_receiving_owner() {
    let mut foreign = Mission::started(Behavior::StopFails);
    foreign.fill_inputs();
    let mut receiving = Mission::started(Behavior::StopFails);
    receiving.fill_inputs();
    assert_eq!(foreign.selected, receiving.selected);
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    let error = receiving
        .runtime
        .stop_with_failure_event(
            foreign.selected,
            Diagnostic::StopFailed,
            &clock,
            &mut events,
        )
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(
        error.into_operation_error().into_stop_error(),
        expected_error(receiving.selected)
    );
    assert_eq!(
        receiving.runtime.state(receiving.selected),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(receiving.runtime.pending(receiving.selected), Ok(0));
    assert_eq!(receiving.runtime.pending(receiving.peer), Ok(2));
    assert_eq!(
        foreign.runtime.state(foreign.selected),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        foreign
            .runtime
            .application(foreign.selected)
            .unwrap()
            .stop_calls,
        0
    );
    assert_eq!(foreign.runtime.pending(foreign.selected), Ok(2));
    assert_eq!(foreign.runtime.pending(foreign.peer), Ok(2));
    assert_eq!(events.dequeue(), Some(expected_event(receiving.selected)));
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}
