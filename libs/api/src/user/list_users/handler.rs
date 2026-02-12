use actix_web::{Responder, get, web};
use usecase::user::service::UserService;
use validator::Validate as _;

use super::{ListUsersRequest, ListUsersResponse};
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;
use crate::{error::ApiError, middleware::AuthenticatedUserContext};

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        get,
        path = "/users/list",
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
        tag = OpenApiTag::Users.as_ref(),
    )
)]
#[get("/list")]
#[tracing::instrument(skip(service))]
pub async fn list_users_handler(
    user: AuthenticatedUserContext,
    query: web::Query<ListUsersRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    query.validate()?;

    let input = query.into_inner().into();

    let output = service.list_users(user.into(), input).await?;

    Ok(ListUsersResponse::from(output))
}
