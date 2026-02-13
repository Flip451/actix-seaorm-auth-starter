use crate::shared::identity::IdentityWrapper;
use crate::usecase_error::UseCaseError;
use crate::user::dto::{
    GetOwnProfileInput, GetProfileInput, ListUsersInput, ListUsersOutput, SuspendUserInput,
    SuspendUserOutput, UpdateUserEmailInput, UpdateUserEmailOutput, UpdateUserProfileInput,
    UpdateUserProfileOutput, UserDetailedProfile, UserPublicProfile,
};
use crate::user::service::UserService;
use async_trait::async_trait;
use domain::auth::policies::{
    change_email::ChangeEmailPayload, list_users::ListUsersPayload,
    suspend_user::SuspendUserPayload, update_profile::UpdateProfilePayload,
    view_detailed_profile::ViewDetailedProfilePayload,
    view_public_profile::ViewPublicProfilePayload,
};
use domain::auth::policy::{Actor as _, AuthorizationService, UserAction};
use domain::shared::service::clock::Clock;
use domain::transaction::TransactionManager;
use domain::tx;
use domain::user::UserUniquenessService;
use std::sync::Arc;
use validator::Validate as _;

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
    #[tracing::instrument(skip(self, identity), fields(
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

    #[tracing::instrument(skip(self, identity), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn get_own_profile(
        &self,
        identity: IdentityWrapper,
        _input: GetOwnProfileInput,
    ) -> Result<UserDetailedProfile, UseCaseError> {
        let user = tx!(self.transaction_manager, |factory| {
            // プロフィールの取得
            let user_repo = factory.user_repository();
            let user = user_repo
                .find_by_id(identity.actor_id())
                .await?
                .ok_or(UseCaseError::NotFound)?;
            // ポリシーチェック
            AuthorizationService::can(
                &identity,
                UserAction::ViewDetailedProfile(ViewDetailedProfilePayload { target: &user }),
            )?;

            Ok::<_, UseCaseError>(user)
        })
        .await?;

        Ok(user.into())
    }

    #[tracing::instrument(skip(self, identity), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn get_public_profile(
        &self,
        identity: IdentityWrapper,
        input: GetProfileInput,
    ) -> Result<UserPublicProfile, UseCaseError> {
        input.validate()?;

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
                UserAction::ViewPublicProfile(ViewPublicProfilePayload { target: &user }),
            )?;

            Ok::<_, UseCaseError>(user)
        })
        .await?;

        Ok(user.into())
    }

    #[tracing::instrument(skip(self, identity), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn update_user_profile(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserProfileInput,
    ) -> Result<UpdateUserProfileOutput, UseCaseError> {
        let clock = self.clock.clone();

        input.validate()?;

        let updated_user = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let user_uniqueness_service = UserUniquenessService::new(user_repo.clone());

            let mut user = user_repo
                .find_by_id(input.target_id.into())
                .await?
                .ok_or(UseCaseError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                &identity,
                UserAction::UpdateProfile(UpdateProfilePayload { target: &user }),
            )?;

            if let Some(username) = input.username {
                // ユーザー名の重複チェック
                let username = user_uniqueness_service
                    .ensure_unique_username(&username)
                    .await?;

                // ドメインロジックの実行
                user.change_username(username, clock.as_ref())?;
            }

            // 変更の保存
            let updated_user = user_repo.save(user).await?;

            Ok::<_, UseCaseError>(updated_user)
        })
        .await?;

        Ok(updated_user.into())
    }

    #[tracing::instrument(skip(self, identity), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn update_user_email(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserEmailInput,
    ) -> Result<UpdateUserEmailOutput, UseCaseError> {
        let clock = self.clock.clone();

        input.validate()?;

        let updated_user = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let user_uniqueness_service = UserUniquenessService::new(user_repo.clone());

            let mut user = user_repo
                .find_by_id(input.target_id.into())
                .await?
                .ok_or(UseCaseError::NotFound)?;

            // ポリシーチェック
            AuthorizationService::can(
                &identity,
                UserAction::ChangeEmail(ChangeEmailPayload { target: &user }),
            )?;

            // メールアドレスの重複チェック
            let email = user_uniqueness_service
                .ensure_unique_email(&input.new_email)
                .await?;

            // ドメインロジックの実行
            user.change_email(email, clock.as_ref())?;

            // 変更の保存
            let updated_user = user_repo.save(user).await?;

            Ok::<_, UseCaseError>(updated_user)
        })
        .await?;

        Ok(updated_user.into())
    }

    #[tracing::instrument(skip(self, identity), fields(
        actor_id = %identity.actor_id(),
        actor_role = %identity.actor_role(),
    ))]
    async fn suspend_user(
        &self,
        identity: IdentityWrapper,
        input: SuspendUserInput,
    ) -> Result<SuspendUserOutput, UseCaseError> {
        let clock = self.clock.clone();

        input.validate()?;

        let SuspendUserInput { target_id, reason } = input;

        let updated_user = tx!(self.transaction_manager, |factory| {
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
            let updated_user = user_repo.save(target_user).await?;

            Ok::<_, UseCaseError>(updated_user)
        })
        .await?;

        Ok(updated_user.into())
    }
}
