use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;
use validator::ValidationErrors;

use crate::usecase::auth::error::AuthError;

#[derive(Debug, Error)]
pub enum ApiAuthError {
    #[error("入力の形式が不正です: {0}")]
    InvalidInput(ValidationErrors),

    #[error(transparent)]
    AuthError(#[from] AuthError),
}

impl ResponseError for ApiAuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiAuthError::InvalidInput(_validation_errors) => StatusCode::BAD_REQUEST,
            ApiAuthError::AuthError(auth_error) => match auth_error {
                AuthError::InvalidEmail(_) => StatusCode::BAD_REQUEST,
                AuthError::PasswordTooShort => StatusCode::BAD_REQUEST,
                AuthError::PasswordHashingFailed(_error) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                AuthError::EmailAlreadyExists => StatusCode::CONFLICT,
                AuthError::UsernameAlreadyExists => StatusCode::CONFLICT,
                AuthError::Forbidden => StatusCode::FORBIDDEN,
                AuthError::TxError(_error) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::PersistenceError(_error) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::TokenNotDetected => StatusCode::UNAUTHORIZED,
                AuthError::TokenIssuanceFailed(_error) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "status": "error",
            "message": self.to_string()
        }))
    }
}
