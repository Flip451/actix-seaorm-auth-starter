use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("無効なメールアドレス形式です: {_0}")]
    InvalidEmail(String),
    #[error("パスワードは8文字以上である必要があります")]
    PasswordTooShort,
    #[error("パスワードの処理に失敗しました: {_0}")]
    PasswordProcessError(String),
    #[error("データが既に存在します: {0}")]
    AlreadyExists(String),
    #[error("データの保存または取得に失敗しました: {0}")]
    Persistence(String),
}