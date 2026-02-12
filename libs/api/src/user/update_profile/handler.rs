use actix_web::{Responder, patch, web};
use usecase::user::service::UserService;
use validator::Validate as _;

use super::{UpdateProfileRequest, UpdateProfileResponse};
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;
use crate::{error::ApiError, middleware::AuthenticatedUserContext};

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        patch,
        path = "/users/update-profile/{user_id}",
        responses(
            (status = 200, description = "ユーザー情報更新成功", body = UpdateProfileResponse),
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
#[patch("/update-profile/{user_id}")]
#[tracing::instrument(skip(service))]
pub async fn update_profile_handler(
    user: AuthenticatedUserContext,
    user_id: web::Path<uuid::Uuid>,
    service: web::Data<dyn UserService>,
    body: web::Json<UpdateProfileRequest>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = body.into_inner().into_input(*user_id);

    let output = service.update_user_profile(user.into(), input).await?;

    Ok(UpdateProfileResponse::from(output))
}
