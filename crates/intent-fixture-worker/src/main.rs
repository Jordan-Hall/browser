#![forbid(unsafe_code)]

#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod broker_effect;

#[cfg(target_os = "linux")]
mod progress_pressure;

#[cfg(target_os = "linux")]
mod startup_gate;

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use intent_contracts::{TraceId, UnixTimestampMicros};
    use intent_ipc::{Envelope, EnvelopeKind};
    use intent_supervisor::{BootstrapPacket, ControlMessage, wire_limits, worker::WorkerClient};
    use std::{
        fs::File,
        io::Write,
        process::{Command, Stdio},
        time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    };
    use uuid::Uuid;

    let args: Vec<_> = std::env::args().skip(1).take(17).collect();
    let mode = args.first().map(String::as_str).unwrap_or("normal");
    let mut progress_pressure = if mode == "progress-pressure" {
        Some(progress_pressure::ProgressPressure::new(
            args.get(1).ok_or("missing pressure start path")?,
            args.get(2).ok_or("missing pressure report path")?,
        ))
    } else {
        None
    };
    let cancellation_release = match mode {
        "progress-pressure" => args
            .get(2)
            .map(|path| std::path::Path::new(path).with_extension("release")),
        "flood" => args.get(1).map(std::path::PathBuf::from),
        _ => None,
    };
    let packet = WorkerClient::read_bootstrap(&mut std::io::stdin().lock())?;
    if mode == "startup-gate" {
        return startup_gate::run(
            packet,
            args.get(1).ok_or("missing startup gate directory")?,
            args.get(2).map(String::as_str).unwrap_or("ready"),
        );
    }
    if mode == "inherited-fd-probe" {
        let expected_absent = args.get(1).ok_or("missing descriptor target")?;
        for descriptor in std::fs::read_dir("/proc/self/fd")?.take(4097) {
            let path = descriptor?.path();
            if std::fs::read_link(path)
                .is_ok_and(|target| target.as_os_str() == expected_absent.as_str())
            {
                return Err("worker inherited a supervisor descriptor".into());
            }
        }
    }
    if mode == "silent" {
        loop {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    if mode == "wrong-peer" {
        let bytes = intent_ipc::encode_control(
            &Envelope::event(TraceId::from_uuid(Uuid::new_v4()), packet),
            wire_limits(),
        )?
        .encode(wire_limits())?;
        let mut child = Command::new("/proc/self/exe")
            .arg("normal")
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        child
            .stdin
            .take()
            .ok_or("no forwarding pipe")?
            .write_all(&bytes)?;
        let _ = child.wait()?;
        loop {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    let packet = if matches!(mode, "wrong-token" | "wrong-role" | "wrong-generation") {
        let mut value = serde_json::to_value(&packet)?;
        for lane in ["control", "progress"] {
            match mode {
                "wrong-token" => {
                    value[lane]["identity"]["bootstrap_token"] = serde_json::json!(vec![0_u8; 32])
                }
                "wrong-role" => {
                    value[lane]["identity"]["role"] = serde_json::json!("policy_broker")
                }
                _ => {
                    value[lane]["identity"]["instance_id"] =
                        serde_json::json!("00000000-0000-4000-8000-000000000001")
                }
            }
        }
        serde_json::from_value(value)?
    } else {
        packet
    };
    let saved = if mode == "replay" {
        Some(serde_json::from_value::<BootstrapPacket>(
            serde_json::to_value(&packet)?,
        )?)
    } else {
        None
    };
    let scope = packet.scope;
    let mut client = WorkerClient::connect(packet)?;
    if let Some(saved) = saved {
        client.send(&Envelope::event(
            TraceId::from_uuid(Uuid::new_v4()),
            ControlMessage::Hello(saved.control),
        ))?;
    }
    if mode == "exit" {
        std::process::exit(23);
    }
    if mode == "cpu-hog" {
        let mut x = 1_u64;
        loop {
            x = std::hint::black_box(x.wrapping_mul(6364136223846793005).wrapping_add(1));
        }
    }
    if mode == "silent-after-ready" {
        loop {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    let mut descendant = if mode == "descendant" {
        let child = Command::new("/bin/sleep")
            .arg("60")
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        if let Some(path) = args.get(1) {
            std::fs::write(path, child.id().to_string())?;
        }
        Some(child)
    } else {
        None
    };
    let mut heartbeat = Instant::now();
    let mut last_work = 0;
    let active_since = Instant::now();
    let mut late_response = None;
    if mode == "environment-probe" && std::env::vars_os().next().is_some() {
        return Err("worker inherited environment".into());
    }
    loop {
        if mode == "exit-delayed" && active_since.elapsed() >= Duration::from_millis(150) {
            std::process::exit(23);
        }
        if let Some(envelope) = client.poll_control()? {
            let kind = envelope.message();
            let trace = envelope.trace_id();
            let cancellation = envelope.cancellation_id();
            let deadline_header = envelope.deadline();
            match envelope.into_payload() {
                ControlMessage::Cancel {
                    generation,
                    cancellation_id,
                } if generation == client.generation() && cancellation == Some(cancellation_id) => {
                    if mode == "ignore-cancel" {
                        continue;
                    }
                    if let Some(response) = late_response.take() {
                        client.send(&response)?;
                    }
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
                    if let Some(child) = descendant.as_mut() {
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                    if let Some(release) = &cancellation_release {
                        let deadline = Instant::now() + Duration::from_secs(3);
                        while !release.try_exists()? {
                            let _ = client.poll_control()?;
                            if Instant::now() >= deadline {
                                return Err(
                                    "parent did not observe cancellation acknowledgement".into()
                                );
                            }
                            std::thread::sleep(Duration::from_millis(1));
                        }
                        return Ok(());
                    }
                    // Leave time for the supervisor to consume the acknowledgement before exit.
                    std::thread::sleep(Duration::from_millis(15));
                    return Ok(());
                }
                ControlMessage::Yield { generation } if generation == client.generation() => {
                    client.send(&Envelope::event(
                        trace,
                        ControlMessage::Yielded { generation },
                    ))?;
                }
                ControlMessage::Execute {
                    generation,
                    request_id,
                    sequence,
                    scope: work_scope,
                    deadline,
                    input,
                    capability,
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
                        return Err("expired worker request".into());
                    }
                    #[cfg(all(target_os = "linux", target_env = "gnu"))]
                    if mode.starts_with("broker-") {
                        broker_effect::execute(
                            mode,
                            &args,
                            input.as_str(),
                            generation,
                            request_id,
                            work_scope,
                            capability,
                            wall,
                        )?;
                    }
                    last_work = sequence;
                    if mode == "stalled-work" {
                        continue;
                    }
                    if mode == "memory-probe" {
                        let mut bytes = Vec::<u8>::new();
                        if bytes.try_reserve_exact(2 * 1024 * 1024 * 1024).is_ok() {
                            return Err(
                                "address-space limit did not reject excessive allocation".into()
                            );
                        }
                    }
                    if mode == "fd-probe" {
                        let mut files = Vec::new();
                        let mut denied = false;
                        for _ in 0..1024 {
                            match File::open("/dev/null") {
                                Ok(file) => files.push(file),
                                Err(error) if error.raw_os_error() == Some(24) => {
                                    denied = true;
                                    break;
                                }
                                Err(error) => return Err(error.into()),
                            }
                        }
                        if !denied {
                            return Err("descriptor limit was not enforced".into());
                        }
                    }
                    let mut response = Envelope::response(
                        trace,
                        request_id,
                        ControlMessage::Observed {
                            generation,
                            request_id,
                            sequence: if mode == "bad-response" {
                                sequence + 1
                            } else {
                                sequence
                            },
                        },
                    )
                    .with_deadline(deadline);
                    if let Some(cancellation) = cancellation {
                        response = response.with_cancellation_id(cancellation);
                    }
                    if mode == "late-result" {
                        late_response = Some(response);
                    } else {
                        client.send(&response)?;
                    }
                }
                _ => return Err("invalid fixture control message".into()),
            }
        }
        if heartbeat.elapsed() >= Duration::from_millis(10) {
            client.heartbeat()?;
            heartbeat = Instant::now();
        }
        if mode == "flood" {
            for _ in 0..128 {
                let _ = client.progress(last_work)?;
            }
            std::io::stdout().write_all(&[b'x'; 4096])?;
            std::io::stderr().write_all(&[b'y'; 4096])?;
        }
        if let Some(pressure) = &mut progress_pressure {
            pressure.poll(&mut client)?;
        }
        if mode != "flood" {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    let _schema_family = intent_ipc::IPC_SCHEMA_FAMILY;
}
