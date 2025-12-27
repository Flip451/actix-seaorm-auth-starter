use crate::domain::user::UserDomainError;
use async_trait::async_trait;
use thiserror::Error;

use super::entity::User;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error(transparent)]
    DomainError(#[from] UserDomainError),

    #[error("データの保存または取得に失敗しまsした: {0}")]
    Persistence(#[source] anyhow::Error),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn save(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn find_all(&self) -> Result<Vec<User>, UserRepositoryError>;
}
