use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UserPublicProfile;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

use crate::impl_responder_for;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct GetProfileResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(example = "550e8400-e29b-41d4-a716-446655440000")
    )]
    pub user_id: Uuid,
    #[cfg_attr(feature = "api-docs", schema(example = "exampleuser"))]
    pub username: String,
    #[cfg_attr(feature = "api-docs", schema(example = "user"))]
    pub role: String,
}

impl From<UserPublicProfile> for GetProfileResponse {
    fn from(user: UserPublicProfile) -> Self {
        let UserPublicProfile { id, username, role } = user;

        GetProfileResponse {
            user_id: id,
            username,
            role: role.to_string(),
        }
    }
}

impl_responder_for!(GetProfileResponse, StatusCode::OK);
