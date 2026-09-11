//! One finite host scenario using the real mission and framework owners.
//!
//! The fixed report copies observations after each explicit operation. It is
//! not an operational log or a scheduler. All output I/O stays in main.

use std::error::Error;
use std::time::Duration;

use rust_flight_framework::{
    ApplicationId, ApplicationState, Event, EventQueue, FrameworkInstant, ManualClock,
    MessageDispatchOutcome, MessagingOperationError, PublishClassification,
    RuntimeConfigurationError, RuntimeWorkEventError, ScheduledWork, ScheduledWorkOutcome,
    WorkSchedule,
};

use super::mission::applications::OutputMailbox;
use super::mission::work::{
    MissionConfigurationError, MissionRole, MissionWorkError, WorkMonitor, WorkObservation,
};
use super::mission::{ComposedMission, IngestError, compose, ingest};

type SampleResult<T> = Result<T, Box<dyn Error>>;
type ConfigurationFailure = RuntimeConfigurationError<MissionConfigurationError>;
type WorkFailure = MessagingOperationError<RuntimeWorkEventError<MissionWorkError, SampleEvent>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SampleEvent {
    EchoWorkFailed,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ConfigurationTrace {
    pub(crate) revisions: [u64; 3],
    pub(crate) observations: [WorkObservation; 4],
    pub(crate) rejection: ConfigurationFailure,
    pub(crate) consumed_rollback: ConfigurationFailure,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct FaultTrace {
    pub(crate) failure: WorkFailure,
    pub(crate) pending_after_failure: [usize; 2],
    pub(crate) observations: [WorkObservation; 2],
    pub(crate) peer_work: ApplicationState,
    pub(crate) retained_telemetry: [u8; 2],
    pub(crate) event: Event<SampleEvent>,
    pub(crate) next_event: Option<Event<SampleEvent>>,
    pub(crate) final_states: [ApplicationState; 2],
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SampleReport {
    pub(crate) applications: [ApplicationId; 2],
    // Each pair is echo then telemetry; rows follow lifecycle chronology.
    pub(crate) lifecycle: [[ApplicationState; 2]; 4],
    pub(crate) malformed_command: IngestError,
    pub(crate) telemetry: [[u8; 2]; 3],
    pub(crate) schedule: [ScheduledWorkOutcome; 4],
    pub(crate) scheduled_observations: [WorkObservation; 2],
    pub(crate) configuration: ConfigurationTrace,
    pub(crate) restarted_observations: [WorkObservation; 2],
    pub(crate) fault: FaultTrace,
}

/// Executes exactly one fresh scenario. Expected injected failures are data;
/// unexpected setup/operation failures terminate with their original error.
pub(crate) fn run() -> SampleResult<SampleReport> {
    let mailbox = OutputMailbox::default();
    let monitor = WorkMonitor::default();
    let mut mission = compose(&mailbox, Some(&monitor))?;
    let mut clock = ManualClock::default();
    let registered = states(&mission)?;
    let applications = [mission.echo_id, mission.telemetry_id];
    for id in applications {
        mission.runtime.start(id)?;
    }
    let started = states(&mission)?;
    let malformed_command = ingest(&mut mission.runtime, &[0xff, 255])
        .err()
        .ok_or("malformed command unexpectedly accepted")?;
    let mut telemetry = [[0; 2]; 3];
    for (record, value) in telemetry.iter_mut().zip([0, 42, 100]) {
        *record = command_round_trip(&mut mission, &mailbox, value)?;
    }
    let schedule = scheduled_work(&mut mission, &mut clock)?;
    let scheduled_observations = observations(&monitor)?;
    let configuration = configuration_changes(&mut mission, &monitor)?;
    let [stopped, restarted] = restart_both(&mut mission)?;
    for id in applications {
        mission.runtime.work(id)?;
    }
    let restarted_observations = observations(&monitor)?;
    let fault = cooperative_failure(&mut mission, &mailbox, &monitor, &mut clock)?;
    Ok(SampleReport {
        applications,
        lifecycle: [registered, started, stopped, restarted],
        malformed_command,
        telemetry,
        schedule,
        scheduled_observations,
        configuration,
        restarted_observations,
        fault,
    })
}

fn command_round_trip(
    mission: &mut ComposedMission<'_>,
    mailbox: &OutputMailbox,
    value: u8,
) -> SampleResult<[u8; 2]> {
    queue_command(mission, value)?;
    dispatch(mission, mission.echo_id)?;
    dispatch(mission, mission.telemetry_id)?;
    mailbox
        .drain()
        .ok_or_else(|| "expected one host record".into())
}

fn scheduled_work(
    mission: &mut ComposedMission<'_>,
    clock: &mut ManualClock,
) -> SampleResult<[ScheduledWorkOutcome; 4]> {
    let due = FrameworkInstant::from_elapsed(Duration::from_millis(10));
    let mut schedule = WorkSchedule::new(&[
        ScheduledWork::new(mission.echo_id, due),
        ScheduledWork::new(mission.telemetry_id, due),
    ])?;
    let waiting = mission
        .runtime
        .run_next_scheduled_work(&mut schedule, clock)?;
    clock.advance(Duration::from_millis(10))?;
    let echo = mission
        .runtime
        .run_next_scheduled_work(&mut schedule, clock)?;
    let telemetry = mission
        .runtime
        .run_next_scheduled_work(&mut schedule, clock)?;
    let complete = mission
        .runtime
        .run_next_scheduled_work(&mut schedule, clock)?;
    Ok([waiting, echo, telemetry, complete])
}

fn configuration_changes(
    mission: &mut ComposedMission<'_>,
    monitor: &WorkMonitor,
) -> SampleResult<ConfigurationTrace> {
    let activated = mission.runtime.replace_configuration(&[20])?;
    let active = echo_work(mission, monitor)?;
    let rejection = mission
        .runtime
        .replace_configuration(&[101])
        .err()
        .ok_or("invalid configuration unexpectedly accepted")?;
    let after_rejection = echo_work(mission, monitor)?;
    let rolled_back = mission.runtime.rollback_configuration()?;
    let after_rollback = echo_work(mission, monitor)?;
    let consumed_rollback = mission
        .runtime
        .rollback_configuration()
        .err()
        .ok_or("rollback unexpectedly reusable")?;
    let fresh = mission.runtime.replace_configuration(&[30])?;
    let after_fresh = echo_work(mission, monitor)?;
    Ok(ConfigurationTrace {
        revisions: [activated, rolled_back, fresh],
        observations: [active, after_rejection, after_rollback, after_fresh],
        rejection,
        consumed_rollback,
    })
}

fn restart_both(mission: &mut ComposedMission<'_>) -> SampleResult<[[ApplicationState; 2]; 2]> {
    let ids = [mission.echo_id, mission.telemetry_id];
    for id in ids {
        mission.runtime.stop(id)?;
    }
    let stopped = states(mission)?;
    for id in ids {
        mission.runtime.restart(id)?;
    }
    Ok([stopped, states(mission)?])
}

fn cooperative_failure(
    mission: &mut ComposedMission<'_>,
    mailbox: &OutputMailbox,
    monitor: &WorkMonitor,
    clock: &mut ManualClock,
) -> SampleResult<FaultTrace> {
    // Retain telemetry 7 in the peer inbox and command 9 in the echo inbox.
    queue_command(mission, 7)?;
    dispatch(mission, mission.echo_id)?;
    queue_command(mission, 9)?;
    let mut events = EventQueue::new(1)?;
    monitor.arm_echo_failure();
    clock.advance(Duration::from_millis(10))?;
    let failure = mission
        .runtime
        .work_with_failure_event(
            mission.echo_id,
            SampleEvent::EchoWorkFailed,
            clock,
            &mut events,
        )
        .err()
        .ok_or("injected echo work unexpectedly succeeded")?;
    let pending_after_failure = [
        mission.runtime.pending(mission.echo_id)?,
        mission.runtime.pending(mission.telemetry_id)?,
    ];
    let peer_work = mission.runtime.work(mission.telemetry_id)?;
    let observations = observations(monitor)?;
    dispatch(mission, mission.telemetry_id)?;
    let retained_telemetry = mailbox.drain().ok_or("expected retained peer telemetry")?;
    let event = events.dequeue().ok_or("expected recorded work failure")?;
    Ok(FaultTrace {
        failure,
        pending_after_failure,
        observations,
        peer_work,
        retained_telemetry,
        event,
        next_event: events.dequeue(),
        final_states: states(mission)?,
    })
}

fn states(mission: &ComposedMission<'_>) -> SampleResult<[ApplicationState; 2]> {
    Ok([
        mission.runtime.state(mission.echo_id)?,
        mission.runtime.state(mission.telemetry_id)?,
    ])
}

fn observations(monitor: &WorkMonitor) -> SampleResult<[WorkObservation; 2]> {
    Ok([
        monitor
            .take_observation(MissionRole::Echo)
            .ok_or("missing echo observation")?,
        monitor
            .take_observation(MissionRole::Telemetry)
            .ok_or("missing telemetry observation")?,
    ])
}

fn echo_work(
    mission: &mut ComposedMission<'_>,
    monitor: &WorkMonitor,
) -> SampleResult<WorkObservation> {
    mission.runtime.work(mission.echo_id)?;
    monitor
        .take_observation(MissionRole::Echo)
        .ok_or_else(|| "missing echo work observation".into())
}

fn queue_command(mission: &mut ComposedMission<'_>, value: u8) -> SampleResult<()> {
    let report = ingest(&mut mission.runtime, &[0x01, value])?;
    if report.classification() != PublishClassification::Complete {
        return Err(format!("command not fully queued: {report:?}").into());
    }
    Ok(())
}

fn dispatch(mission: &mut ComposedMission<'_>, id: ApplicationId) -> SampleResult<()> {
    if mission.runtime.dispatch_one(id)? != MessageDispatchOutcome::Dispatched {
        return Err("expected one message for explicit dispatch".into());
    }
    Ok(())
}
