use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub target_id: Uuid,

    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    #[schema(example = "exampleuser")]
    pub username: Option<String>,

    #[validate(email(message = "無効なメールアドレス形式です"))]
    #[schema(example = "updated@example.com")]
    pub email: Option<String>,
}

impl From<UpdateProfileRequest> for usecase::user::dto::UpdateUserInput {
    fn from(req: UpdateProfileRequest) -> Self {
        Self {
            target_id: req.target_id,
            username: req.username,
            email: req.email,
        }
    }
}
