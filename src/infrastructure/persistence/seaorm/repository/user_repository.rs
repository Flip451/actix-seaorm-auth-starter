use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, RuntimeErr, Set};
use std::str::FromStr;
use uuid::Uuid;

use super::super::entities::user as user_entity;
use crate::{
    domain::user::{
        Email, HashedPassword, User, UserDomainError, UserRepository, UserRepositoryError,
        UserRole, UserUniqueConstraint,
    },
    infrastructure::persistence::seaorm::connect::Connectable,
};

pub struct SeaOrmUserRepository<C, T>
where
    C: Connectable<T>,
    T: sea_orm::ConnectionTrait,
{
    conn: C,
    _marker: std::marker::PhantomData<T>,
}

impl<C: Connectable<T>, T: sea_orm::ConnectionTrait> SeaOrmUserRepository<C, T> {
    pub fn new(conn: C) -> Self {
        Self {
            conn,
            _marker: std::marker::PhantomData,
        }
    }

    /// DBモデルからドメインモデルへの変換
    fn map_to_domain(&self, model: user_entity::Model) -> Result<User, UserRepositoryError> {
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
impl<C, T> UserRepository for SeaOrmUserRepository<C, T>
where
    C: Connectable<T> + Send + Sync,
    T: sea_orm::ConnectionTrait + Send + Sync,
{
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError> {
        let model = user_entity::Entity::find_by_id(id)
            .one(self.conn.connect())
            .await
            .map_err(|e| UserRepositoryError::Persistence(anyhow::Error::new(e)))?;

        match model {
            Some(m) => Ok(Some(self.map_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let model = user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(self.conn.connect())
            .await
            .map_err(|e| UserRepositoryError::Persistence(anyhow::Error::new(e)))?;

        match model {
            Some(m) => Ok(Some(self.map_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserRepositoryError> {
        let model = user_entity::Entity::find()
            .filter(user_entity::Column::Username.eq(username))
            .one(self.conn.connect())
            .await
            .map_err(|e| UserRepositoryError::Persistence(anyhow::Error::new(e)))?;

        match model {
            Some(m) => Ok(Some(self.map_to_domain(m)?)),
            None => Ok(None),
        }
    }

    /// 保存（新規作成 or 更新）を行うメソッド
    async fn save(&self, user: User) -> Result<User, UserRepositoryError> {
        // 1. 既存レコードの確認
        let existing_model = user_entity::Entity::find_by_id(user.id)
            .one(self.conn.connect())
            .await
            .map_err(|e| UserRepositoryError::Persistence(anyhow::Error::new(e)))?;

        match existing_model {
            // A. 更新 (UPDATE)
            Some(model) => {
                let mut active_model: user_entity::ActiveModel = model.into();
                active_model.username = Set(user.username);
                active_model.email = Set(user.email.as_str().to_string());
                active_model.password_hash = Set(user.password.as_str().to_string());
                active_model.role = Set(user.role.to_string());
                active_model.is_active = Set(user.is_active);
                // 更新時は updated_at を現在時刻に更新
                active_model.updated_at = Set(chrono::Utc::now().fixed_offset());

                let updated_model = active_model
                    .update(self.conn.connect())
                    .await
                    .map_err(|e| UserRepositoryError::Persistence(anyhow::Error::new(e)))?;

                self.map_to_domain(updated_model)
            }
            // B. 新規作成 (INSERT)
            None => {
                let username = user.username.clone();
                let email = user.email.clone();

                let active_model = user_entity::ActiveModel {
                    id: Set(user.id),
                    username: Set(user.username),
                    email: Set(user.email.as_str().to_string()),
                    password_hash: Set(user.password.as_str().to_string()),
                    is_active: Set(user.is_active),
                    role: Set(user.role.to_string()),
                    created_at: Set(user.created_at),
                    updated_at: Set(user.updated_at),
                };

                let saved_model = active_model
                    .insert(self.conn.connect())
                    .await
                    .map_err(|e| {
                        // エラーハンドリングの詳細化
                        match &e {
                            DbErr::Query(RuntimeErr::SqlxError(sqlx_err)) => {
                                // Postgresのエラーコード "23505" (unique_violation) をチェック
                                if let Some(db_err) = sqlx_err.as_database_error() {
                                    if let Some(code) = db_err.code() {
                                        if code == "23505" {
                                            let constraint = db_err.constraint().unwrap_or("");

                                            if constraint.contains("email") {
                                                return UserDomainError::AlreadyExists(
                                                    UserUniqueConstraint::Email(
                                                        email.clone().as_str().to_string(),
                                                    ),
                                                )
                                                .into();
                                            } else if constraint.contains("username") {
                                                return UserDomainError::AlreadyExists(
                                                    UserUniqueConstraint::Username(
                                                        username.clone().as_str().to_string(),
                                                    ),
                                                )
                                                .into();
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        // その他のエラーはPersistenceとして扱う
                        UserRepositoryError::Persistence(anyhow::Error::new(e))
                    })?;

                self.map_to_domain(saved_model)
            }
        }
    }

    async fn find_all(&self) -> Result<Vec<User>, UserRepositoryError> {
        let models = user_entity::Entity::find()
            .all(self.conn.connect())
            .await
            .map_err(|e| UserRepositoryError::Persistence(anyhow::Error::new(e)))?;

        models.into_iter().map(|m| self.map_to_domain(m)).collect()
    }
}
