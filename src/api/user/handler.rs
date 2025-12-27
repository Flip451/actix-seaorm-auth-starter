use crate::api::middleware::{AdminContext, AuthenticatedUserContext};
use crate::api::{error::AppError, user::error::ApiUserError};
use crate::domain::transaction::TransactionManager;
use crate::usecase::user::dto::UpdateUserInput;
use crate::usecase::user::service::UserService;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use validator::Validate;

// --- API層専用のリクエスト構造体 ---
#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    pub username: Option<String>,
    #[validate(email(message = "無効なメールアドレス形式です"))]
    pub email: Option<String>,
}

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

pub async fn update_user_handler<TM: TransactionManager>(
    user: AuthenticatedUserContext,
    service: web::Data<UserService<TM>>,
    body: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate().map_err(|e| ApiUserError::InvalidInput(e))?;

    let input = UpdateUserInput {
        username: body.username.clone(),
        email: body.email.clone(),
    };

    let updated_user = service
        .update_user(user.user_id, input)
        .await
        .map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(updated_user))
}

pub fn user_config<TM: TransactionManager + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users") // /user/me というパスになる
            .route("/me", web::get().to(get_user_handler::<TM>))
            .route("/me", web::patch().to(update_user_handler::<TM>)),
    );
    cfg.service(
        web::scope("/admin") // /admin/users というパスになる
            .route("/users", web::get().to(list_users_handler::<TM>)),
    );
}
