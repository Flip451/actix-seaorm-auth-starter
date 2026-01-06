use async_trait::async_trait;
use domain::user::User;

use crate::auth::{
    dto::{LoginInput, SignupInput},
    error::AuthError,
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn signup(&self, input: SignupInput) -> Result<User, AuthError>;
    async fn login(&self, input: LoginInput) -> Result<String, AuthError>;
}
