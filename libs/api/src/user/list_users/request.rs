use serde::Deserialize;
use usecase::user::dto::ListUsersInput;
use utoipa::ToSchema;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct ListUsersQuery {
    // Define query parameters here, e.g.:
    // pub page: Option<u32>,
    // pub per_page: Option<u32>,
}

impl From<ListUsersQuery> for ListUsersInput {
    fn from(_query: ListUsersQuery) -> Self {
        ListUsersInput {
            // Map fields from ListUsersQuery to ListUsersInput here
        }
    }
}
