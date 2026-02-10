use domain::user::User;
use uuid::Uuid;

use crate::{
    shared::identity::UserRoleData,
    usecase_error::{UseCaseError, ValidationError},
};

#[derive(derive_more::Debug)]
pub struct UserDetailedProfile {
    pub id: Uuid,
    pub username: String,
    #[debug(skip)]
    pub email: String,
    pub role: UserRoleData,
}

impl From<User> for UserDetailedProfile {
    fn from(user: User) -> Self {
        UserDetailedProfile {
            id: user.id().into(),
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
    pub id: Uuid,
    pub username: String,
    pub role: UserRoleData,
}

impl From<User> for UserItem {
    fn from(user: User) -> Self {
        UserItem {
            id: user.id().into(),
            username: user.username().to_string(),
            role: user.role().into(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct UpdateUserProfileInput {
    pub target_id: Uuid,
    pub username: Option<String>,
}

impl UpdateUserProfileInput {
    fn is_empty(&self) -> bool {
        [
            self.username.is_none(),
            // Add other optional fields here
        ]
        .iter()
        .all(|x| *x)
    }

    pub(crate) fn validate_not_empty(&self) -> Result<(), UseCaseError> {
        if self.is_empty() {
            let fields = vec![
                "username",
                // Add other optional fields here
            ];
            return Err(UseCaseError::InvalidInput(
                fields
                    .into_iter()
                    .map(|field| {
                        ValidationError::new(field, "少なくとも1つのフィールドを指定してください")
                    })
                    .collect(),
            ));
        }

        Ok(())
    }
}

#[derive(derive_more::Debug)]
pub struct UpdateUserEmailInput {
    pub target_id: Uuid,
    #[debug(skip)]
    pub new_email: String,
}

#[derive(derive_more::Debug)]
pub struct SuspendUserInput {
    pub target_id: Uuid,
    #[debug(skip)]
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
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub role: UserRoleData,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        UserProfile {
            id: user.id().into(),
            username: user.username().to_string(),
            role: user.role().into(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct UpdateUserProfileOutput {
    pub id: Uuid,
    pub username: String,
}

impl From<User> for UpdateUserProfileOutput {
    fn from(user: User) -> Self {
        UpdateUserProfileOutput {
            id: user.id().into(),
            username: user.username().to_string(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct UpdateUserEmailOutput {
    pub user_id: Uuid,
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
