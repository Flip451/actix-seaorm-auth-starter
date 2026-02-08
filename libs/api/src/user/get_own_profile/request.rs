use serde::Deserialize;
use usecase::user::dto::GetOwnProfileInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct GetOwnProfileRequest {
    // Add query parameters here if needed
}

impl GetOwnProfileRequest {
    pub(super) fn into_input(self, user_id: Uuid) -> GetOwnProfileInput {
        GetOwnProfileInput { user_id }
    }
}
