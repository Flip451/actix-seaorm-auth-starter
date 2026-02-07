use actix_web::{Responder, patch, web};
use usecase::user::service::UserService;
use uuid::Uuid;
use validator::Validate;

use super::SuspendUserRequest;
use crate::{error::ApiError, middleware::AdminContext, user::suspend_user::SuspendUserResponse};

#[utoipa::path(
    patch,
    path = "/users/suspend/{user_id}",
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
    tag = "users",
)]
#[patch("/users/suspend/{user_id}")]
#[tracing::instrument(skip(service))]
pub async fn suspend_user_handler(
    admin: AdminContext,
    user_id: web::Path<Uuid>,
    body: web::Json<SuspendUserRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = body.into_inner().into_input(*user_id);

    let output = service.suspend_user(admin.into(), input).await?;

    Ok(SuspendUserResponse::from(output))
}
