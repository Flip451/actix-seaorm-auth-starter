use actix_web::{Responder, web};
use usecase::user::service::UserService;
use validator::Validate as _;

use super::{UpdateProfileRequest, UpdateProfileResponse};
use crate::{error::ApiError, middleware::AuthenticatedUserContext};

#[tracing::instrument(skip(service))]
pub async fn update_profile_handler(
    user: AuthenticatedUserContext,
    service: web::Data<dyn UserService>,
    body: web::Json<UpdateProfileRequest>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = body.into_inner().into();

    let output = service.update_user(user.into(), input).await?;

    Ok(UpdateProfileResponse::from(output))
}
