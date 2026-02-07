use serde::Serialize;
use usecase::user::dto::SuspendUserOutput;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct SuspendUserResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    user_id: Uuid,
    #[schema(example = true)]
    suspended: bool,
}

impl From<SuspendUserOutput> for SuspendUserResponse {
    fn from(output: SuspendUserOutput) -> Self {
        SuspendUserResponse {
            user_id: output.user_id,
            suspended: output.suspended,
        }
    }
}

crate::impl_responder_for!(SuspendUserResponse);
