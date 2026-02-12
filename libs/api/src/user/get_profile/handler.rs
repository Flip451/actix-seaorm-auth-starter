use actix_web::{Responder, get, web};
use usecase::user::service::UserService;
use uuid::Uuid;
use validator::Validate as _;

use super::{GetProfileRequest, GetProfileResponse};
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;
use crate::{error::ApiError, middleware::AuthenticatedUserContext};

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        get,
        path = "/users/show-profile/{user_id}",
        responses(
            (status = 200, description = "ユーザー情報取得成功", body = GetProfileResponse),
            (status = 400, description = "リクエストエラー"),
            (status = 401, description = "認証エラー"),
            (status = 403, description = "権限エラー"),
            (status = 404, description = "ユーザーが見つかりません"),
            (status = 500, description = "サーバーエラー"),
        ),
        security(
            ("bearer_auth" = []) // Swagger UIで鍵マークを表示
        ),
        tag = OpenApiTag::User.into(),
    )
)]
#[get("/show-profile/{user_id}")]
#[tracing::instrument(skip(service))]
pub async fn get_profile_handler(
    user: AuthenticatedUserContext,
    user_id: web::Path<Uuid>,
    query: web::Query<GetProfileRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    query.validate()?;

    let input = query.into_inner().into_input(*user_id);

    let output = service.get_public_profile(user.into(), input).await?;

    Ok(GetProfileResponse::from(output))
}
