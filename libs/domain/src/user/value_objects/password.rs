use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::user::UserDomainError;

#[derive(Debug, Clone)]
pub struct RawPassword(String);

impl RawPassword {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        // TODO: パスワードポリシーを強化する
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
    pub fn from_raw_str(hash: &str) -> Self {
        Self(hash.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
