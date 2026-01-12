use std::fmt;

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
pub trait EmailTrait: Sized {
    fn new(value: &str) -> Result<Self, UserDomainError>;

    fn as_str(&self) -> &str;
}

// EmailTraitの実装
impl EmailTrait for VerifiedEmail {
    fn new(value: &str) -> Result<Self, UserDomainError> {
        #[derive(Validate)]
        struct EmailCheck<'a> {
            #[validate(email)]
            email: &'a str,
        }
        let check = EmailCheck { email: value };
        if check.validate().is_ok() {
            Ok(Self(value.to_string()))
        } else {
            Err(UserDomainError::InvalidEmail(value.to_string()))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl EmailTrait for UnverifiedEmail {
    fn new(value: &str) -> Result<Self, UserDomainError> {
        #[derive(Validate)]
        struct EmailCheck<'a> {
            #[validate(email)]
            email: &'a str,
        }
        let check = EmailCheck { email: value };
        if check.validate().is_ok() {
            Ok(Self(value.to_string()))
        } else {
            Err(UserDomainError::InvalidEmail(value.to_string()))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VerifiedEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for UnverifiedEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
