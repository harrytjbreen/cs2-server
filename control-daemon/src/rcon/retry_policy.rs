use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub initial_delay: Duration,
    pub backoff_multiplier: f64,
    pub max_delay: Duration,
}

#[allow(dead_code)]
impl RetryPolicy {
    pub fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(250),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(5),
        }
    }

    #[allow(dead_code)]
    pub fn delay_for_attempt(&self, attempt: usize) -> Duration {
        if attempt <= 1 {
            return Duration::ZERO;
        }

        let exp = (attempt - 2) as f64;
        let delay_ms = self.initial_delay.as_millis() as f64 * self.backoff_multiplier.powf(exp);

        let delay = Duration::from_millis(delay_ms as u64);
        delay.min(self.max_delay)
    }
}
