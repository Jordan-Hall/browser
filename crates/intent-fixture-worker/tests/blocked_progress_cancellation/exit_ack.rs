use super::*;
use intent_supervisor::WorkerFailure;

struct Marker(std::path::PathBuf);

impl Drop for Marker {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn acknowledgement_survives_child_exit_before_the_next_poll() -> TestResult {
    exit_before_poll(0, "ack")
}

#[test]
fn acknowledgement_survives_multiple_bounded_windows_after_exit() -> TestResult {
    exit_before_poll(24, "ack")
}

#[test]
fn exit_without_acknowledgement_is_not_reported_as_acknowledged() -> TestResult {
    exit_before_poll(0, "no-ack")
}

#[test]
fn wrong_cancellation_identity_after_exit_is_rejected() -> TestResult {
    exit_before_poll(0, "wrong-id")
}

fn exit_before_poll(exit_heartbeats: u8, acknowledgement: &str) -> TestResult {
    let marker = Marker(std::env::temp_dir().join(format!("intent-exit-ack-{}", Uuid::new_v4())));
    let health = HealthPolicy {
        stop_grace: Duration::from_secs(2),
        terminate_grace: Duration::from_secs(1),
        ..HealthPolicy::default()
    };
    let scope = scope();
    let capability = capability();
    let mut supervisor = supervisor()?;
    let id = supervisor.launch(
        image()?,
        config(scope, capability, health)?,
        &[
            marker.0.to_string_lossy().into_owned(),
            exit_heartbeats.to_string(),
            acknowledgement.to_owned(),
        ],
    )?;
    let ready = until(&mut supervisor, id, Duration::from_secs(5), |snapshot| {
        matches!(snapshot.state, WorkerState::Ready | WorkerState::Failed)
    })?;
    if ready.state != WorkerState::Ready {
        return Err(format!("worker did not become ready: {ready:?}").into());
    }
    let lease = supervisor.lease(id)?;
    let request = RequestId::from_uuid(Uuid::new_v4());
    let permit = lease.admit(
        request,
        lease.scope(),
        capability,
        MessageFamily::LifecycleControl,
        Instant::now() + Duration::from_secs(10),
    )?;
    supervisor.submit_immediate(permit, BoundedText::try_new("start progress flood")?)?;

    let blocked_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match std::fs::read(&marker.0) {
            Ok(contents) if contents == b"progress transport backpressured" => break,
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        if Instant::now() >= blocked_deadline {
            return Err("worker did not reach measured progress backpressure".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    let receipt = supervisor.cancel(id, CancellationId::from_uuid(Uuid::new_v4()))?;
    assert!(receipt.newly_revoked);
    supervisor.poll();
    let pid = supervisor.snapshot(id)?.process_id;
    let exit_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let stat = std::fs::read_to_string(format!("/proc/{pid}/stat"))?;
        let state = stat.rsplit_once(") ").ok_or("malformed child stat")?.1;
        if state.starts_with("Z ") {
            break;
        }
        if Instant::now() >= exit_deadline {
            return Err("child did not exit before the supervisor's next poll".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    supervisor.poll();
    let first = supervisor.snapshot(id)?;
    assert!(lease.is_revoked());
    assert!(supervisor.lease(id).is_err());
    assert!(supervisor.observation(id).is_none());
    if exit_heartbeats > 8 {
        assert_eq!(first.state, WorkerState::Draining);
        assert!(!first.cancellation_acknowledged);
        assert!(supervisor.retire(id).is_err());
    }
    let stopped = until(&mut supervisor, id, Duration::from_secs(5), |snapshot| {
        matches!(snapshot.state, WorkerState::Stopped | WorkerState::Failed)
    })?;
    assert_eq!(stopped.cancellation_acknowledged, acknowledgement == "ack");
    assert_eq!(
        stopped.failure,
        (acknowledgement == "wrong-id").then_some(WorkerFailure::ProtocolViolation)
    );
    assert!(!stopped.stop_escalated);
    assert_eq!(
        std::fs::read(&marker.0)?,
        b"cancel received while progress backpressured"
    );
    assert_eq!(supervisor.retire(id)?.unresolved_requests, vec![request]);
    Ok(())
}
