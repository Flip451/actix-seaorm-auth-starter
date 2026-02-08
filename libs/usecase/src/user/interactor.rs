use std::sync::Arc;

use async_trait::async_trait;
use domain::auth::policies::change_email::ChangeEmailPayload;
use domain::auth::policies::list_users::ListUsersPayload;
use domain::auth::policies::suspend_user::SuspendUserPayload;
use domain::auth::policies::update_profile::UpdateProfilePayload;
use domain::auth::policies::view_own_profile::ViewOwnProfilePayload;
use domain::auth::policies::view_profile::ViewProfilePayload;
use domain::shared::service::clock::Clock;

use crate::shared::identity::IdentityWrapper;
use crate::usecase_error::UseCaseError;
use crate::user::dto::{
    GetOwnProfileInput, GetProfileInput, ListUsersInput, ListUsersOutput, SuspendUserInput,
    SuspendUserOutput, UpdateUserProfileInput,
};
use crate::user::service::UserService;

use super::dto::UserData;
use domain::auth::policy::{Actor as _, AuthorizationService, UserAction};
use domain::transaction::TransactionManager;
use domain::tx;
use domain::user::UserUniquenessService;

pub struct UserInteractor<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
    clock: Arc<dyn Clock>,
}

impl<TM: TransactionManager> UserInteractor<TM> {
    pub fn new(transaction_manager: Arc<TM>, clock: Arc<dyn Clock>) -> Self {
        Self {
            transaction_manager,
            clock,
        }
    }
}

#[async_trait]
impl<TM: TransactionManager> UserService for UserInteractor<TM> {
    #[tracing::instrument(skip(self), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn list_users(
        &self,
        identity: IdentityWrapper,
        _input: ListUsersInput,
    ) -> Result<ListUsersOutput, UseCaseError> {
        let users = tx!(self.transaction_manager, |factory| {
            // ポリシーチェック
            AuthorizationService::can(&identity, UserAction::ListUsers(ListUsersPayload))?;

            let user_repo = factory.user_repository();
            Ok::<_, UseCaseError>(user_repo.find_all().await?)
        })
        .await?;

        Ok(ListUsersOutput {
            users: users.into_iter().map(|u| u.into()).collect(),
        })
    }

    #[tracing::instrument(skip(self), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn get_own_profile(
        &self,
        identity: IdentityWrapper,
        input: GetOwnProfileInput,
    ) -> Result<UserData, UseCaseError> {
        let user = tx!(self.transaction_manager, |factory| {
            // プロフィールの取得
            let user_repo = factory.user_repository();
            let user = user_repo
                .find_by_id(input.user_id.into())
                .await?
                .ok_or(UseCaseError::NotFound)?;
            // ポリシーチェック
            AuthorizationService::can(
                &identity,
                UserAction::ViewOwnProfile(ViewOwnProfilePayload { target: &user }),
            )?;

            Ok::<_, UseCaseError>(user)
        })
        .await?;

        Ok(user.into())
    }

    #[tracing::instrument(skip(self), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn get_user_profile(
        &self,
        identity: IdentityWrapper,
        input: GetProfileInput,
    ) -> Result<UserData, UseCaseError> {
        let user = tx!(self.transaction_manager, |factory| {
            // プロフィールの取得
            let user_repo = factory.user_repository();
            let user = user_repo
                .find_by_id(input.user_id.into())
                .await?
                .ok_or(UseCaseError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                &identity,
                UserAction::ViewProfile(ViewProfilePayload { target: &user }),
            )?;

            Ok::<_, UseCaseError>(user)
        })
        .await?;

        Ok(user.into())
    }

    #[tracing::instrument(skip(self), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn update_user_profile(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserProfileInput,
    ) -> Result<UserData, UseCaseError> {
        let clock = self.clock.clone();

        let updated_user = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let user_uniqueness_service = UserUniquenessService::new(user_repo.clone());

            let mut user = user_repo
                .find_by_id(input.target_id.into())
                .await?
                .ok_or(UseCaseError::NotFound)?;

            if let Some(username) = input.username {
                // ポリシーチェック
                AuthorizationService::can(
                    &identity,
                    UserAction::UpdateProfile(UpdateProfilePayload { target: &user }),
                )?;

                // ユーザー名の重複チェック
                let username = user_uniqueness_service
                    .ensure_unique_username(&username)
                    .await?;

                // ドメインロジックの実行
                user.change_username(username, clock.as_ref())?;
            }
            if let Some(email) = input.email {
                // ポリシーチェック
                AuthorizationService::can(
                    &identity,
                    UserAction::ChangeEmail(ChangeEmailPayload { target: &user }),
                )?;

                // メールアドレスの重複チェック
                let email = user_uniqueness_service.ensure_unique_email(&email).await?;

                // ドメインロジックの実行
                user.change_email(email, clock.as_ref())?;
            }

            // 変更の保存
            let updated_user = user_repo.save(user).await?;

            Ok::<_, UseCaseError>(updated_user)
        })
        .await?;

        Ok(updated_user.into())
    }

    #[tracing::instrument(skip(self), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn suspend_user(
        &self,
        identity: IdentityWrapper,
        input: SuspendUserInput,
    ) -> Result<SuspendUserOutput, UseCaseError> {
        let clock = self.clock.clone();
        let SuspendUserInput { target_id, reason } = input;

        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            let mut target_user = user_repo
                .find_by_id(target_id.into())
                .await?
                .ok_or(UseCaseError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                &identity,
                UserAction::SuspendUser(SuspendUserPayload {
                    target: &target_user,
                }),
            )?;

            // ユーザーの状態を停止に変更
            target_user.suspend(reason, clock.as_ref())?;

            // 変更を保存
            user_repo.save(target_user).await?;

            Ok::<_, UseCaseError>(SuspendUserOutput {
                user_id: target_id,
                suspended: true,
            })
        })
        .await
    }
}
