use chrono::{DateTime, Utc};

pub enum NextAttemptStatus {
    RetryAt(DateTime<Utc>),
    PermanentlyFailed,
}

pub trait NextAttemptCalculator: Send + Sync {
    fn next_attempt_status(
        &self,
        retry_count: u32,
        last_failed_at: DateTime<Utc>,
    ) -> NextAttemptStatus;
}
