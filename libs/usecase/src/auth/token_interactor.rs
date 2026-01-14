use crate::auth::token_service::{Claims, TokenService};

use super::error::AuthError;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use domain::user::{UserId, UserRole};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Clone)] // Clone可能にしておく（ActixのStateで共有するため）
pub struct TokenInteractor {
    jwt_secret: String,
}

impl TokenInteractor {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

#[async_trait]
impl TokenService for TokenInteractor {
    /// トークンの発行 (Login時に使用)
    fn issue_token(&self, user_id: UserId, role: UserRole) -> Result<String, AuthError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            role,
            iat: Utc::now().timestamp() as usize,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| AuthError::TokenIssuanceFailed(e.into()))
    }

    /// トークンの検証 (Middlewareで使用)
    fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidCredentials)?;

        Ok(token_data.claims)
    }
}
