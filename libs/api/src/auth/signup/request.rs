use serde::Deserialize;
use usecase::auth::dto::SignupInput;
#[cfg(feature = "api-docs")]
use utoipa::ToSchema;

#[derive(derive_more::Debug, Deserialize)]
#[cfg_attr(feature = "api-docs", derive(ToSchema))]
pub struct SignupRequest {
    #[cfg_attr(feature = "api-docs", schema(examples("exampleuser")))]
    pub username: String,

    #[cfg_attr(feature = "api-docs", schema(examples("user@example.com")))]
    #[debug(skip)]
    pub email: String,

    #[cfg_attr(feature = "api-docs", schema(examples("password123")))]
    #[debug(skip)]
    pub password: String,
}

impl From<SignupRequest> for SignupInput {
    fn from(req: SignupRequest) -> Self {
        Self {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}
