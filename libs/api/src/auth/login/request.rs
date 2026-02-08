use serde::Deserialize;
use validator::Validate;

#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct LoginRequest {
    #[validate(email(message = "無効なメールアドレス形式です"))]
    #[cfg_attr(feature = "api-docs", schema(example = "user@example.com"))]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
    #[cfg_attr(feature = "api-docs", schema(example = "password123"))]
    #[debug(skip)]
    pub password: String,
}

impl From<LoginRequest> for usecase::auth::dto::LoginInput {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}
