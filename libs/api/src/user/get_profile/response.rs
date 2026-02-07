use serde::Serialize;
use usecase::user::dto::UserData;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::impl_responder_for;

#[derive(Serialize, ToSchema)]
pub struct GetProfileResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: Uuid,
    #[schema(example = "exampleuser")]
    pub username: String,
    #[schema(example = "user@example.com")]
    pub email: String,
}

impl From<UserData> for GetProfileResponse {
    fn from(user: UserData) -> Self {
        GetProfileResponse {
            user_id: user.id.into(),
            username: user.username,
            email: user.email,
        }
    }
}

impl_responder_for!(GetProfileResponse);
