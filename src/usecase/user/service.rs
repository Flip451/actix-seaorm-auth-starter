use std::sync::Arc;

use uuid::Uuid;

use super::dto::UserResponse;
use super::error::UserError;
use crate::domain::transaction::TransactionManager;
use crate::domain::user::{EmailTrait, UnverifiedEmail};
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
                id: u.id(),
                username: u.username().to_string(),
                email: u.email().as_str().to_string(),
                role: u.role().clone(),
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
            id: user.id(),
            username: user.username().to_string(),
            email: user.email().as_str().to_string(),
            role: user.role().clone(),
        })
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        input: super::dto::UpdateUserInput,
    ) -> Result<UserResponse, UserError> {
        let updated_user = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let mut user = user_repo
                .find_by_id(user_id)
                .await?
                .ok_or(UserError::NotFound)?;

            if let Some(username) = input.username {
                user.change_username(username.clone())?;
                if let Some(_) = user_repo.find_by_username(&username).await? {
                    return Err(UserError::UsernameAlreadyExists(username));
                }
            }
            if let Some(email) = input.email {
                user.change_email(UnverifiedEmail::new(&email)?)?;
                if let Some(_) = user_repo.find_by_email(&email).await? {
                    return Err(UserError::EmailAlreadyExists(email));
                }
            }

            let updated_user = user_repo.save(user).await?;
            Ok::<_, UserError>(updated_user)
        })
        .await?;

        Ok(UserResponse {
            id: updated_user.id(),
            username: updated_user.username().to_string(),
            email: updated_user.email().as_str().to_string(),
            role: updated_user.role().clone(),
        })
    }
}
