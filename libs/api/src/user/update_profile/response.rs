use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UserData;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub(crate) struct UpdateProfileResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "exampleuser")]
    pub username: String,
    #[schema(example = "example@example.com")]
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
