use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use chrono::Utc;
use migration::constants::UniqueConstraints;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, Set, sea_query::OnConflict};

use super::super::entities::user as user_entity;
use crate::persistence::{
    db_error_mapper::DbErrorMapper,
    seaorm::{connect::Connectable, transaction::EntityTracker},
};
use domain::user::{
    HashedPassword, User, UserId, UserRepository, UserRepositoryError, UserRole, UserStateRaw,
    UserUniqueConstraint,
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
        let user = User::reconstruct(
            model.id.into(),
            model.username,
            HashedPassword::from_raw_str(&model.password_hash),
            UserRole::from_str(&model.role).unwrap_or(UserRole::User),
            UserStateRaw {
                status: model.status,
                email: model.email,
            },
            model.created_at.into(),
            model.updated_at.into(),
        )?;
        Ok(user)
    }

    fn map_save_error(&self, e: DbErr, username: &str, email: &str) -> UserRepositoryError {
        if e.is_unique_violation() {
            let constraint = e.constraint_name().unwrap_or("");
            let email_unique_key = UniqueConstraints::UserEmailKey.to_string();
            let username_unique_key = UniqueConstraints::UserUsernameKey.to_string();

            if constraint == email_unique_key {
                return UserUniqueConstraint::Email(email.to_string()).into();
            } else if constraint == username_unique_key {
                return UserUniqueConstraint::Username(username.to_string()).into();
            }

            // 既知の一意制約名以外で一意制約違反が発生した場合は、デバッグしやすいように詳細なエラーを返す
            return UserRepositoryError::Persistence(anyhow::anyhow!(
                "Unexpected unique constraint violation (constraint: '{}', username: '{}', email: '{}'): {}",
                constraint,
                username,
                email,
                e
            ));
        }
        // その他のエラーはPersistenceとして扱う
        UserRepositoryError::Persistence(e.into())
    }
}

trait StateStr {
    fn state_str(&self) -> &str;
}

impl StateStr for User {
    fn state_str(&self) -> &str {
        self.state().kind().into()
    }
}

#[async_trait]
impl<C, T> UserRepository for SeaOrmUserRepository<C, T>
where
    C: Connectable<T> + Send + Sync,
    T: sea_orm::ConnectionTrait + Send + Sync,
{
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, UserRepositoryError> {
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
            id: Set(user.id().into()),
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
            .map_err(|e| self.map_save_error(e, username, email.as_str()))?;

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
