use domain::user::{UserId, UserRole};
use serde::{Deserialize, Serialize};

use crate::usecase_error::UseCaseError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

pub trait TokenService: Send + Sync {
    fn issue_token(&self, user_id: UserId, role: UserRole) -> Result<String, UseCaseError>;
    fn verify_token(&self, token: &str) -> Result<Claims, UseCaseError>;
}
