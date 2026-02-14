use serde::Deserialize;
use usecase::user::dto::GetOwnProfileInput;
#[cfg(feature = "api-docs")]
use utoipa::{IntoParams, ToSchema};

#[derive(derive_more::Debug, Deserialize)]
#[cfg_attr(feature = "api-docs", derive(IntoParams, ToSchema))]
pub struct GetOwnProfileRequest {
    // Add query parameters here if needed
}

impl GetOwnProfileRequest {
    pub(super) fn into_input(self) -> GetOwnProfileInput {
        GetOwnProfileInput
    }
}
