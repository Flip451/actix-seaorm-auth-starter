use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UserProfile;
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
}

impl From<UserProfile> for GetProfileResponse {
    fn from(user: UserProfile) -> Self {
        GetProfileResponse {
            user_id: user.id,
            username: user.username,
        }
    }
}

impl_responder_for!(GetProfileResponse, StatusCode::OK);
