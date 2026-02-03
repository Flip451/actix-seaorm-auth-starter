use domain::user::PasswordHashingError;

use crate::usecase_error::UseCaseError;

impl From<PasswordHashingError> for UseCaseError {
    fn from(error: PasswordHashingError) -> Self {
        match error {
            PasswordHashingError::HashingFailed => UseCaseError::Internal(error.into()),
        }
    }
}
