use serde::Deserialize;
use usecase::user::dto::GetProfileInput;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct GetProfileRequest {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: Uuid,
}

impl From<GetProfileRequest> for GetProfileInput {
    fn from(req: GetProfileRequest) -> Self {
        Self {
            user_id: req.user_id,
        }
    }
}
