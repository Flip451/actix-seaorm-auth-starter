use async_trait::async_trait;
use domain::user::UserRole;
use uuid::Uuid;

use crate::user::{dto::UserResponse, error::UserError};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn list_users(
        &self,
        actor_id: Uuid,
        actor_role: UserRole,
    ) -> Result<Vec<UserResponse>, UserError>;

    async fn get_user_by_id(
        &self,
        actor_id: Uuid,
        actor_role: UserRole,
        user_id: Uuid,
    ) -> Result<UserResponse, UserError>;

    async fn update_user(
        &self,
        user_id: Uuid,
        input: super::dto::UpdateUserInput,
    ) -> Result<UserResponse, UserError>;

    async fn suspend_user(
        &self,
        actor_id: Uuid,
        actor_role: UserRole,
        target_id: Uuid,
        reason: String,
    ) -> Result<(), UserError>;
}
