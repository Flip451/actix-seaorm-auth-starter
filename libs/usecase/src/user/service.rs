use async_trait::async_trait;

use crate::{
    shared::identity::IdentityWrapper,
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
        identity: IdentityWrapper,
        input: ListUsersInput,
    ) -> Result<ListUsersOutput, UseCaseError>;

    async fn get_own_profile(
        &self,
        identity: IdentityWrapper,
        input: GetOwnProfileInput,
    ) -> Result<UserDetailedProfile, UseCaseError>;

    async fn get_public_profile(
        &self,
        identity: IdentityWrapper,
        input: GetProfileInput,
    ) -> Result<UserPublicProfile, UseCaseError>;

    async fn update_user_profile(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserProfileInput,
    ) -> Result<UpdateUserProfileOutput, UseCaseError>;

    async fn update_user_email(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserEmailInput,
    ) -> Result<UpdateUserEmailOutput, UseCaseError>;

    async fn suspend_user(
        &self,
        identity: IdentityWrapper,
        input: SuspendUserInput,
    ) -> Result<SuspendUserOutput, UseCaseError>;
}
