use serde::Deserialize;
use usecase::user::dto::GetProfileInput;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct GetProfileRequest {
    // Add query parameters here if needed
}

impl GetProfileRequest {
    pub(super) fn into_input(self, user_id: Uuid) -> GetProfileInput {
        GetProfileInput { user_id }
    }
}
