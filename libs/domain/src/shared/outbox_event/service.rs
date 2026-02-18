use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::shared::outbox_event::OutboxEventId;

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub trait OutboxEventIdGenerator: Send + Sync {
    fn generate(&self) -> OutboxEventId;
}

pub trait OutboxEventIdGeneratorFactory: Send + Sync {
    fn create_outbox_event_id_generator(&self) -> Arc<dyn OutboxEventIdGenerator>;
}
