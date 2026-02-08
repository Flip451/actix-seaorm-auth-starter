use domain::user::{User, UserId, UserRole};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserData {
    pub id: UserId,
    pub username: String,
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

#[derive(Debug)]
pub struct GetOwnProfileInput {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct GetProfileInput {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct ListUsersInput {
    // Add fields for filtering, pagination, etc. if needed
}

pub struct ListUsersOutput {
    pub users: Vec<UserData>,
}

#[derive(Debug)]
pub struct UpdateUserProfileInput {
    pub target_id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug)]
pub struct SuspendUserInput {
    pub target_id: Uuid,
    pub reason: String,
}

#[derive(Debug)]
pub struct SuspendUserOutput {
    pub user_id: Uuid,
    pub suspended: bool,
}
