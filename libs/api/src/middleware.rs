use crate::error::ApiError;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use domain::user::{UserId, UserRole};
use futures_util::future::{Ready, ready};
use usecase::auth::token_service::TokenService;
use usecase::shared::identity::Identity;
use usecase::usecase_error::UseCaseError;

#[derive(Clone, Copy)]
pub struct AdminContext {
    pub user_id: UserId,
}

impl Identity for AdminContext {
    fn actor_id(&self) -> UserId {
        self.user_id
    }

    fn actor_role(&self) -> UserRole {
        UserRole::Admin
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
            None => return ready(Err(ApiError::UseCaseError(UseCaseError::Unauthorized))),
        };

        match token_service.verify_token(token) {
            // ロールが Admin であることを確認
            Ok(claims) if claims.role == UserRole::Admin => ready(Ok(AdminContext {
                user_id: claims.sub,
            })),
            // Admin でない場合は Forbidden を返す
            Ok(_) => ready(Err(ApiError::UseCaseError(UseCaseError::Forbidden))),
            Err(e) => ready(Err(ApiError::UseCaseError(e))),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AuthenticatedUserContext {
    pub user_id: UserId,
    pub user_role: UserRole,
}

impl Identity for AuthenticatedUserContext {
    fn actor_id(&self) -> UserId {
        self.user_id
    }

    fn actor_role(&self) -> UserRole {
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
            None => return ready(Err(ApiError::UseCaseError(UseCaseError::Unauthorized))),
        };

        // ロールにかかわらず検証を行う
        match token_service.verify_token(token) {
            Ok(claims) => ready(Ok(AuthenticatedUserContext {
                user_id: claims.sub,
                user_role: claims.role,
            })),
            Err(e) => ready(Err(ApiError::UseCaseError(e))),
        }
    }
}
