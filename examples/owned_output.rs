//! Application-owned output moved to a serial host through shared lookup.
//!
//! One application slot and one host slot retain whole records. A definite
//! sink rejection returns ownership; the host decides when to try again.

#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use rust_flight_framework::{
    Application, ApplicationMessageContext, ApplicationState, ApplicationWorkContext, Message,
    MessagingApplication, Runtime,
};

// Deliberately neither Copy nor Clone. The payload has a fixed inline bound.
#[derive(Debug, Eq, PartialEq)]
struct Output {
    source: u8,
    work_call: usize,
    bytes: [u8; 4],
}

struct Producer {
    source: u8,
    output: Cell<Option<Output>>,
    work_calls: usize,
    message_calls: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OutputOccupied;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TakeTurn {
    Empty,
    Taken,
    HostOccupied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProductionTurn {
    Worked,
    HostOccupied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SinkError {
    Backpressured,
}

struct SinkRejection {
    reason: SinkError,
    output: Output,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SinkTurn {
    Empty,
    Accepted,
    Rejected(SinkError),
}

#[derive(Default)]
struct HostOutput {
    pending: Option<Output>,
}

impl Producer {
    fn new(source: u8) -> Self {
        Self {
            source,
            output: Cell::new(None),
            work_calls: 0,
            message_calls: 0,
        }
    }

    // Option supplies Default, so taking needs no Copy or Clone output.
    fn take_output(&self) -> Option<Output> {
        self.output.take()
    }
}

impl Application for Producer {
    type StartError = Infallible;
    type WorkError = OutputOccupied;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.work_calls += 1;
        let output = self.output.get_mut();
        if output.is_some() {
            return Err(OutputOccupied);
        }
        *output = Some(Output {
            source: self.source,
            work_call: self.work_calls,
            bytes: [2, 3, 5, 8],
        });
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<(), 1> for Producer {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        _message: &Message<(), 1>,
        _context: &mut ApplicationMessageContext<'_, (), 1>,
    ) -> Result<(), Self::MessageError> {
        self.message_calls += 1;
        Ok(())
    }
}

impl fmt::Display for OutputOccupied {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("application output slot is occupied")
    }
}

impl Error for OutputOccupied {}

impl HostOutput {
    fn take_turn(&mut self, application: &Producer) -> TakeTurn {
        if self.pending.is_some() {
            return TakeTurn::HostOccupied;
        }
        self.pending = application.take_output();
        if self.pending.is_some() {
            TakeTurn::Taken
        } else {
            TakeTurn::Empty
        }
    }

    fn produce_turn<E>(
        &self,
        work: impl FnOnce() -> Result<ApplicationState, E>,
    ) -> Result<ProductionTurn, E> {
        if self.pending.is_some() {
            return Ok(ProductionTurn::HostOccupied);
        }
        work()?;
        Ok(ProductionTurn::Worked)
    }

    // This contract requires rejection before acceptance or external effects.
    // An ambiguous transport outcome cannot safely use this whole-record retry.
    fn sink_turn(&mut self, sink: impl FnOnce(Output) -> Result<(), SinkRejection>) -> SinkTurn {
        let Some(output) = self.pending.take() else {
            return SinkTurn::Empty;
        };
        match sink(output) {
            Ok(()) => SinkTurn::Accepted,
            Err(rejection) => {
                self.pending = Some(rejection.output);
                SinkTurn::Rejected(rejection.reason)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    run(&mut io::stdout().lock())
}

fn run(output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new(1)?;
    let producer = runtime.register(Producer::new(7))?;
    runtime.start(producer)?;
    let mut host = HostOutput::default();
    host.produce_turn(|| runtime.work(producer))?;
    writeln!(
        output,
        "Experimental host output; local ownership transfer only."
    )?;
    writeln!(
        output,
        "take: {:?}",
        host.take_turn(runtime.application(producer)?)
    )?;
    writeln!(
        output,
        "work while held: {:?}",
        host.produce_turn(|| runtime.work(producer))?
    )?;
    let rejected = host.sink_turn(|output| {
        Err(SinkRejection {
            reason: SinkError::Backpressured,
            output,
        })
    });
    writeln!(output, "sink: {rejected:?}")?;
    let mut accepted = None;
    let delivered = host.sink_turn(|output| {
        accepted = Some(output);
        Ok(())
    });
    writeln!(output, "sink: {delivered:?}")?;
    writeln!(output, "accepted: {accepted:?}")?;
    writeln!(output, "empty sink: {:?}", host.sink_turn(|_| Ok(())))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_flight_framework::{
        ApplicationId, DeliveryStatus, MessageDispatchOutcome, MessagingRuntime,
        RuntimeInboxConfig, RuntimeWorkError,
    };

    fn registered_runtime() -> (Runtime<Producer>, [ApplicationId; 2]) {
        let mut runtime = Runtime::new(2).unwrap();
        let identities = [1, 2].map(|source| runtime.register(Producer::new(source)).unwrap());
        (runtime, identities)
    }

    fn messaging_runtime() -> (MessagingRuntime<Producer, (), 1>, [ApplicationId; 2]) {
        let (runtime, identities) = registered_runtime();
        let inboxes = [RuntimeInboxConfig::new(2, &[()]); 2];
        (
            MessagingRuntime::new(runtime, &inboxes).unwrap(),
            identities,
        )
    }

    fn expected(source: u8, work_call: usize) -> Output {
        Output {
            source,
            work_call,
            bytes: [2, 3, 5, 8],
        }
    }

    fn publish_two(runtime: &mut MessagingRuntime<Producer, (), 1>) {
        for value in [4, 9] {
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
    fn direct_take_moves_one_record_and_repeated_take_is_empty() {
        let (mut runtime, [selected, peer]) = registered_runtime();
        assert_eq!(runtime.application(selected).unwrap().take_output(), None);
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Registered));
        runtime.start(selected).unwrap();
        runtime.work(selected).unwrap();
        let output = runtime.application(selected).unwrap().take_output();
        assert_eq!(output, Some(expected(1, 1)));
        assert_eq!(runtime.application(selected).unwrap().take_output(), None);
        assert_eq!(runtime.application(selected).unwrap().work_calls, 1);
        assert_eq!(runtime.application(peer).unwrap().work_calls, 0);
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Running));
    }

    #[test]
    fn occupied_host_defers_work_and_take_and_retains_rejected_ownership() {
        let (mut runtime, [selected, peer]) = registered_runtime();
        for identity in [selected, peer] {
            runtime.start(identity).unwrap();
            runtime.work(identity).unwrap();
        }
        let mut host = HostOutput::default();
        assert_eq!(
            host.take_turn(runtime.application(selected).unwrap()),
            TakeTurn::Taken
        );
        assert_eq!(
            host.produce_turn(|| runtime.work(selected)),
            Ok(ProductionTurn::HostOccupied)
        );
        assert_eq!(runtime.application(selected).unwrap().work_calls, 1);
        assert_eq!(
            host.take_turn(runtime.application(peer).unwrap()),
            TakeTurn::HostOccupied
        );
        for _ in 0..2 {
            assert_eq!(
                host.sink_turn(|output| Err(SinkRejection {
                    reason: SinkError::Backpressured,
                    output,
                })),
                SinkTurn::Rejected(SinkError::Backpressured)
            );
            assert_eq!(host.pending.as_ref(), Some(&expected(1, 1)));
        }
        assert_eq!(
            host.sink_turn(|output| {
                assert_eq!(output, expected(1, 1));
                Ok(())
            }),
            SinkTurn::Accepted
        );
        assert_eq!(host.pending, None);
        assert_eq!(
            host.produce_turn(|| runtime.work(selected)),
            Ok(ProductionTurn::Worked)
        );
        assert_eq!(runtime.application(selected).unwrap().work_calls, 2);
        assert_eq!(
            host.take_turn(runtime.application(peer).unwrap()),
            TakeTurn::Taken
        );
        assert_eq!(host.pending.as_ref(), Some(&expected(2, 1)));
        assert_eq!(runtime.application(peer).unwrap().take_output(), None);
    }

    #[test]
    fn empty_host_turns_do_not_call_the_sink_or_produce_output() {
        let (runtime, [selected, _]) = registered_runtime();
        let mut host = HostOutput::default();
        for _ in 0..2 {
            assert_eq!(
                host.take_turn(runtime.application(selected).unwrap()),
                TakeTurn::Empty
            );
            assert_eq!(
                host.sink_turn(|_| panic!("an empty turn must not invoke the sink")),
                SinkTurn::Empty
            );
        }
        assert_eq!(runtime.application(selected).unwrap().work_calls, 0);
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Registered));
    }

    #[test]
    fn direct_stop_restart_preserves_application_and_host_slots() {
        let (mut runtime, [selected, _]) = registered_runtime();
        runtime.start(selected).unwrap();
        runtime.work(selected).unwrap();
        runtime.stop(selected).unwrap();
        runtime.restart(selected).unwrap();
        runtime.stop(selected).unwrap();
        let mut host = HostOutput::default();
        assert_eq!(
            host.take_turn(runtime.application(selected).unwrap()),
            TakeTurn::Taken
        );
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Stopped));
        runtime.restart(selected).unwrap();
        assert_eq!(host.pending.as_ref(), Some(&expected(1, 1)));
        assert_eq!(runtime.application(selected).unwrap().take_output(), None);
        assert_eq!(runtime.application(selected).unwrap().work_calls, 1);
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Running));
        assert_eq!(
            host.sink_turn(|output| {
                assert_eq!(output, expected(1, 1));
                Ok(())
            }),
            SinkTurn::Accepted
        );
    }

    #[test]
    fn direct_full_failure_keeps_the_old_output_for_drain_and_peer_work() {
        let (mut runtime, [selected, peer]) = registered_runtime();
        for identity in [selected, peer] {
            runtime.start(identity).unwrap();
        }
        runtime.work(selected).unwrap();
        assert_eq!(
            runtime.work(selected),
            Err(RuntimeWorkError::Application {
                application_id: selected,
                source: OutputOccupied,
            })
        );
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Failed));
        assert_eq!(runtime.application(selected).unwrap().work_calls, 2);
        assert_eq!(
            runtime.application(selected).unwrap().take_output(),
            Some(expected(1, 1))
        );
        assert_eq!(runtime.application(selected).unwrap().take_output(), None);
        assert!(runtime.work(selected).is_err());
        assert_eq!(runtime.application(selected).unwrap().work_calls, 2);
        runtime.work(peer).unwrap();
        assert_eq!(
            runtime.application(peer).unwrap().take_output(),
            Some(expected(2, 1))
        );
        assert_eq!(runtime.state(peer), Ok(ApplicationState::Running));
    }

    #[test]
    fn messaging_take_and_host_gate_preserve_inboxes_and_lifecycle_output() {
        let (mut runtime, [selected, peer]) = messaging_runtime();
        for identity in [selected, peer] {
            runtime.start(identity).unwrap();
        }
        runtime.work(selected).unwrap();
        publish_two(&mut runtime);
        let mut host = HostOutput::default();
        assert_eq!(
            host.take_turn(runtime.application(selected).unwrap()),
            TakeTurn::Taken
        );
        assert_eq!(
            host.produce_turn(|| runtime.work(selected)),
            Ok(ProductionTurn::HostOccupied)
        );
        for identity in [selected, peer] {
            assert_eq!(runtime.pending(identity), Ok(2));
            assert_eq!(runtime.application(identity).unwrap().message_calls, 0);
            assert_eq!(runtime.state(identity), Ok(ApplicationState::Running));
        }
        assert_eq!(runtime.application(selected).unwrap().work_calls, 1);
        host.sink_turn(|_| Ok(()));
        runtime.work(selected).unwrap();
        assert_eq!(runtime.stop(selected).unwrap().discarded_deliveries(), 2);
        assert_eq!(runtime.pending(peer), Ok(2));
        runtime.restart(selected).unwrap();
        assert_eq!(
            runtime.application(selected).unwrap().take_output(),
            Some(expected(1, 2))
        );
        assert_eq!(runtime.application(selected).unwrap().take_output(), None);
        assert_eq!(runtime.pending(selected), Ok(0));
        assert_eq!(runtime.pending(peer), Ok(2));
        assert_eq!(runtime.application(selected).unwrap().message_calls, 0);
    }

    #[test]
    fn messaging_full_failure_clears_only_selected_inbox_and_allows_failed_drain() {
        let (mut runtime, [selected, peer]) = messaging_runtime();
        for identity in [selected, peer] {
            runtime.start(identity).unwrap();
        }
        publish_two(&mut runtime);
        runtime.work(selected).unwrap();
        let error = runtime.work(selected).unwrap_err();
        assert_eq!(error.discarded_deliveries(), 2);
        assert_eq!(
            error.operation_error(),
            &RuntimeWorkError::Application {
                application_id: selected,
                source: OutputOccupied,
            }
        );
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Failed));
        let mut host = HostOutput::default();
        assert_eq!(
            host.take_turn(runtime.application(selected).unwrap()),
            TakeTurn::Taken
        );
        assert_eq!(host.pending.as_ref(), Some(&expected(1, 1)));
        assert_eq!(runtime.pending(selected), Ok(0));
        assert_eq!(runtime.pending(peer), Ok(2));
        runtime.work(peer).unwrap();
        assert_eq!(
            runtime.dispatch_one(peer),
            Ok(MessageDispatchOutcome::Dispatched)
        );
        assert_eq!(runtime.application(peer).unwrap().message_calls, 1);
        assert_eq!(
            runtime.application(peer).unwrap().take_output(),
            Some(expected(2, 1))
        );
        assert_eq!(runtime.pending(peer), Ok(1));
        assert_eq!(runtime.state(peer), Ok(ApplicationState::Running));
        assert_eq!(runtime.application(selected).unwrap().take_output(), None);
        assert_eq!(runtime.state(selected), Ok(ApplicationState::Failed));
    }

    #[test]
    fn runnable_trace_separates_take_rejection_and_acceptance() {
        let mut output = Vec::new();
        run(&mut output).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                "Experimental host output; local ownership transfer only.\n",
                "take: Taken\n",
                "work while held: HostOccupied\n",
                "sink: Rejected(Backpressured)\n",
                "sink: Accepted\n",
                "accepted: Some(Output { source: 7, work_call: 1, bytes: [2, 3, 5, 8] })\n",
                "empty sink: Empty\n",
            )
        );
    }
}
