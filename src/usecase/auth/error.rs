use crate::domain::{
    transaction::{IntoTxError},
    user::{PasswordHashingError, UserDomainError, UserRepositoryError, UserUniqueConstraint},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("メールアドレスの形式が不正です: {0}")]
    InvalidEmail(String),

    #[error("パスワードが短すぎます")]
    PasswordTooShort,

    #[error("パスワードのハッシュ化に失敗しました: {0}")]
    PasswordHashingFailed(#[source] anyhow::Error),

    #[error("メールアドレスまたはパスワードが正しくありません")]
    InvalidCredentials,

    #[error("このメールアドレスは既に登録されています")]
    EmailAlreadyExists,

    #[error("このユーザ名は既に登録されています")]
    UsernameAlreadyExists,

    #[error("アクセス権限がありません")]
    Forbidden,

    #[error("トランザクションエラー: {0}")]
    TxError(#[source] anyhow::Error),

    #[error("永続化エラー: {0}")]
    PersistenceError(#[source] anyhow::Error),

    #[error("トークンが検出されませんでした")]
    TokenNotDetected,

    #[error("トークンの発行に失敗しました: {0}")]
    TokenIssuanceFailed(#[source] anyhow::Error),
}

impl IntoTxError for AuthError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        AuthError::TxError(error.into())
    }
}

impl From<UserRepositoryError> for AuthError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::DomainError(UserDomainError::EmailAlreadyExists(_)) => {
                AuthError::EmailAlreadyExists
            }
            UserRepositoryError::DomainError(UserDomainError::AlreadyExists(
                UserUniqueConstraint::Email(_),
            )) => AuthError::EmailAlreadyExists,
            UserRepositoryError::DomainError(UserDomainError::AlreadyExists(
                UserUniqueConstraint::Username(_),
            )) => AuthError::UsernameAlreadyExists,
            UserRepositoryError::DomainError(UserDomainError::InvalidEmail(invalid_email)) => {
                AuthError::InvalidEmail(invalid_email)
            }
            UserRepositoryError::DomainError(UserDomainError::PasswordTooShort) => {
                AuthError::PasswordTooShort
            }
            UserRepositoryError::Persistence(source) => AuthError::PersistenceError(source),
        }
    }
}

impl From<PasswordHashingError> for AuthError {
    fn from(error: PasswordHashingError) -> Self {
        AuthError::PasswordHashingFailed(anyhow::Error::new(error))
    }
}
