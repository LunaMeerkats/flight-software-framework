//! Public-only host inspection of ordinary runtime-owned observations.

use std::convert::Infallible;
use std::error::Error;
use std::fmt;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, ConfigurationTable, DeliveryStatus, LifecycleError, Message,
    MessageDispatchError, MessageDispatchOutcome, MessagingApplication, MessagingRuntime, Runtime,
    RuntimeInboxConfig, RuntimeWorkError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CallbackFailure {
    Work,
    Message,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Observation {
    starts: usize,
    works: usize,
    stops: usize,
    restarts: usize,
    messages: usize,
    work_identity: Option<ApplicationId>,
    last_message: Option<u8>,
}

#[derive(Debug)]
struct ObservedApplication {
    observation: Observation,
    failure: Option<CallbackFailure>,
}

type ObservedRuntime = MessagingRuntime<ObservedApplication, (), 1>;

impl fmt::Display for CallbackFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "observed {self:?} failure")
    }
}

impl Error for CallbackFailure {}

impl Application for ObservedApplication {
    type StartError = Infallible;
    type WorkError = CallbackFailure;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.observation.starts += 1;
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.observation.works += 1;
        self.observation.work_identity = Some(context.application_id());
        if self.failure == Some(CallbackFailure::Work) {
            return Err(CallbackFailure::Work);
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.observation.stops += 1;
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.observation.restarts += 1;
        Ok(())
    }
}

impl MessagingApplication<(), 1> for ObservedApplication {
    type MessageError = CallbackFailure;

    fn handle_message(
        &mut self,
        message: &Message<(), 1>,
        _context: &mut ApplicationMessageContext<'_, (), 1>,
    ) -> Result<(), Self::MessageError> {
        self.observation.messages += 1;
        self.observation.last_message = message.payload().first().copied();
        if self.failure == Some(CallbackFailure::Message) {
            return Err(CallbackFailure::Message);
        }
        Ok(())
    }
}

fn registered_runtime(
    failures: [Option<CallbackFailure>; 2],
) -> (Runtime<ObservedApplication>, [ApplicationId; 2]) {
    let mut runtime = Runtime::new(2).unwrap();
    let identities = failures.map(|failure| {
        runtime
            .register(ObservedApplication {
                observation: Observation::default(),
                failure,
            })
            .unwrap()
    });
    (runtime, identities)
}

fn messaging_runtime(
    failures: [Option<CallbackFailure>; 2],
) -> (ObservedRuntime, [ApplicationId; 2]) {
    let (runtime, identities) = registered_runtime(failures);
    let inboxes = [RuntimeInboxConfig::new(2, &[()]); 2];
    let runtime = MessagingRuntime::new(runtime, &inboxes).unwrap();
    (runtime, identities)
}

fn publish_two(runtime: &mut ObservedRuntime) {
    for value in [7, 8] {
        let message = Message::try_new((), &[value]).unwrap();
        let report = runtime.publish(&message).unwrap();
        assert_eq!(report.outcomes().len(), 2);
        assert!(
            report
                .outcomes()
                .iter()
                .all(|outcome| outcome.status() == DeliveryStatus::Delivered)
        );
    }
}

#[test]
fn direct_inspection_retains_owned_observations_across_lifecycle() {
    let (mut runtime, [selected, peer]) = registered_runtime([None; 2]);
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Registered));
    assert_eq!(
        runtime.application(selected).unwrap().observation,
        Observation::default()
    );
    assert!(runtime.work(selected).is_err());
    assert_eq!(runtime.application(selected).unwrap().observation.works, 0);

    let application = runtime.application(selected).unwrap();
    assert_eq!(application.observation.starts, 0);
    runtime.start(selected).unwrap();
    runtime.work(selected).unwrap();
    runtime.stop(selected).unwrap();
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Stopped));
    assert_eq!(runtime.application(selected).unwrap().observation.stops, 1);
    runtime.restart(selected).unwrap();
    runtime.work(selected).unwrap();

    assert_eq!(runtime.state(selected), Ok(ApplicationState::Running));
    assert_eq!(
        runtime.application(selected).unwrap().observation,
        Observation {
            starts: 1,
            works: 2,
            stops: 1,
            restarts: 1,
            work_identity: Some(selected),
            ..Observation::default()
        }
    );
    assert_eq!(
        runtime.application(peer).unwrap().observation,
        Observation::default()
    );
    assert_eq!(runtime.state(peer), Ok(ApplicationState::Registered));
}

#[test]
fn direct_failed_application_remains_inspectable_while_peer_progresses() {
    let (mut runtime, [selected, peer]) = registered_runtime([Some(CallbackFailure::Work), None]);
    runtime.start(selected).unwrap();
    runtime.start(peer).unwrap();
    assert_eq!(
        runtime.work(selected),
        Err(RuntimeWorkError::Application {
            application_id: selected,
            source: CallbackFailure::Work,
        })
    );
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Failed));
    let failed = runtime.application(selected).unwrap().observation;
    assert_eq!(failed.works, 1);
    assert_eq!(failed.work_identity, Some(selected));
    assert!(runtime.work(selected).is_err());
    assert_eq!(runtime.application(selected).unwrap().observation, failed);
    runtime.work(peer).unwrap();
    assert_eq!(runtime.state(peer), Ok(ApplicationState::Running));
    assert_eq!(runtime.application(peer).unwrap().observation.works, 1);
    assert_eq!(runtime.application(selected).unwrap().observation, failed);
}

#[test]
fn messaging_inspection_preserves_queues_and_in_place_lifecycle() {
    let (mut runtime, [selected, peer]) = messaging_runtime([None; 2]);
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Registered));
    assert_eq!(
        runtime.application(selected).unwrap().observation,
        Observation::default()
    );
    runtime.start(selected).unwrap();
    runtime.start(peer).unwrap();
    publish_two(&mut runtime);
    for identity in [selected, peer, selected] {
        assert_eq!(
            runtime.application(identity).unwrap().observation.messages,
            0
        );
        assert_eq!(runtime.pending(identity), Ok(2));
        assert_eq!(runtime.state(identity), Ok(ApplicationState::Running));
    }
    assert_eq!(
        runtime.dispatch_one(selected),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(runtime.pending(selected), Ok(1));
    let dispatched = runtime.application(selected).unwrap().observation;
    assert_eq!(dispatched.messages, 1);
    assert_eq!(dispatched.last_message, Some(7));
    assert_eq!(runtime.stop(selected).unwrap().discarded_deliveries(), 1);
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Stopped));
    assert_eq!(runtime.application(selected).unwrap().observation.stops, 1);
    runtime.restart(selected).unwrap();
    assert_eq!(
        runtime.dispatch_one(selected),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    let restarted = runtime.application(selected).unwrap().observation;
    assert_eq!(restarted.starts, 1);
    assert_eq!(restarted.restarts, 1);
    assert_eq!(restarted.messages, dispatched.messages);
    assert_eq!(restarted.last_message, dispatched.last_message);
    assert_eq!(runtime.pending(peer), Ok(2));
    assert_eq!(runtime.application(peer).unwrap().observation.messages, 0);
}

#[test]
fn messaging_work_failure_inspection_preserves_exact_selected_clearing() {
    let (mut runtime, [selected, peer]) = messaging_runtime([Some(CallbackFailure::Work), None]);
    runtime.start(selected).unwrap();
    runtime.start(peer).unwrap();
    publish_two(&mut runtime);
    let error = runtime.work(selected).unwrap_err();
    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(
        error.operation_error(),
        &RuntimeWorkError::Application {
            application_id: selected,
            source: CallbackFailure::Work,
        }
    );
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Failed));
    let failed = runtime.application(selected).unwrap().observation;
    assert_eq!(failed.works, 1);
    assert_eq!(failed.work_identity, Some(selected));
    assert_eq!(failed.messages, 0);
    assert_eq!(runtime.pending(selected), Ok(0));
    assert_eq!(runtime.pending(peer), Ok(2));
    runtime.work(peer).unwrap();
    runtime.dispatch_one(peer).unwrap();
    let healthy = runtime.application(peer).unwrap().observation;
    assert_eq!(healthy.works, 1);
    assert_eq!(healthy.messages, 1);
    assert_eq!(healthy.last_message, Some(7));
    assert_eq!(runtime.pending(peer), Ok(1));
    assert_eq!(runtime.application(selected).unwrap().observation, failed);
}

#[test]
fn messaging_callback_failure_inspection_excludes_the_in_flight_message() {
    let (mut runtime, [selected, peer]) = messaging_runtime([Some(CallbackFailure::Message), None]);
    runtime.start(selected).unwrap();
    runtime.start(peer).unwrap();
    publish_two(&mut runtime);
    let error = runtime.dispatch_one(selected).unwrap_err();
    assert_eq!(error.discarded_deliveries(), 1);
    assert_eq!(
        error.operation_error(),
        &MessageDispatchError::Application {
            application_id: selected,
            source: CallbackFailure::Message,
        }
    );
    assert_eq!(runtime.state(selected), Ok(ApplicationState::Failed));
    let failed = runtime.application(selected).unwrap().observation;
    assert_eq!(failed.messages, 1);
    assert_eq!(failed.last_message, Some(7));
    assert_eq!(runtime.pending(selected), Ok(0));
    assert_eq!(runtime.pending(peer), Ok(2));
    runtime.dispatch_one(peer).unwrap();
    assert_eq!(
        runtime.application(peer).unwrap().observation.last_message,
        Some(7)
    );
    assert_eq!(runtime.pending(peer), Ok(1));
    assert_eq!(runtime.application(selected).unwrap().observation, failed);
}

#[test]
fn lookup_without_application_trait_retains_runtime_local_key_limits() {
    let mut foreign = Runtime::new(2).unwrap();
    let alias = foreign.register(vec![9_u8]).unwrap();
    let unknown = foreign.register(vec![10_u8]).unwrap();
    let mut runtime = Runtime::new(1).unwrap();
    assert_eq!(
        runtime.application(alias),
        Err(LifecycleError::UnknownApplication {
            application_id: alias
        })
    );
    let identity = runtime.register(vec![3_u8]).unwrap();
    assert_eq!(identity, alias);
    assert_eq!(runtime.application(alias).unwrap(), &[3]);
    assert_eq!(foreign.application(alias).unwrap(), &[9]);
    assert_eq!(
        runtime.application(unknown),
        Err(LifecycleError::UnknownApplication {
            application_id: unknown
        })
    );
    let inboxes = [RuntimeInboxConfig::new(1, &[()])];
    let runtime = MessagingRuntime::<_, (), 0>::new(runtime, &inboxes).unwrap();
    assert_eq!(runtime.application(alias).unwrap(), &[3]);
    assert_eq!(
        runtime.application(unknown),
        Err(LifecycleError::UnknownApplication {
            application_id: unknown
        })
    );
    assert_eq!(runtime.state(identity), Ok(ApplicationState::Registered));
    assert_eq!(runtime.pending(identity), Ok(0));
}

#[test]
fn configured_owner_lookup_requires_no_application_trait_bound() {
    let table = ConfigurationTable::<Infallible, 1>::new(&[1], |_| Ok(())).unwrap();
    let mut runtime = Runtime::with_configuration(1, table).unwrap();
    let identity = runtime.register(vec![4_u8]).unwrap();
    assert_eq!(runtime.application(identity).unwrap(), &[4]);
    assert_eq!(runtime.replace_configuration(&[2]), Ok(2));
    assert_eq!(runtime.application(identity).unwrap(), &[4]);
    let inboxes = [RuntimeInboxConfig::new(1, &[()])];
    let mut runtime = MessagingRuntime::<_, (), 0, _, 1>::new(runtime, &inboxes).unwrap();
    assert_eq!(runtime.application(identity).unwrap(), &[4]);
    assert_eq!(runtime.rollback_configuration(), Ok(1));
    assert_eq!(runtime.application(identity).unwrap(), &[4]);
    assert_eq!(runtime.pending(identity), Ok(0));
    assert_eq!(runtime.state(identity), Ok(ApplicationState::Registered));
}
