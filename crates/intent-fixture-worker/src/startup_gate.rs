use intent_contracts::{SchemaVersion, TraceId};
use intent_ipc::{Envelope, FrameDecoder, decode_control, encode_control};
use intent_supervisor::{BootstrapPacket, ControlMessage, wire_limits};
use std::{
    error::Error,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::Path,
    time::{Duration, Instant},
};
use uuid::Uuid;

fn send(stream: &mut UnixStream, message: ControlMessage) -> Result<(), Box<dyn Error>> {
    let envelope = Envelope::event(TraceId::from_uuid(Uuid::new_v4()), message);
    stream.write_all(&encode_control(&envelope, wire_limits())?.encode(wire_limits())?)?;
    Ok(())
}

fn record_send(
    directory: &Path,
    result: &Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let outcome = match result {
        Ok(()) => b"sent".as_slice(),
        Err(error)
            if error.downcast_ref::<std::io::Error>().is_some_and(|error| {
                matches!(
                    error.kind(),
                    std::io::ErrorKind::BrokenPipe
                        | std::io::ErrorKind::ConnectionReset
                        | std::io::ErrorKind::NotConnected
                )
            }) =>
        {
            b"closed".as_slice()
        }
        Err(_) => return Ok(()),
    };
    complete(directory, outcome)?;
    Ok(())
}

fn complete(directory: &Path, outcome: &[u8]) -> Result<(), Box<dyn Error>> {
    std::fs::write(directory.join("completed.tmp"), outcome)?;
    std::fs::rename(directory.join("completed.tmp"), directory.join("completed"))?;
    Ok(())
}

fn receive(stream: &mut UnixStream) -> Result<Envelope<ControlMessage>, Box<dyn Error>> {
    let mut decoder = FrameDecoder::new(wire_limits());
    loop {
        let mut byte = [0];
        stream.read_exact(&mut byte)?;
        if let Some(frame) = decoder.push(&byte)?.into_frames().into_iter().next() {
            return Ok(decode_control(&frame, wire_limits())?);
        }
    }
}

fn await_release(directory: &Path) -> Result<(), Box<dyn Error>> {
    std::fs::write(
        directory.join("waiting.tmp"),
        std::process::id().to_string(),
    )?;
    std::fs::rename(directory.join("waiting.tmp"), directory.join("waiting"))?;
    let deadline = Instant::now() + Duration::from_secs(10);
    while !directory.join("release").try_exists()? {
        if Instant::now() >= deadline {
            return Err("parent did not release startup gate".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    Ok(())
}

pub fn run(packet: BootstrapPacket, directory: &str, gate_at: &str) -> Result<(), Box<dyn Error>> {
    if !matches!(gate_at, "control" | "progress" | "ready" | "disconnect") {
        return Err("unknown startup gate".into());
    }
    let directory = Path::new(directory);
    let generation = packet.control.identity.instance_id();
    let mut control = UnixStream::connect(packet.control_endpoint.as_str())?;
    if gate_at == "disconnect" {
        control.set_read_timeout(Some(Duration::from_secs(10)))?;
        control.set_write_timeout(Some(Duration::from_secs(10)))?;
        send(&mut control, ControlMessage::Hello(packet.control))?;
        if !matches!(receive(&mut control)?.payload(), ControlMessage::Welcome { generation: id, selected }
            if *id == generation && *selected == SchemaVersion::V1)
        {
            return Err("unexpected control welcome before disconnect".into());
        }
        std::fs::write(directory.join("control-authenticated"), b"authenticated")?;
        await_release(directory)?;
        drop(control);
        complete(directory, b"closed")?;
        std::thread::sleep(Duration::from_secs(5));
        return Ok(());
    }
    let mut progress = UnixStream::connect(packet.progress_endpoint.as_str())?;
    for (stage, stream, hello) in [
        ("control", &mut control, packet.control),
        ("progress", &mut progress, packet.progress),
    ] {
        stream.set_read_timeout(Some(Duration::from_secs(10)))?;
        stream.set_write_timeout(Some(Duration::from_secs(10)))?;
        if stage == gate_at {
            await_release(directory)?;
        }
        let sent = send(stream, ControlMessage::Hello(hello));
        if stage == gate_at {
            record_send(directory, &sent)?;
        }
        sent?;
        if !matches!(receive(stream)?.payload(), ControlMessage::Welcome { generation: id, selected }
            if *id == generation && *selected == SchemaVersion::V1)
        {
            return Err("unexpected startup welcome".into());
        }
        std::fs::write(
            directory.join(format!("{stage}-authenticated")),
            b"authenticated",
        )?;
    }
    if gate_at == "ready" {
        await_release(directory)?;
    }
    let sent = send(&mut control, ControlMessage::Ready { generation });
    if gate_at == "ready" {
        record_send(directory, &sent)?;
    }
    sent?;
    let cancellation = receive(&mut control)?;
    if let ControlMessage::Cancel {
        generation: id,
        cancellation_id,
    } = cancellation.payload()
        && *id == generation
        && cancellation.cancellation_id() == Some(*cancellation_id)
    {
        let acknowledgement = Envelope::event(
            cancellation.trace_id(),
            ControlMessage::Cancelled {
                generation,
                cancellation_id: *cancellation_id,
            },
        )
        .with_cancellation_id(*cancellation_id);
        control
            .write_all(&encode_control(&acknowledgement, wire_limits())?.encode(wire_limits())?)?;
        std::thread::sleep(Duration::from_secs(5));
        return Ok(());
    }
    Err("expected generation-scoped cancellation".into())
}
