use chrono::{DateTime, Duration, Utc};
use domain::shared::outbox_event::{NextAttemptCalculator, service::NextAttemptStatus};
use rand::Rng;

pub struct BackoffCalculatorConfig {
    pub max_retries: u32,
    pub max_factor: f64,
    pub base_factor: f64,
    pub base_delay_seconds: f64,
    pub jitter_max_millis: i64,
}

pub(crate) struct BackoffNextAttemptCalculator {
    /// The maximum number of retries before giving up
    max_retries: u32,

    /// The maximum factor to cap the exponential backoff
    /// For example, if max_factor is 60, the delay will not exceed base_delay_seconds * 60
    max_factor: f64,

    /// The base factor for exponential backoff
    /// Typically set to 2.0 for doubling the delay each retry
    base_factor: f64,

    /// The base delay in seconds
    base_delay_seconds: f64,

    /// The maximum jitter in milliseconds to add to the delay
    jitter_max_millis: i64,
}

impl BackoffNextAttemptCalculator {
    pub fn new(config: BackoffCalculatorConfig) -> Self {
        Self {
            max_retries: config.max_retries,
            max_factor: config.max_factor,
            base_factor: config.base_factor,
            base_delay_seconds: config.base_delay_seconds,
            jitter_max_millis: config.jitter_max_millis,
        }
    }
}

impl NextAttemptCalculator for BackoffNextAttemptCalculator {
    fn next_attempt_status(
        &self,
        retry_count: u32,
        last_failed_at: DateTime<Utc>,
    ) -> NextAttemptStatus {
        let factor = self
            .base_factor
            .powi(retry_count as i32)
            .min(self.max_factor); // 指数関数的に増大するファクターが大きくなりすぎないように制限
        let delay_seconds = (self.base_delay_seconds * factor) as i64;

        // ジッター: ランダムな揺らぎを追加
        let jitter_millis = rand::rng().random_range(0..self.jitter_max_millis);

        let next_attempt_time = last_failed_at
            + Duration::seconds(delay_seconds)
            + Duration::milliseconds(jitter_millis);

        if retry_count >= self.max_retries {
            NextAttemptStatus::PermanentlyFailed
        } else {
            NextAttemptStatus::RetryAt(next_attempt_time)
        }
    }
}
