use super::*;
use crate::{Priority, ProcessLimits, RestartPolicy};
use intent_contracts::{AccountId, CapabilityId};
use intent_local_transport::WorkerRole;
use sha2::{Digest, Sha256};
use std::{error::Error, os::unix::net::UnixStream};

fn starting_worker() -> Result<(Supervisor, WorkerInstanceId), Box<dyn Error>> {
    let mut supervisor = Supervisor::new(
        &std::env::temp_dir(),
        AdmissionLimits::new(1, 128 * 1024 * 1024, 100, [0; 2], [0; 2], [0; 2])?,
        SchedulerLimits::default(),
    )?;
    let path = Path::new("/bin/sleep");
    let hash = ContentHash::from_bytes(Sha256::digest(std::fs::read(path)?).into());
    let image = ExecutableImage::load(path, hash)?;
    let config = WorkerConfig::new(
        WorkerScope {
            task_id: TaskId::from_uuid(Uuid::new_v4()),
            account_id: AccountId::from_uuid(Uuid::new_v4()),
        },
        WorkerRole::FixtureWorker,
        [CapabilityId::from_uuid(Uuid::new_v4())],
        Priority::Background,
        ProcessLimits::new(128 * 1024 * 1024, 10, 64, 100)?,
        HealthPolicy::default(),
        Duration::from_secs(20),
        RestartPolicy::never(),
        crate::ExecutionBoundary::CooperativeLocal,
    )?;
    let id = supervisor.launch(image, config, &["10".to_owned()])?;
    Ok((supervisor, id))
}

fn fixture_identity(
    id: WorkerInstanceId,
    stream: &UnixStream,
) -> Result<WorkerIdentity, Box<dyn Error>> {
    let (token, pending) = issue_worker_authentication(id, WorkerRole::FixtureWorker)
        .map_err(|error| format!("bootstrap entropy: {error}"))?;
    let mut verifier = pending.bind(ExpectedPeer::unix_process(
        std::process::id(),
        geteuid().as_raw(),
        getegid().as_raw(),
    )?);
    Ok(verifier.authenticate_unix(
        &WorkerHello::new(id, WorkerRole::FixtureWorker, token),
        stream,
    )?)
}

#[test]
fn exact_deadline_revokes_startup_and_discards_both_unused_credentials()
-> Result<(), Box<dyn Error>> {
    let (mut supervisor, id) = starting_worker()?;
    let entry = supervisor.entries.get_mut(&id).ok_or("missing entry")?;
    let deadline = entry.startup_deadline;
    assert!(!entry.expire_startup(deadline - Duration::from_nanos(1)));
    assert!(entry.control.authenticator.is_some());
    assert!(entry.progress.authenticator.is_some());
    assert!(entry.expire_startup(deadline));
    assert!(entry.lease.is_revoked());
    assert!(entry.control.authenticator.is_none());
    assert!(entry.progress.authenticator.is_none());
    assert_eq!(entry.failure, Some(WorkerFailure::HandshakeTimeout));
    assert_eq!(entry.state, WorkerState::Draining);
    Ok(())
}

#[test]
fn an_expired_silent_startup_ignores_zero_budget_and_stale_poll_time() -> Result<(), Box<dyn Error>>
{
    let (mut supervisor, id) = starting_worker()?;
    let entry = supervisor.entries.get_mut(&id).ok_or("missing entry")?;
    entry.startup_deadline = Instant::now();
    let stale_poll_time = entry.startup_deadline - Duration::from_secs(1);
    let mut budget = ReadBudget::new(0);
    assert!(!entry.poll(stale_poll_time, &mut budget)?);
    assert_eq!(budget.consumed(), 0);
    assert_eq!(entry.failure, Some(WorkerFailure::HandshakeTimeout));
    assert_eq!(entry.state, WorkerState::Draining);
    assert!(entry.lease.is_revoked());
    Ok(())
}

#[test]
fn ready_admission_rechecks_time_after_the_outer_poll_started() -> Result<(), Box<dyn Error>> {
    let (mut supervisor, id) = starting_worker()?;
    let entry = supervisor.entries.get_mut(&id).ok_or("missing entry")?;
    let (receiver, mut sender) = UnixStream::pair()?;
    entry.progress.identity = Some(fixture_identity(id, &receiver)?);
    entry.control.socket = Some(FramedSocket::new(receiver)?);
    sender.write_all(&encode_event(
        ControlMessage::Ready { generation: id },
        None,
    )?)?;
    entry.startup_deadline = Instant::now();
    let stale_poll_time = entry.startup_deadline - Duration::from_secs(1);
    entry.read_control(
        stale_poll_time,
        &mut ReadBudget::new(MAX_READ_BYTES_PER_POLL),
    )?;
    assert_eq!(entry.failure, Some(WorkerFailure::HandshakeTimeout));
    assert_eq!(entry.state, WorkerState::Draining);
    assert!(entry.lease.is_revoked());
    Ok(())
}

#[test]
fn late_hello_cannot_consume_the_lane_credential() -> Result<(), Box<dyn Error>> {
    let (mut supervisor, id) = starting_worker()?;
    let entry = supervisor.entries.get_mut(&id).ok_or("missing entry")?;
    let (token, pending) = issue_worker_authentication(id, WorkerRole::FixtureWorker)
        .map_err(|error| format!("bootstrap entropy: {error}"))?;
    entry.control.authenticator = Some(pending.bind(ExpectedPeer::unix_process(
        std::process::id(),
        geteuid().as_raw(),
        getegid().as_raw(),
    )?));
    let (receiver, mut sender) = UnixStream::pair()?;
    entry.control.socket = Some(FramedSocket::new(receiver)?);
    sender.write_all(&encode_event(
        ControlMessage::Hello(ChannelHello {
            channel: ChannelKind::Control,
            identity: WorkerHello::new(id, WorkerRole::FixtureWorker, token),
            offer: offer()?,
        }),
        None,
    )?)?;
    let mut budget = ReadBudget::new(MAX_READ_BYTES_PER_POLL);
    assert_eq!(
        entry.control.poll_authentication(
            ChannelKind::Control,
            id,
            Instant::now(),
            &mut entry.rejected_peers,
            &mut budget,
        )?,
        AuthenticationProgress::Expired
    );
    assert!(entry.control.authenticator.is_some());
    assert!(entry.control.identity.is_none());
    assert!(entry.control.codec.is_none());
    assert_eq!(budget.consumed(), 0);
    Ok(())
}

fn serviced_ready_signal(heartbeat: bool) -> Result<(), Box<dyn Error>> {
    let (mut supervisor, id) = starting_worker()?;
    let entry = supervisor.entries.get_mut(&id).ok_or("missing entry")?;
    entry.state = WorkerState::Ready;
    entry.config.health.heartbeat_timeout = Duration::from_millis(100);
    entry.config.health.work_progress_timeout = Duration::from_millis(100);
    let stale_poll_time = Instant::now() - Duration::from_secs(2);
    entry.heartbeat = if heartbeat {
        stale_poll_time - Duration::from_secs(1)
    } else {
        Instant::now()
    };
    entry.progress_at = stale_poll_time - Duration::from_secs(1);
    let (receiver, mut sender) = UnixStream::pair()?;
    let identity = fixture_identity(id, &receiver)?;
    if heartbeat {
        entry.control.identity = Some(identity);
        entry.control.socket = Some(FramedSocket::new(receiver)?);
        sender.write_all(&encode_event(
            ControlMessage::Heartbeat {
                generation: id,
                sequence: 1,
            },
            None,
        )?)?;
    } else {
        entry.progress.identity = Some(identity);
        entry.progress.socket = Some(FramedSocket::new(receiver)?);
        entry.pending.insert(
            RequestId::from_uuid(Uuid::new_v4()),
            PendingRequest {
                sequence: 1,
                expires: Instant::now() + Duration::from_secs(10),
                sent: true,
            },
        );
        sender.write_all(&encode_event(
            ProgressMessage {
                generation: id,
                sequence: 1,
                work_sequence: 1,
            },
            None,
        )?)?;
    }
    entry.poll(
        stale_poll_time,
        &mut ReadBudget::new(MAX_READ_BYTES_PER_POLL),
    )?;
    assert_eq!(
        if heartbeat {
            entry.last_heartbeat
        } else {
            entry.last_progress
        },
        1
    );
    assert_eq!(entry.state, WorkerState::Ready);
    assert_eq!(entry.failure, None);
    assert!(!entry.lease.is_revoked());
    Ok(())
}

#[test]
fn a_serviced_ready_heartbeat_keeps_the_existing_poll_clock_policy() -> Result<(), Box<dyn Error>> {
    serviced_ready_signal(true)
}

#[test]
fn serviced_ready_progress_keeps_the_existing_poll_clock_policy() -> Result<(), Box<dyn Error>> {
    serviced_ready_signal(false)
}
