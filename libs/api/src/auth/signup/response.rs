use serde::Serialize;
use usecase::auth::dto::SignupOutput;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct SignupResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    user_id: Uuid,
}

impl From<SignupOutput> for SignupResponse {
    fn from(output: SignupOutput) -> Self {
        SignupResponse {
            user_id: output.user_id.into(),
        }
    }
}

crate::impl_responder_for!(SignupResponse);
