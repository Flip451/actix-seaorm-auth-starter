use super::error::UserDomainError;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
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
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RawPassword(String);

impl RawPassword {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        if value.len() >= 8 {
            Ok(Self(value.to_string()))
        } else {
            Err(UserDomainError::PasswordTooShort)
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_str(hash: &str) -> Self {
        Self(hash.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}
