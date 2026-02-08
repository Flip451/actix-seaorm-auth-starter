use actix_web::http::StatusCode;
use serde::Serialize;
use usecase::auth::dto::LoginOutput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(Serialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub(crate) struct LoginResponse {
    #[cfg_attr(
        feature = "api-docs",
        schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")
    )]
    token: String,
}

impl From<LoginOutput> for LoginResponse {
    fn from(output: LoginOutput) -> Self {
        LoginResponse {
            token: output.token,
        }
    }
}

crate::impl_responder_for!(LoginResponse, StatusCode::OK);
