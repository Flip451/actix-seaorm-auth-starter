use actix_web::{HttpResponse, ResponseError, http::StatusCode};

use crate::usecase::user::error::UserError;

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "status": "error",
            "message": self.to_string()
        }))
    }
}
