use domain::user::UserRole;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
}
