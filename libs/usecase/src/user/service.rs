use async_trait::async_trait;

use crate::{
    shared::identity::IdentityWrapper,
    usecase_error::UseCaseError,
    user::dto::{
        GetOwnProfileInput, GetProfileInput, ListUsersInput, ListUsersOutput, SuspendUserInput,
        SuspendUserOutput, UpdateUserEmailInput, UpdateUserProfileInput, UserData,
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
    ) -> Result<UserData, UseCaseError>;

    async fn get_user_profile(
        &self,
        identity: IdentityWrapper,
        input: GetProfileInput,
    ) -> Result<UserData, UseCaseError>;

    async fn update_user_profile(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserProfileInput,
    ) -> Result<UserData, UseCaseError>;

    async fn update_user_email(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserEmailInput,
    ) -> Result<UserData, UseCaseError>;

    async fn suspend_user(
        &self,
        identity: IdentityWrapper,
        input: SuspendUserInput,
    ) -> Result<SuspendUserOutput, UseCaseError>;
}
