use crate::api::error::AppError;
use crate::usecase::auth::error::AuthError;
use crate::usecase::auth::dto::{LoginInput, SignupInput};
use crate::usecase::auth::service::AuthService;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;
use validator::Validate;

// --- API層専用のリクエスト構造体 ---
#[derive(Deserialize, Validate)]
pub struct SignupRequest {
    pub username: String,
    #[validate(email(message = "メールアドレスの形式が正しくありません"))]
    pub email: String,
    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[tracing::instrument(skip(service, body))]
#[post("/signup")]
pub async fn signup_handler(
    service: web::Data<AuthService>,
    body: web::Json<SignupRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AuthError::InvalidInput(e.to_string()))?;

    let input = SignupInput {
        username: body.username.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
    };

    service.signup(input).await?;

    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(skip(service, body))]
#[post("/login")]
pub async fn login_handler(
    service: web::Data<AuthService>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let input = LoginInput {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    let token = service.login(input).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(signup_handler)
            .service(login_handler),
    );
}
