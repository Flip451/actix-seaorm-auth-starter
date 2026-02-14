use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UpdateUserProfileOutput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct UpdateProfileResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(examples("550e8400-e29b-41d4-a716-446655440000"))
    )]
    pub user_id: Uuid,
    #[cfg_attr(feature = "api-docs", schema(examples("exampleuser")))]
    pub username: String,
}

impl From<UpdateUserProfileOutput> for UpdateProfileResponse {
    fn from(user: UpdateUserProfileOutput) -> Self {
        let UpdateUserProfileOutput { user_id, username } = user;

        UpdateProfileResponse { user_id, username }
    }
}

crate::impl_responder_for!(UpdateProfileResponse, StatusCode::OK);
