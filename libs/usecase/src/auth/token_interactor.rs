use std::sync::Arc;

use crate::{
    auth::token_service::{Claims, TokenService},
    usecase_error::UseCaseError,
};

use chrono::{Duration, Utc};
use domain::{shared::service::clock::Clock, user::{UserId, UserRole}};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Clone)] // Clone可能にしておく（ActixのStateで共有するため）
pub struct TokenInteractor {
    jwt_secret: String,
    clock: Arc<dyn Clock>,
}

impl TokenInteractor {
    pub fn new(jwt_secret: String, clock: Arc<dyn Clock>) -> Self {
        Self { jwt_secret, clock }
    }
}

impl TokenService for TokenInteractor {
    /// トークンの発行 (Login時に使用)
    fn issue_token(&self, user_id: UserId, role: UserRole) -> Result<String, UseCaseError> {
        let expiration = self.clock.now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            role,
            iat: self.clock.now().timestamp() as usize,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| UseCaseError::Internal(e.into()))
    }

    /// トークンの検証 (Middlewareで使用)
    fn verify_token(&self, token: &str) -> Result<Claims, UseCaseError> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())
            .map_err(|_| UseCaseError::Unauthorized)?;

        Ok(token_data.claims)
    }
}
