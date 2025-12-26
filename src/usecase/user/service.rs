use std::sync::Arc;

use super::dto::UserResponse;
use super::error::UserError;
use crate::domain::user::UserRepository;

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn list_users(&self) -> Result<Vec<UserResponse>, UserError> {
        let users = self.user_repo.find_all().await?;

        Ok(users
            .into_iter()
            .map(|u| UserResponse {
                id: u.id,
                username: u.username,
                email: u.email.as_str().to_string(),
                role: u.role,
            })
            .collect())
    }
}
