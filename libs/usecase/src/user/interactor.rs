use std::sync::Arc;

use async_trait::async_trait;
use domain::auth::policies::change_email::ChangeEmailPayload;
use domain::auth::policies::list_users::ListUsersPayload;
use domain::auth::policies::suspend_user::SuspendUserPayload;
use domain::auth::policies::update_profile::UpdateProfilePayload;
use domain::auth::policies::view_profile::ViewProfilePayload;

use crate::shared::identity::Identity;
use crate::usecase_error::UseCaseError;
use crate::user::service::UserService;

use super::dto::UserResponse;
use domain::auth::policy::{AuthorizationService, UserAction};
use domain::transaction::TransactionManager;
use domain::tx;
use domain::user::{UserDomainError, UserId, UserUniquenessService};

pub struct UserInteractor<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
}

impl<TM: TransactionManager> UserInteractor<TM> {
    pub fn new(transaction_manager: Arc<TM>) -> Self {
        Self {
            transaction_manager,
        }
    }
}

#[async_trait]
impl<TM: TransactionManager> UserService for UserInteractor<TM> {
    #[tracing::instrument(
        skip(self, identity),
        fields(
            actor_id = %identity.actor_id(),
            actor_role = %identity.actor_role(),
        )
    )]
    async fn list_users(
        &self,
        identity: Box<dyn Identity>,
    ) -> Result<Vec<UserResponse>, UseCaseError> {
        let users = tx!(self.transaction_manager, |factory| {
            // ポリシーチェック
            AuthorizationService::can(
                identity.actor_id(),
                identity.actor_role(),
                UserAction::ListUsers(ListUsersPayload),
            )?;

            let user_repo = factory.user_repository();
            Ok::<_, UseCaseError>(user_repo.find_all().await?)
        })
        .await?;

        Ok(users
            .into_iter()
            .map(|u| UserResponse {
                id: u.id(),
                username: u.username().to_string(),
                email: u.email().as_str().to_string(),
                role: u.role(),
            })
            .collect::<Vec<UserResponse>>())
    }

    #[tracing::instrument(
        skip(self, identity),
        fields(
            actor_id = %identity.actor_id(),
            actor_role = %identity.actor_role(),
            user_id = %user_id,
        )
    )]
    async fn get_user_by_id(
        &self,
        identity: Box<dyn Identity>,
        user_id: UserId,
    ) -> Result<UserResponse, UseCaseError> {
        let user = tx!(self.transaction_manager, |factory| {
            // プロフィールの取得
            let user_repo = factory.user_repository();
            let user = user_repo
                .find_by_id(user_id)
                .await?
                .ok_or(UseCaseError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                identity.actor_id(),
                identity.actor_role(),
                UserAction::ViewProfile(ViewProfilePayload { target: &user }),
            )?;

            Ok::<_, UseCaseError>(user)
        })
        .await?;

        Ok(UserResponse {
            id: user.id(),
            username: user.username().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role(),
        })
    }

    #[tracing::instrument(
        skip(self, identity, input),
        fields(
            actor_id = %identity.actor_id(),
            actor_role = %identity.actor_role(),
            target_id = %target_id,
        )
    )]
    async fn update_user(
        &self,
        identity: Box<dyn Identity>,
        target_id: UserId,
        input: super::dto::UpdateUserInput,
    ) -> Result<UserResponse, UseCaseError> {
        let updated_user = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let user_uniqueness_service = UserUniquenessService::new(user_repo.clone());

            let mut user = user_repo
                .find_by_id(target_id)
                .await?
                .ok_or(UseCaseError::NotFound)?;

            if let Some(username) = input.username {
                // ポリシーチェック
                AuthorizationService::can(
                    identity.actor_id(),
                    identity.actor_role(),
                    UserAction::UpdateProfile(UpdateProfilePayload { target: &user }),
                )?;

                // ユーザー名の重複チェック
                let username = user_uniqueness_service
                    .ensure_unique_username(&username)
                    .await?;

                // ドメインロジックの実行
                user.change_username(username)
                    .map_err(UserDomainError::from)?;
            }
            if let Some(email) = input.email {
                // ポリシーチェック
                AuthorizationService::can(
                    identity.actor_id(),
                    identity.actor_role(),
                    UserAction::ChangeEmail(ChangeEmailPayload { target: &user }),
                )?;

                // メールアドレスの重複チェック
                let email = user_uniqueness_service.ensure_unique_email(&email).await?;

                // ドメインロジックの実行
                user.change_email(email).map_err(UserDomainError::from)?;
            }

            // 変更の保存
            let updated_user = user_repo.save(user).await?;

            Ok::<_, UseCaseError>(updated_user)
        })
        .await?;

        Ok(UserResponse {
            id: updated_user.id(),
            username: updated_user.username().to_string(),
            email: updated_user.email().as_str().to_string(),
            role: updated_user.role(),
        })
    }

    #[tracing::instrument(
        skip(self, identity),
        fields(
            actor_id = %identity.actor_id(),
            actor_role = %identity.actor_role(),
            target_id = %target_id,
        )
    )]
    async fn suspend_user(
        &self,
        identity: Box<dyn Identity>,
        target_id: UserId,
        reason: String,
    ) -> Result<(), UseCaseError> {
        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            let mut target_user = user_repo
                .find_by_id(target_id)
                .await?
                .ok_or(UseCaseError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                identity.actor_id(),
                identity.actor_role(),
                UserAction::SuspendUser(SuspendUserPayload {
                    target: &target_user,
                }),
            )?;

            // ユーザーの状態を停止に変更
            target_user.suspend(reason)?;

            // 変更を保存
            user_repo.save(target_user).await?;

            Ok::<_, UseCaseError>(())
        })
        .await
    }
}
