use actix_web::web;

use crate::admin::user_management;

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/admin")
        .configure(user_management::user_management_config))
        // Add more admin configurations here as needed
        ;
}

#[cfg(feature = "api-docs")]
pub use openapi::*;

#[cfg(feature = "api-docs")]
pub mod openapi {
    use super::*;
    use crate::openapi::OpenApiExt;
    use utoipa::OpenApi as _;

    #[derive(utoipa::OpenApi)]
    pub struct AdminApi;

    impl OpenApiExt for AdminApi {
        fn get_merged_doc(&self) -> utoipa::openapi::OpenApi {
            let mut doc = AdminApi::openapi();

            doc.merge(user_management::UserManagementApi::openapi());
            // Add more merges here as needed

            doc
        }
    }

    #[derive(strum::Display)]
    #[strum(serialize_all = "snake_case")]
    pub(crate) enum AdminApiTag {
        UserManagement,
    }

    impl AdminApiTag {
        pub fn as_ref(&self) -> &'static str {
            match self {
                AdminApiTag::UserManagement => "admin/user_management",
            }
        }
    }
}
