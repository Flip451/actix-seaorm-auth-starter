use super::OutboxEventStatus;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxEventDomainError {
    #[error(transparent)]
    InvalidStatusTransition(#[from] OutboxStatusTransitionError),
}

#[derive(Debug, Error)]
pub enum OutboxStatusTransitionError {
    #[error(
        "すでに完了済みのイベントのステータス変更を試みました: {from:?} からの遷移は許可されていません"
    )]
    AlreadyCompleted { from: OutboxEventStatus },
    #[error(
        "恒久的に失敗したイベントのステータス変更を試みました: {from:?} からの遷移は許可されていません"
    )]
    AlreadyPermanentlyFailed { from: OutboxEventStatus },
}
