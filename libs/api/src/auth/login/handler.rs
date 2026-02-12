use actix_web::{Responder, post, web};
use usecase::auth::service::AuthService;
use validator::Validate as _;

use super::{LoginRequest, LoginResponse};
use crate::error::ApiError;
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        post,
        path = "/auth/login",
        responses(
            (status = 200, description = "ログイン成功", body = LoginResponse),
            (status = 400, description = "リクエストエラー"),
            (status = 401, description = "認証エラー"),
            (status = 500, description = "サーバーエラー"),
        ),
        tag = OpenApiTag::Auth.as_ref(),
    )
)]
#[post("/login")]
#[tracing::instrument(skip(service))]
pub async fn login_handler(
    service: web::Data<dyn AuthService>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = body.into_inner().into();

    let output = service.login(input).await?;

    Ok(LoginResponse::from(output))
}
