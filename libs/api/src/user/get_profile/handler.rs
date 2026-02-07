use actix_web::{Responder, web};
use usecase::user::service::UserService;
use validator::Validate as _;

use crate::{error::ApiError, middleware::AuthenticatedUserContext};

use super::{GetProfileRequest, GetProfileResponse};

#[tracing::instrument(skip(service))]
pub async fn get_user_handler(
    user: AuthenticatedUserContext,
    query: web::Query<GetProfileRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    query.validate()?;

    let input = query.into_inner().into();

    let output = service.get_user_by_id(user.into(), input).await?;

    Ok(GetProfileResponse::from(output))
}
