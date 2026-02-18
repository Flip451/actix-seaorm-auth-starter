use std::sync::Arc;

use chrono::{DateTime, Utc};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum OutboxEventIdGenerationError {
    #[error("アウトボックスイベントIDの生成に失敗しました: {0}")]
    GenerationFailed(#[source] anyhow::Error),
}

pub trait OutboxEventIdGenerator: Send + Sync {
    fn generate(&self) -> Result<OutboxEventId, OutboxEventIdGenerationError>;
}

pub trait OutboxEventIdGeneratorFactory: Send + Sync {
    fn create_outbox_event_id_generator(&self) -> Arc<dyn OutboxEventIdGenerator>;
}
