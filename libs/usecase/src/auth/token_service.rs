use super::error::AuthError;
use async_trait::async_trait;
use domain::user::UserRole;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

#[async_trait]
pub trait TokenService: Send + Sync {
    fn issue_token(&self, user_id: Uuid, role: UserRole) -> Result<String, AuthError>;
    fn verify_token(&self, token: &str) -> Result<Claims, AuthError>;
}
