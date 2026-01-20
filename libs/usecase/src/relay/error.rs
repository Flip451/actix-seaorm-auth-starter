use domain::{
    shared::outbox_event::{
        OutboxEventDomainError, OutboxReconstructionError, OutboxRepositoryError,
    },
    transaction::IntoTxError,
    user::{UserId, UserRepositoryError},
};
use thiserror::Error;

use crate::shared::email_service::EmailServiceError;

#[derive(Debug, Error)]
pub enum RelayError {
    #[error(transparent)]
    DomainError(#[from] OutboxEventDomainError),

    #[error("ユーザーが見つかりません: {0}")]
    UserNotFound(UserId),

    #[error(transparent)]
    OutboxRepositoryError(#[from] OutboxRepositoryError),

    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),

    #[error(transparent)]
    EmailServiceError(#[from] EmailServiceError),

    #[error(transparent)]
    ReconstructionError(#[from] OutboxReconstructionError),

    #[error("イベントの処理に失敗しました: {0}")]
    ProcessingError(#[source] anyhow::Error),

    #[error("トランザクションエラー: {0}")]
    TxError(#[source] anyhow::Error),
}

impl IntoTxError for RelayError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        RelayError::TxError(error.into())
    }
}
