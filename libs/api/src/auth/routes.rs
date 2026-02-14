use actix_web::web;

use super::{login, signup};

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(signup::signup_handler)
        .service(login::login_handler);
}

#[cfg(feature = "api-docs")]
pub use openapi::*;

#[cfg(feature = "api-docs")]
pub mod openapi {
    use utoipa::OpenApi;

    use crate::openapi::OpenApiTag;

    use super::*;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            signup::signup_handler,
            login::login_handler
        ),
        components(
            schemas(
                signup::SignupRequest,
                signup::SignupResponse,
                login::LoginRequest,
                login::LoginResponse
            )
        ),
        tags((
            name = OpenApiTag::Auth.as_ref(),
            description = "認証関連API"
        ))
    )]
    pub struct AuthApi;

    impl crate::openapi::OpenApiExt for AuthApi {
        fn get_merged_doc(&self) -> utoipa::openapi::OpenApi {
            AuthApi::openapi()
        }
    }
}
