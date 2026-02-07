use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UserData;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::impl_responder_for;

#[derive(Serialize, ToSchema)]
pub(crate) struct GetOwnProfileResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: Uuid,
    #[schema(example = "exampleuser")]
    pub username: String,
    #[schema(example = "user@example.com")]
    pub email: String,
}

impl From<UserData> for GetOwnProfileResponse {
    fn from(user: UserData) -> Self {
        GetOwnProfileResponse {
            user_id: user.id.into(),
            username: user.username,
            email: user.email,
        }
    }
}

impl_responder_for!(GetOwnProfileResponse, StatusCode::OK);
