use domain::{shared::outbox_event::OutboxRepositoryError, user::PasswordHashingError};

use crate::usecase_error::UseCaseError;

impl From<OutboxRepositoryError> for UseCaseError {
    fn from(error: OutboxRepositoryError) -> Self {
        match error {
            OutboxRepositoryError::Persistence(source) => UseCaseError::Internal(source),
        }
    }
}

impl From<PasswordHashingError> for UseCaseError {
    fn from(error: PasswordHashingError) -> Self {
        match error {
            PasswordHashingError::HashingFailed => UseCaseError::Internal(error.into()),
        }
    }
}
