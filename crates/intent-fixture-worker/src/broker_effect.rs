//! Independent fixture effect ledger: every invocation appends, so accidental resend is visible.
use intent_broker::WorkerInvocation;
use intent_contracts::{CapabilityId, RequestId, UnixTimestampMicros, WorkerInstanceId};
use intent_supervisor::WorkerScope;
use std::{error::Error, fs::OpenOptions, io::Write, os::unix::fs::OpenOptionsExt};

#[allow(clippy::too_many_arguments)]
pub fn execute(
    mode: &str,
    args: &[String],
    input: &str,
    generation: WorkerInstanceId,
    request: RequestId,
    scope: WorkerScope,
    capability: CapabilityId,
    now: UnixTimestampMicros,
) -> Result<(), Box<dyn Error>> {
    let invocation = WorkerInvocation::decode(input, generation, request, scope, capability, now)?;
    if invocation.destination() != "fixture://external-ledger"
        || invocation.message_kind() != "append"
    {
        return Err("fixture destination or message kind differs from configured endpoint".into());
    }
    if mode != "broker-ack-only" {
        let path = args.get(1).ok_or("fixture external ledger path missing")?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .mode(0o600)
            .open(path)?;
        let record = serde_json::json!({"binding": invocation.binding(), "payload": invocation.payload(), "epoch": invocation.runtime_epoch()});
        file.write_all(serde_json::to_string(&record)?.as_bytes())?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }
    if mode == "broker-effect-lost-ack" {
        std::process::exit(24);
    }
    Ok(())
}
