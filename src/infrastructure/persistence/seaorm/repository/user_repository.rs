use std::str::FromStr;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::domain::user::{DomainError, Email, HashedPassword, User, UserRepository, UserRole};
use super::super::entities::user as user_entity;

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_to_domain(&self, model: user_entity::Model) -> Result<User, DomainError> {
        Ok(User {
            id: model.id,
            username: model.username,
            email: Email::new(&model.email)?,
            password: HashedPassword::from_str(&model.password_hash),
            role: UserRole::from_str(&model.role).unwrap_or(UserRole::User),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let model = user_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Persistence(e.to_string()))?;

        match model {
            Some(m) => Ok(Some(self.map_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let model = user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Persistence(e.to_string()))?;

        match model {
            Some(m) => Ok(Some(self.map_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn save(&self, user: User) -> Result<User, DomainError> {
        let active_model = user_entity::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email.as_str().to_string()),
            password_hash: Set(user.password.as_str().to_string()),
            is_active: Set(user.is_active),
            role: Set(user.role.to_string()), // role の保存を追加
            ..Default::default()
        };

        let saved_model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::Persistence(e.to_string()))?;
            
        self.map_to_domain(saved_model)
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        let models = user_entity::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Persistence(e.to_string()))?;

        models.into_iter()
            .map(|m| self.map_to_domain(m))
            .collect()
    }
}