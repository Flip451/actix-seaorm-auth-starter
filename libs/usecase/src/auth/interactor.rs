use crate::{
    auth::{
        dto::{LoginInput, SignupInput},
        service::AuthService,
        token_service::TokenService,
    },
    usecase_error::UseCaseError,
};
use async_trait::async_trait;
use domain::{
    transaction::TransactionManager,
    tx,
    user::{
        EmailTrait, HashedPassword, PasswordHasher, RawPassword, UnverifiedEmail, User,
        UserDomainError, UserFactory, UserUniquenessService,
    },
};
use std::sync::Arc;

pub struct AuthInteractor<TM> {
    transaction_manager: Arc<TM>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
    user_factory: Arc<UserFactory>,
    dummy_hash: HashedPassword,
}

impl<TM> AuthInteractor<TM> {
    pub fn new(
        transaction_manager: Arc<TM>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<dyn TokenService>,
        user_factory: Arc<UserFactory>,
    ) -> Self {
        let dummy_password = RawPassword::new("dummy_password_for_timing_attack").unwrap();
        let dummy_hash = password_hasher.hash(&dummy_password).unwrap();

        Self {
            transaction_manager,
            password_hasher,
            token_service,
            user_factory,
            dummy_hash,
        }
    }
}

#[async_trait]
impl<TM: TransactionManager> AuthService for AuthInteractor<TM> {
    /// サインアップ（ユーザー登録）
    #[tracing::instrument(
        skip(self, input),
        fields(
            username = %input.username,
            email = %input.email
        )
    )]
    async fn signup(&self, input: SignupInput) -> Result<User, UseCaseError> {
        // ここでDTOからValueObjectへの変換を行う
        let username = input.username;
        let email = input.email;
        let password = RawPassword::new(&input.password).map_err(UserDomainError::from)?;

        // パスワードのハッシュ化
        let hashed_password = self.password_hasher.hash(&password)?;

        let user_factory = self.user_factory.clone();

        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            let user_uniqueness_service = UserUniquenessService::new(user_repo.clone());

            // 1. ドメインモデル作成と保存
            let user = user_factory
                .create_new_user(user_uniqueness_service, &username, &email, hashed_password)
                .await?;

            // 2. 永続化
            let user = user_repo.save(user).await?;

            Ok(user)
        })
        .await
    }

    /// ログイン
    #[tracing::instrument(
        skip(self, input),
        fields(email = %input.email)
    )]
    async fn login(&self, input: LoginInput) -> Result<String, UseCaseError> {
        // ここでDTOからValueObjectへの変換を行う
        let email = UnverifiedEmail::new(&input.email).map_err(UserDomainError::from)?;
        let password = RawPassword::new(&input.password).map_err(UserDomainError::from)?;
        // 1. ユーザーを検索
        let user_opt: Option<User> = tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();
            Ok::<_, UseCaseError>(user_repo.find_by_email(email.as_str()).await?)
        })
        .await?;

        // 2. 検証 (HashedPassword に委譲)
        // ※タイミング攻撃に対する脆弱性を回避するため、ユーザーの有無に関わらず検証処理を行う
        let (is_valid, user) = match user_opt {
            Some(user) => {
                let is_valid = self.password_hasher.verify(&password, user.password());
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
            return Err(UseCaseError::UnAuthorized);
        }

        let user = user.unwrap();

        // 3. JWT トークンの生成
        let token = self.token_service.issue_token(user.id(), user.role())?;

        Ok(token)
    }
}
