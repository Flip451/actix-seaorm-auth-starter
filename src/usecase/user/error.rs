use crate::domain::{
    transaction::IntoTxError,
    user::{UserDomainError, UserRepositoryError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("バリデーション失敗: {0}")]
    InvalidInput(String),

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
            UserRepositoryError::Persistence(source) => UserError::PersistenceError(source),
        }
    }
}

impl From<UserDomainError> for UserError {
    fn from(error: UserDomainError) -> Self {
        match error {
            UserDomainError::InvalidEmail(invalid_email) => UserError::InvalidInput(invalid_email),
            UserDomainError::AlreadyExists(_) | UserDomainError::PasswordTooShort => UserError::UnexpectedError(error.into())
        }
    }
}
