use crate::api::auth::error::ApiAuthError;
use crate::api::error::AppError;
use crate::domain::transaction::TransactionManager;
use crate::usecase::auth::dto::{LoginInput, SignupInput};
use crate::usecase::auth::service::AuthService;
use actix_web::{HttpResponse, Responder, web};
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
pub async fn signup_handler<TM: TransactionManager>(
    service: web::Data<AuthService<TM>>,
    body: web::Json<SignupRequest>,
) -> Result<impl Responder, AppError> {
    body.validate().map_err(|e| ApiAuthError::InvalidInput(e))?;

    let input = SignupInput {
        username: body.username.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
    };

    service.signup(input).await.map_err(ApiAuthError::from)?;

    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(skip(service, body))]
pub async fn login_handler<TM: TransactionManager>(
    service: web::Data<AuthService<TM>>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let input = LoginInput {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    let token = service.login(input).await.map_err(ApiAuthError::from)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}

pub fn auth_config<TM: TransactionManager + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(signup_handler::<TM>))
            .route("/login", web::post().to(login_handler::<TM>)),
    );
}
