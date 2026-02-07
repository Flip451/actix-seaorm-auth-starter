use serde::Deserialize;
use usecase::user::dto::SuspendUserInput;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct SuspendUserRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub target_id: Uuid,

    #[validate(length(min = 1, message = "理由は空にできません"))]
    #[schema(example = "規約違反")]
    pub reason: String,
}

impl From<SuspendUserRequest> for SuspendUserInput {
    fn from(req: SuspendUserRequest) -> Self {
        SuspendUserInput {
            target_id: req.target_id,
            reason: req.reason,
        }
    }
}
