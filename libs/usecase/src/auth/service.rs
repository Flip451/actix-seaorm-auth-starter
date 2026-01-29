use async_trait::async_trait;
use domain::user::User;

use crate::{
    auth::dto::{LoginInput, SignupInput},
    usecase_error::UseCaseError,
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn signup(&self, input: SignupInput) -> Result<User, UseCaseError>;
    async fn login(&self, input: LoginInput) -> Result<String, UseCaseError>;
}
