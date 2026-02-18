use thiserror::Error;

use crate::{
    shared::outbox_event::OutboxEventIdGenerationError,
    user::{
        UserIdGenerationError, UserState, UserStateKind,
        value_objects::{email::EmailFormatError, password::PasswordPolicyViolation},
    },
};

use super::EmailVerificationError;

/// Domain-level errors related to `User`.
///
/// Note: `UserDomainError` does **not** implement `PartialEq` on purpose.
/// Some variants (e.g. those wrapping `UserIdGenerationError` or
/// `OutboxEventIdGenerationError`) contain `anyhow::Error`, which does not
/// implement `PartialEq`. As a result, equality comparison on
/// `UserDomainError` values is not supported. If older code relied on
/// comparing `UserDomainError` instances, that usage must be updated.
#[derive(Debug, Error)]
pub enum UserDomainError {
    #[error(transparent)]
    InvalidEmail(#[from] EmailFormatError),

    #[error(transparent)]
    PasswordPolicyViolation(#[from] PasswordPolicyViolation),

    #[error(transparent)]
    AlreadyExists(#[from] UserUniqueConstraintViolation),

    #[error("メールアドレスの検証に失敗しました: {0}")]
    EmailVerificationError(#[from] EmailVerificationError),

    #[error(transparent)]
    ModificationWithInvalidStateError(#[from] ModificationWithInvalidStateError),

    #[error(transparent)]
    StateTransitionError(#[from] UserStateTransitionError),

    #[error(transparent)]
    IdGenerationError(#[from] UserIdGenerationError),

    #[error(transparent)]
    OutboxEventIdGenerationError(#[from] OutboxEventIdGenerationError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserUniqueConstraintViolation {
    #[error("ユーザー名 {duplicated_name} は使用されています")]
    Username { duplicated_name: String },
    #[error("メールアドレス {duplicated_email} はすでに登録されています")]
    Email { duplicated_email: String },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ModificationWithInvalidStateError {
    #[error("以下のユーザー状態でのメールアドレスの変更はできません: {state}")]
    EmailModification { state: UserStateKind },
    #[error("以下のユーザー状態でのユーザー名の変更はできません: {state}")]
    UsernameModification { state: UserStateKind },
}

impl ModificationWithInvalidStateError {
    pub fn message_for_client(&self) -> &'static str {
        match self {
            ModificationWithInvalidStateError::EmailModification { state: _ } => {
                "該当ユーザーはメールアドレスの変更ができない状態です"
            }
            ModificationWithInvalidStateError::UsernameModification { state: _ } => {
                "該当ユーザーはユーザー名の変更ができない状態です"
            }
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserStateTransitionError {
    #[error("ユーザーは既に退会しています: {to:?}への遷移は許可されていません")]
    AlreadyDeactivated { to: UserStateKind },

    #[error("ユーザーは既に停止されています: {to:?}への遷移は許可されていません")]
    AlreadySuspended { to: UserStateKind },

    #[error("ユーザーのメールアドレスが未検証です: {from:?}からの遷移は許可されていません")]
    NotVerified { from: UserState },

    #[error("指定のユーザーは停止されていません： {from:?}からの遷移は許可されていません")]
    NotSuspended { from: UserState },
}

#[derive(Debug, Error, PartialEq)]
pub enum UserReconstructionError {
    #[error("不正な形式のメールアドレスが保存されています: {0}")]
    InvalidEmail(#[from] EmailFormatError),
    #[error("不正な形式のステータスが保存されています: {invalid_status}")]
    InvalidStatus { invalid_status: String },
    #[error("不正な形式のロールが保存されています: {invalid_role}")]
    InvalidRole { invalid_role: String },
}
