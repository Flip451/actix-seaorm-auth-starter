use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::admin::routes::{AdminApi, AdminApiTag};
use crate::auth::routes::AuthApi;
use crate::user::routes::UserApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "My Application API",
        version = "0.1.0",
        description = "This is the API documentation for My Application."
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Bearer Token 認証定義の追加
        let components = openapi.components.get_or_insert_with(Default::default);
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

pub fn generate_api_doc() -> utoipa::openapi::OpenApi {
    let sub_docs: Vec<&dyn OpenApiExt> = vec![
        &AdminApi, &AuthApi,
        &UserApi,
        // 他のモジュールのAPIドキュメントをここに追加
    ];

    let mut doc = ApiDoc::openapi();

    for sub_doc in sub_docs {
        doc.merge(sub_doc.get_merged_doc());
    }

    doc
}

pub(crate) trait OpenApiExt {
    fn get_merged_doc(&self) -> utoipa::openapi::OpenApi;
}

pub(crate) enum OpenApiTag {
    Admin(AdminApiTag),
    Auth,
    Users,
}

impl OpenApiTag {
    pub fn as_ref(&self) -> &'static str {
        match self {
            OpenApiTag::Admin(admin_api_tag) => admin_api_tag.as_ref(),
            OpenApiTag::Auth => "auth",
            OpenApiTag::Users => "users",
        }
    }
}
