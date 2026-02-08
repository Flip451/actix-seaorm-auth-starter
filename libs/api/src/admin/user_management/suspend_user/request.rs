use serde::Deserialize;
use usecase::user::dto::SuspendUserInput;
use uuid::Uuid;
use validator::Validate;

#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize, Validate)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct SuspendUserRequest {
    #[validate(length(min = 1, message = "理由は空にできません"))]
    #[cfg_attr(feature = "api-docs", schema(example = "規約違反"))]
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
