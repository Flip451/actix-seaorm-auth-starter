use actix_web::{Responder, web};
use usecase::user::service::UserService;
use validator::Validate;

use super::SuspendUserRequest;
use crate::{error::ApiError, middleware::AdminContext, user::suspend_user::SuspendUserResponse};

#[tracing::instrument(skip(service))]
pub async fn suspend_user_handler(
    admin: AdminContext,
    body: web::Json<SuspendUserRequest>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = body.into_inner().into();

    let output = service.suspend_user(admin.into(), input).await?;

    Ok(SuspendUserResponse::from(output))
}
