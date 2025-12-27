use crate::api::middleware::{AdminContext, AuthenticatedUserContext};
use crate::api::{error::AppError, user::error::ApiUserError};
use crate::domain::transaction::TransactionManager;
use crate::usecase::user::service::UserService;
use actix_web::{HttpResponse, Responder, web};

#[tracing::instrument(skip(_admin, service))]
pub async fn list_users_handler<TM: TransactionManager>(
    _admin: AdminContext,
    service: web::Data<UserService<TM>>,
) -> Result<impl Responder, AppError> {
    let users = service.list_users().await.map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(users))
}

#[tracing::instrument(skip(service, user))]
pub async fn get_user_handler<TM: TransactionManager>(
    user: AuthenticatedUserContext,
    service: web::Data<UserService<TM>>,
) -> Result<impl Responder, AppError> {
    let user = service
        .get_user_by_id(user.user_id)
        .await
        .map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn user_config<TM: TransactionManager + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users") // /user/me というパスになる
            .route("/me", web::get().to(get_user_handler::<TM>)),
    );
    cfg.service(
        web::scope("/admin") // /admin/users というパスになる
            .route("/users", web::get().to(list_users_handler::<TM>)),
    );
}
