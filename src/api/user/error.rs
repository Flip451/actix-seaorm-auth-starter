use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;
use validator::ValidationErrors;

use crate::usecase::user::error::UserError;

#[derive(Debug, Error)]
pub enum ApiUserError {
    #[error("入力の形式が不正です: {0}")]
    InvalidInput(ValidationErrors),

    #[error(transparent)]
    UserError(#[from] UserError),
}

impl ResponseError for ApiUserError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiUserError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            ApiUserError::UserError(user_error) => {
                match user_error {
                    UserError::InvalidInput(_) => StatusCode::BAD_REQUEST,
                    UserError::TxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    UserError::PersistenceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    UserError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    UserError::NotFound => StatusCode::NOT_FOUND,
                    UserError::UsernameAlreadyExists(_) => StatusCode::CONFLICT,
                    UserError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
                    UserError::EmailVerificationError(_) => StatusCode::BAD_REQUEST,
                    UserError::StateTransitionError(_) => StatusCode::CONFLICT,
                    UserError::AuthorizationError(_) => StatusCode::FORBIDDEN,
                }
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
