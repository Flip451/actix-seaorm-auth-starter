use serde::Deserialize;
use usecase::user::dto::GetProfileInput;
use uuid::Uuid;
use validator::Validate;

#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct GetProfileRequest {
    // Add query parameters here if needed
}

impl GetProfileRequest {
    pub(super) fn into_input(self, user_id: Uuid) -> GetProfileInput {
        GetProfileInput { user_id }
    }
}
