#![forbid(unsafe_code)]

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use intent_contracts::UnixTimestampMicros;
    use intent_ipc::{Envelope, EnvelopeKind};
    use intent_supervisor::{ControlMessage, worker::WorkerClient};
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    let marker = std::env::args()
        .nth(1)
        .ok_or("missing progress backpressure marker")?;
    let exit_heartbeats = std::env::args()
        .nth(2)
        .map(|value| value.parse::<u8>())
        .transpose()?;
    if exit_heartbeats.is_some_and(|count| count > 64) {
        return Err("too many exit heartbeat fixtures".into());
    }
    let acknowledgement = std::env::args().nth(3).unwrap_or_else(|| "ack".to_owned());
    if !matches!(acknowledgement.as_str(), "ack" | "no-ack" | "wrong-id") {
        return Err("invalid acknowledgement fixture mode".into());
    }
    let packet = WorkerClient::read_bootstrap(&mut std::io::stdin().lock())?;
    let scope = packet.scope;
    let mut client = WorkerClient::connect(packet)?;
    let mut heartbeat = Instant::now();
    let mut flood = false;
    let mut blocked = false;

    loop {
        if blocked {
            for _ in 0..16 {
                if !client.progress(1)? {
                    break;
                }
            }
        }
        if let Some(envelope) = client.poll_control()? {
            let kind = envelope.message();
            let trace = envelope.trace_id();
            let cancellation = envelope.cancellation_id();
            let deadline_header = envelope.deadline();
            match envelope.into_payload() {
                ControlMessage::Execute {
                    generation,
                    request_id,
                    scope: work_scope,
                    deadline,
                    ..
                } if generation == client.generation()
                    && work_scope == scope
                    && kind == (EnvelopeKind::Request { request_id })
                    && deadline_header == Some(deadline) =>
                {
                    let wall = UnixTimestampMicros::try_new(i64::try_from(
                        SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros(),
                    )?)?;
                    if wall.get() >= deadline.get() {
                        return Err("expired backpressure trigger".into());
                    }
                    flood = true;
                }
                ControlMessage::Cancel {
                    generation,
                    cancellation_id,
                } if generation == client.generation() && cancellation == Some(cancellation_id) => {
                    std::fs::write(&marker, b"cancel received while progress backpressured")?;
                    for _ in 0..exit_heartbeats.unwrap_or(0) {
                        client.heartbeat()?;
                    }
                    if acknowledgement == "no-ack" {
                        return Ok(());
                    }
                    let cancellation_id = if acknowledgement == "wrong-id" {
                        "018f47f7-5a86-7c00-8000-000000000fff".parse()?
                    } else {
                        cancellation_id
                    };
                    client.send(
                        &Envelope::event(
                            trace,
                            ControlMessage::Cancelled {
                                generation,
                                cancellation_id,
                            },
                        )
                        .with_cancellation_id(cancellation_id),
                    )?;
                    if exit_heartbeats.is_some() {
                        return Ok(());
                    }
                    let release = std::path::Path::new(&marker).with_extension("release");
                    let release_deadline = Instant::now() + Duration::from_secs(3);
                    while !release.try_exists()? {
                        if Instant::now() >= release_deadline {
                            return Err(
                                "parent did not observe cancellation acknowledgement".into()
                            );
                        }
                        client.poll_control()?;
                        std::thread::sleep(Duration::from_millis(1));
                    }
                    return Ok(());
                }
                _ => return Err("invalid blocked-progress fixture message".into()),
            }
        }

        if heartbeat.elapsed() >= Duration::from_millis(10) {
            client.heartbeat()?;
            heartbeat = Instant::now();
        }
        if flood && !blocked {
            for _ in 0..4096 {
                if !client.progress(1)? {
                    std::fs::write(&marker, b"progress transport backpressured")?;
                    blocked = true;
                    break;
                }
            }
        }
        if blocked {
            std::thread::yield_now();
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    let _schema_family = intent_ipc::IPC_SCHEMA_FAMILY;
}
