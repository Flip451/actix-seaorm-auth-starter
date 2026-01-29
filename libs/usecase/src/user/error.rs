use domain::user::{
    EmailFormatError, EmailVerificationError, ModificationWithInvalidStateError,
    PasswordPolicyViolation, UserDomainError, UserReconstructionError, UserRepositoryError,
    UserStateTransitionError, UserUniqueConstraintViolation,
};

use crate::usecase_error::{UseCaseError, ValidationError};

impl From<UserRepositoryError> for UseCaseError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::DomainError(user_domain_error) => user_domain_error.into(),
            UserRepositoryError::ReconstructionError(user_reconstruction_error) => {
                user_reconstruction_error.into()
            }
            UserRepositoryError::Persistence(error) => UseCaseError::Internal(error),
        }
    }
}

impl From<UserDomainError> for UseCaseError {
    fn from(domain_error: UserDomainError) -> Self {
        match domain_error {
            UserDomainError::InvalidEmail(email_format_error) => email_format_error.into(),
            UserDomainError::PasswordPolicyViolation(password_policy_violation) => {
                password_policy_violation.into()
            }
            UserDomainError::AlreadyExists(user_unique_constraint_violation) => {
                user_unique_constraint_violation.into()
            }
            UserDomainError::EmailVerificationError(email_verification_error) => {
                email_verification_error.into()
            }
            UserDomainError::ModificationWithInvalidStateError(
                modification_with_invalid_state_error,
            ) => modification_with_invalid_state_error.into(),
            UserDomainError::StateTransitionError(user_state_transition_error) => {
                user_state_transition_error.into()
            }
        }
    }
}

impl From<UserReconstructionError> for UseCaseError {
    fn from(reconstruction_error: UserReconstructionError) -> Self {
        UseCaseError::Internal(reconstruction_error.into())
    }
}

impl From<EmailFormatError> for UseCaseError {
    fn from(email_format_error: EmailFormatError) -> Self {
        match email_format_error {
            EmailFormatError::InvalidFormat {
                invalid_email,
                error: _,
            } => UseCaseError::InvalidInput(vec![ValidationError::new(
                "email",
                format!("メールアドレスの形式として不正です: {invalid_email}"),
            )]),
        }
    }
}

impl From<PasswordPolicyViolation> for UseCaseError {
    fn from(violation: PasswordPolicyViolation) -> Self {
        match violation {
            PasswordPolicyViolation::TooShort => {
                UseCaseError::InvalidInput(vec![ValidationError::new(
                    "password",
                    "パスワードが短すぎます".to_string(),
                )])
            }
        }
    }
}

impl From<UserUniqueConstraintViolation> for UseCaseError {
    fn from(violation: UserUniqueConstraintViolation) -> Self {
        match violation {
            UserUniqueConstraintViolation::Username { duplicated_name } => UseCaseError::Conflict {
                message: format!("ユーザー名 '{duplicated_name}' は既に使用されています"),
            },
            UserUniqueConstraintViolation::Email { duplicated_email } => UseCaseError::Conflict {
                message: format!("メールアドレス '{duplicated_email}' は既に使用されています"),
            },
        }
    }
}

impl From<EmailVerificationError> for UseCaseError {
    fn from(email_verification_error: EmailVerificationError) -> Self {
        match email_verification_error {}
    }
}

impl From<ModificationWithInvalidStateError> for UseCaseError {
    fn from(invalid_state_error: ModificationWithInvalidStateError) -> Self {
        match invalid_state_error {
            ModificationWithInvalidStateError::EmailModification { state: _ }
            | ModificationWithInvalidStateError::UsernameModification { state: _ } => {
                UseCaseError::Conflict {
                    message: invalid_state_error.message_for_client().to_string(),
                }
            }
        }
    }
}

impl From<UserStateTransitionError> for UseCaseError {
    fn from(invalid_transition_error: UserStateTransitionError) -> Self {
        let message = match invalid_transition_error {
            UserStateTransitionError::AlreadyDeactivated { to: _ } => {
                "ユーザーは既に退会しています".to_string()
            }
            UserStateTransitionError::AlreadySuspended { to: _ } => {
                "ユーザーは既に停止されています".to_string()
            }
            UserStateTransitionError::NotVerified { from: _ } => {
                "ユーザーのメールアドレスが未検証です".to_string()
            }
            UserStateTransitionError::NotSuspended { from: _ } => {
                "指定のユーザーは停止されていません：".to_string()
            }
        };

        UseCaseError::Conflict { message }
    }
}
