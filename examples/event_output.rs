//! Bounded host event presentation with explicit partial-write progress.
//!
//! One turn attempts one write. The presenter owns its stream and retains the
//! pending event and unwritten suffix across errors. Acceptance alone does not
//! establish delivery.

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use std::io::{self, Cursor, Write};
use std::time::Duration;

use rust_flight_framework::{
    ApplicationId, Event, EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp,
    LifecycleRegistry,
};

const LINE_CAPACITY: usize = 96;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    Started,
    Degraded,
}

type MissionEvent = Event<MissionEventId>;
type EventLine = Cursor<[u8; LINE_CAPACITY]>;

#[derive(Debug)]
enum OutputError {
    UnknownSource(ApplicationId),
    Render(io::Error),
    Write(io::Error),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OutputOutcome {
    Idle,
    Pending { remaining: usize },
    Accepted(MissionEvent),
}

// The queue may refill after dequeue: total event ownership is queue plus one.
// Owning the writer prevents ordinary retries from switching output streams.
struct EventOutput<W> {
    writer: W,
    application: ApplicationId,
    pending: Option<MissionEvent>,
    line: EventLine,
    written: usize,
}

impl fmt::Display for OutputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSource(id) => write!(formatter, "no mission output name for {id:?}"),
            Self::Render(error) => write!(formatter, "event rendering failed: {error}"),
            Self::Write(error) => write!(formatter, "event output failed: {error}"),
        }
    }
}

impl Error for OutputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UnknownSource(_) => None,
            Self::Render(error) | Self::Write(error) => Some(error),
        }
    }
}

impl<W: Write> EventOutput<W> {
    fn new(writer: W, application: ApplicationId) -> Self {
        Self {
            writer,
            application,
            pending: None,
            line: Cursor::new([0; LINE_CAPACITY]),
            written: 0,
        }
    }

    // Errors preserve the original event, rendered bytes and accepted prefix.
    // No next record is removed until a later turn after complete acceptance.
    fn turn(
        &mut self,
        events: &mut EventQueue<MissionEventId>,
    ) -> Result<OutputOutcome, OutputError> {
        if self.pending.is_none() {
            self.pending = events.dequeue();
        }
        let Some(event) = self.pending else {
            return Ok(OutputOutcome::Idle);
        };
        if self.line.position() == 0 {
            self.line = render_event(&event, self.application)?;
        }

        let length = self.line.position() as usize;
        let remainder = &self.line.get_ref()[self.written..length];
        let accepted = self.writer.write(remainder).map_err(OutputError::Write)?;
        if accepted == 0 {
            return Err(OutputError::Write(io::ErrorKind::WriteZero.into()));
        }
        if accepted > remainder.len() {
            // A broken Write implementation cannot establish valid progress.
            return Err(OutputError::Write(io::ErrorKind::InvalidData.into()));
        }
        self.written += accepted;
        if self.written < length {
            return Ok(OutputOutcome::Pending {
                remaining: length - self.written,
            });
        }

        self.pending = None;
        self.line.set_position(0);
        self.written = 0;
        Ok(OutputOutcome::Accepted(event))
    }
}

fn render_event(
    event: &MissionEvent,
    application: ApplicationId,
) -> Result<EventLine, OutputError> {
    let source = match event.source() {
        EventSource::Framework => "framework",
        EventSource::Application(id) if id == application => "worker",
        EventSource::Application(id) => return Err(OutputError::UnknownSource(id)),
    };
    let severity = match event.severity() {
        EventSeverity::Informational => "info",
        EventSeverity::Warning => "warning",
        EventSeverity::Error => "error",
    };
    let identifier = match event.identifier() {
        MissionEventId::Started => "started",
        MissionEventId::Degraded => "degraded",
    };
    let elapsed = event.timestamp().elapsed();
    let mut line = Cursor::new([0; LINE_CAPACITY]);
    writeln!(
        line,
        "t={}.{:09} source={source} severity={severity} event={identifier}",
        elapsed.as_secs(),
        elapsed.subsec_nanos(),
    )
    .map_err(OutputError::Render)?;
    Ok(line)
}

fn event(source: EventSource, severity: EventSeverity, elapsed: Duration) -> MissionEvent {
    let identifier = match severity {
        EventSeverity::Informational => MissionEventId::Started,
        EventSeverity::Warning | EventSeverity::Error => MissionEventId::Degraded,
    };
    Event::new(
        source,
        severity,
        identifier,
        EventTimestamp::from_elapsed(elapsed),
    )
}

fn run_sample<W: Write>(writer: W) -> Result<(), Box<dyn Error>> {
    let mut registry = LifecycleRegistry::new(1)?;
    let application = registry.register()?;
    let mut events = EventQueue::new(2)?;
    let records = [
        event(
            EventSource::Framework,
            EventSeverity::Informational,
            Duration::ZERO,
        ),
        event(
            EventSource::Application(application),
            EventSeverity::Warning,
            Duration::from_millis(250),
        ),
    ];
    for record in records {
        if events.emit(&record) != EventEmitOutcome::Recorded {
            return Err(io::Error::other("sample event queue unexpectedly full").into());
        }
    }

    let mut output = EventOutput::new(writer, application);
    // Two finite lines fit even if every successful turn accepts one byte.
    // This sample stops on any error; a host may choose a later explicit retry.
    for _ in 0..records.len() * LINE_CAPACITY {
        output.turn(&mut events)?;
        if events.is_empty() && output.pending.is_none() {
            // Flush is separate: failure here does not restore accepted events.
            output.writer.flush()?;
            return Ok(());
        }
    }
    Err(io::Error::other("sample host turn budget exhausted").into())
}

fn main() -> Result<(), Box<dyn Error>> {
    run_sample(io::stdout().lock())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_flight_framework::{EventFilter, FilteredEventOutcome};

    const FIRST: &[u8] = b"t=0.000000000 source=framework severity=info event=started\n";
    const SECOND: &[u8] = b"t=0.250000000 source=worker severity=warning event=degraded\n";

    enum WriteStep {
        Accept(usize),
        Reject(io::ErrorKind),
    }

    struct ScriptedWriter {
        steps: std::collections::VecDeque<WriteStep>,
        accepted: Vec<u8>,
        calls: usize,
        flushes: usize,
        flush_error: bool,
    }

    impl ScriptedWriter {
        fn new(steps: impl IntoIterator<Item = WriteStep>) -> Self {
            Self {
                steps: steps.into_iter().collect(),
                accepted: Vec::new(),
                calls: 0,
                flushes: 0,
                flush_error: false,
            }
        }
    }

    impl Write for ScriptedWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            match self
                .steps
                .pop_front()
                .expect("each writer call is scripted")
            {
                WriteStep::Accept(limit) => {
                    let accepted = limit.min(bytes.len());
                    self.accepted.extend_from_slice(&bytes[..accepted]);
                    Ok(accepted)
                }
                WriteStep::Reject(kind) => Err(io::Error::new(kind, "scripted write error")),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.flush_error {
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "scripted flush error",
                ));
            }
            Ok(())
        }
    }

    fn application_ids() -> (ApplicationId, ApplicationId) {
        let mut registry = LifecycleRegistry::new(2).unwrap();
        (registry.register().unwrap(), registry.register().unwrap())
    }

    fn two_records(application: ApplicationId) -> (EventQueue<MissionEventId>, [MissionEvent; 2]) {
        let records = [
            event(
                EventSource::Framework,
                EventSeverity::Informational,
                Duration::ZERO,
            ),
            event(
                EventSource::Application(application),
                EventSeverity::Warning,
                Duration::from_millis(250),
            ),
        ];
        let mut events = EventQueue::new(2).unwrap();
        for record in records {
            assert_eq!(events.emit(&record), EventEmitOutcome::Recorded);
        }
        (events, records)
    }

    #[test]
    fn partial_writes_and_transient_errors_preserve_exact_fifo_bytes() {
        let (application, _) = application_ids();
        let (mut events, records) = two_records(application);
        let writer = ScriptedWriter::new([
            WriteStep::Accept(5),
            WriteStep::Reject(io::ErrorKind::Interrupted),
            WriteStep::Reject(io::ErrorKind::WouldBlock),
            WriteStep::Accept(2),
            WriteStep::Accept(usize::MAX),
            WriteStep::Accept(usize::MAX),
        ]);
        let mut output = EventOutput::new(writer, application);
        assert_eq!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Pending {
                remaining: FIRST.len() - 5
            }
        );
        for (index, kind) in [io::ErrorKind::Interrupted, io::ErrorKind::WouldBlock]
            .into_iter()
            .enumerate()
        {
            let error = output.turn(&mut events).unwrap_err();
            assert!(matches!(&error, OutputError::Write(source) if source.kind() == kind));
            assert_eq!(error.source().unwrap().to_string(), "scripted write error");
            assert_eq!(output.pending, Some(records[0]));
            assert_eq!(output.written, 5);
            assert_eq!(output.writer.calls, index + 2);
            assert_eq!(events.pending(), 1);
        }
        assert_eq!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Pending {
                remaining: FIRST.len() - 7
            }
        );
        assert_eq!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Accepted(records[0])
        );
        assert_eq!(events.pending(), 1);
        assert_eq!(output.writer.accepted, FIRST);
        assert_eq!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Accepted(records[1])
        );
        assert_eq!(output.writer.accepted, [FIRST, SECOND].concat());
        assert_eq!(output.writer.calls, 6);
        assert_eq!(output.writer.flushes, 0);
        assert_eq!(output.turn(&mut events).unwrap(), OutputOutcome::Idle);
        assert_eq!(output.writer.calls, 6);
    }

    #[test]
    fn zero_and_permanent_write_errors_retain_the_unwritten_suffix() {
        let (application, _) = application_ids();
        let (mut events, records) = two_records(application);
        let writer = ScriptedWriter::new([
            WriteStep::Accept(3),
            WriteStep::Accept(0),
            WriteStep::Reject(io::ErrorKind::BrokenPipe),
            WriteStep::Accept(usize::MAX),
        ]);
        let mut output = EventOutput::new(writer, application);
        assert!(matches!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Pending { .. }
        ));
        for kind in [io::ErrorKind::WriteZero, io::ErrorKind::BrokenPipe] {
            let error = output.turn(&mut events).unwrap_err();
            assert!(matches!(error, OutputError::Write(source) if source.kind() == kind));
            assert_eq!(output.pending, Some(records[0]));
            assert_eq!(output.written, 3);
            assert_eq!(output.writer.accepted, FIRST[..3]);
            assert_eq!(events.pending(), 1);
        }
        assert_eq!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Accepted(records[0])
        );
        assert_eq!(output.writer.accepted, FIRST);
        assert_eq!(events.dequeue(), Some(records[1]));
    }

    #[test]
    fn rendering_rejection_retains_the_raw_event_without_sink_calls() {
        let (application, unknown) = application_ids();
        let mut events = EventQueue::new(2).unwrap();
        let rejected = event(
            EventSource::Application(unknown),
            EventSeverity::Error,
            Duration::MAX,
        );
        let later = event(
            EventSource::Framework,
            EventSeverity::Warning,
            Duration::ZERO,
        );
        assert_eq!(events.emit(&rejected), EventEmitOutcome::Recorded);
        assert_eq!(events.emit(&later), EventEmitOutcome::Recorded);
        let mut output = EventOutput::new(ScriptedWriter::new([]), application);
        for _ in 0..2 {
            let error = output.turn(&mut events).unwrap_err();
            assert!(matches!(error, OutputError::UnknownSource(id) if id == unknown));
            assert_eq!(output.pending, Some(rejected));
            assert_eq!(output.line.position(), 0);
            assert_eq!(output.written, 0);
            assert_eq!(output.writer.calls, 0);
            assert_eq!(events.pending(), 1);
        }
        assert_eq!(events.dequeue(), Some(later));
    }

    #[test]
    fn maximum_timestamp_and_every_label_fit_the_fixed_line() {
        let (application, _) = application_ids();
        for source in [
            EventSource::Framework,
            EventSource::Application(application),
        ] {
            for severity in [
                EventSeverity::Informational,
                EventSeverity::Warning,
                EventSeverity::Error,
            ] {
                for identifier in [MissionEventId::Started, MissionEventId::Degraded] {
                    let record = Event::new(
                        source,
                        severity,
                        identifier,
                        EventTimestamp::from_elapsed(Duration::MAX),
                    );
                    let line = render_event(&record, application).unwrap();
                    let bytes = &line.get_ref()[..line.position() as usize];
                    assert!(bytes.len() <= LINE_CAPACITY);
                    assert!(bytes.starts_with(b"t=18446744073709551615.999999999 source="));
                    assert!(bytes.ends_with(b"\n"));
                }
            }
        }
    }

    #[test]
    fn filtered_omission_and_idle_turns_do_not_call_the_writer() {
        let (application, _) = application_ids();
        let mut events = EventQueue::new(1).unwrap();
        let optional = event(
            EventSource::Framework,
            EventSeverity::Informational,
            Duration::ZERO,
        );
        assert_eq!(
            events.emit_filtered(&optional, EventFilter::new(EventSeverity::Warning)),
            FilteredEventOutcome::FilteredOut
        );
        let mut output = EventOutput::new(ScriptedWriter::new([]), application);
        assert_eq!(output.turn(&mut events).unwrap(), OutputOutcome::Idle);
        assert_eq!(output.turn(&mut events).unwrap(), OutputOutcome::Idle);
        assert_eq!(output.writer.calls, 0);
    }

    #[test]
    fn dequeued_pending_record_is_additional_to_refilled_queue_capacity() {
        let (application, _) = application_ids();
        let mut events = EventQueue::new(1).unwrap();
        let first = event(
            EventSource::Framework,
            EventSeverity::Informational,
            Duration::ZERO,
        );
        let later = event(
            EventSource::Framework,
            EventSeverity::Error,
            Duration::from_secs(1),
        );
        assert_eq!(events.emit(&first), EventEmitOutcome::Recorded);
        let writer = ScriptedWriter::new([WriteStep::Accept(1), WriteStep::Accept(usize::MAX)]);
        let mut output = EventOutput::new(writer, application);
        assert!(matches!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Pending { .. }
        ));
        assert!(events.is_empty());
        assert_eq!(events.emit(&later), EventEmitOutcome::Recorded);
        assert_eq!(
            events.emit(&first),
            EventEmitOutcome::QueueFull { capacity: 1 }
        );
        assert_eq!(output.pending, Some(first));
        assert_eq!(
            output.turn(&mut events).unwrap(),
            OutputOutcome::Accepted(first)
        );
        assert_eq!(events.dequeue(), Some(later));
    }

    #[test]
    fn sample_writes_exact_lines_and_flushes_only_after_acceptance() {
        let mut writer =
            ScriptedWriter::new([WriteStep::Accept(usize::MAX), WriteStep::Accept(usize::MAX)]);
        run_sample(&mut writer).unwrap();
        assert_eq!(writer.accepted, [FIRST, SECOND].concat());
        assert_eq!(writer.calls, 2);
        assert_eq!(writer.flushes, 1);
    }

    #[test]
    fn flush_failure_reports_after_bytes_have_left_the_presenter() {
        let mut writer =
            ScriptedWriter::new([WriteStep::Accept(usize::MAX), WriteStep::Accept(usize::MAX)]);
        writer.flush_error = true;
        let error = run_sample(&mut writer).unwrap_err();
        assert_eq!(
            error.downcast_ref::<io::Error>().unwrap().kind(),
            io::ErrorKind::BrokenPipe
        );
        assert_eq!(writer.accepted, [FIRST, SECOND].concat());
        assert_eq!(writer.flushes, 1);
    }
}
