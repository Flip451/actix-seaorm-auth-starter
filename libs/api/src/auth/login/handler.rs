use actix_web::{Responder, web};
use usecase::auth::service::AuthService;
use validator::Validate as _;

use super::{LoginRequest, LoginResponse};
use crate::error::ApiError;

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
