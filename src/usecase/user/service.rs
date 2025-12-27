use std::sync::Arc;

use uuid::Uuid;

use super::dto::UserResponse;
use super::error::UserError;
use crate::domain::transaction::TransactionManager;
use crate::tx;

pub struct UserService<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
}

impl<TM: TransactionManager> UserService<TM> {
    pub fn new(transaction_manager: Arc<TM>) -> Self {
        Self {
            transaction_manager,
        }
    }

    pub async fn list_users(&self) -> Result<Vec<UserResponse>, UserError> {
        let users = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            Ok::<_, UserError>(user_repo.find_all().await?)
        })
        .await?;

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

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserResponse, UserError> {
        let user = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let user = user_repo.find_by_id(user_id).await?;
            Ok::<_, UserError>(user)
        })
        .await?
        .ok_or(UserError::NotFound)?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email.as_str().to_string(),
            role: user.role,
        })
    }
}
