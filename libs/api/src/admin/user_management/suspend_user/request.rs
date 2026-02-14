use serde::Deserialize;
use usecase::user::dto::SuspendUserInput;
use uuid::Uuid;

#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct SuspendUserRequest {
    #[cfg_attr(feature = "api-docs", schema(examples("規約違反")))]
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
