use serde::Deserialize;
use usecase::user::dto::SuspendUserInput;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(derive_more::Debug, Deserialize, Validate, ToSchema)]
pub struct SuspendUserRequest {
    #[validate(length(min = 1, message = "理由は空にできません"))]
    #[schema(example = "規約違反")]
    pub reason: String,
}

impl SuspendUserRequest {
    pub(super) fn into_input(self, target_id: Uuid) -> SuspendUserInput {
        SuspendUserInput {
            target_id,
            reason: self.reason,
        }
    }
}
