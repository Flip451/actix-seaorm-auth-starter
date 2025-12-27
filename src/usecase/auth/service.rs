use crate::{
    domain::{repository::TxRepositories, transaction::TransactionManager, user::{
        Email, PasswordHasher, RawPassword, User, UserRepository, UserRole
    }},
    usecase::auth::{dto::{LoginInput, SignupInput}, error::AuthError, token_service::TokenService},
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthService<TM> {
    user_repo: Arc<dyn UserRepository>,
    transaction_manager: TM,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<TokenService>,
}

impl<TM: TransactionManager> AuthService<TM> {
    pub fn new(user_repo: Arc<dyn UserRepository>, transaction_manager: TM, password_hasher: Arc<dyn PasswordHasher>, token_service: Arc<TokenService>) -> Self {
        Self {
            user_repo,
            transaction_manager,
            password_hasher,
            token_service,
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

        // パスワードのハッシュ化
        let hashed_password = self.password_hasher.hash(&password)?;

        self.transaction_manager.execute::<User, AuthError, _>(move |repos:TxRepositories<'_> | Box::pin(async move {
             // 1. 重複チェック
            if repos.user
                .find_by_email(email.as_str())
                .await
                .map_err(AuthError::Domain)?
                .is_some()
            {
                return Err(AuthError::EmailAlreadyExists);
            }

            // 2. ドメインモデル作成と保存
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

            repos.user
                .save(user)
                .await
                .map_err(AuthError::Domain)
        })).await
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
        let token = self.token_service.issue_token(user.id, user.role)?;

        Ok(token)
    }
}
