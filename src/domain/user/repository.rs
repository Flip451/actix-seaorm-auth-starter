use async_trait::async_trait;
use crate::domain::user::DomainError;

use super::entity::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: User) -> Result<User, DomainError>;
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;
}