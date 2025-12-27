use crate::api::middleware::AdminContext;
use crate::api::{error::AppError, user::error::ApiUserError};
use crate::usecase::user::service::UserService;
use actix_web::{HttpResponse, Responder, get, web};

#[get("/users")]
pub async fn list_users_handler(
    _admin: AdminContext,
    service: web::Data<UserService>,
) -> Result<impl Responder, AppError> {
    let users = service.list_users().await.map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(users))
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin") // /admin/users というパスになる
            .service(list_users_handler),
    );
}
