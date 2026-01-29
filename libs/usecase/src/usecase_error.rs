use domain::{auth::policy::AuthorizationError, transaction::IntoTxError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("不正な入力を受け取りました: {0:?}")]
    InvalidInput(Vec<ValidationError>),
    #[error("認証が必要です")]
    Unauthorized,
    #[error("権限が足りていません")]
    Forbidden,
    #[error("リソースが見つかりませんでした")]
    NotFound,
    #[error("リソースの競合が検知されました: {message}")]
    Conflict { message: String },
    #[error("サーバー内部でエラーが発生しました: {0}")]
    Internal(#[source] anyhow::Error),
}

#[derive(Debug, Serialize)]
pub struct ValidationError {
    field: String,
    message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl IntoTxError for UseCaseError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        UseCaseError::Internal(error.into())
    }
}

impl From<AuthorizationError> for UseCaseError {
    fn from(authz_error: AuthorizationError) -> Self {
        match authz_error {
            AuthorizationError::Forbidden => UseCaseError::Forbidden,
            other => UseCaseError::Conflict {
                message: other.message_for_client().to_string(),
            },
        }
    }
}
