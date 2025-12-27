use std::sync::Arc;

use super::dto::UserResponse;
use super::error::UserError;
use crate::domain::transaction::TransactionManager;
use crate::domain::user::User;

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
        let users = self
            .transaction_manager
            .execute::<Vec<User>, UserError, _>(move |factory| {
                Box::pin(async move {
                    let user_repo = factory.user_repository();
                    Ok(user_repo.find_all().await?)
                })
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
}
