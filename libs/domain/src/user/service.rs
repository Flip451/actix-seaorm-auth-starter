use std::sync::Arc;

use thiserror::Error;

use crate::user::{
    EmailTrait, UnverifiedEmail, UserDomainError, UserId, UserRepository, UserRepositoryError,
    UserUniqueConstraintViolation, VerifiedEmail,
};

use super::{HashedPassword, RawPassword};

#[derive(Debug, Error)]
pub enum PasswordHashingError {
    #[error("パスワードのハッシュ化に失敗しました")]
    HashingFailed,
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, PasswordHashingError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> bool;
}

#[derive(Debug, Error, PartialEq)]
pub enum EmailVerificationError {
    // TODO: #35 でエラーの詳細を追加する
}

pub trait EmailVerifier {
    fn verify(&self, email: &UnverifiedEmail) -> Result<VerifiedEmail, EmailVerificationError>;
}

pub trait IdGenerator {
    fn generate(&self) -> UserId;
}

pub trait IdGeneratorFactory: Send + Sync {
    fn create_user_id_generator(&self) -> Box<dyn IdGenerator>;
}

pub struct UserUniquenessService<'a> {
    user_repo: Arc<dyn UserRepository + 'a>,
}

pub struct UniqueUserInfo {
    pub(crate) username: String,
    pub(crate) email: UnverifiedEmail,
}

pub struct UniqueEmail(pub(crate) UnverifiedEmail);

pub struct UniqueUsername(pub(crate) String);

impl<'a> UserUniquenessService<'a> {
    pub fn new(user_repo: Arc<dyn UserRepository + 'a>) -> Self {
        Self { user_repo }
    }

    pub async fn ensure_unique(
        &self,
        username: &str,
        email: &str,
    ) -> Result<UniqueUserInfo, UserRepositoryError> {
        let UniqueEmail(email) = self.ensure_unique_email(email).await?;
        let UniqueUsername(username) = self.ensure_unique_username(username).await?;

        Ok(UniqueUserInfo { username, email })
    }

    pub async fn ensure_unique_email(
        &self,
        email: &str,
    ) -> Result<UniqueEmail, UserRepositoryError> {
        if self.user_repo.find_by_email(email).await?.is_some() {
            Err(UserDomainError::AlreadyExists(
                UserUniqueConstraintViolation::Email {
                    duplicated_email: email.to_string(),
                },
            ))?;
        }

        Ok(UniqueEmail(UnverifiedEmail::new(email)?))
    }

    pub async fn ensure_unique_username(
        &self,
        username: &str,
    ) -> Result<UniqueUsername, UserRepositoryError> {
        if self.user_repo.find_by_username(username).await?.is_some() {
            Err(UserDomainError::AlreadyExists(
                UserUniqueConstraintViolation::Username {
                    duplicated_name: username.to_string(),
                },
            ))?;
        }

        Ok(UniqueUsername(username.to_string()))
    }
}
