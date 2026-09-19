use super::*;
use std::error::Error;

fn pair() -> Result<(FramedSocket, UnixStream), Box<dyn Error>> {
    let (receiver, sender) = UnixStream::pair()?;
    Ok((FramedSocket::new(receiver)?, sender))
}

#[test]
fn aggregate_truncation_blocks_only_after_the_allowed_bytes_are_consumed()
-> Result<(), Box<dyn Error>> {
    let (mut socket, mut sender) = pair()?;
    let frame = Frame::new(intent_ipc::FrameLane::Control, vec![7; 100]);
    let encoded = frame.encode(wire_limits())?;
    sender.write_all(&encoded)?;
    let mut budget = ReadBudget::new(16);
    assert!(matches!(
        budget.read_one(&mut socket, 4096)?,
        ReadOutcome::Pending
    ));
    assert_eq!(budget.consumed(), 16);
    assert_eq!(budget.blocked_reads(), 1);
    let mut next = ReadBudget::new(4096);
    let ReadOutcome::Frame(decoded) = next.read_one(&mut socket, 4096)? else {
        return Err("resumed frame missing".into());
    };
    assert_eq!(decoded, frame);
    assert_eq!(next.consumed(), encoded.len() - 16);
    assert_eq!(next.blocked_reads(), 0);
    Ok(())
}

#[test]
fn unused_allowance_and_closed_sockets_do_not_extend_health_deadlines() -> Result<(), Box<dyn Error>>
{
    let (mut socket, mut sender) = pair()?;
    let mut budget = ReadBudget::new(16);
    assert!(matches!(
        budget.read_one(&mut socket, 4096)?,
        ReadOutcome::Pending
    ));
    assert_eq!(budget.consumed(), 0);
    assert_eq!(budget.blocked_reads(), 0);
    sender.write_all(&[1, 0])?;
    assert!(matches!(
        budget.read_one(&mut socket, 4096)?,
        ReadOutcome::Pending
    ));
    assert_eq!(budget.consumed(), 2);
    assert_eq!(budget.blocked_reads(), 0);
    drop(sender);
    assert!(budget.read_one(&mut socket, 4096).is_err());
    assert_eq!(budget.consumed(), 2);
    assert_eq!(budget.blocked_reads(), 0);
    let (mut closed, sender) = pair()?;
    drop(sender);
    assert!(matches!(
        budget.read_one(&mut closed, 4096)?,
        ReadOutcome::Closed
    ));
    assert_eq!(budget.blocked_reads(), 0);
    Ok(())
}

#[test]
fn completed_and_buffered_frames_are_not_aggregate_blocks() -> Result<(), Box<dyn Error>> {
    let (mut socket, mut sender) = pair()?;
    let encoded = Frame::new(intent_ipc::FrameLane::Control, vec![3; 27]).encode(wire_limits())?;
    assert_eq!(encoded.len(), 32);
    sender.write_all(&encoded.repeat(2))?;
    let mut budget = ReadBudget::new(64);
    for _ in 0..2 {
        assert!(matches!(
            budget.read_one(&mut socket, 4096)?,
            ReadOutcome::Frame(_)
        ));
        assert_eq!(budget.consumed(), 64);
        assert_eq!(budget.blocked_reads(), 0);
    }
    assert!(matches!(
        budget.read_one(&mut socket, 4096)?,
        ReadOutcome::Pending
    ));
    assert_eq!(budget.blocked_reads(), 1);
    Ok(())
}

#[test]
fn an_exact_lane_allowance_or_zero_lane_allowance_is_not_an_aggregate_block()
-> Result<(), Box<dyn Error>> {
    let (mut socket, mut sender) = pair()?;
    let encoded = Frame::new(intent_ipc::FrameLane::Control, vec![4; 100]).encode(wire_limits())?;
    sender.write_all(&encoded)?;
    let mut budget = ReadBudget::new(16);
    assert!(matches!(
        budget.read_one(&mut socket, 16)?,
        ReadOutcome::Pending
    ));
    assert_eq!(budget.consumed(), 16);
    assert_eq!(budget.blocked_reads(), 0);
    assert!(matches!(
        budget.read_one(&mut socket, 0)?,
        ReadOutcome::Pending
    ));
    assert_eq!(budget.blocked_reads(), 0);
    assert!(matches!(
        budget.read_one(&mut socket, 1)?,
        ReadOutcome::Pending
    ));
    assert_eq!(budget.blocked_reads(), 1);
    Ok(())
}

#[test]
fn malformed_reads_are_charged_without_creating_a_health_deferral() -> Result<(), Box<dyn Error>> {
    let (mut socket, mut sender) = pair()?;
    sender.write_all(&[255, 0, 0, 0, 0])?;
    let mut budget = ReadBudget::new(5);
    assert!(budget.read_one(&mut socket, 4096).is_err());
    assert_eq!(budget.consumed(), 5);
    assert_eq!(budget.blocked_reads(), 0);
    Ok(())
}
