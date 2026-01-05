use std::sync::Arc;

use uuid::Uuid;

use super::dto::UserResponse;
use super::error::UserError;
use domain::auth::policy::{AuthorizationService, UserAction};
use domain::transaction::TransactionManager;
use domain::user::{EmailTrait, UnverifiedEmail, UserRole};
use domain::tx;

pub struct UserService<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
}

impl<TM: TransactionManager> UserService<TM> {
    pub fn new(transaction_manager: Arc<TM>) -> Self {
        Self {
            transaction_manager,
        }
    }

    pub async fn list_users(
        &self,
        actor_id: Uuid,
        actor_role: UserRole,
    ) -> Result<Vec<UserResponse>, UserError> {
        let users = tx!(self.transaction_manager, |factory| {
            // ポリシーチェック
            AuthorizationService::can(actor_id, &actor_role, UserAction::ListUsers)?;

            let user_repo = factory.user_repository();
            Ok::<_, UserError>(user_repo.find_all().await?)
        })
        .await?;

        Ok(users
            .into_iter()
            .map(|u| UserResponse {
                id: u.id(),
                username: u.username().to_string(),
                email: u.email().as_str().to_string(),
                role: u.role().clone(),
            })
            .collect::<Vec<UserResponse>>())
    }

    pub async fn get_user_by_id(
        &self,
        actor_id: Uuid,
        actor_role: UserRole,
        user_id: Uuid,
    ) -> Result<UserResponse, UserError> {
        let user = tx!(self.transaction_manager, |factory| {
            // プロフィールの取得
            let user_repo = factory.user_repository();
            let user = user_repo
                .find_by_id(user_id)
                .await?
                .ok_or(UserError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                actor_id,
                &actor_role,
                UserAction::ViewProfile { target: &user },
            )?;

            Ok::<_, UserError>(user)
        })
        .await?;

        Ok(UserResponse {
            id: user.id(),
            username: user.username().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role().clone(),
        })
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        input: super::dto::UpdateUserInput,
    ) -> Result<UserResponse, UserError> {
        let updated_user = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let mut user = user_repo
                .find_by_id(user_id)
                .await?
                .ok_or(UserError::NotFound)?;

            if let Some(username) = input.username {
                // ポリシーチェック
                AuthorizationService::can(
                    user_id,
                    &user.role(),
                    UserAction::UpdateProfile { target: &user },
                )?;

                // ドメインロジックの実行
                user.change_username(username.clone())?;

                // ユーザー名の重複チェック
                if let Some(_) = user_repo.find_by_username(&username).await? {
                    return Err(UserError::UsernameAlreadyExists(username));
                }
            }
            if let Some(email) = input.email {
                // ポリシーチェック
                AuthorizationService::can(
                    user_id,
                    &user.role(),
                    UserAction::ChangeEmail { target: &user },
                )?;

                // ドメインロジックの実行
                user.change_email(UnverifiedEmail::new(&email)?)?;

                // メールアドレスの重複チェック
                if let Some(_) = user_repo.find_by_email(&email).await? {
                    return Err(UserError::EmailAlreadyExists(email));
                }
            }

            let updated_user = user_repo.save(user).await?;
            Ok::<_, UserError>(updated_user)
        })
        .await?;

        Ok(UserResponse {
            id: updated_user.id(),
            username: updated_user.username().to_string(),
            email: updated_user.email().as_str().to_string(),
            role: updated_user.role().clone(),
        })
    }

    pub async fn suspend_user(
        &self,
        actor_id: Uuid,
        actor_role: UserRole,
        target_id: Uuid,
    ) -> Result<(), UserError> {
        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let mut target_user = user_repo
                .find_by_id(target_id)
                .await?
                .ok_or(UserError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                actor_id,
                &actor_role,
                UserAction::SuspendUser {
                    target: &target_user,
                },
            )?;

            // ユーザーの状態を停止に変更
            target_user.suspend()?;

            // 変更を保存
            user_repo.save(target_user).await?;

            Ok::<_, UserError>(())
        })
        .await
    }
}
