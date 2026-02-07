use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::auth::{login, signup};
use crate::user::{get_profile, list_users, suspend_user, update_profile};

#[derive(OpenApi)]
#[openapi(
    paths(
        login::login_handler,
        signup::signup_handler,
        get_profile::get_profile_handler,
        list_users::list_users_handler,
        suspend_user::suspend_user_handler,
        update_profile::update_profile_handler,
    ),
    components(
        schemas(
            login::LoginRequest,
            login::LoginResponse,
            signup::SignupRequest,
            signup::SignupResponse,
            get_profile::GetProfileRequest,
            get_profile::GetProfileResponse,
            list_users::ListUsersRequest,
            list_users::ListUsersResponse,
            suspend_user::SuspendUserRequest,
            suspend_user::SuspendUserResponse,
            update_profile::UpdateProfileRequest,
            update_profile::UpdateProfileResponse,
        )
    ),
    tags(
        (name = "auth", description = "認証関連API"),
        (name = "users", description = "ユーザー操作API")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Bearer Token 認証定義の追加
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
