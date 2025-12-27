use super::error::AuthError;
use crate::domain::user::UserRole;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)] // Clone可能にしておく（ActixのStateで共有するため）
pub struct TokenService {
    jwt_secret: String,
}

impl TokenService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    /// トークンの発行 (Login時に使用)
    pub fn issue_token(&self, user_id: Uuid, role: UserRole) -> Result<String, AuthError> {
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
    pub fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidCredentials)?;

        Ok(token_data.claims)
    }
}
