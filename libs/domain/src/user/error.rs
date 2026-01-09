use derive_more::Display;
use thiserror::Error;

use super::EmailVerificationError;

#[derive(Debug, Error)]
pub enum UserDomainError {
    #[error("無効なメールアドレス形式です: {0}")]
    InvalidEmail(String),

    #[error("パスワードは8文字以上である必要があります")]
    PasswordTooShort,

    #[error("既存のユーザーと重複しています: {0}")]
    AlreadyExists(UserUniqueConstraint),

    #[error("メールアドレスの検証に失敗しました: {0}")]
    EmailVerificationError(#[from] EmailVerificationError),

    #[error(transparent)]
    StateTransitionError(#[from] UserStateTransitionError),
}

#[derive(Debug, Display)]
pub enum UserUniqueConstraint {
    Username(String),
    Email(String),
}

#[derive(Debug, Error)]
pub enum UserStateTransitionError {
    #[error("ユーザーは既に退会しています: {from:?} からの遷移は許可されていません")]
    AlreadyDeactivated { from: super::UserState },

    #[error("ユーザーは既に停止されています: {from:?} からの遷移は許可されていません")]
    AlreadySuspended { from: super::UserState },

    #[error("ユーザーのメールアドレスが未検証です: {from:?} からの遷移は許可されていません")]
    NotVerified { from: super::UserState },

    #[error("指定のユーザーは停止されていません： {from:?} からの遷移は許可されていません")]
    NotSuspended { from: super::UserState },
}
