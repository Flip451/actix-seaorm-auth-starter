use crate::domain::user::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("バリデーション失敗: {0}")]
    InvalidInput(String),

    #[error("データアクセスエラー: {0}")]
    RepositoryError(String),

    #[error("内部処理エラー")]
    InternalError,
}