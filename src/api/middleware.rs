use crate::usecase::auth::error::AuthError;
use crate::{domain::user::UserRole, usecase::auth::token_service::TokenService};
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use futures_util::future::{Ready, ready};

pub struct AdminContext {
    pub user_id: uuid::Uuid,
}

impl FromRequest for AdminContext {
    type Error = AuthError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token_service = req.app_data::<web::Data<TokenService>>().cloned();

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(t) => t,
            None => return ready(Err(AuthError::InvalidCredentials)),
        };

        if let Some(service) = token_service {
            match service.verify_token(token) {
                // ロールが Admin であることを確認
                Ok(claims) if claims.role == UserRole::Admin => {
                    return ready(Ok(AdminContext {
                        user_id: claims.sub,
                    }));
                }
                // Admin でない場合は Forbidden を返す
                Ok(_) => return ready(Err(AuthError::Forbidden)),
                Err(e) => return ready(Err(e)),
            }
        }

        ready(Err(AuthError::InternalError))
    }
}
