use super::super::error::UserDomainError;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Email {
    Verified(VerifiedEmail),
    Unverified(UnverifiedEmail),
}

impl Email {
    pub fn as_str(&self) -> &str {
        match self {
            Email::Verified(email) => email.as_str(),
            Email::Unverified(email) => email.as_str(),
        }
    }
}

// メールアドレス（検証済み）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedEmail(String);

// メールアドレス（未検証）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnverifiedEmail(String);

// 検証済みメールアドレスを未検証メールアドレスに変換する
impl VerifiedEmail {
    pub fn unverify(&self) -> UnverifiedEmail {
        UnverifiedEmail(self.0.clone())
    }
}

// メールアドレスの共通トレイト
pub trait EmailTrait: FromStr + Sized {
    fn new(value: &str) -> Result<Self, UserDomainError> {
        #[derive(Validate)]
        struct EmailCheck<'a> {
            #[validate(email)]
            email: &'a str,
        }
        let check = EmailCheck { email: value };
        if check.validate().is_ok() {
            Ok(Self::from_str(&value))
        } else {
            Err(UserDomainError::InvalidEmail(value.to_string()))
        }
    }

    fn as_str(&self) -> &str;
}

// Email トレイトの実装
impl FromStr for VerifiedEmail {
    fn from_str(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl EmailTrait for VerifiedEmail {
    fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for UnverifiedEmail {
    fn from_str(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl EmailTrait for UnverifiedEmail {
    fn as_str(&self) -> &str {
        &self.0
    }
}

// Email トレイト構築のための非公開トレイト
trait FromStr {
    fn from_str(value: &str) -> Self;
}
