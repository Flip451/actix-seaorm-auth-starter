use crate::domain::user::error::DomainError;
use super::value_object::{RawPassword, HashedPassword};

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, DomainError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> bool;
}