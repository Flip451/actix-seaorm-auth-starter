use actix_web::web;

use crate::user::{get_own_profile, get_profile, update_email, update_profile};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_own_profile::get_own_profile_handler)
        .service(get_profile::get_public_profile_handler)
        .service(update_email::update_email_handler)
        .service(update_profile::update_profile_handler);
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
            get_own_profile::get_own_profile_handler,
            get_profile::get_public_profile_handler,
            update_email::update_email_handler,
            update_profile::update_profile_handler,
        ),
        components(
            schemas(
                get_own_profile::GetOwnProfileRequest,
                get_own_profile::GetOwnProfileResponse,
                get_profile::GetProfileRequest,
                get_profile::GetProfileResponse,
                update_email::UpdateEmailRequest,
                update_email::UpdateEmailResponse,
                update_profile::UpdateProfileRequest,
                update_profile::UpdateProfileResponse
            )
        ),
        tags((
            name = OpenApiTag::Users.as_ref(),
            description = "ユーザー関連のエンドポイント"
        ))
    )]
    pub struct UserApi;

    impl crate::openapi::OpenApiExt for UserApi {
        fn get_merged_doc(&self) -> utoipa::openapi::OpenApi {
            UserApi::openapi()
        }
    }
}
