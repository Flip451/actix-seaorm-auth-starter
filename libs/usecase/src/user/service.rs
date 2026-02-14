use async_trait::async_trait;

use crate::{
    shared::identity::Identity,
    usecase_error::UseCaseError,
    user::dto::{
        GetOwnProfileInput, GetProfileInput, ListUsersInput, ListUsersOutput, SuspendUserInput,
        SuspendUserOutput, UpdateUserEmailInput, UpdateUserEmailOutput, UpdateUserProfileInput,
        UpdateUserProfileOutput, UserDetailedProfile, UserPublicProfile,
    },
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn list_users(
        &self,
        identity: Box<dyn Identity>,
        input: ListUsersInput,
    ) -> Result<ListUsersOutput, UseCaseError>;

    async fn get_own_profile(
        &self,
        identity: Box<dyn Identity>,
        input: GetOwnProfileInput,
    ) -> Result<UserDetailedProfile, UseCaseError>;

    async fn get_public_profile(
        &self,
        identity: Box<dyn Identity>,
        input: GetProfileInput,
    ) -> Result<UserPublicProfile, UseCaseError>;

    async fn update_user_profile(
        &self,
        identity: Box<dyn Identity>,
        input: UpdateUserProfileInput,
    ) -> Result<UpdateUserProfileOutput, UseCaseError>;

    async fn update_user_email(
        &self,
        identity: Box<dyn Identity>,
        input: UpdateUserEmailInput,
    ) -> Result<UpdateUserEmailOutput, UseCaseError>;

    async fn suspend_user(
        &self,
        identity: Box<dyn Identity>,
        input: SuspendUserInput,
    ) -> Result<SuspendUserOutput, UseCaseError>;
}
