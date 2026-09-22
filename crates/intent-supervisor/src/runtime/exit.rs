use super::{
    Entry, HealthPolicy, Instant, ReadBudget, SupervisorError, WorkerFailure, WorkerState,
};

fn may_defer_exit(
    now: Instant,
    exited_at: Instant,
    stop_at: Option<Instant>,
    health: HealthPolicy,
) -> bool {
    now.saturating_duration_since(exited_at) < health.stop_grace
        && stop_at.is_some_and(|stop| {
            now.saturating_duration_since(stop)
                < health.stop_grace.saturating_add(health.terminate_grace)
        })
}

impl Entry {
    pub(super) fn finish_exit(
        &mut self,
        now: Instant,
        exited_at: Instant,
        read_budget: &mut ReadBudget,
    ) -> bool {
        // The OS child is already reaped and authority revoked. Only the existing
        // authenticated control lane remains, with its unchanged byte/frame budgets.
        if self.cancel_sent && !self.cancel_ack && self.control.identity.is_some() {
            match self.read_control(now, read_budget) {
                Ok(false)
                    if !self.cancel_ack
                        && may_defer_exit(now, exited_at, self.stop_at, self.config.health) =>
                {
                    return false;
                }
                Err(SupervisorError::Protocol) => {
                    self.failure.get_or_insert(WorkerFailure::ProtocolViolation);
                }
                Err(_) => {
                    self.failure.get_or_insert(WorkerFailure::OsFailure);
                }
                _ => {}
            }
        }
        for lane in [&mut self.control, &mut self.progress] {
            lane.authenticator = None;
            lane.socket = None;
        }
        self.reaped = true;
        self.state = if self.failure.is_some() {
            WorkerState::Failed
        } else {
            WorkerState::Stopped
        };
        if self.state == WorkerState::Failed {
            self.restart.failed(now);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn exit_drain_never_extends_the_original_stop_window() {
        let start = Instant::now();
        let health = HealthPolicy {
            stop_grace: Duration::from_millis(100),
            terminate_grace: Duration::from_millis(50),
            ..HealthPolicy::default()
        };
        let exit = start + Duration::from_millis(149);
        assert!(may_defer_exit(exit, exit, Some(start), health));
        assert!(!may_defer_exit(
            start + Duration::from_millis(150),
            exit,
            Some(start),
            health
        ));
        assert!(!may_defer_exit(exit, exit, None, health));
        assert!(may_defer_exit(
            start + Duration::from_millis(99),
            start,
            Some(start),
            health
        ));
        assert!(!may_defer_exit(
            start + Duration::from_millis(100),
            start,
            Some(start),
            health
        ));
    }
}
