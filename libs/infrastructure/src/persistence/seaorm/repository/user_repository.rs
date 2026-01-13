use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, QueryFilter, RuntimeErr, Set, sea_query::OnConflict,
};
use uuid::Uuid;

use super::super::entities::user as user_entity;
use crate::persistence::seaorm::{connect::Connectable, transaction::EntityTracker};
use domain::user::{
    EmailTrait, HashedPassword, UnverifiedEmail, User, UserDomainError, UserRepository,
    UserRepositoryError, UserRole, UserState, UserUniqueConstraint, VerifiedEmail,
};

pub struct SeaOrmUserRepository<C, T>
where
    C: Connectable<T>,
    T: sea_orm::ConnectionTrait,
{
    conn: C,
    tracker: Arc<EntityTracker>,
    _marker: std::marker::PhantomData<T>,
}

impl<C: Connectable<T>, T: sea_orm::ConnectionTrait> SeaOrmUserRepository<C, T> {
    pub fn new(conn: C, tracker: Arc<EntityTracker>) -> Self {
        Self {
            conn,
            tracker,
            _marker: std::marker::PhantomData,
        }
    }

    /// DBモデルからドメインモデルへの変換
    fn map_to_domain(&self, model: user_entity::Model) -> Result<User, UserRepositoryError> {
        let state = match model.status.as_str() {
            "active" => UserState::Active {
                email: VerifiedEmail::new(&model.email)?,
            },
            "suspended_by_admin" => UserState::SuspendedByAdmin {
                email: UnverifiedEmail::new(&model.email)?,
            },
            "deactivated_by_user" => UserState::DeactivatedByUser {
                email: UnverifiedEmail::new(&model.email)?,
            },
            "pending_verification" => UserState::PendingVerification {
                email: UnverifiedEmail::new(&model.email)?,
            },
            "active_with_unverified_email" => UserState::ActiveWithUnverifiedEmail {
                email: UnverifiedEmail::new(&model.email)?,
            },
            other => {
                return Err(UserRepositoryError::Persistence(anyhow::anyhow!(
                    "不明なユーザーステータス: {}",
                    other
                )));
            }
        };

        let user = User::reconstruct(
            model.id,
            model.username,
            HashedPassword::from_raw_str(&model.password_hash),
            UserRole::from_str(&model.role).unwrap_or(UserRole::User),
            state,
            model.created_at.into(),
            model.updated_at.into(),
        )?;
        Ok(user)
    }
}

trait StateStr {
    fn state_str(&self) -> &str;
}

impl StateStr for User {
    fn state_str(&self) -> &str {
        match &self.state() {
            UserState::Active { .. } => "active",
            UserState::SuspendedByAdmin { .. } => "suspended_by_admin",
            UserState::DeactivatedByUser { .. } => "deactivated_by_user",
            UserState::PendingVerification { .. } => "pending_verification",
            UserState::ActiveWithUnverifiedEmail { .. } => "active_with_unverified_email",
        }
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
            .map_err(|e| UserRepositoryError::Persistence(e.into()))?;

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
            .map_err(|e| UserRepositoryError::Persistence(e.into()))?;

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
            .map_err(|e| UserRepositoryError::Persistence(e.into()))?;

        match model {
            Some(m) => Ok(Some(self.map_to_domain(m)?)),
            None => Ok(None),
        }
    }

    /// 保存（新規作成 or 更新）を行うメソッド
    async fn save(&self, user: User) -> Result<User, UserRepositoryError> {
        let username = user.username();
        let email = user.email();

        let active_model = user_entity::ActiveModel {
            id: Set(user.id()),
            username: Set(user.username().to_string()),
            email: Set(user.email().as_str().to_string()),
            password_hash: Set(user.password().to_string()),
            status: Set(user.state_str().to_string()),
            role: Set(user.role().to_string()),
            created_at: Set(user.created_at().into()), // 新規作成時は引数の値、更新時は無視される
            updated_at: Set(Utc::now().into()),
        };

        // ON CONFLICT (id) DO UPDATE ...
        let saved_model = user_entity::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(user_entity::Column::Id)
                    .update_columns([
                        user_entity::Column::Username,
                        user_entity::Column::Email,
                        user_entity::Column::PasswordHash,
                        user_entity::Column::Role,
                        user_entity::Column::Status,
                        user_entity::Column::UpdatedAt, // 更新時は日時を更新
                    ])
                    .to_owned(),
            )
            .exec_with_returning(self.conn.connect())
            .await
            .map_err(|e| {
                // エラーハンドリングの詳細化
                if let DbErr::Query(RuntimeErr::SqlxError(sqlx_err)) = &e {
                    // Postgresのエラーコード "23505" (unique_violation) をチェック
                    if let Some(db_err) = sqlx_err.as_database_error()
                        && let Some(code) = db_err.code()
                        && code == "23505"
                    {
                        let constraint = db_err.constraint().unwrap_or("");

                        if constraint.contains("email") {
                            return UserDomainError::AlreadyExists(UserUniqueConstraint::Email(
                                email.as_str().to_string(),
                            ))
                            .into();
                        } else if constraint.contains("username") {
                            return UserDomainError::AlreadyExists(UserUniqueConstraint::Username(
                                username.to_string(),
                            ))
                            .into();
                        }
                    }
                }
                // その他のエラーはPersistenceとして扱う
                UserRepositoryError::Persistence(e.into())
            })?;

        self.tracker.track(Box::new(user));

        self.map_to_domain(saved_model)
    }

    async fn find_all(&self) -> Result<Vec<User>, UserRepositoryError> {
        let models = user_entity::Entity::find()
            .all(self.conn.connect())
            .await
            .map_err(|e| UserRepositoryError::Persistence(e.into()))?;

        models.into_iter().map(|m| self.map_to_domain(m)).collect()
    }
}
