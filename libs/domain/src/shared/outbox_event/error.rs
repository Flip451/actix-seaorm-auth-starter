use super::OutboxEventStatus;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxEventDomainError {
    #[error(transparent)]
    InvalidStatusTransition(#[from] OutboxStatusTransitionError),
}

#[derive(Debug, Error)]
pub enum OutboxStatusTransitionError {
    #[error("すでに完了済みのイベントです: {from:?} からの遷移は許可されていません")]
    AlreadyCompleted { from: OutboxEventStatus },
}
