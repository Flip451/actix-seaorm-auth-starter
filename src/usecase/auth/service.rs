use crate::{
    domain::{
        repository::TxRepositories,
        transaction::TransactionManager,
        user::{
            Email, HashedPassword, PasswordHasher, RawPassword, User,
            UserRepository, UserRole,
        },
    },
    usecase::auth::{
        dto::{LoginInput, SignupInput},
        error::AuthError,
        token_service::TokenService,
    },
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthService<TM> {
    user_repo: Arc<dyn UserRepository>,
    transaction_manager: TM,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<TokenService>,
    dummy_hash: HashedPassword,
}

impl<TM: TransactionManager> AuthService<TM> {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        transaction_manager: TM,
        password_hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<TokenService>,
    ) -> Self {
        let dummy_password = RawPassword::new("dummy_password_for_timing_attack").unwrap();
        let dummy_hash = password_hasher.hash(&dummy_password).unwrap();

        Self {
            user_repo,
            transaction_manager,
            password_hasher,
            token_service,
            dummy_hash,
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

        self.transaction_manager
            .execute::<User, AuthError, _>(move |repos: TxRepositories<'_>| {
                Box::pin(async move {
                    // 1. 重複チェック
                    if repos.user.find_by_email(email.as_str()).await?.is_some() {
                        return Err(AuthError::EmailAlreadyExists(email.as_str().to_string()));
                    }

                    if repos.user.find_by_username(&username).await?.is_some() {
                        return Err(AuthError::UsernameAlreadyExists(username.to_string()));
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

                    Ok(repos.user.save(user).await?)
                })
            })
            .await
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
        let user_opt = self.user_repo.find_by_email(email.as_str()).await?;

        // 2. 検証 (HashedPassword に委譲)
        // ※タイミング攻撃に対する脆弱性を回避するため、ユーザーの有無に関わらず検証処理を行う
        let (is_valid, user) = match user_opt {
            Some(user) => {
                let is_valid = self.password_hasher.verify(&password, &user.password);
                (is_valid, Some(user))
            }
            None => {
                // ユーザーがいない場合も、計算コストを合わせるためにダミーのハッシュと比較する
                // (実際にはダミーハッシュを定数などで持っておき、verifyを走らせる)
                let _ = self.password_hasher.verify(&password, &self.dummy_hash);
                (false, None)
            }
        };

        if !is_valid || user.is_none() {
            return Err(AuthError::InvalidCredentials);
        }

        let user = user.unwrap();

        // 3. JWT トークンの生成
        let token = self.token_service.issue_token(user.id, user.role)?;

        Ok(token)
    }
}
