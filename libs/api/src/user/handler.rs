use crate::middleware::{AdminContext, AuthenticatedUserContext};
use crate::{error::AppError, user::error::ApiUserError};
use actix_web::{HttpResponse, Responder, web};
use domain::user::UserId;
use serde::Deserialize;
use usecase::user::dto::UpdateUserInput;
use usecase::user::service::UserService;
use validator::Validate;

// --- API層専用のリクエスト構造体 ---
#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, message = "ユーザー名は空にできません"))]
    pub username: Option<String>,
    #[validate(email(message = "無効なメールアドレス形式です"))]
    pub email: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct SuspendUserRequest {
    #[validate(length(min = 1, message = "理由は空にできません"))]
    pub reason: String,
}

#[tracing::instrument(skip(service, admin, body))]
pub async fn suspend_user_handler(
    admin: AdminContext,
    path: web::Path<UserId>,
    service: web::Data<dyn UserService>,
    body: web::Json<SuspendUserRequest>,
) -> Result<impl Responder, AppError> {
    let target_id = path.into_inner();

    service
        .suspend_user(Box::new(admin), target_id, body.reason.clone())
        .await
        .map_err(ApiUserError::from)?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(skip(admin, service))]
pub async fn list_users_handler(
    admin: AdminContext,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, AppError> {
    let users = service
        .list_users(Box::new(admin))
        .await
        .map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(users))
}

#[tracing::instrument(skip(service, user))]
pub async fn get_user_handler(
    user: AuthenticatedUserContext,
    service: web::Data<dyn UserService>,
) -> Result<impl Responder, AppError> {
    let user = service
        .get_user_by_id(Box::new(user), user.user_id)
        .await
        .map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(user))
}

#[tracing::instrument(skip(service, body, user))]
pub async fn update_user_handler(
    user: AuthenticatedUserContext,
    service: web::Data<dyn UserService>,
    body: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate().map_err(ApiUserError::InvalidInput)?;

    let input = UpdateUserInput {
        username: body.username.clone(),
        email: body.email.clone(),
    };

    let updated_user = service
        .update_user(Box::new(user), user.user_id, input)
        .await
        .map_err(ApiUserError::from)?;
    Ok(HttpResponse::Ok().json(updated_user))
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users") // /user/me というパスになる
            .route("/me", web::get().to(get_user_handler))
            .route("/me", web::patch().to(update_user_handler)),
    );
    cfg.service(
        web::scope("/admin") // /admin/users というパスになる
            .route(
                "/users/{user_id}/suspend",
                web::post().to(suspend_user_handler),
            )
            .route("/users", web::get().to(list_users_handler)),
    );
}
