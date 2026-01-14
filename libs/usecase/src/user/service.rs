use async_trait::async_trait;
use domain::user::UserId;

use crate::{
    shared::identity::Identity,
    user::{dto::UserResponse, error::UserError},
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn list_users(&self, identity: Box<dyn Identity>)
    -> Result<Vec<UserResponse>, UserError>;

    async fn get_user_by_id(
        &self,
        identity: Box<dyn Identity>,
        user_id: UserId,
    ) -> Result<UserResponse, UserError>;

    async fn update_user(
        &self,
        identity: Box<dyn Identity>,
        target_id: UserId,
        input: super::dto::UpdateUserInput,
    ) -> Result<UserResponse, UserError>;

    async fn suspend_user(
        &self,
        identity: Box<dyn Identity>,
        target_id: UserId,
        reason: String,
    ) -> Result<(), UserError>;
}
