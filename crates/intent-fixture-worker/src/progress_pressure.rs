use intent_supervisor::worker::WorkerClient;
use std::{error::Error, path::PathBuf};

pub(crate) struct ProgressPressure {
    start: PathBuf,
    report: PathBuf,
    state: State,
}

enum State {
    Waiting,
    Filling { admitted: u64, blocked: u8 },
    Saturated,
}

impl ProgressPressure {
    pub(crate) fn new(start: &str, report: &str) -> Self {
        Self {
            start: start.into(),
            report: report.into(),
            state: State::Waiting,
        }
    }

    pub(crate) fn poll(&mut self, client: &mut WorkerClient) -> Result<(), Box<dyn Error>> {
        if matches!(self.state, State::Waiting) && self.start.try_exists()? {
            self.state = State::Filling {
                admitted: 0,
                blocked: 0,
            };
        }
        let State::Filling { admitted, blocked } = &mut self.state else {
            return Ok(());
        };
        for _ in 0..128 {
            if client.progress(0)? {
                *admitted += 1;
                *blocked = 0;
                if *admitted > 65_536 {
                    return Err("progress socket did not reach bounded fixture pressure".into());
                }
            } else {
                *blocked += 1;
                if *blocked == 8 {
                    let pending = self.report.with_extension("pending");
                    std::fs::write(&pending, admitted.to_string())?;
                    std::fs::rename(pending, &self.report)?;
                    self.state = State::Saturated;
                }
                break;
            }
        }
        Ok(())
    }
}
