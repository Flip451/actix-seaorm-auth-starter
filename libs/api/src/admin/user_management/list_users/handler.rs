use actix_web::{Responder, get, web};
use usecase::user::service::UserService;

use super::{ListUsersRequest, ListUsersResponse};
#[cfg(feature = "api-docs")]
use crate::admin::routes::AdminApiTag;
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;
use crate::{error::ApiError, middleware::AdminContext};

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        get,
        params(
            ListUsersRequest
        ),
        responses(
            (status = 200, description = "ユーザー一覧取得成功", body = ListUsersResponse),
            (status = 400, description = "リクエストエラー"),
            (status = 401, description = "認証エラー"),
            (status = 403, description = "権限エラー"),
            (status = 500, description = "サーバーエラー"),
        ),
        security(
            ("bearer_auth" = []) // Swagger UIで鍵マークを表示
        ),
        tag = OpenApiTag::Admin(AdminApiTag::UserManagement).as_ref(),
    )
)]
#[get("/admin/users/list")]
#[tracing::instrument(skip(service))]
pub async fn list_users_handler(
    admin: AdminContext,
    query: web::Query<ListUsersRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    let input = query.into_inner().into();

    let output = service.list_users(admin.into(), input).await?;

    Ok(ListUsersResponse::from(output))
}
