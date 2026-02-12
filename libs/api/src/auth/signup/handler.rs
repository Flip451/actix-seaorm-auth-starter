use actix_web::{Responder, post, web};
use usecase::auth::service::AuthService;
use validator::Validate as _;

use super::{SignupRequest, SignupResponse};
use crate::error::ApiError;
#[cfg(feature = "api-docs")]
use crate::openapi::OpenApiTag;

#[cfg_attr(
    feature = "api-docs",
    utoipa::path(
        post,
        path = "/auth/signup",
        responses(
            (status = 201, description = "ユーザー登録成功", body = SignupResponse),
            (status = 400, description = "リクエストエラー"),
            (status = 500, description = "サーバーエラー"),
        ),
        tag = OpenApiTag::Auth.into(),
    )
)]
#[post("/signup")]
#[tracing::instrument(skip(service))]
pub async fn signup_handler(
    service: web::Data<dyn AuthService>,
    body: web::Json<SignupRequest>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = body.into_inner().into();

    let output = service.signup(input).await?;

    Ok(SignupResponse::from(output))
}
