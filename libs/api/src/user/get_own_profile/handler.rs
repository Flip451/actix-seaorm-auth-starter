use actix_web::{Responder, get, web};
use usecase::user::service::UserService;

use super::{GetOwnProfileRequest, GetOwnProfileResponse};
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;
use crate::{error::ApiError, middleware::AuthenticatedUserContext};

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        get,
        path = "/users/me",
        responses(
            (status = 200, description = "ユーザー情報取得成功", body = GetOwnProfileResponse),
            (status = 400, description = "リクエストエラー"),
            (status = 401, description = "認証エラー"),
            (status = 403, description = "権限エラー"),
            (status = 404, description = "ユーザーが見つかりません"),
            (status = 500, description = "サーバーエラー"),
        ),
        security(
            ("bearer_auth" = []) // Swagger UIで鍵マークを表示
        ),
        tag = OpenApiTag::Users.as_ref(),
    )
)]
#[get("/me")]
#[tracing::instrument(skip(service))]
pub async fn get_own_profile_handler(
    user: AuthenticatedUserContext,
    query: web::Query<GetOwnProfileRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    let input = query.into_inner().into_input();

    let output = service.get_own_profile(user.into(), input).await?;

    Ok(GetOwnProfileResponse::from(output))
}
