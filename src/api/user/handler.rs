use crate::api::middleware::AdminContext;
use crate::api::{error::AppError, user::error::ApiUserError};
use crate::domain::transaction::TransactionManager;
use crate::usecase::user::service::UserService;
use actix_web::{HttpResponse, Responder, web};

pub async fn list_users_handler<TM: TransactionManager>(
    _admin: AdminContext,
    service: web::Data<UserService<TM>>,
) -> Result<impl Responder, AppError> {
    let users = service.list_users().await.map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(users))
}

pub fn user_config<TM: TransactionManager + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin") // /admin/users というパスになる
            .route("/users", web::get().to(list_users_handler::<TM>)),
    );
}
