use crate::SupervisorError;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct RestartPolicy {
    max_restarts: u8,
    initial_backoff: Duration,
    max_backoff: Duration,
}
impl RestartPolicy {
    pub const fn never() -> Self {
        Self {
            max_restarts: 0,
            initial_backoff: Duration::ZERO,
            max_backoff: Duration::ZERO,
        }
    }
    pub fn bounded(
        max_restarts: u8,
        initial_backoff: Duration,
        max_backoff: Duration,
    ) -> Result<Self, SupervisorError> {
        if max_restarts > 8
            || initial_backoff.is_zero()
            || max_backoff < initial_backoff
            || max_backoff > Duration::from_secs(60)
        {
            return Err(SupervisorError::InvalidConfiguration(
                "restart policy outside supported bounds",
            ));
        }
        Ok(Self {
            max_restarts,
            initial_backoff,
            max_backoff,
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestartDecision {
    BackoffUntil(Instant),
    Eligible,
    CircuitOpen,
    LifetimeExpired,
}
#[derive(Clone, Debug)]
pub(crate) struct RestartBudget {
    policy: RestartPolicy,
    consumed: u8,
    failed_at: Option<Instant>,
    lifetime_end: Instant,
}
impl RestartBudget {
    pub(crate) fn new(policy: RestartPolicy, lifetime_end: Instant) -> Self {
        Self {
            policy,
            consumed: 0,
            failed_at: None,
            lifetime_end,
        }
    }
    pub(crate) fn failed(&mut self, now: Instant) {
        if self.failed_at.is_none() {
            self.failed_at = Some(now);
        }
    }
    pub(crate) fn decision(&self, now: Instant) -> RestartDecision {
        if now >= self.lifetime_end {
            return RestartDecision::LifetimeExpired;
        }
        if self.consumed >= self.policy.max_restarts {
            return RestartDecision::CircuitOpen;
        }
        match self.failed_at {
            None => RestartDecision::CircuitOpen,
            Some(failed) => {
                let delay = self
                    .policy
                    .initial_backoff
                    .saturating_mul(1_u32 << self.consumed)
                    .min(self.policy.max_backoff);
                let at = failed + delay;
                if now < at {
                    RestartDecision::BackoffUntil(at)
                } else {
                    RestartDecision::Eligible
                }
            }
        }
    }
    pub(crate) fn consume(&mut self, now: Instant) -> Result<(), SupervisorError> {
        if self.decision(now) != RestartDecision::Eligible {
            return Err(SupervisorError::RestartDenied);
        }
        self.consumed += 1;
        self.failed_at = None;
        Ok(())
    }
    pub(crate) const fn lifetime_end(&self) -> Instant {
        self.lifetime_end
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backoff_circuit_and_original_lifetime_survive_generations() -> Result<(), SupervisorError> {
        let now = Instant::now();
        let policy =
            RestartPolicy::bounded(2, Duration::from_millis(10), Duration::from_millis(100))?;
        let mut budget = RestartBudget::new(policy, now + Duration::from_secs(1));
        budget.failed(now);
        assert!(matches!(
            budget.decision(now),
            RestartDecision::BackoffUntil(_)
        ));
        assert!(budget.consume(now).is_err());
        budget.consume(now + Duration::from_millis(10))?;
        budget.failed(now + Duration::from_millis(20));
        assert!(budget.consume(now + Duration::from_millis(30)).is_err());
        budget.consume(now + Duration::from_millis(40))?;
        budget.failed(now + Duration::from_millis(50));
        assert_eq!(
            budget.decision(now + Duration::from_millis(100)),
            RestartDecision::CircuitOpen
        );
        assert_eq!(
            budget.decision(now + Duration::from_secs(1)),
            RestartDecision::LifetimeExpired
        );
        Ok(())
    }
}
