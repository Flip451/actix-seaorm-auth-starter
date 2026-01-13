use async_trait::async_trait;
use uuid::Uuid;

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
        user_id: Uuid,
    ) -> Result<UserResponse, UserError>;

    async fn update_user(
        &self,
        identity: Box<dyn Identity>,
        target_id: Uuid,
        input: super::dto::UpdateUserInput,
    ) -> Result<UserResponse, UserError>;

    async fn suspend_user(
        &self,
        identity: Box<dyn Identity>,
        target_id: Uuid,
        reason: String,
    ) -> Result<(), UserError>;
}
