use thiserror::Error;

use crate::domain::user::{UnverifiedEmail, VerifiedEmail};

use super::{HashedPassword, RawPassword};

#[derive(Debug, Error)]
pub enum PasswordHashingError {
    #[error("パスワードのハッシュ化に失敗しました")]
    HashingFailed,
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, PasswordHashingError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> bool;
}

#[derive(Debug, Error)]
pub enum EmailVerificationError {
    // TODO: エラーの詳細を追加する
}

pub trait EmailVerifier {
    fn verify(&self, email: &UnverifiedEmail) -> Result<VerifiedEmail, EmailVerificationError>;
}
