use domain::{
    auth::policy::AuthorizationError,
    shared::outbox_event::OutboxRepositoryError,
    transaction::IntoTxError,
    user::{
        EmailVerificationError, UserDomainError, UserRepositoryError, UserStateTransitionError,
    },
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("バリデーション失敗: {0}")]
    InvalidInput(String),

    #[error("ユーザーが見つかりません")]
    NotFound,

    #[error("ユーザー名 '{0}' は既に存在します")]
    UsernameAlreadyExists(String),

    #[error("メールアドレス '{0}' は既に存在します")]
    EmailAlreadyExists(String),

    #[error(transparent)]
    EmailVerificationError(#[from] EmailVerificationError),

    #[error(transparent)]
    StateTransitionError(#[from] UserStateTransitionError),

    #[error(transparent)]
    AuthorizationError(#[from] AuthorizationError),

    #[error("トランザクションエラー: {0}")]
    TxError(#[source] anyhow::Error),

    #[error("永続化エラー: {0}")]
    PersistenceError(#[source] anyhow::Error),

    #[error("想定外のエラーが発生しました: {0}")]
    UnexpectedError(#[source] anyhow::Error),
}

impl IntoTxError for UserError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        UserError::TxError(error.into())
    }
}

impl From<UserRepositoryError> for UserError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::DomainError(domain_error) => UserError::from(domain_error),
            UserRepositoryError::ReconstructionError(user_reconstruction_error) => {
                UserError::PersistenceError(user_reconstruction_error.into())
            }
            UserRepositoryError::Persistence(source) => UserError::PersistenceError(source),
        }
    }
}

impl From<OutboxRepositoryError> for UserError {
    fn from(error: OutboxRepositoryError) -> Self {
        match error {
            OutboxRepositoryError::Persistence(source) => UserError::PersistenceError(source),
        }
    }
}

impl From<UserDomainError> for UserError {
    fn from(error: UserDomainError) -> Self {
        match error {
            UserDomainError::InvalidEmail(invalid_email) => UserError::InvalidInput(invalid_email),
            UserDomainError::PasswordTooShort => UserError::UnexpectedError(error.into()),
            UserDomainError::AlreadyExists(user_unique_constraint) => {
                match user_unique_constraint {
                    domain::user::UserUniqueConstraint::Username(username) => {
                        UserError::UsernameAlreadyExists(username)
                    }
                    domain::user::UserUniqueConstraint::Email(email) => {
                        UserError::EmailAlreadyExists(email)
                    }
                }
            }
            UserDomainError::EmailVerificationError(email_verification_error) => {
                UserError::EmailVerificationError(email_verification_error)
            }
            UserDomainError::StateTransitionError(user_state_transition_error) => {
                UserError::StateTransitionError(user_state_transition_error)
            }
        }
    }
}
