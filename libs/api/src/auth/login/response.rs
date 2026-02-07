use serde::Serialize;
use usecase::auth::dto::LoginOutput;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    token: String,
}

impl From<LoginOutput> for LoginResponse {
    fn from(output: LoginOutput) -> Self {
        LoginResponse {
            token: output.token,
        }
    }
}

crate::impl_responder_for!(LoginResponse);
