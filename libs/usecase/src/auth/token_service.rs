use super::error::AuthError;
use async_trait::async_trait;
use domain::user::{UserId, UserRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

#[async_trait]
pub trait TokenService: Send + Sync {
    fn issue_token(&self, user_id: UserId, role: UserRole) -> Result<String, AuthError>;
    fn verify_token(&self, token: &str) -> Result<Claims, AuthError>;
}
