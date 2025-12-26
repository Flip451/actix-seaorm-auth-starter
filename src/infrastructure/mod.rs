pub mod auth;
pub mod persistence;

use std::sync::Arc;

use crate::domain::transaction::TransactionManager;
use crate::domain::user::UserRepository;
// 各レイヤーのインポート
use crate::infrastructure::auth::argon2::password_service::Argon2PasswordHasher;
use crate::infrastructure::persistence::seaorm::repository::user_repository::SeaOrmUserRepository;
use crate::infrastructure::persistence::seaorm::transaction::SeaOrmTransactionManager;
use crate::usecase::auth::service::AuthService;
use crate::usecase::auth::token_service::TokenService;
use crate::usecase::user::service::UserService;

pub struct RepoRegistry<TM: TransactionManager> {
    pub user_repo: Arc<dyn UserRepository>,
    pub transaction_manager: TM,
}

impl RepoRegistry<SeaOrmTransactionManager> {
    /// SeaORM 用の具体的な実装で構築
    pub fn new_seaorm(db: sea_orm::DatabaseConnection) -> Self {
        let db = Arc::new(db);
        let user_repo = Arc::new(SeaOrmUserRepository::new(db.clone()));
        let transaction_manager = SeaOrmTransactionManager::new(db.clone());
        // let post_repo = Arc::new(SeaOrmPostRepository::new(db.clone()));
        Self { user_repo, transaction_manager }
    }
}

/// アプリケーション全体の依存関係を保持する構造体
pub struct AppRegistry<TM: TransactionManager> {
    pub auth_service: Arc<AuthService<TM>>,
    pub user_service: Arc<UserService>,
    pub token_service: Arc<TokenService>,
}

impl<TM: TransactionManager> AppRegistry<TM> {
    pub fn new(repos: RepoRegistry<TM>, jwt_secret: String) -> Self {
        let password_hasher = Arc::new(Argon2PasswordHasher);

        let token_service = Arc::new(TokenService::new(jwt_secret));

        let auth_service = Arc::new(AuthService::<TM>::new(
            repos.user_repo.clone(),
            repos.transaction_manager,
            password_hasher,
            token_service.clone(),
        ));

        let user_service = Arc::new(UserService::new(repos.user_repo.clone()));

        Self {
            auth_service,
            user_service,
            token_service,
        }
    }
}
