use domain::user::{User, UserId, UserRole};
use uuid::Uuid;

#[derive(derive_more::Debug)]
pub struct UserData {
    pub id: UserId,
    pub username: String,
    #[debug(skip)]
    pub email: String,
    pub role: UserRole,
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            id: user.id(),
            username: user.username().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role(),
        }
    }
}

#[derive(derive_more::Debug)]
pub struct GetOwnProfileInput {
    pub user_id: Uuid,
}

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
    pub users: Vec<UserData>,
}

#[derive(derive_more::Debug)]
pub struct UpdateUserProfileInput {
    pub target_id: Uuid,
    pub username: Option<String>,
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
