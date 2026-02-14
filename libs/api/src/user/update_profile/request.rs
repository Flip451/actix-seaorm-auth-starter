use serde::Deserialize;
use usecase::user::dto::UpdateUserProfileInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(derive_more::Debug, Deserialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct UpdateProfileRequest {
    #[cfg_attr(feature = "api-docs", schema(examples("exampleuser")))]
    pub username: Option<String>,
    // 他に更新可能なプロフィール情報があればここに追加
}

impl UpdateProfileRequest {
    pub(super) fn into_input(self, target_id: Uuid) -> UpdateUserProfileInput {
        UpdateUserProfileInput {
            target_id,
            username: self.username,
        }
    }
}
