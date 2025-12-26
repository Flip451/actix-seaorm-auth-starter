use crate::{
    domain::user::{
        Email, PasswordHasher, RawPassword, User, UserRepository, UserRole
    },
    usecase::{auth::{dto::{LoginInput, SignupInput}, error::AuthError}},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,          // ユーザーID
    pub role: UserRole,     // ユーザーロール
    pub exp: usize,         // 有効期限
    pub iat: usize,         // 発行時刻
}

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>, password_hasher: Arc<dyn PasswordHasher>, jwt_secret: String) -> Self {
        Self {
            user_repo,
            password_hasher,
            jwt_secret,
        }
    }

    /// サインアップ（ユーザー登録）
    #[tracing::instrument(
        skip(self, input), 
        fields(
            username = %input.username, 
            email = %input.email
        )
    )]
    pub async fn signup(&self, input: SignupInput) -> Result<User, AuthError> {
        // ここでDTOからValueObjectへの変換を行う
        let username = input.username;
        let email = Email::new(&input.email)?;
        let password = RawPassword::new(&input.password)?;
        // 1. 重複チェック
        if self
            .user_repo
            .find_by_email(email.as_str())
            .await
            .map_err(AuthError::Domain)?
            .is_some()
        {
            return Err(AuthError::EmailAlreadyExists);
        }

        // 2. ハッシュ化 (ValueError から自動変換される)
        let hashed_password = self.password_hasher.hash(&password)?;

        // 3. ドメインモデル作成と保存
        let now = Utc::now().fixed_offset();
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email,
            password: hashed_password,
            role: UserRole::User,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.user_repo
            .save(user)
            .await
            .map_err(AuthError::Domain)
    }

    /// ログイン
    #[tracing::instrument(
        skip(self, input), 
        fields(email = %input.email)
    )]
    pub async fn login(&self, input: LoginInput) -> Result<String, AuthError> {
        // ここでDTOからValueObjectへの変換を行う
        let email = Email::new(&input.email)?;
        let password = RawPassword::new(&input.password)?;
        // 1. ユーザーを検索
        let user = self
            .user_repo
            .find_by_email(email.as_str())
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        // 2. 検証 (HashedPassword に委譲)
        let hashed = &user.password;
        if !self.password_hasher.verify(&password, hashed) {
            return Err(AuthError::InvalidCredentials);
        }

        // 3. JWT トークンの生成
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            role: user.role,
            iat: Utc::now().timestamp() as usize,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AuthError::InternalError)?;

        Ok(token)
    }

    pub fn get_jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_ref());
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &decoding_key,
            &jsonwebtoken::Validation::default(),
        ).map_err(|_| AuthError::InvalidCredentials)?;

        Ok(token_data.claims)
    }
}
