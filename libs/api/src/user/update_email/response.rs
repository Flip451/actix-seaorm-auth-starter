use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::user::dto::UpdateUserEmailOutput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct UpdateEmailResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(example = "550e8400-e29b-41d4-a716-446655440000")
    )]
    pub id: Uuid,
    #[cfg_attr(feature = "api-docs", schema(example = "user@example.com"))]
    pub email: String,
}

impl From<UpdateUserEmailOutput> for UpdateEmailResponse {
    fn from(user: UpdateUserEmailOutput) -> Self {
        let UpdateUserEmailOutput { user_id, email } = user;

        UpdateEmailResponse { id: user_id, email }
    }
}

crate::impl_responder_for!(UpdateEmailResponse, StatusCode::OK);
