use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use crate::auth::error::ApiAuthError;
use domain::user::UserRole;
use futures_util::future::{ready, Ready};
use usecase::auth::error::AuthError;
use usecase::auth::token_service::TokenService;

pub struct AdminContext {
    pub user_id: uuid::Uuid,
}

impl FromRequest for AdminContext {
    type Error = ApiAuthError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token_service = req.app_data::<web::Data<dyn TokenService>>().expect(
            "TokenService がアプリデータに登録されていません。 main.rs を確認してください。",
        );

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(t) => t,
            None => return ready(Err(ApiAuthError::AuthError(AuthError::InvalidCredentials))),
        };

        match token_service.verify_token(token) {
            // ロールが Admin であることを確認
            Ok(claims) if claims.role == UserRole::Admin => {
                return ready(Ok(AdminContext {
                    user_id: claims.sub,
                }));
            }
            // Admin でない場合は Forbidden を返す
            Ok(_) => return ready(Err(ApiAuthError::AuthError(AuthError::Forbidden))),
            Err(e) => return ready(Err(ApiAuthError::AuthError(e))),
        }
    }
}

pub struct AuthenticatedUserContext {
    pub user_id: uuid::Uuid,
}

impl FromRequest for AuthenticatedUserContext {
    type Error = ApiAuthError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token_service = req.app_data::<web::Data<dyn TokenService>>().expect(
            "TokenService がアプリデータに登録されていません。 main.rs を確認してください。",
        );

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(t) => t,
            None => return ready(Err(ApiAuthError::AuthError(AuthError::InvalidCredentials))),
        };

        // ロールにかかわらず検証を行う
        match token_service.verify_token(token) {
            Ok(claims) => ready(Ok(AuthenticatedUserContext {
                user_id: claims.sub,
            })),
            Err(e) => ready(Err(ApiAuthError::AuthError(e))),
        }
    }
}
