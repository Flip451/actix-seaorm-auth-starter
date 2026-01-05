pub mod auth;
pub mod persistence;

use std::sync::Arc;

use domain::transaction::TransactionManager;
// 各レイヤーのインポート
use crate::auth::argon2::password_service::Argon2PasswordHasher;
use crate::persistence::seaorm::transaction::SeaOrmTransactionManager;
use usecase::auth::service::AuthService;
use usecase::auth::token_service::TokenService;
use usecase::user::service::UserService;

pub struct RepoRegistry<TM: TransactionManager> {
    pub transaction_manager: Arc<TM>,
}

impl RepoRegistry<SeaOrmTransactionManager> {
    /// SeaORM 用の具体的な実装で構築
    pub fn new_seaorm(db: sea_orm::DatabaseConnection) -> Self {
        let db = Arc::new(db);
        let transaction_manager = Arc::new(SeaOrmTransactionManager::new(db.clone()));
        Self {
            transaction_manager,
        }
    }
}

/// アプリケーション全体の依存関係を保持する構造体
pub struct AppRegistry<TM: TransactionManager> {
    pub auth_service: Arc<AuthService<TM>>,
    pub user_service: Arc<UserService<TM>>,
    pub token_service: Arc<TokenService>,
}

impl<TM: TransactionManager> AppRegistry<TM> {
    pub fn new(repos: RepoRegistry<TM>, jwt_secret: String) -> Self {
        let password_hasher = Arc::new(Argon2PasswordHasher);

        let token_service = Arc::new(TokenService::new(jwt_secret));

        let auth_service = Arc::new(AuthService::<TM>::new(
            repos.transaction_manager.clone(),
            password_hasher,
            token_service.clone(),
        ));

        let user_service = Arc::new(UserService::new(repos.transaction_manager.clone()));

        Self {
            auth_service,
            user_service,
            token_service,
        }
    }
}
