use serde::Deserialize;
use usecase::auth::dto::SignupInput;
use utoipa::ToSchema;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct SignupRequest {
    #[schema(example = "exampleuser")]
    pub username: String,

    #[validate(email(message = "無効なメールアドレス形式です"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
    #[schema(example = "password123")]
    #[debug(skip)]
    pub password: String,
}

impl From<SignupRequest> for SignupInput {
    fn from(req: SignupRequest) -> Self {
        Self {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}
