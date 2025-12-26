use crate::domain::user::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("バリデーション失敗: {0}")]
    InvalidInput(String),

    #[error("メールアドレスまたはパスワードが正しくありません")]
    InvalidCredentials,

    #[error("このメールアドレスは既に登録されています")]
    EmailAlreadyExists,

    #[error("アクセス権限がありません")]
    Forbidden,

    #[error("データアクセスエラー: {0}")]
    RepositoryError(String),

    #[error("内部処理エラー")]
    InternalError,
}