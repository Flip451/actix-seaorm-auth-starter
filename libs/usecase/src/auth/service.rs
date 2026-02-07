use async_trait::async_trait;

use crate::{
    auth::dto::{LoginInput, LoginOutput, SignupInput, SignupOutput},
    usecase_error::UseCaseError,
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn signup(&self, input: SignupInput) -> Result<SignupOutput, UseCaseError>;
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, UseCaseError>;
}
