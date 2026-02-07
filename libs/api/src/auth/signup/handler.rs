use actix_web::{Responder, web};
use usecase::auth::service::AuthService;
use validator::Validate as _;

use crate::{auth::signup::SignupResponse, error::ApiError};

use super::SignupRequest;

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
