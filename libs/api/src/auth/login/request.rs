use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "無効なメールアドレス形式です"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
    #[schema(example = "password123")]
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
