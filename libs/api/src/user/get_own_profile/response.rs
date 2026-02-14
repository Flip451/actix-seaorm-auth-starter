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
    #[cfg_attr(feature = "api-docs", schema(examples("admin", "user")))]
    pub role: String,
}

impl From<UserDetailedProfile> for GetOwnProfileResponse {
    fn from(user: UserDetailedProfile) -> Self {
        let UserDetailedProfile {
            user_id,
            username,
            email,
            role,
        } = user;

        GetOwnProfileResponse {
            user_id,
            username,
            email,
            role: role.to_string(),
        }
    }
}

crate::impl_responder_for!(GetOwnProfileResponse, StatusCode::OK);
