use super::*;
use intent_ipc::{Frame, FrameLane};
use std::{error::Error, os::unix::net::UnixStream};

fn policy() -> HealthPolicy {
    HealthPolicy {
        handshake_timeout: Duration::from_secs(1),
        heartbeat_timeout: Duration::from_secs(1),
        work_progress_timeout: Duration::from_secs(1),
        ..HealthPolicy::default()
    }
}

#[test]
fn a_fully_serviced_last_worker_is_not_exempt_from_heartbeat_expiry() -> Result<(), Box<dyn Error>>
{
    let frame = Frame::new(FrameLane::Control, vec![0; 1019]).encode(crate::wire::wire_limits())?;
    assert_eq!(frame.len(), 1024);
    let mut budget = ReadBudget::new(MAX_READ_BYTES_PER_POLL);
    for worker in 0..8 {
        let (receiver, mut sender) = UnixStream::pair()?;
        sender.write_all(&frame.repeat(4))?;
        let mut socket = FramedSocket::new(receiver)?;
        let before_blocks = budget.blocked_reads();
        let mut lane_budget = 4096;
        for _ in 0..4 {
            let before = budget.consumed();
            assert!(matches!(
                budget.read_one(&mut socket, lane_budget)?,
                ReadOutcome::Frame(_)
            ));
            lane_budget -= budget.consumed() - before;
        }
        assert_eq!(lane_budget, 0);
        assert_eq!(budget.blocked_reads(), before_blocks);
        assert_eq!(
            health_failure_after_reads(
                WorkerState::Ready,
                HealthAges {
                    starting: Duration::ZERO,
                    heartbeat: Duration::from_secs(10),
                    progress: Duration::ZERO,
                },
                false,
                policy(),
                HealthReadBlocks {
                    control: budget.blocked_reads() != before_blocks,
                    progress: false,
                },
            ),
            Some(WorkerFailure::HeartbeatTimeout),
            "worker {worker} had its full bounded read opportunity"
        );
    }
    assert_eq!(budget.consumed(), MAX_READ_BYTES_PER_POLL);
    Ok(())
}

#[test]
fn progress_lane_saturation_cannot_mask_a_serviced_control_lane_timeout() {
    assert_eq!(
        health_failure_after_reads(
            WorkerState::Ready,
            HealthAges {
                starting: Duration::ZERO,
                heartbeat: Duration::from_secs(1),
                progress: Duration::ZERO,
            },
            false,
            policy(),
            HealthReadBlocks {
                control: false,
                progress: true
            },
        ),
        Some(WorkerFailure::HeartbeatTimeout)
    );
}

#[test]
fn only_reads_that_can_carry_the_missing_health_signal_defer_its_timeout() {
    for control in [false, true] {
        for progress in [false, true] {
            let blocked = HealthReadBlocks { control, progress };
            let old = Duration::from_secs(1);
            let ages = HealthAges {
                starting: old,
                heartbeat: old,
                progress: old,
            };
            assert_eq!(
                health_failure_after_reads(WorkerState::Starting, ages, false, policy(), blocked),
                if control || progress {
                    None
                } else {
                    Some(WorkerFailure::HandshakeTimeout)
                }
            );
            assert_eq!(
                health_failure_after_reads(WorkerState::Ready, ages, false, policy(), blocked),
                if control {
                    None
                } else {
                    Some(WorkerFailure::HeartbeatTimeout)
                }
            );
            let ages = HealthAges {
                heartbeat: Duration::ZERO,
                ..ages
            };
            for pending in [false, true] {
                assert_eq!(
                    health_failure_after_reads(
                        WorkerState::Ready,
                        ages,
                        pending,
                        policy(),
                        blocked
                    ),
                    if !pending || control || progress {
                        None
                    } else {
                        Some(WorkerFailure::ProgressTimeout)
                    }
                );
            }
            for state in [
                WorkerState::Draining,
                WorkerState::Stopped,
                WorkerState::Failed,
            ] {
                assert_eq!(
                    health_failure_after_reads(state, ages, true, policy(), blocked),
                    None
                );
            }
        }
    }
}

#[test]
fn read_cursor_services_all_sixty_four_sockets_in_bounded_windows() -> Result<(), Box<dyn Error>> {
    let frame = Frame::new(FrameLane::Control, vec![1; 4091]).encode(crate::wire::wire_limits())?;
    assert_eq!(frame.len(), 4096);
    let mut senders = Vec::new();
    let mut sockets = Vec::new();
    for _ in 0..64 {
        let (receiver, mut sender) = UnixStream::pair()?;
        sender.write_all(&frame)?;
        senders.push(sender);
        sockets.push(FramedSocket::new(receiver)?);
    }
    let mut visits = [0; 64];
    let mut cursor = 0;
    for _ in 0..8 {
        let mut budget = ReadBudget::new(MAX_READ_BYTES_PER_POLL);
        let mut last = None;
        for offset in 0..sockets.len() {
            let index = (cursor + offset) % sockets.len();
            let before = budget.consumed();
            if let ReadOutcome::Frame(_) = budget.read_one(&mut sockets[index], 4096)? {
                visits[index] += 1;
                senders[index].write_all(&frame)?;
            }
            if budget.consumed() > before {
                last = Some(offset);
            }
        }
        assert_eq!(budget.consumed(), MAX_READ_BYTES_PER_POLL);
        cursor = next_read_cursor(cursor, sockets.len(), last);
    }
    assert_eq!(visits, [1; 64]);
    Ok(())
}
