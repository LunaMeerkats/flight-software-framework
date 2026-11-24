//! Public-API evidence for events after returned message-callback errors.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, DeliveryStatus, Event, EventEmitOutcome, EventQueue,
    EventSeverity, EventSource, EventTimestamp, FrameworkInstant, LifecycleError,
    LifecycleRegistry, Message, MessageDispatchError, MessageDispatchEventError,
    MessageDispatchOutcome, MessagingApplication, MessagingOperationError, MessagingRuntime,
    PublishError, Runtime, RuntimeInboxConfig,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Input,
    Reply,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Diagnostic {
    Older,
    MessageFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Behavior {
    Passive,
    PublishThenFail,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MessageFailure {
    Requested(u16),
    Publication(PublishError),
}

struct MissionApplication {
    behavior: Behavior,
    messages: Vec<u8>,
    publications: Vec<(ApplicationId, DeliveryStatus)>,
    work_calls: usize,
}

struct CountingClock {
    reads: Cell<usize>,
    elapsed: Cell<Duration>,
}

type MissionRuntime = MessagingRuntime<MissionApplication, Topic, 1>;
type DispatchEventError =
    MessagingOperationError<MessageDispatchEventError<MessageFailure, Diagnostic>>;

struct Mission {
    runtime: MissionRuntime,
    selected: ApplicationId,
    peer: ApplicationId,
}

impl fmt::Display for MessageFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested(code) => write!(formatter, "requested message failure {code}"),
            Self::Publication(error) => error.fmt(formatter),
        }
    }
}

impl Error for MessageFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Requested(_) => None,
            Self::Publication(error) => Some(error),
        }
    }
}

impl Application for MissionApplication {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.work_calls += 1;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for MissionApplication {
    type MessageError = MessageFailure;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        self.messages.extend_from_slice(message.payload());
        if self.behavior == Behavior::Passive {
            return Ok(());
        }
        let report = context
            .publish(&input(Topic::Reply, 77))
            .map_err(MessageFailure::Publication)?;
        self.publications.extend(
            report
                .outcomes()
                .iter()
                .map(|outcome| (outcome.application_id(), outcome.status())),
        );
        Err(MessageFailure::Requested(17))
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
    fn registered() -> Self {
        let mut runtime = Runtime::new(2).unwrap();
        let [selected, peer] = [Behavior::PublishThenFail, Behavior::Passive].map(|behavior| {
            runtime
                .register(MissionApplication {
                    behavior,
                    messages: Vec::new(),
                    publications: Vec::new(),
                    work_calls: 0,
                })
                .unwrap()
        });
        let subscriptions = [Topic::Input, Topic::Reply];
        let inboxes = [
            RuntimeInboxConfig::new(3, &subscriptions),
            RuntimeInboxConfig::new(3, &subscriptions),
        ];
        Self {
            runtime: MessagingRuntime::new(runtime, &inboxes).unwrap(),
            selected,
            peer,
        }
    }

    fn started() -> Self {
        let mut mission = Self::registered();
        mission.runtime.start(mission.selected).unwrap();
        mission.runtime.start(mission.peer).unwrap();
        mission
    }

    fn fill_inputs(&mut self) {
        for value in [11, 22] {
            self.runtime.publish(&input(Topic::Input, value)).unwrap();
        }
        assert_eq!(self.runtime.pending(self.selected), Ok(2));
        assert_eq!(self.runtime.pending(self.peer), Ok(2));
    }

    fn selected_in_state(state: ApplicationState) -> Self {
        let mut mission = Self::registered();
        match state {
            ApplicationState::Registered => {}
            ApplicationState::Stopped => {
                mission.runtime.start(mission.selected).unwrap();
                mission.runtime.stop(mission.selected).unwrap();
            }
            ApplicationState::Failed => {
                mission.runtime.start(mission.selected).unwrap();
                mission.runtime.publish(&input(Topic::Input, 5)).unwrap();
                mission.runtime.dispatch_one(mission.selected).unwrap_err();
            }
            ApplicationState::Running => unreachable!("test requires a rejected state"),
        }
        mission.runtime.start(mission.peer).unwrap();
        mission.runtime.publish(&input(Topic::Input, 11)).unwrap();
        mission
    }
}

fn input(topic: Topic, value: u8) -> Message<Topic, 1> {
    Message::try_new(topic, &[value]).unwrap()
}

fn expected_error(application_id: ApplicationId) -> MessageDispatchError<MessageFailure> {
    MessageDispatchError::Application {
        application_id,
        source: MessageFailure::Requested(17),
    }
}

fn expected_event(application_id: ApplicationId) -> Event<Diagnostic> {
    Event::new(
        EventSource::Application(application_id),
        EventSeverity::Error,
        Diagnostic::MessageFailed,
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

fn assert_source_chain(error: &DispatchEventError) {
    let event_error = Error::source(error)
        .and_then(|source| {
            source.downcast_ref::<MessageDispatchEventError<MessageFailure, Diagnostic>>()
        })
        .unwrap();
    assert_eq!(event_error, error.operation_error());
    let dispatch_error = Error::source(event_error)
        .and_then(|source| source.downcast_ref::<MessageDispatchError<MessageFailure>>())
        .unwrap();
    assert_eq!(dispatch_error, event_error.dispatch_error());
    assert_eq!(
        Error::source(dispatch_error).and_then(|source| source.downcast_ref()),
        Some(&MessageFailure::Requested(17))
    );
}

#[test]
fn empty_and_successful_dispatch_do_not_read_clock_or_emit_events() {
    let mut mission = Mission::started();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    assert_eq!(events.emit(&older_event()), EventEmitOutcome::Recorded);
    assert_eq!(
        mission.runtime.dispatch_one_with_failure_event(
            mission.peer,
            Diagnostic::MessageFailed,
            &clock,
            &mut events,
        ),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    mission.fill_inputs();
    assert_eq!(
        mission.runtime.dispatch_one_with_failure_event(
            mission.peer,
            Diagnostic::MessageFailed,
            &clock,
            &mut events,
        ),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(
        mission.runtime.application(mission.peer).unwrap().messages,
        [11]
    );
    assert_eq!(mission.runtime.pending(mission.peer), Ok(1));
    assert_eq!(mission.runtime.pending(mission.selected), Ok(2));
    assert_eq!(
        mission.runtime.state(mission.peer),
        Ok(ApplicationState::Running)
    );
    assert_eq!(clock.reads.get(), 0);
    assert_eq!(events.dequeue(), Some(older_event()));
    assert!(events.is_empty());
}

#[test]
fn every_non_running_state_rejects_without_callback_clock_or_event() {
    for state in [
        ApplicationState::Registered,
        ApplicationState::Stopped,
        ApplicationState::Failed,
    ] {
        let mut mission = Mission::selected_in_state(state);
        let before = mission
            .runtime
            .application(mission.selected)
            .unwrap()
            .messages
            .clone();
        let clock = CountingClock::default();
        let mut events = EventQueue::new(1).unwrap();
        assert_eq!(events.emit(&older_event()), EventEmitOutcome::Recorded);
        let error = mission
            .runtime
            .dispatch_one_with_failure_event(
                mission.selected,
                Diagnostic::MessageFailed,
                &clock,
                &mut events,
            )
            .unwrap_err();
        assert_eq!(error.discarded_deliveries(), 0);
        let (dispatch_error, attempt) = error.into_operation_error().into_parts();
        assert_eq!(
            dispatch_error,
            MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
                application_id: mission.selected,
                state,
            })
        );
        assert_eq!(attempt, None);
        assert_eq!(clock.reads.get(), 0);
        assert_eq!(
            mission
                .runtime
                .application(mission.selected)
                .unwrap()
                .messages,
            before
        );
        assert_eq!(mission.runtime.pending(mission.selected), Ok(0));
        assert_eq!(mission.runtime.pending(mission.peer), Ok(1));
        assert_eq!(mission.runtime.state(mission.selected), Ok(state));
        assert_eq!(events.dequeue(), Some(older_event()));
        assert!(events.is_empty());
    }
}

#[test]
fn unknown_identity_preserves_both_inboxes_without_reporting() {
    let mut mission = Mission::started();
    mission.fill_inputs();
    let mut registry = LifecycleRegistry::new(3).unwrap();
    registry.register().unwrap();
    registry.register().unwrap();
    let unknown = registry.register().unwrap();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    let error = mission
        .runtime
        .dispatch_one_with_failure_event(unknown, Diagnostic::MessageFailed, &clock, &mut events)
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(error.operation_error().failure_event_attempt(), None);
    assert_eq!(
        error.into_operation_error().into_dispatch_error(),
        MessageDispatchError::Lifecycle(LifecycleError::UnknownApplication {
            application_id: unknown
        })
    );
    assert_eq!(mission.runtime.pending(mission.selected), Ok(2));
    assert_eq!(mission.runtime.pending(mission.peer), Ok(2));
    assert!(
        mission
            .runtime
            .application(mission.selected)
            .unwrap()
            .messages
            .is_empty()
    );
    assert!(
        mission
            .runtime
            .application(mission.peer)
            .unwrap()
            .messages
            .is_empty()
    );
    assert_eq!(clock.reads.get(), 0);
    assert!(events.is_empty());
}

#[test]
fn callback_failure_clears_selected_tail_and_keeps_peer_effects_and_progress() {
    let mut mission = Mission::started();
    mission.fill_inputs();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    let error = mission
        .runtime
        .dispatch_one_with_failure_event(
            mission.selected,
            Diagnostic::MessageFailed,
            &clock,
            &mut events,
        )
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(
        error.operation_error().dispatch_error(),
        &expected_error(mission.selected)
    );
    assert_source_chain(&error);
    let attempt = error.operation_error().failure_event_attempt().unwrap();
    assert_eq!(attempt.event(), &expected_event(mission.selected));
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(events.dequeue(), Some(expected_event(mission.selected)));
    assert_eq!(clock.reads.get(), 1);
    let selected = mission.runtime.application(mission.selected).unwrap();
    assert_eq!(selected.messages, [11]);
    assert_eq!(
        selected.publications,
        [
            (mission.selected, DeliveryStatus::Delivered),
            (mission.peer, DeliveryStatus::Delivered)
        ]
    );
    assert_eq!(mission.runtime.pending(mission.selected), Ok(0));
    assert_eq!(mission.runtime.pending(mission.peer), Ok(3));
    assert_eq!(
        mission.runtime.state(mission.selected),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(
        mission.runtime.work(mission.peer),
        Ok(ApplicationState::Running)
    );
    for _ in 0..3 {
        assert_eq!(
            mission.runtime.dispatch_one(mission.peer),
            Ok(MessageDispatchOutcome::Dispatched)
        );
    }
    let peer = mission.runtime.application(mission.peer).unwrap();
    assert_eq!(peer.work_calls, 1);
    assert_eq!(peer.messages, [11, 22, 77]);
    assert_eq!(mission.runtime.pending(mission.peer), Ok(0));
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}

#[test]
fn saturated_event_retains_error_and_original_timestamp_for_explicit_retry() {
    let mut mission = Mission::started();
    mission.fill_inputs();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    assert_eq!(events.emit(&older_event()), EventEmitOutcome::Recorded);
    let error = mission
        .runtime
        .dispatch_one_with_failure_event(
            mission.selected,
            Diagnostic::MessageFailed,
            &clock,
            &mut events,
        )
        .unwrap_err();
    assert_source_chain(&error);
    assert_eq!(error.discarded_deliveries(), 2);
    assert!(error.to_string().contains("requested message failure 17"));
    assert!(
        error
            .to_string()
            .contains("failure event was rejected at queue capacity 1")
    );
    assert!(error.to_string().contains("discarded 2 queued deliveries"));
    let (dispatch_error, attempt) = error.into_operation_error().into_parts();
    assert_eq!(dispatch_error, expected_error(mission.selected));
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
}

#[test]
fn later_failed_dispatch_never_duplicates_the_recorded_event() {
    let mut mission = Mission::started();
    mission.fill_inputs();
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    mission
        .runtime
        .dispatch_one_with_failure_event(
            mission.selected,
            Diagnostic::MessageFailed,
            &clock,
            &mut events,
        )
        .unwrap_err();
    let error = mission
        .runtime
        .dispatch_one_with_failure_event(
            mission.selected,
            Diagnostic::MessageFailed,
            &clock,
            &mut events,
        )
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(error.operation_error().failure_event_attempt(), None);
    assert_eq!(
        error.operation_error().dispatch_error(),
        &MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
            application_id: mission.selected,
            state: ApplicationState::Failed,
        })
    );
    assert_eq!(
        mission
            .runtime
            .application(mission.selected)
            .unwrap()
            .messages,
        [11]
    );
    assert_eq!(mission.runtime.pending(mission.peer), Ok(3));
    assert_eq!(clock.reads.get(), 1);
    assert_eq!(events.dequeue(), Some(expected_event(mission.selected)));
    assert!(events.is_empty());
}

#[test]
fn equal_slot_foreign_identity_reports_only_the_receiving_application() {
    let mut foreign = Mission::started();
    foreign.fill_inputs();
    let mut receiving = Mission::started();
    receiving.fill_inputs();
    assert_eq!(foreign.selected, receiving.selected);
    let clock = CountingClock::default();
    let mut events = EventQueue::new(1).unwrap();
    let error = receiving
        .runtime
        .dispatch_one_with_failure_event(
            foreign.selected,
            Diagnostic::MessageFailed,
            &clock,
            &mut events,
        )
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(
        error.into_operation_error().into_dispatch_error(),
        expected_error(receiving.selected)
    );
    assert_eq!(
        receiving.runtime.state(receiving.selected),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(receiving.runtime.pending(receiving.selected), Ok(0));
    assert_eq!(receiving.runtime.pending(receiving.peer), Ok(3));
    assert_eq!(
        foreign.runtime.state(foreign.selected),
        Ok(ApplicationState::Running)
    );
    assert_eq!(foreign.runtime.pending(foreign.selected), Ok(2));
    assert_eq!(foreign.runtime.pending(foreign.peer), Ok(2));
    assert!(
        foreign
            .runtime
            .application(foreign.selected)
            .unwrap()
            .messages
            .is_empty()
    );
    assert_eq!(events.dequeue(), Some(expected_event(receiving.selected)));
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}
