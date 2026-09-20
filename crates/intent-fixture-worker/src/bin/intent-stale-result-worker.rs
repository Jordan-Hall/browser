#![forbid(unsafe_code)]

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use intent_contracts::{TraceId, UnixTimestampMicros};
    use intent_ipc::{Envelope, EnvelopeKind};
    use intent_supervisor::{ControlMessage, worker::WorkerClient};
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    let marker = std::env::args()
        .nth(1)
        .ok_or("missing replacement-ready marker")?;
    let packet = WorkerClient::read_bootstrap(&mut std::io::stdin().lock())?;
    let scope = packet.scope;
    let mut client = WorkerClient::connect(packet)?;
    let mut heartbeat = Instant::now();
    let mut late_response = None;

    loop {
        if let Some(envelope) = client.poll_control()? {
            let kind = envelope.message();
            let trace = envelope.trace_id();
            let cancellation = envelope.cancellation_id();
            let deadline_header = envelope.deadline();
            match envelope.into_payload() {
                ControlMessage::Execute {
                    generation,
                    request_id,
                    sequence,
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
                    if wall >= deadline {
                        return Err("expired stale-result request".into());
                    }
                    let mut response = Envelope::response(
                        trace,
                        request_id,
                        ControlMessage::Observed {
                            generation,
                            request_id,
                            sequence,
                        },
                    )
                    .with_deadline(deadline);
                    if let Some(cancellation) = cancellation {
                        response = response.with_cancellation_id(cancellation);
                    }
                    late_response = Some(response);
                }
                ControlMessage::Cancel {
                    generation,
                    cancellation_id,
                } if generation == client.generation() && cancellation == Some(cancellation_id) => {
                    let marker_deadline = Instant::now() + Duration::from_secs(5);
                    loop {
                        match std::fs::read(&marker) {
                            Ok(contents) if contents == b"replacement ready" => break,
                            Ok(_) => {}
                            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                            Err(error) => return Err(error.into()),
                        }
                        if Instant::now() >= marker_deadline {
                            return Err("replacement-ready marker timed out".into());
                        }
                        std::thread::sleep(Duration::from_millis(1));
                    }
                    client.send(&late_response.take().ok_or("missing withheld result")?)?;
                    client.send(
                        &Envelope::event(
                            TraceId::from_uuid(Uuid::new_v4()),
                            ControlMessage::Cancelled {
                                generation,
                                cancellation_id,
                            },
                        )
                        .with_cancellation_id(cancellation_id),
                    )?;
                    std::thread::sleep(Duration::from_millis(15));
                    return Ok(());
                }
                _ => return Err("invalid stale-result fixture message".into()),
            }
        }
        if heartbeat.elapsed() >= Duration::from_millis(10) {
            client.heartbeat()?;
            heartbeat = Instant::now();
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    let _schema_family = intent_ipc::IPC_SCHEMA_FAMILY;
}
