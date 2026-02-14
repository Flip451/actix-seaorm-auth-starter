use serde::Deserialize;
use usecase::user::dto::ListUsersInput;
#[cfg(feature = "api-docs")]
use utoipa::{IntoParams, ToSchema};

#[derive(derive_more::Debug, Deserialize)]
#[cfg_attr(feature = "api-docs", derive(IntoParams, ToSchema))]
pub struct ListUsersRequest {
    // Define query parameters here, e.g.:
    // pub page: Option<u32>,
    // pub per_page: Option<u32>,
}

impl From<ListUsersRequest> for ListUsersInput {
    fn from(_req: ListUsersRequest) -> Self {
        ListUsersInput {
            // Map fields from ListUsersRequest to ListUsersInput here
        }
    }
}
