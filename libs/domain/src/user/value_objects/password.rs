use serde::{Deserialize, Serialize};

use crate::user::UserDomainError;

#[derive(Debug, Clone)]
pub struct RawPassword(String);

impl RawPassword {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        // TODO: #54 でパスワードポリシーを強化する
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

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Display, derive_more::Into,
)]
pub struct HashedPassword(String);

// TODO: #60 で正しくハッシュ化されているかの検証機構を追加する必要あり
impl HashedPassword {
    pub fn from_raw_str(hash: &str) -> Self {
        Self(hash.to_string())
    }
}
