use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UserData;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct UpdateProfileResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(example = "550e8400-e29b-41d4-a716-446655440000")
    )]
    pub id: Uuid,
    #[cfg_attr(feature = "api-docs", schema(example = "exampleuser"))]
    pub username: String,
    #[cfg_attr(feature = "api-docs", schema(example = "example@example.com"))]
    pub email: String,
}

impl From<UserData> for UpdateProfileResponse {
    fn from(user: UserData) -> Self {
        UpdateProfileResponse {
            id: user.id.into(),
            username: user.username,
            email: user.email,
        }
    }
}

crate::impl_responder_for!(UpdateProfileResponse, StatusCode::OK);
