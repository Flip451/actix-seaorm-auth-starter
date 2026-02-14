use domain::user::User;
use uuid::Uuid;
use validator::Validate;

use crate::shared::identity::UserRoleData;

#[derive(derive_more::Debug)]
pub struct UserDetailedProfile {
    pub user_id: Uuid,
    pub username: String,
    #[debug(skip)]
    pub email: String,
    pub role: UserRoleData,
}

impl From<User> for UserDetailedProfile {
    fn from(user: User) -> Self {
        UserDetailedProfile {
            user_id: user.id().into(),
            username: user.username().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role().into(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct GetOwnProfileInput;

#[derive(derive_more::Debug)]
pub struct GetProfileInput {
    pub user_id: Uuid,
}

#[derive(derive_more::Debug)]
pub struct ListUsersInput {
    // Add fields for filtering, pagination, etc. if needed
}

#[derive(derive_more::Debug)]
pub struct ListUsersOutput {
    pub users: Vec<UserItem>,
}

#[derive(derive_more::Debug)]
pub struct UserItem {
    pub user_id: Uuid,
    pub username: String,
    #[debug(skip)]
    pub email: String,
    pub role: UserRoleData,
}

impl From<User> for UserItem {
    fn from(user: User) -> Self {
        UserItem {
            user_id: user.id().into(),
            username: user.username().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role().into(),
        }
    }
}

#[derive(derive_more::Debug, Validate)]
#[validate(schema(function = "validate_at_least_one_field"))]
pub struct UpdateUserProfileInput {
    pub target_id: Uuid,
    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    pub username: Option<String>,
}

impl UpdateUserProfileInput {
    fn is_empty(&self) -> bool {
        self.username.is_none()
    }
}

fn validate_at_least_one_field(
    input: &UpdateUserProfileInput,
) -> Result<(), validator::ValidationError> {
    if input.is_empty() {
        let mut error = validator::ValidationError::new("at_least_one_field_required");
        error.message = Some("少なくとも1つの項目を変更してください".into());
        return Err(error);
    }
    Ok(())
}

#[derive(derive_more::Debug, Validate)]
pub struct UpdateUserEmailInput {
    pub target_id: Uuid,
    #[debug(skip)]
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    pub new_email: String,
}

#[derive(derive_more::Debug, Validate)]
pub struct SuspendUserInput {
    pub target_id: Uuid,
    #[validate(length(min = 1, message = "理由を入力してください"))]
    pub reason: String,
}

#[derive(derive_more::Debug)]
pub struct SuspendUserOutput {
    pub user_id: Uuid,
    pub suspended: bool,
}

impl From<User> for SuspendUserOutput {
    fn from(user: User) -> Self {
        SuspendUserOutput {
            user_id: user.id().into(),
            suspended: user.is_suspended(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct UserPublicProfile {
    pub user_id: Uuid,
    pub username: String,
    pub role: UserRoleData,
}

impl From<User> for UserPublicProfile {
    fn from(user: User) -> Self {
        UserPublicProfile {
            user_id: user.id().into(),
            username: user.username().to_string(),
            role: user.role().into(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct UpdateUserProfileOutput {
    pub user_id: Uuid,
    pub username: String,
}

impl From<User> for UpdateUserProfileOutput {
    fn from(user: User) -> Self {
        UpdateUserProfileOutput {
            user_id: user.id().into(),
            username: user.username().to_string(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct UpdateUserEmailOutput {
    pub user_id: Uuid,
    #[debug(skip)]
    pub email: String,
}

impl From<User> for UpdateUserEmailOutput {
    fn from(user: User) -> Self {
        UpdateUserEmailOutput {
            user_id: user.id().into(),
            email: user.email().as_str().to_string(),
        }
    }
}
