use serde::Deserialize;
use usecase::user::dto::ListUsersInput;
use utoipa::ToSchema;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
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
