use crate::domain::{transaction::IntoTxError, user::{UserDomainError, UserRepositoryError}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("バリデーション失敗: {0}")]
    InvalidInput(String),

    #[error("データアクセスエラー: {0}")]
    RepositoryError(String),

    #[error("トランザクションエラー: {0}")]
    TxError(#[source] anyhow::Error),

    #[error("永続化エラー: {0}")]
    PersistenceError(#[source] anyhow::Error),
}

impl IntoTxError for UserError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        UserError::TxError(error.into())
    }
}

impl From<UserRepositoryError> for UserError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::DomainError(domain_error) => match domain_error {
                UserDomainError::EmailAlreadyExists(email) => {
                    UserError::InvalidInput(format!("Email already exists: {}", email))
                }
                UserDomainError::AlreadyExists(constraint) => {
                    UserError::InvalidInput(format!("Already exists: {:?}", constraint))
                }
                UserDomainError::InvalidEmail(invalid_email) => {
                    UserError::InvalidInput(format!("Invalid email: {}", invalid_email))
                }
                UserDomainError::PasswordTooShort => {
                    UserError::InvalidInput("Password too short".to_string())
                }
            },
            UserRepositoryError::Persistence(source) => UserError::PersistenceError(source),
        }
    }
}
