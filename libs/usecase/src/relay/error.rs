use domain::{
    shared::outbox_event::{
        OutboxEventDomainError, OutboxEventReconstructionError, OutboxRepositoryError,
    },
    transaction::IntoTxError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelayError {
    #[error("イベントの処理に失敗しました: {0}")]
    ProcessingError(#[source] anyhow::Error),

    #[error("イベントのステータス遷移が無効です: {message}")]
    InvalidEventStatusTransition { message: String },

    #[error("サーバー内部でエラーが発生しました: {0}")]
    Internal(#[source] anyhow::Error),
}

impl IntoTxError for RelayError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        RelayError::Internal(error.into())
    }
}

impl From<OutboxRepositoryError> for RelayError {
    fn from(error: OutboxRepositoryError) -> Self {
        match error {
            OutboxRepositoryError::DataStoreError(source) => RelayError::Internal(source),
            OutboxRepositoryError::DomainError(outbox_event_domain_error) => {
                outbox_event_domain_error.into()
            }
            OutboxRepositoryError::ReconstructionError(outbox_event_reconstruction_error) => {
                outbox_event_reconstruction_error.into()
            }
            OutboxRepositoryError::DomainEventSerializationError(
                domain_event_serialization_error,
            ) => RelayError::Internal(domain_event_serialization_error.into()),
        }
    }
}

impl From<OutboxEventDomainError> for RelayError {
    fn from(error: OutboxEventDomainError) -> Self {
        match error {
            OutboxEventDomainError::InvalidStatusTransition(outbox_status_transition_error) => {
                RelayError::InvalidEventStatusTransition {
                    message: outbox_status_transition_error
                        .message_for_client()
                        .to_string(),
                }
            }
        }
    }
}

impl From<OutboxEventReconstructionError> for RelayError {
    fn from(error: OutboxEventReconstructionError) -> Self {
        // 再構築エラーは通常発生しないはずなので、ここでは内部エラーとして扱う
        RelayError::Internal(error.into())
    }
}
