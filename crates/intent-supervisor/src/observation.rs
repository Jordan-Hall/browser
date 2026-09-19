use std::{
    io,
    process::Child,
    time::{Duration, Instant},
};

/// A best-effort sample of one process, not its descendants or enforced resource limits.
///
/// CPU durations include all threads. Resident bytes mean RSS on Linux and working-set size on
/// Windows; shared pages may be counted in multiple processes. These values are not atomic.
#[derive(Clone, Debug)]
pub struct ProcessObservation {
    pub sampled_at: Instant,
    pub process_id: u32,
    pub user_cpu: Duration,
    pub system_cpu: Duration,
    pub resident_bytes: u64,
    /// Linux virtual address-space size. Windows returns None, not private commit or pagefile usage.
    pub virtual_bytes: Option<u64>,
}

impl ProcessObservation {
    /// Sample a running owned child on Linux or Windows; other platforms return Unsupported.
    ///
    /// An exited child returns NotFound and may be reaped by this call. Callers must not reap the
    /// child externally. This prevents a later Linux PID reuse from being sampled as this child.
    /// Exit during sampling can still produce a final sample or an OS error.
    pub fn sample(child: &mut Child) -> io::Result<Self> {
        sample(child)
    }
}

#[cfg(any(target_os = "linux", windows))]
fn sample(child: &mut Child) -> io::Result<ProcessObservation> {
    if child.try_wait()?.is_some() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "child has exited"));
    }
    #[cfg(target_os = "linux")]
    {
        observe_unreaped(child.id())
    }
    #[cfg(windows)]
    {
        windows_observation(child)
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn sample(_child: &mut Child) -> io::Result<ProcessObservation> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "process observation requires Linux or Windows",
    ))
}

#[cfg(any(target_os = "linux", windows, test))]
fn duration_from_ticks(ticks: u64, ticks_per_second: u64) -> Duration {
    Duration::new(
        ticks / ticks_per_second,
        ((u128::from(ticks % ticks_per_second) * 1_000_000_000) / u128::from(ticks_per_second))
            as u32,
    )
}

#[cfg(target_os = "linux")]
fn invalid_stat() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "invalid process accounting data",
    )
}

/// The Linux supervisor owns an unreaped leader until process-group cleanup. Sampling must not
/// reap that leader, because doing so would allow its PID/PGID to be reused before cleanup.
#[cfg(target_os = "linux")]
pub(crate) fn observe_unreaped(pid: u32) -> io::Result<ProcessObservation> {
    use nix::unistd::{SysconfVar, sysconf};
    use std::{fs::File, io::Read};

    let mut bytes = Vec::new();
    File::open(format!("/proc/{pid}/stat"))?
        .take(4097)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 4096 {
        return Err(invalid_stat());
    }
    let positive = |variable| -> io::Result<u64> {
        sysconf(variable)
            .map_err(io::Error::from)?
            .and_then(|value| u64::try_from(value).ok())
            .filter(|value| *value > 0)
            .ok_or_else(invalid_stat)
    };
    parse_linux_stat(
        pid,
        &bytes,
        positive(SysconfVar::CLK_TCK)?,
        positive(SysconfVar::PAGE_SIZE)?,
    )
}

#[cfg(target_os = "linux")]
fn parse_linux_stat(
    pid: u32,
    bytes: &[u8],
    ticks_per_second: u64,
    page_size: u64,
) -> io::Result<ProcessObservation> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid_stat())?;
    let (_, rest) = text.rsplit_once(") ").ok_or_else(invalid_stat)?;
    let fields: Vec<_> = rest.split_whitespace().take(22).collect();
    let value = |index: usize| -> io::Result<u64> {
        fields
            .get(index)
            .and_then(|value| value.parse().ok())
            .ok_or_else(invalid_stat)
    };
    Ok(ProcessObservation {
        sampled_at: Instant::now(),
        process_id: pid,
        user_cpu: duration_from_ticks(value(11)?, ticks_per_second),
        system_cpu: duration_from_ticks(value(12)?, ticks_per_second),
        resident_bytes: value(21)?.checked_mul(page_size).ok_or_else(invalid_stat)?,
        virtual_bytes: Some(value(20)?),
    })
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn windows_observation(child: &Child) -> io::Result<ProcessObservation> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::{
        Foundation::FILETIME,
        System::{
            ProcessStatus::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
            Threading::GetProcessTimes,
        },
    };

    let mut created = FILETIME::default();
    let mut exited = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();
    // SAFETY: Child owns the process handle for this borrow. Every output points to distinct,
    // initialized FILETIME storage, and GetProcessTimes retains none of the pointers.
    if unsafe {
        GetProcessTimes(
            child.as_raw_handle(),
            &mut created,
            &mut exited,
            &mut kernel,
            &mut user,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    let size = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    let mut memory = PROCESS_MEMORY_COUNTERS {
        cb: size,
        ..Default::default()
    };
    // SAFETY: Child keeps its handle alive and memory is writable storage of exactly size bytes.
    // K32GetProcessMemoryInfo retains neither the handle nor the output pointer.
    if unsafe { K32GetProcessMemoryInfo(child.as_raw_handle(), &mut memory, size) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let cpu = |time: FILETIME| {
        duration_from_ticks(
            (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime),
            10_000_000,
        )
    };
    Ok(ProcessObservation {
        sampled_at: Instant::now(),
        process_id: child.id(),
        user_cpu: cpu(user),
        system_cpu: cpu(kernel),
        resident_bytes: memory.WorkingSetSize as u64,
        virtual_bytes: None,
    })
}

#[cfg(test)]
mod tests {
    use super::duration_from_ticks;
    use std::time::Duration;

    #[test]
    fn cpu_time_conversion_preserves_units_without_multiplication_overflow() {
        assert_eq!(duration_from_ticks(150, 100), Duration::from_millis(1500));
        assert_eq!(
            duration_from_ticks(1, 10_000_000),
            Duration::from_nanos(100)
        );
        assert_eq!(
            duration_from_ticks(u64::MAX, 10_000_000),
            Duration::new(1_844_674_407_370, 955_161_500)
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_stat_handles_parentheses_and_rejects_invalid_memory()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut fields = vec!["0"; 22];
        fields[0] = "R";
        fields[11] = "150";
        fields[12] = "25";
        fields[20] = "1048576";
        fields[21] = "16";
        let stat =
            |fields: &[&str]| format!("123 (worker (name) with spaces) {}", fields.join(" "));
        let sample = super::parse_linux_stat(123, stat(&fields).as_bytes(), 100, 4096)?;
        assert_eq!(sample.user_cpu, Duration::from_millis(1500));
        assert_eq!(sample.system_cpu, Duration::from_millis(250));
        assert_eq!(sample.resident_bytes, 65536);
        assert_eq!(sample.virtual_bytes, Some(1048576));
        for invalid in ["-1", "18446744073709551615", "bad"] {
            fields[21] = invalid;
            assert!(super::parse_linux_stat(123, stat(&fields).as_bytes(), 100, 4096).is_err());
        }
        assert!(super::parse_linux_stat(123, b"123 (truncated) R", 100, 4096).is_err());
        Ok(())
    }
}
