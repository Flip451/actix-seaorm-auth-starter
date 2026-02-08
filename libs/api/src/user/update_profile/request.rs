use serde::Deserialize;
use usecase::user::dto::UpdateUserProfileInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    #[cfg_attr(feature = "api-docs", schema(example = "exampleuser"))]
    pub username: Option<String>,

    #[validate(email(message = "無効なメールアドレス形式です"))]
    #[cfg_attr(feature = "api-docs", schema(example = "updated@example.com"))]
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
