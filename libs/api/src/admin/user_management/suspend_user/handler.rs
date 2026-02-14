use actix_web::{Responder, patch, web};
use usecase::user::service::UserService;
use uuid::Uuid;

use super::{SuspendUserRequest, SuspendUserResponse};
#[cfg(feature = "api-docs")]
use crate::admin::routes::AdminApiTag;
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;
use crate::{error::ApiError, middleware::AdminContext};

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        patch,
        params(
            ("user_id" = uuid::Uuid, Path, description = "停止対象のユーザーID")
        ),
        request_body = SuspendUserRequest,
        responses(
            (status = 200, description = "ユーザー停止成功", body = SuspendUserResponse),
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
#[patch("/admin/users/{user_id}/suspend")]
#[tracing::instrument(skip(service))]
pub async fn suspend_user_handler(
    admin: AdminContext,
    user_id: web::Path<Uuid>,
    body: web::Json<SuspendUserRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    let input = body.into_inner().into_input(*user_id);

    let output = service.suspend_user(admin.into(), input).await?;

    Ok(SuspendUserResponse::from(output))
}
