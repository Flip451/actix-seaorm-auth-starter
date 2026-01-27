use domain::{
    shared::outbox_event::OutboxRepositoryError,
    transaction::IntoTxError,
    user::{PasswordHashingError, UserDomainError, UserRepositoryError, UserUniqueConstraint},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("無効なメールアドレス形式です: {0}")]
    InvalidEmail(String),

    #[error("パスワードが短すぎます")]
    PasswordTooShort,

    #[error("パスワードのハッシュ化に失敗しました: {0}")]
    PasswordHashingFailed(#[source] anyhow::Error),

    #[error("認証情報が無効です")]
    InvalidCredentials,

    #[error("メールアドレス '{0}' は既に存在します")]
    EmailAlreadyExists(String),

    #[error("ユーザー名 '{0}' は既に存在します")]
    UsernameAlreadyExists(String),

    #[error("アクセス権限がありません")]
    Forbidden,

    #[error("トランザクションエラー: {0}")]
    TxError(#[source] anyhow::Error),

    #[error("永続化エラー: {0}")]
    PersistenceError(#[source] anyhow::Error),

    #[error("トークンの発行に失敗しました: {0}")]
    TokenIssuanceFailed(#[source] anyhow::Error),

    #[error("想定外のエラーが発生しました: {0}")]
    UnexpectedError(#[source] anyhow::Error),
}

impl IntoTxError for AuthError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        AuthError::TxError(error.into())
    }
}

impl From<UserRepositoryError> for AuthError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::DomainError(source) => AuthError::from(source),
            UserRepositoryError::ReconstructionError(user_reconstruction_error) => {
                AuthError::PersistenceError(user_reconstruction_error.into())
            }
            UserRepositoryError::Persistence(source) => AuthError::PersistenceError(source),
        }
    }
}

impl From<OutboxRepositoryError> for AuthError {
    fn from(error: OutboxRepositoryError) -> Self {
        match error {
            OutboxRepositoryError::Persistence(source) => AuthError::PersistenceError(source),
        }
    }
}

impl From<PasswordHashingError> for AuthError {
    fn from(error: PasswordHashingError) -> Self {
        AuthError::PasswordHashingFailed(anyhow::Error::new(error))
    }
}

impl From<UserDomainError> for AuthError {
    fn from(error: UserDomainError) -> Self {
        match error {
            UserDomainError::AlreadyExists(constraint) => match constraint {
                UserUniqueConstraint::Email(email) => AuthError::EmailAlreadyExists(email),
                UserUniqueConstraint::Username(username) => {
                    AuthError::UsernameAlreadyExists(username)
                }
            },
            UserDomainError::InvalidEmail(invalid_email) => AuthError::InvalidEmail(invalid_email),
            UserDomainError::PasswordTooShort => AuthError::PasswordTooShort,
            UserDomainError::EmailVerificationError(email_verification_error) => {
                AuthError::UnexpectedError(email_verification_error.into())
            }
            UserDomainError::StateTransitionError(state_transition_error) => {
                AuthError::UnexpectedError(state_transition_error.into())
            }
        }
    }
}
