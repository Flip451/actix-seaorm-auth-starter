use crate::shared::outbox_event::entity::OutboxEventStatusKind;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxEventDomainError {
    #[error(transparent)]
    InvalidStatusTransition(#[from] OutboxStatusTransitionError),
}

#[derive(Debug, Error)]
pub enum OutboxEventReconstructionError {
    #[error("ドメインイベントの再構築に失敗しました: {0}")]
    DomainEventReconstructionError(#[source] anyhow::Error),

    #[error("トレースIDのパースに失敗しました: {0}")]
    ParseTraceIdError(#[from] std::num::ParseIntError),

    #[error("Outboxイベントのステータス文字列が不正です: {invalid_status}")]
    InvalidStatus { invalid_status: String },

    #[error("Failed にもかかわらず next_attempt_at が None です")]
    FailedButNoNextAttemptAt,
    #[error("Failed にもかかわらず last_attempted_at が None です")]
    FailedButNoLastAttemptedAt,
    #[error("Failed にもかかわらず processed_at が None です")]
    FailedButNoProcessedAt,

    #[error("Completed にもかかわらず last_attempted_at が None です")]
    CompletedButNoLastAttemptedAt,
    #[error("Completed にもかかわらず processed_at が None です")]
    CompletedButNoProcessedAt,

    #[error("PermanentlyFailed にもかかわらず last_attempted_at が None です")]
    PermanentlyFailedButNoLastAttemptedAt,
    #[error("PermanentlyFailed にもかかわらず processed_at が None です")]
    PermanentlyFailedButNoProcessedAt,
}

#[derive(Debug, Error)]
pub enum OutboxStatusTransitionError {
    #[error(
        "すでに完了済みのイベントのステータス変更を試みました: 以下のステータスへの遷移は許可されていません: {to:?}"
    )]
    AlreadyCompleted { to: OutboxEventStatusKind },
    #[error(
        "恒久的に失敗したイベントのステータス変更を試みました: 以下のステータスへの遷移は許可されていません: {to:?}"
    )]
    AlreadyPermanentlyFailed { to: OutboxEventStatusKind },
}

impl OutboxStatusTransitionError {
    pub fn message_for_client(&self) -> &str {
        match self {
            OutboxStatusTransitionError::AlreadyCompleted { .. } => {
                "すでに完了済みのイベントのステータス変更は許可されていません"
            }
            OutboxStatusTransitionError::AlreadyPermanentlyFailed { .. } => {
                "恒久的に失敗したイベントのステータス変更は許可されていません"
            }
        }
    }
}
