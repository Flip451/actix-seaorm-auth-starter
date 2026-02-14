use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::SuspendUserOutput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct SuspendUserResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(examples("123e4567-e89b-12d3-a456-426614174000"))
    )]
    user_id: Uuid,
    #[cfg_attr(feature = "api-docs", schema(examples(true, false)))]
    suspended: bool,
}

impl From<SuspendUserOutput> for SuspendUserResponse {
    fn from(output: SuspendUserOutput) -> Self {
        let SuspendUserOutput { user_id, suspended } = output;

        SuspendUserResponse { user_id, suspended }
    }
}

crate::impl_responder_for!(SuspendUserResponse, StatusCode::OK);
