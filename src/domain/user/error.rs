use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserDomainError {
    #[error("無効なメールアドレス形式です: {0}")]
    InvalidEmail(String),

    #[error("パスワードは8文字以上である必要があります")]
    PasswordTooShort,

    #[error("既存のユーザーと重複しています: {0}")]
    AlreadyExists(UserUniqueConstraint),
}

#[derive(Debug, Display)]
pub enum UserUniqueConstraint {
    Username(String),
    Email(String),
}
