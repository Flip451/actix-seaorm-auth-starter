use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use crate::usecase::auth::error::AuthError;
use crate::usecase::user::error::UserError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    User(#[from] UserError),
}

// AppError 自体の実装は「中身に任せる」という一貫したルールにする
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Auth(e) => e.status_code(),
            AppError::User(e) => e.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Auth(e) => e.error_response(),
            AppError::User(e) => e.error_response(),
        }
    }
}