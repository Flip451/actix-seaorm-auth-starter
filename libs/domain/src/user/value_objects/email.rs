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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Display)]
pub struct VerifiedEmail(String);

// メールアドレス（未検証）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Display)]
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

    fn check_format(value: &str) -> Result<(), UserDomainError> {
        #[derive(Validate)]
        struct EmailCheck<'a> {
            #[validate(email)]
            email: &'a str,
        }
        let check = EmailCheck { email: value };
        check
            .validate()
            .map_err(|_| UserDomainError::InvalidEmail(value.to_string()))
    }

    fn as_str(&self) -> &str;
}

// EmailTraitの実装
impl EmailTrait for VerifiedEmail {
    fn new(value: &str) -> Result<Self, UserDomainError> {
        Self::check_format(value)?;
        Ok(Self(value.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl EmailTrait for UnverifiedEmail {
    fn new(value: &str) -> Result<Self, UserDomainError> {
        Self::check_format(value)?;
        Ok(Self(value.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("user@example.com", true)]
    #[case("invalid-email", false)]
    fn test_verified_email_valid_or_invalid(#[case] email: &str, #[case] is_valid: bool) {
        let verified_email = VerifiedEmail::new(email);
        assert_eq!(verified_email.is_ok(), is_valid);
    }

    #[rstest]
    #[case("user@example.com", true)]
    #[case("invalid-email", false)]
    fn test_unverified_email_valid_or_invalid(#[case] email: &str, #[case] is_valid: bool) {
        let unverified_email = UnverifiedEmail::new(email);
        assert_eq!(unverified_email.is_ok(), is_valid);
    }
}
