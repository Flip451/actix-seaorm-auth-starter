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

fn check_email_format(value: &str) -> Result<(), UserDomainError> {
    #[derive(Validate)]
    struct EmailFormat<'a> {
        #[validate(email)]
        email: &'a str,
    }

    let email_format = EmailFormat { email: value };
    email_format
        .validate()
        .map_err(|_| UserDomainError::InvalidEmail(value.to_string()))
}

// メールアドレスの共通トレイト
pub trait EmailTrait: Sized + std::fmt::Debug {
    fn new(value: &str) -> Result<Self, UserDomainError>;

    fn as_str(&self) -> &str;
}

// EmailTraitの実装
impl EmailTrait for VerifiedEmail {
    fn new(value: &str) -> Result<Self, UserDomainError> {
        check_email_format(value)?;
        Ok(Self(value.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl EmailTrait for UnverifiedEmail {
    fn new(value: &str) -> Result<Self, UserDomainError> {
        check_email_format(value)?;
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
    #[case(VerifiedEmail::new("user@example.com").unwrap(), "user@example.com")]
    #[case(UnverifiedEmail::new("user@example.com").unwrap(), "user@example.com")]
    fn test_valid_email_as_str(#[case] email: impl EmailTrait, #[case] email_str: &str) {
        assert_eq!(email.as_str(), email_str);
    }

    #[test]
    fn test_verified_email_to_string() {
        let email_str = "user@example.com";
        let valid_verified_email = VerifiedEmail::new(email_str).unwrap();
        assert_eq!(valid_verified_email.to_string(), email_str);
    }

    #[test]
    fn test_unverified_email_to_string() {
        let email_str = "user@example.com";
        let valid_unverified_email = UnverifiedEmail::new(email_str).unwrap();
        assert_eq!(valid_unverified_email.to_string(), email_str);
    }

    #[rstest]
    #[case(VerifiedEmail::new("invalid-email").unwrap_err(), UserDomainError::InvalidEmail("invalid-email".to_string()))]
    #[case(UnverifiedEmail::new("invalid-email").unwrap_err(), UserDomainError::InvalidEmail("invalid-email".to_string()))]
    fn test_invalid_email_error(
        #[case] email_creation_error: UserDomainError,
        #[case] expected: UserDomainError,
    ) {
        assert_eq!(email_creation_error, expected);
    }
}
