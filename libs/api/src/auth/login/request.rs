use serde::Deserialize;

#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct LoginRequest {
    #[cfg_attr(feature = "api-docs", schema(examples("user@example.com")))]
    pub email: String,

    #[cfg_attr(feature = "api-docs", schema(examples("password123")))]
    #[debug(skip)]
    pub password: String,
}

impl From<LoginRequest> for usecase::auth::dto::LoginInput {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}
