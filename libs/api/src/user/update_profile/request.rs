use serde::Deserialize;
use usecase::user::dto::UpdateUserProfileInput;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    #[schema(example = "exampleuser")]
    pub username: Option<String>,

    #[validate(email(message = "無効なメールアドレス形式です"))]
    #[schema(example = "updated@example.com")]
    pub email: Option<String>,
}

impl UpdateProfileRequest {
    pub(super) fn into_input(self, target_id: Uuid) -> usecase::user::dto::UpdateUserProfileInput {
        UpdateUserProfileInput {
            target_id,
            username: self.username,
            email: self.email,
        }
    }
}
