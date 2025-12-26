use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::domain::user::UserRole;

use super::value_object::{Email, HashedPassword};

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Email,
    pub password: HashedPassword,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
