use async_trait::async_trait;
use thiserror::Error;

use super::OutboxEvent;

#[derive(Debug, Error)]
pub enum OutboxRepositoryError {
    #[error("イベントの保存に失敗しました: {0}")]
    Persistence(#[source] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum OutboxReconstructionError {
    #[error("無効なイベントステータスです: {0}")]
    InvalidOutboxEventStatus(#[from] strum::ParseError),

    #[error("イベントの再構築に失敗しました: {0}")]
    EventReconstructionError(#[source] anyhow::Error),
}

#[async_trait]
pub trait OutboxRepository: Send + Sync {
    async fn save(&self, event: OutboxEvent) -> Result<(), OutboxRepositoryError>;
    async fn save_all(&self, events: Vec<OutboxEvent>) -> Result<(), OutboxRepositoryError>;
}
