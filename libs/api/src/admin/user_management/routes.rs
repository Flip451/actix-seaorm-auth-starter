use actix_web::web;

use super::{list_users, suspend_user};

pub fn user_management_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(list_users::list_users_handler)
            .service(suspend_user::suspend_user_handler),
    );
}

#[cfg(feature = "api-docs")]
pub use openapi::*;

#[cfg(feature = "api-docs")]
pub mod openapi {
    use super::*;
    use crate::{admin::routes::AdminApiTag, openapi::OpenApiTag};
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            list_users::list_users_handler,
            suspend_user::suspend_user_handler,
        ),
        components(
            schemas(
                list_users::ListUsersRequest,
                list_users::ListUsersResponse,
                suspend_user::SuspendUserRequest,
                suspend_user::SuspendUserResponse,
            )
        ),
        tags((
                name = OpenApiTag::Admin(AdminApiTag::UserManagement).as_ref(),
                description = "管理者用ユーザー管理API"
        ))
    )]
    pub struct UserManagementApi;
}
