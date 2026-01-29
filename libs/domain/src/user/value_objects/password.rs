use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct RawPassword(String);

#[derive(Debug, Error, PartialEq)]
pub enum PasswordPolicyViolation {
    #[error("パスワードは8文字以上である必要があります")]
    TooShort,
}

impl RawPassword {
    pub fn new(value: &str) -> Result<Self, PasswordPolicyViolation> {
        // TODO: #54 でパスワードポリシーを強化する
        if value.len() >= 8 {
            Ok(Self(value.to_string()))
        } else {
            Err(PasswordPolicyViolation::TooShort)
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::Into,
    derive_more::AsRef,
)]
pub struct HashedPassword(String);

// TODO: #60 で正しくハッシュ化されているかの検証機構を追加する必要あり
impl HashedPassword {
    pub fn from_raw_str(hash: &str) -> Self {
        Self(hash.to_string())
    }
}
