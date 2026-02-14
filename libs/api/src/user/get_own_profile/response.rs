use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UserDetailedProfile;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct GetOwnProfileResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(examples("550e8400-e29b-41d4-a716-446655440000"))
    )]
    pub user_id: Uuid,
    #[cfg_attr(feature = "api-docs", schema(examples("exampleuser")))]
    pub username: String,
    #[cfg_attr(feature = "api-docs", schema(examples("user@example.com")))]
    pub email: String,
}

impl From<UserDetailedProfile> for GetOwnProfileResponse {
    fn from(user: UserDetailedProfile) -> Self {
        GetOwnProfileResponse {
            user_id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

crate::impl_responder_for!(GetOwnProfileResponse, StatusCode::OK);
