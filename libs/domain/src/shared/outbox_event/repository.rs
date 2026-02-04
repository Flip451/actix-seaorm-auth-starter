use async_trait::async_trait;
use thiserror::Error;

use crate::shared::{
    outbox_event::{OutboxEventDomainError, error::OutboxEventReconstructionError},
    service::clock::Clock,
};

use super::OutboxEvent;

#[derive(Debug, Error)]
pub enum OutboxRepositoryError {
    #[error(transparent)]
    DomainError(#[from] OutboxEventDomainError),

    #[error("ドメインイベントのシリアライズに失敗しました: {0}")]
    DomainEventSerializationError(#[from] serde_json::Error),

    #[error(transparent)]
    ReconstructionError(#[from] OutboxEventReconstructionError),

    #[error("データストアのエラー: {0}")]
    DataStoreError(#[source] anyhow::Error),
}

#[async_trait]
pub trait OutboxRepository: Send + Sync {
    async fn save(&self, event: OutboxEvent) -> Result<(), OutboxRepositoryError>;
    async fn save_all(&self, events: Vec<OutboxEvent>) -> Result<(), OutboxRepositoryError>;

    async fn lock_pending_events(
        &self,
        limit: u64,
        clock: &dyn Clock,
    ) -> Result<Vec<OutboxEvent>, OutboxRepositoryError>;
}
