//! Configuration-driven ordinary work with an explicit bounded host bridge.
//!
//! Work stages one value; the host publishes it to one frozen subscriber.
//! No work callback publishes, and rejected transfers retain the pending value.

#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationWorkContext,
    ConfigurationTable, DeliveryStatus, FrameworkInstant, ManualClock, Message, MessageCreateError,
    MessagingApplication, MessagingRuntime, PublishClassification, PublishError, PublishReport,
    Runtime, RuntimeInboxConfig, ScheduledWork, ScheduledWorkOutcome, WorkSchedule,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Output,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Produced {
    revision: u64,
    value: u8,
}

// Both slots belong to the host. Lifecycle changes do not clear them.
// Observations record the configuration seen even when staging returns full.
enum MissionApplication<'a> {
    Producer {
        pending: &'a Cell<Option<Produced>>,
        observed: &'a Cell<Option<Produced>>,
    },
    Sink {
        output: &'a Cell<Option<u8>>,
        observed: &'a Cell<Option<Produced>>,
    },
}

type MissionRuntime<'a> = MessagingRuntime<MissionApplication<'a>, Topic, 1, ValueError, 1>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValueError {
    Length { actual: usize },
    Percent { actual: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkError {
    MissingConfiguration,
    Configuration(ValueError),
    PendingFull {
        retained: Produced,
        rejected: Produced,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeliveryError {
    UnexpectedProducerDelivery,
    Value(ValueError),
    OutputFull { retained: u8, rejected: u8 },
}

#[derive(Debug, Eq, PartialEq)]
enum TransferError {
    Message(MessageCreateError),
    Publish(PublishError),
    // Publication may already have delivered. End host transfer on this error.
    UnexpectedDestination(PublishReport),
}

#[derive(Debug, Eq, PartialEq)]
enum TransferOutcome {
    Empty,
    Delivered(Produced),
    Retained {
        observation: Produced,
        report: PublishReport,
    },
}

impl Application for MissionApplication<'_> {
    type StartError = Infallible;
    type WorkError = WorkError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        let configuration = context
            .configuration()
            .ok_or(WorkError::MissingConfiguration)?;
        let observation = Produced {
            revision: configuration.revision(),
            value: percent(configuration.bytes()).map_err(WorkError::Configuration)?,
        };
        match self {
            Self::Producer { pending, observed } => {
                observed.set(Some(observation));
                if let Some(retained) = pending.get() {
                    return Err(WorkError::PendingFull {
                        retained,
                        rejected: observation,
                    });
                }
                pending.set(Some(observation));
            }
            Self::Sink { observed, .. } => observed.set(Some(observation)),
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for MissionApplication<'_> {
    type MessageError = DeliveryError;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        _context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        let Self::Sink { output, .. } = self else {
            return Err(DeliveryError::UnexpectedProducerDelivery);
        };
        let value = percent(message.payload()).map_err(DeliveryError::Value)?;
        if let Some(retained) = output.get() {
            return Err(DeliveryError::OutputFull {
                retained,
                rejected: value,
            });
        }
        output.set(Some(value));
        Ok(())
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length { actual } => write!(formatter, "expected one byte, got {actual}"),
            Self::Percent { actual } => write!(formatter, "percentage {actual} exceeds 100"),
        }
    }
}

impl Error for ValueError {}

impl fmt::Display for WorkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingConfiguration => formatter.write_str("work configuration is absent"),
            Self::Configuration(error) => error.fmt(formatter),
            Self::PendingFull { retained, rejected } => {
                write!(formatter, "pending {retained:?} rejected {rejected:?}")
            }
        }
    }
}

impl Error for WorkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Configuration(error) => Some(error),
            _ => None,
        }
    }
}

impl fmt::Display for DeliveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedProducerDelivery => formatter.write_str("producer has no input route"),
            Self::Value(error) => error.fmt(formatter),
            Self::OutputFull { retained, rejected } => {
                write!(formatter, "host output {retained} rejected {rejected}")
            }
        }
    }
}

impl Error for DeliveryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Value(error) => Some(error),
            _ => None,
        }
    }
}

impl fmt::Display for TransferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(error) => error.fmt(formatter),
            Self::Publish(error) => error.fmt(formatter),
            Self::UnexpectedDestination(report) => {
                write!(formatter, "expected the sole configured sink: {report:?}")
            }
        }
    }
}

impl Error for TransferError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Message(error) => Some(error),
            Self::Publish(error) => Some(error),
            Self::UnexpectedDestination(_) => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = run()?;
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "Experimental host work output; not flight-qualified or safety-certified."
    )?;
    writeln!(
        output,
        "No NASA affiliation, demonstrated TRL, operational suitability, or compatibility."
    )?;
    writeln!(
        output,
        "ordinary/scheduled work -> host -> sink: {values:?}"
    )?;
    Ok(())
}

fn run() -> Result<[u8; 2], Box<dyn Error>> {
    let pending = Cell::new(None);
    let output = Cell::new(None);
    let observed = [Cell::new(None), Cell::new(None)];
    let (mut runtime, ids) = compose(&pending, &output, &observed)?;
    for id in ids {
        runtime.start(id)?;
    }
    runtime.work(ids[0])?;
    let first = transfer_and_observe(&mut runtime, ids[1], &pending, &output)?;
    runtime.replace_configuration(&[7])?;
    let mut clock = ManualClock::new();
    let mut schedule = WorkSchedule::new(&[ScheduledWork::new(
        ids[0],
        FrameworkInstant::from_elapsed(Duration::from_secs(1)),
    )])?;
    clock.advance(Duration::from_secs(1))?;
    match runtime.run_next_scheduled_work(&mut schedule, &clock)? {
        ScheduledWorkOutcome::Completed { .. } => {}
        outcome => return Err(io::Error::other(format!("unexpected work: {outcome:?}")).into()),
    }
    let second = transfer_and_observe(&mut runtime, ids[1], &pending, &output)?;
    for id in ids {
        runtime.stop(id)?;
    }
    Ok([first, second])
}

fn compose<'a>(
    pending: &'a Cell<Option<Produced>>,
    output: &'a Cell<Option<u8>>,
    observed: &'a [Cell<Option<Produced>>; 2],
) -> Result<(MissionRuntime<'a>, [ApplicationId; 2]), Box<dyn Error>> {
    let configuration = ConfigurationTable::new(&[42], |bytes| percent(bytes).map(|_| ()))?;
    let mut runtime = Runtime::with_configuration(2, configuration)?;
    let mut register = |application| {
        runtime
            .register(application)
            .map_err(|error| io::Error::other(format!("capacity {} full", error.capacity())))
    };
    let ids = [
        register(MissionApplication::Producer {
            pending,
            observed: &observed[0],
        })?,
        register(MissionApplication::Sink {
            output,
            observed: &observed[1],
        })?,
    ];
    // Exactly this sole subscriber makes wholly rejected retry safe.
    let inboxes = [
        RuntimeInboxConfig::new(1, &[]),
        RuntimeInboxConfig::new(1, &[Topic::Output]),
    ];
    let runtime = MessagingRuntime::new(runtime, &inboxes).map_err(|error| error.kind())?;
    Ok((runtime, ids))
}

fn transfer(
    runtime: &mut MissionRuntime<'_>,
    destination: ApplicationId,
    pending: &Cell<Option<Produced>>,
) -> Result<TransferOutcome, TransferError> {
    let Some(observation) = pending.get() else {
        return Ok(TransferOutcome::Empty);
    };
    let message =
        Message::try_new(Topic::Output, &[observation.value]).map_err(TransferError::Message)?;
    let report = runtime.publish(&message).map_err(TransferError::Publish)?;
    let [outcome] = report.outcomes() else {
        return Err(TransferError::UnexpectedDestination(report));
    };
    if outcome.application_id() != destination {
        return Err(TransferError::UnexpectedDestination(report));
    }
    if report.classification() == PublishClassification::Complete
        && outcome.status() == DeliveryStatus::Delivered
    {
        pending.set(None);
        return Ok(TransferOutcome::Delivered(observation));
    }
    Ok(TransferOutcome::Retained {
        observation,
        report,
    })
}

fn transfer_and_observe(
    runtime: &mut MissionRuntime<'_>,
    destination: ApplicationId,
    pending: &Cell<Option<Produced>>,
    output: &Cell<Option<u8>>,
) -> Result<u8, Box<dyn Error>> {
    let outcome = transfer(runtime, destination, pending)?;
    if !matches!(outcome, TransferOutcome::Delivered(_)) {
        return Err(io::Error::other(format!("unexpected transfer: {outcome:?}")).into());
    }
    runtime.dispatch_one(destination)?;
    output
        .take()
        .ok_or_else(|| io::Error::other("missing host output").into())
}

fn percent(bytes: &[u8]) -> Result<u8, ValueError> {
    match bytes {
        [value] if *value <= 100 => Ok(*value),
        [value] => Err(ValueError::Percent { actual: *value }),
        _ => Err(ValueError::Length {
            actual: bytes.len(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use rust_flight_framework::{
        ApplicationState, ConfigurationError, MessageDispatchError, RuntimeConfigurationError,
        RuntimeWorkError,
    };

    use super::*;

    #[test]
    fn ordinary_work_stages_before_host_publication() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        assert!(matches!(
            runtime.work(ids[0]).unwrap_err().operation_error(),
            RuntimeWorkError::Lifecycle(_)
        ));
        assert_eq!(observed[0].get(), None);
        assert_eq!(pending.get(), None);
        for id in ids {
            runtime.start(id).unwrap();
        }
        runtime.work(ids[0]).unwrap();
        let expected = Produced {
            revision: 1,
            value: 42,
        };
        assert_eq!(pending.get(), Some(expected));
        assert_eq!(observed[0].get(), Some(expected));
        assert_eq!(runtime.pending(ids[1]), Ok(0));
        assert_eq!(output.get(), None);
        assert_eq!(
            transfer(&mut runtime, ids[1], &pending),
            Ok(TransferOutcome::Delivered(expected))
        );
        assert_eq!(pending.get(), None);
        assert_eq!(runtime.pending(ids[1]), Ok(1));
        assert_eq!(output.get(), None);
        runtime.dispatch_one(ids[1]).unwrap();
        assert_eq!(output.take(), Some(42));
        assert_eq!(
            transfer(&mut runtime, ids[1], &pending),
            Ok(TransferOutcome::Empty)
        );
    }

    #[test]
    fn configuration_changes_do_not_rewrite_pending_output() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        runtime.work(ids[0]).unwrap();
        let original = pending.get();
        assert_eq!(
            runtime.replace_configuration(&[101]),
            Err(RuntimeConfigurationError::Table(
                ConfigurationError::Rejected(ValueError::Percent { actual: 101 })
            ))
        );
        assert_eq!(runtime.replace_configuration(&[7]), Ok(2));
        assert_eq!(pending.get(), original);
        assert_eq!(
            transfer_and_observe(&mut runtime, ids[1], &pending, &output).unwrap(),
            42
        );
        runtime.work(ids[0]).unwrap();
        assert_eq!(
            pending.get(),
            Some(Produced {
                revision: 2,
                value: 7
            })
        );
        assert_eq!(run().unwrap(), [42, 7]);
    }

    #[test]
    fn full_destination_retains_pending_for_explicit_retry() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        runtime.work(ids[0]).unwrap();
        transfer(&mut runtime, ids[1], &pending).unwrap();
        runtime.replace_configuration(&[7]).unwrap();
        runtime.work(ids[0]).unwrap();
        let retained = pending.get().unwrap();
        let TransferOutcome::Retained {
            observation,
            report,
        } = transfer(&mut runtime, ids[1], &pending).unwrap()
        else {
            panic!("expected full destination");
        };
        assert_eq!(observation, retained);
        assert_eq!(
            report.classification(),
            PublishClassification::WhollyUndelivered
        );
        assert_eq!(report.outcomes()[0].application_id(), ids[1]);
        assert_eq!(report.outcomes()[0].status(), DeliveryStatus::InboxFull);
        assert_eq!(pending.get(), Some(retained));
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Running));
        assert_eq!(runtime.pending(ids[1]), Ok(1));
        runtime.dispatch_one(ids[1]).unwrap();
        assert_eq!(output.take(), Some(42));
        assert_eq!(
            transfer_and_observe(&mut runtime, ids[1], &pending, &output).unwrap(),
            7
        );
        assert_eq!(runtime.pending(ids[1]), Ok(0));
        assert_eq!(pending.get(), None);
    }

    #[test]
    fn unavailable_destination_and_producer_restart_retain_output() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        runtime.start(ids[0]).unwrap();
        runtime.work(ids[0]).unwrap();
        let original = pending.get();
        let TransferOutcome::Retained { report, .. } =
            transfer(&mut runtime, ids[1], &pending).unwrap()
        else {
            panic!("expected unavailable destination");
        };
        assert_eq!(report.outcomes()[0].status(), DeliveryStatus::Unavailable);
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Running));
        assert_eq!(runtime.pending(ids[1]), Ok(0));
        runtime.stop(ids[0]).unwrap();
        runtime.replace_configuration(&[7]).unwrap();
        let failure = runtime.work(ids[0]).unwrap_err();
        assert!(matches!(
            failure.operation_error(),
            RuntimeWorkError::Lifecycle(_)
        ));
        assert_eq!(failure.discarded_deliveries(), 0);
        assert_eq!(observed[0].get(), original);
        assert_eq!(pending.get(), original);
        runtime.restart(ids[0]).unwrap();
        assert_eq!(pending.get(), original);
        runtime.start(ids[1]).unwrap();
        runtime.stop(ids[0]).unwrap();
        assert_eq!(
            transfer_and_observe(&mut runtime, ids[1], &pending, &output).unwrap(),
            42
        );
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Stopped));
    }

    #[test]
    fn pending_full_work_failure_retains_typed_source_and_peer_progress() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        runtime.work(ids[0]).unwrap();
        let retained = pending.get().unwrap();
        runtime.replace_configuration(&[7]).unwrap();
        let failure = runtime.work(ids[0]).unwrap_err();
        let operation = failure
            .source()
            .unwrap()
            .downcast_ref::<RuntimeWorkError<WorkError>>();
        assert_eq!(operation, Some(failure.operation_error()));
        let operation = operation.unwrap();
        let expected = WorkError::PendingFull {
            retained,
            rejected: Produced {
                revision: 2,
                value: 7,
            },
        };
        assert_eq!(
            operation,
            &RuntimeWorkError::Application {
                application_id: ids[0],
                source: expected
            }
        );
        assert_eq!(
            operation.source().unwrap().downcast_ref::<WorkError>(),
            Some(&expected)
        );
        assert_eq!(failure.discarded_deliveries(), 0);
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Failed));
        assert_eq!(pending.get(), Some(retained));
        assert_eq!(
            observed[0].get(),
            Some(Produced {
                revision: 2,
                value: 7
            })
        );
        assert!(runtime.work(ids[0]).is_err());
        assert!(runtime.restart(ids[0]).is_err());
        runtime.work(ids[1]).unwrap();
        assert_eq!(
            observed[1].get(),
            Some(Produced {
                revision: 2,
                value: 7
            })
        );
        assert_eq!(
            transfer_and_observe(&mut runtime, ids[1], &pending, &output).unwrap(),
            42
        );
        assert_eq!(runtime.state(ids[1]), Ok(ApplicationState::Running));
    }

    #[test]
    fn schedule_waits_then_produces_once() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        let instant = FrameworkInstant::from_elapsed(Duration::from_secs(1));
        let mut schedule = WorkSchedule::new(&[ScheduledWork::new(ids[0], instant)]).unwrap();
        let mut clock = ManualClock::new();
        assert!(matches!(
            runtime
                .run_next_scheduled_work(&mut schedule, &clock)
                .unwrap(),
            ScheduledWorkOutcome::Waiting { .. }
        ));
        assert_eq!(schedule.remaining(), 1);
        assert_eq!(pending.get(), None);
        clock.advance(Duration::from_secs(1)).unwrap();
        assert!(matches!(
            runtime
                .run_next_scheduled_work(&mut schedule, &clock)
                .unwrap(),
            ScheduledWorkOutcome::Completed { .. }
        ));
        assert_eq!(runtime.pending(ids[1]), Ok(0));
        assert_eq!(
            pending.get(),
            Some(Produced {
                revision: 1,
                value: 42
            })
        );
        let original = pending.get();
        assert!(schedule.is_complete());
        assert_eq!(
            runtime
                .run_next_scheduled_work(&mut schedule, &clock)
                .unwrap(),
            ScheduledWorkOutcome::Complete
        );
        assert_eq!(pending.get(), original);
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Running));
    }

    #[test]
    fn scheduled_full_slot_failure_consumes_and_allows_peer_progress() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        runtime.work(ids[0]).unwrap();
        let original = pending.get();
        let instant = FrameworkInstant::from_elapsed(Duration::ZERO);
        let mut schedule = WorkSchedule::new(&[
            ScheduledWork::new(ids[0], instant),
            ScheduledWork::new(ids[1], instant),
        ])
        .unwrap();
        let clock = ManualClock::new();
        let failure = runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .unwrap_err();
        assert_eq!(
            failure.operation_error().scheduled_work().application_id(),
            ids[0]
        );
        assert_eq!(failure.operation_error().observed_at(), instant);
        assert!(matches!(
            failure.operation_error().work_error(),
            RuntimeWorkError::Application { .. }
        ));
        assert_eq!(schedule.remaining(), 1);
        assert_eq!(pending.get(), original);
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Failed));
        runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .unwrap();
        assert_eq!(observed[1].get(), original);
        assert!(schedule.is_complete());
    }

    #[test]
    fn scheduled_lifecycle_rejection_consumes_without_running_callback() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        runtime.work(ids[0]).unwrap();
        let original = pending.get();
        runtime.stop(ids[0]).unwrap();
        runtime.replace_configuration(&[7]).unwrap();
        let instant = FrameworkInstant::from_elapsed(Duration::ZERO);
        let mut schedule = WorkSchedule::new(&[
            ScheduledWork::new(ids[0], instant),
            ScheduledWork::new(ids[1], instant),
        ])
        .unwrap();
        let clock = ManualClock::new();
        let failure = runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .unwrap_err();
        assert!(matches!(
            failure.operation_error().work_error(),
            RuntimeWorkError::Lifecycle(_)
        ));
        assert_eq!(
            failure.operation_error().scheduled_work().application_id(),
            ids[0]
        );
        assert_eq!(failure.discarded_deliveries(), 0);
        assert_eq!(schedule.remaining(), 1);
        assert_eq!(observed[0].get(), original);
        assert_eq!(pending.get(), original);
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Stopped));
        runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .unwrap();
        assert_eq!(
            observed[1].get(),
            Some(Produced {
                revision: 2,
                value: 7
            })
        );
        assert!(schedule.is_complete());
    }

    #[test]
    fn invalid_delivery_and_full_host_output_preserve_older_values() {
        for (bytes, initial, expected) in [
            (
                &[][..],
                None,
                DeliveryError::Value(ValueError::Length { actual: 0 }),
            ),
            (
                &[101][..],
                None,
                DeliveryError::Value(ValueError::Percent { actual: 101 }),
            ),
            (
                &[42][..],
                Some(9),
                DeliveryError::OutputFull {
                    retained: 9,
                    rejected: 42,
                },
            ),
        ] {
            let pending = Cell::new(None);
            let output = Cell::new(initial);
            let observed = [Cell::new(None), Cell::new(None)];
            let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
            for id in ids {
                runtime.start(id).unwrap();
            }
            let message = Message::try_new(Topic::Output, bytes).unwrap();
            assert_eq!(
                runtime.publish(&message).unwrap().classification(),
                PublishClassification::Complete
            );
            let failure = runtime.dispatch_one(ids[1]).unwrap_err();
            assert_eq!(
                failure.operation_error(),
                &MessageDispatchError::Application {
                    application_id: ids[1],
                    source: expected,
                }
            );
            assert_eq!(output.get(), initial);
            assert_eq!(runtime.state(ids[1]), Ok(ApplicationState::Failed));
            runtime.work(ids[0]).unwrap();
            assert_eq!(
                pending.get(),
                Some(Produced {
                    revision: 1,
                    value: 42
                })
            );
        }
    }

    #[test]
    fn unexpected_destination_never_clears_pending() {
        let pending = Cell::new(None);
        let output = Cell::new(None);
        let observed = [Cell::new(None), Cell::new(None)];
        let (mut runtime, ids) = compose(&pending, &output, &observed).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        runtime.work(ids[0]).unwrap();
        let retained = pending.get();
        let error = transfer(&mut runtime, ids[0], &pending).unwrap_err();
        let TransferError::UnexpectedDestination(report) = error else {
            panic!("expected wrong destination");
        };
        assert_eq!(report.outcomes()[0].application_id(), ids[1]);
        assert_eq!(report.outcomes()[0].status(), DeliveryStatus::Delivered);
        assert_eq!(pending.get(), retained);
        // A topology error may follow delivery: do not retry this error.
        assert_eq!(runtime.pending(ids[1]), Ok(1));
    }
}
