use intent_supervisor::ProcessObservation;
use std::{
    error::Error,
    hint::black_box,
    io,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

type TestResult = Result<(), Box<dyn Error>>;
struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
#[ignore = "started as an owned process by the resource observation test"]
fn resource_observation_child() {
    let mut memory = vec![0_u8; 32 * 1024 * 1024];
    for (index, byte) in memory.iter_mut().enumerate() {
        *byte = index as u8;
    }
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut accumulator = 0_u64;
    while Instant::now() < deadline {
        for byte in &memory {
            accumulator = black_box(accumulator.wrapping_add(u64::from(*byte)));
        }
    }
    black_box(memory);
    black_box(accumulator);
}

fn fixture() -> io::Result<ChildGuard> {
    Ok(ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "resource_observation_child", "--ignored"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()?,
    ))
}

fn pause(deadline: Instant) -> io::Result<()> {
    if Instant::now() >= deadline {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "resource observation test timed out",
        ));
    }
    thread::sleep(Duration::from_millis(10));
    Ok(())
}

#[cfg(any(target_os = "linux", windows))]
#[test]
fn real_child_reports_cpu_memory_and_refuses_samples_after_reaping() -> TestResult {
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut child = fixture()?;
    let first = ProcessObservation::sample(&mut child.0)?;
    let first_cpu = first.user_cpu + first.system_cpu;
    loop {
        let sample = ProcessObservation::sample(&mut child.0)?;
        assert_eq!(sample.process_id, child.0.id());
        assert!(sample.sampled_at >= first.sampled_at);
        assert!(sample.user_cpu >= first.user_cpu);
        assert!(sample.system_cpu >= first.system_cpu);
        #[cfg(target_os = "linux")]
        assert!(
            sample
                .virtual_bytes
                .is_some_and(|bytes| bytes >= sample.resident_bytes)
        );
        #[cfg(windows)]
        assert_eq!(sample.virtual_bytes, None);
        if sample.user_cpu + sample.system_cpu >= first_cpu + Duration::from_millis(20)
            && sample.resident_bytes >= 16 * 1024 * 1024
        {
            eprintln!(
                "observed child {}: user {:?}, system {:?}, resident {} bytes",
                sample.process_id, sample.user_cpu, sample.system_cpu, sample.resident_bytes
            );
            break;
        }
        pause(deadline)?;
    }
    child.0.kill()?;
    while child.0.try_wait()?.is_none() {
        pause(deadline)?;
    }
    for _ in 0..2 {
        let Err(error) = ProcessObservation::sample(&mut child.0) else {
            return Err("reaped child unexpectedly produced a resource sample".into());
        };
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }
    Ok(())
}

#[cfg(not(any(target_os = "linux", windows)))]
#[test]
fn unavailable_accounting_returns_unsupported() -> TestResult {
    let mut child = fixture()?;
    let Err(error) = ProcessObservation::sample(&mut child.0) else {
        return Err("unsupported platform unexpectedly produced a resource sample".into());
    };
    assert_eq!(error.kind(), io::ErrorKind::Unsupported);
    child.0.kill()?;
    let deadline = Instant::now() + Duration::from_secs(15);
    while child.0.try_wait()?.is_none() {
        pause(deadline)?;
    }
    Ok(())
}
