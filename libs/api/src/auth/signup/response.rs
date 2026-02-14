use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::auth::dto::SignupOutput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct SignupResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(examples("550e8400-e29b-41d4-a716-446655440000"))
    )]
    user_id: Uuid,
}

impl From<SignupOutput> for SignupResponse {
    fn from(output: SignupOutput) -> Self {
        SignupResponse {
            user_id: output.user_id.into(),
        }
    }
}

crate::impl_responder_for!(SignupResponse, StatusCode::CREATED);
