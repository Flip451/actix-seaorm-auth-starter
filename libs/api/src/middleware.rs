use crate::error::ApiError;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use futures_util::future::{Ready, ready};
use usecase::auth::token_service::TokenService;
use usecase::shared::identity::{Identity, UserRoleData};
use uuid::Uuid;

#[derive(derive_more::Debug, Clone, Copy)]
pub struct AdminContext {
    user_id: Uuid,
}

impl Identity for AdminContext {
    fn actor_id(&self) -> Uuid {
        self.user_id
    }

    fn actor_role(&self) -> UserRoleData {
        UserRoleData::Admin
    }
}

impl FromRequest for AdminContext {
    type Error = ApiError;
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
            None => return ready(Err(ApiError::Unauthorized)),
        };

        match token_service.verify_token(token) {
            // ロールが Admin であることを確認
            Ok(claims) if claims.user_role() == UserRoleData::Admin => ready(Ok(AdminContext {
                user_id: claims.user_id(),
            })),
            // Admin でない場合は Forbidden を返す
            Ok(_) => ready(Err(ApiError::Forbidden)),
            Err(e) => ready(Err(ApiError::UseCaseError(e))),
        }
    }
}

#[derive(derive_more::Debug, Clone, Copy)]
pub struct AuthenticatedUserContext {
    user_id: Uuid,
    user_role: UserRoleData,
}

impl Identity for AuthenticatedUserContext {
    fn actor_id(&self) -> Uuid {
        self.user_id
    }

    fn actor_role(&self) -> UserRoleData {
        self.user_role
    }
}

impl FromRequest for AuthenticatedUserContext {
    type Error = ApiError;
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
            None => return ready(Err(ApiError::Unauthorized)),
        };

        // ロールにかかわらず検証を行う
        match token_service.verify_token(token) {
            Ok(claims) => ready(Ok(AuthenticatedUserContext {
                user_id: claims.user_id(),
                user_role: claims.user_role(),
            })),
            Err(e) => ready(Err(ApiError::UseCaseError(e))),
        }
    }
}
