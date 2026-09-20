use super::*;
use intent_local_transport::{WorkerHello, WorkerRole, issue_worker_authentication};
use std::error::Error;

thread_local! {
    static READ_DISPOSALS: std::cell::RefCell<Vec<(usize, bool)>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub(super) fn observe_read_disposal(bytes: &[u8]) {
    READ_DISPOSALS.with(|events| {
        events
            .borrow_mut()
            .push((bytes.len(), bytes.iter().all(|byte| *byte == 0)))
    });
}

#[test]
fn consumed_hello_bytes_are_erased_without_losing_buffered_ready() -> Result<(), Box<dyn Error>> {
    for ready_prefix in [2, usize::MAX] {
        let (receiver, mut sender) = UnixStream::pair()?;
        let mut receiver = FramedSocket::new(receiver)?;
        let generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
        let (token, _verifier) = issue_worker_authentication(generation, WorkerRole::FixtureWorker)
            .map_err(|error| format!("bootstrap entropy: {error}"))?;
        let hello = encode_bootstrap_event(ControlMessage::Hello(ChannelHello {
            channel: ChannelKind::Control,
            identity: WorkerHello::new(generation, WorkerRole::FixtureWorker, token),
            offer: offer()?,
        }))?;
        let ready = encode_event(ControlMessage::Ready { generation }, None)?;
        let prefix = ready_prefix.min(ready.len());
        let hello_len = hello.as_bytes().len();
        receiver.buffer[..hello_len].copy_from_slice(hello.as_bytes());
        receiver.buffer[hello_len..hello_len + prefix].copy_from_slice(&ready[..prefix]);
        receiver.length = hello_len + prefix;
        let ReadOutcome::Frame(frame) = receiver.read_one(&mut 4096)? else {
            return Err("missing Hello".into());
        };
        let decoded: Envelope<ControlMessage> = decode(frame, None)?;
        assert!(matches!(decoded.payload(), ControlMessage::Hello(_)));
        assert_eq!(receiver.offset, hello_len);
        assert!(receiver.buffer[..hello_len].iter().all(|byte| *byte == 0));
        assert_eq!(
            &receiver.buffer[hello_len..receiver.length],
            &ready[..prefix]
        );
        sender.write_all(&ready[prefix..])?;
        let ReadOutcome::Frame(frame) = receiver.read_one(&mut 4096)? else {
            return Err("lost Ready".into());
        };
        let decoded: Envelope<ControlMessage> = decode(frame, None)?;
        assert!(
            matches!(decoded.payload(), ControlMessage::Ready { generation: actual } if *actual == generation)
        );
        assert!(
            receiver.buffer[..receiver.offset]
                .iter()
                .all(|byte| *byte == 0)
        );
    }
    Ok(())
}

#[test]
fn framing_failure_erases_the_rejected_socket_input() -> Result<(), Box<dyn Error>> {
    let (receiver, _sender) = UnixStream::pair()?;
    let mut receiver = FramedSocket::new(receiver)?;
    receiver.buffer[..8].copy_from_slice(&[9, 0, 0, 0, 3, 171, 172, 173]);
    receiver.length = 8;
    assert!(receiver.read_one(&mut 4096).is_err());
    assert!(receiver.buffer.iter().all(|byte| *byte == 0));
    assert_eq!((receiver.offset, receiver.length), (0, 0));
    assert!(receiver.decoder.is_poisoned());
    Ok(())
}

#[test]
fn blocking_partial_payload_still_reports_unexpected_eof() {
    READ_DISPOSALS.with(|events| events.borrow_mut().clear());
    let mut bytes = [1, 0, 0, 0, 5, 171, 172].as_slice();
    let result = read_blocking::<BootstrapPacket>(&mut bytes, None);
    assert!(
        matches!(result, Err(SupervisorError::Io(error)) if error.kind() == io::ErrorKind::UnexpectedEof)
    );
    READ_DISPOSALS.with(|events| assert_eq!(*events.borrow(), vec![(5, true)]));
}
