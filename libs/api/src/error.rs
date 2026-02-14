use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

use usecase::usecase_error::UseCaseError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("認証が必要です")]
    Unauthorized,

    #[error("権限が足りていません")]
    Forbidden,

    #[error(transparent)]
    UseCaseError(#[from] UseCaseError),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::UseCaseError(usecase_error) => match usecase_error {
                UseCaseError::InvalidInput(_validation_errors) => StatusCode::BAD_REQUEST,
                UseCaseError::Unauthorized => StatusCode::UNAUTHORIZED,
                UseCaseError::Forbidden { message: _ } => StatusCode::FORBIDDEN,
                UseCaseError::NotFound => StatusCode::NOT_FOUND,
                UseCaseError::Conflict { message: _ } => StatusCode::CONFLICT,
                UseCaseError::Internal(_error) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::UseCaseError(UseCaseError::InvalidInput(errors)) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "status": "error",
                    "code": 400,
                    "message": "入力内容に誤りがあります",
                    "errors": errors,
                }))
            }
            // Internalエラー（予期せぬ技術的エラー）の場合
            ApiError::UseCaseError(UseCaseError::Internal(e)) => {
                // 1. 構造化ログとしてエラー詳細を出力する
                // `?e` (Debugフォーマット) を使うことで、anyhowが保持する
                // エラーチェーン（原因の連鎖）とバックトレースを記録します。
                tracing::error!(
                    error = ?e,
                    "Internal Server Error occurred: An unexpected error was caught at the API boundary."
                );

                // 2. ユーザーには詳細を見せず、汎用的なメッセージのみを返す
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "code": 500,
                    "message": "Internal Server Error"
                }))
            }

            // その他のエラー（バリデーションエラーやビジネスルール違反）
            // これらはクライアントに理由を伝えるべきなので、メッセージを含める
            other => HttpResponse::build(self.status_code()).json(serde_json::json!({
                "status": "error",
                "code": self.status_code().as_u16(),
                "message": other.to_string()
            })),
        }
    }
}
