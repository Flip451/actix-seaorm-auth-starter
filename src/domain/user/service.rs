use thiserror::Error;

use super::value_object::{HashedPassword, RawPassword};

#[derive(Debug, Error)]
pub enum PasswordHashingError {
    #[error("パスワードのハッシュ化に失敗しました")]
    HashingFailed,
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, PasswordHashingError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> bool;
}
