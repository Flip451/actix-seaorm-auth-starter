use async_trait::async_trait;

use crate::{
    shared::identity::IdentityWrapper,
    usecase_error::UseCaseError,
    user::dto::{
        GetProfileInput, ListUsersInput, ListUsersOutput, SuspendUserInput, SuspendUserOutput,
        UpdateUserInput, UserData,
    },
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn list_users(
        &self,
        identity: IdentityWrapper,
        input: ListUsersInput,
    ) -> Result<ListUsersOutput, UseCaseError>;

    async fn get_user_by_id(
        &self,
        identity: IdentityWrapper,
        input: GetProfileInput,
    ) -> Result<UserData, UseCaseError>;

    async fn update_user(
        &self,
        identity: IdentityWrapper,
        input: UpdateUserInput,
    ) -> Result<UserData, UseCaseError>;

    async fn suspend_user(
        &self,
        identity: IdentityWrapper,
        input: SuspendUserInput,
    ) -> Result<SuspendUserOutput, UseCaseError>;
}
