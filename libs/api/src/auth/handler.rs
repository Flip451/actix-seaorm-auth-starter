use crate::error::ApiError;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use usecase::auth::dto::{LoginInput, SignupInput};
use usecase::auth::service::AuthService;
use validator::Validate;

// --- API層専用のリクエスト構造体 ---
#[derive(Deserialize, Validate)]
pub struct SignupRequest {
    pub username: String,
    #[validate(email(message = "無効なメールアドレス形式です"))]
    pub email: String,
    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "無効なメールアドレス形式です"))]
    pub email: String,
    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
    pub password: String,
}

#[tracing::instrument(skip(service, body))]
pub async fn signup_handler(
    service: web::Data<dyn AuthService>,
    body: web::Json<SignupRequest>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = SignupInput {
        username: body.username.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
    };

    service.signup(input).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(skip(service, body))]
pub async fn login_handler(
    service: web::Data<dyn AuthService>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, ApiError> {
    body.validate()?;

    let input = LoginInput {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    let token = service.login(input).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(signup_handler))
            .route("/login", web::post().to(login_handler)),
    );
}
