use crate::domain::user::UserRole;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}
