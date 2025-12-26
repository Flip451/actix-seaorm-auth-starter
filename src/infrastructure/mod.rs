pub mod auth;
pub mod persistence;

use std::sync::Arc;

use crate::domain::user::UserRepository;
// 各レイヤーのインポート
use crate::infrastructure::auth::argon2::password_service::Argon2PasswordHasher;
use crate::infrastructure::persistence::seaorm::repository::user_repository::SeaOrmUserRepository;
use crate::usecase::auth::service::AuthService;
use crate::usecase::user::service::UserService;

pub struct RepoRegistry {
    pub user_repo: Arc<dyn UserRepository>,
    // 将来的に追加されるリポジトリ
    // pub post_repo: Arc<dyn PostRepository>,
    // pub order_repo: Arc<dyn OrderRepository>,
}

impl RepoRegistry {
    /// SeaORM 用の具体的な実装で構築
    pub fn new_seaorm(db: sea_orm::DatabaseConnection) -> Self {
        Self {
            user_repo: Arc::new(SeaOrmUserRepository::new(db)),
        }
    }

    /// テスト用のモック実装で構築（必要に応じて）
    #[cfg(test)]
    pub fn new_mock() -> Self { todo!()}
}

/// アプリケーション全体の依存関係を保持する構造体
pub struct AppRegistry {
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}

impl AppRegistry {
    pub fn new(repos: RepoRegistry, jwt_secret: String) -> Self {
        let password_hasher = Arc::new(Argon2PasswordHasher);

        let auth_service = Arc::new(AuthService::new(
            repos.user_repo.clone(),
            password_hasher,
            jwt_secret,
        ));

        let user_service = Arc::new(UserService::new(repos.user_repo.clone()));

        Self {
            auth_service,
            user_service,
        }
    }
}