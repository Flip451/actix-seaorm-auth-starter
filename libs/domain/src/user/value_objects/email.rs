use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::{Validate, ValidationErrors};

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

#[derive(Debug, Error, PartialEq)]
pub enum EmailFormatError {
    #[error("以下のメールアドレスは形式が正しくありません: {invalid_email}")]
    InvalidFormat {
        invalid_email: String,
        #[source]
        error: ValidationErrors,
    },
}

fn check_email_format(value: &str) -> Result<(), EmailFormatError> {
    #[derive(Validate)]
    struct EmailFormat<'a> {
        #[validate(email)]
        email: &'a str,
    }

    let email_format = EmailFormat { email: value };
    email_format
        .validate()
        .map_err(|error| EmailFormatError::InvalidFormat {
            invalid_email: value.to_string(),
            error,
        })
}

// メールアドレスの共通トレイト
pub trait EmailTrait: Sized + std::fmt::Debug {
    fn new(value: &str) -> Result<Self, EmailFormatError>;

    fn as_str(&self) -> &str;
}

// EmailTraitの実装
impl EmailTrait for VerifiedEmail {
    fn new(value: &str) -> Result<Self, EmailFormatError> {
        check_email_format(value)?;
        Ok(Self(value.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl EmailTrait for UnverifiedEmail {
    fn new(value: &str) -> Result<Self, EmailFormatError> {
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
    #[case(VerifiedEmail::new("invalid-email"), "invalid-email")]
    #[case(UnverifiedEmail::new("invalid-email"), "invalid-email")]
    fn test_invalid_email_error(
        #[case] new_email_result: Result<impl EmailTrait, EmailFormatError>,
        #[case] expected_invalid_email_in_error: &str,
    ) {
        if let Err(EmailFormatError::InvalidFormat {
            invalid_email,
            error: _,
        }) = new_email_result
        {
            assert_eq!(invalid_email, expected_invalid_email_in_error)
        } else {
            panic!()
        }
    }
}
