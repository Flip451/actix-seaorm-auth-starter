use actix_web::{Responder, web};
use usecase::user::service::UserService;
use validator::Validate as _;

use crate::{error::ApiError, middleware::AdminContext};

use super::{ListUsersQuery, ListUsersResponse};

#[tracing::instrument(skip(service))]
pub async fn list_users_handler(
    admin: AdminContext,
    query: web::Query<ListUsersQuery>,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, ApiError> {
    query.validate()?;

    let input = query.into_inner().into();

    let output = service.list_users(admin.into(), input).await?;

    Ok(ListUsersResponse::from(output))
}
