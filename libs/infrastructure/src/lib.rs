pub mod auth;
pub mod email_service;
pub mod persistence;
pub mod user;

use std::sync::Arc;

use crate::auth::argon2::password_service::Argon2PasswordHasher;
use crate::persistence::seaorm::repository::user_repository::SeaOrmUserRepository;
use crate::persistence::seaorm::transaction::{EntityTracker, SeaOrmTransactionManager};
use crate::user::uuid_generator::UuidUserIdGenerator;
use domain::transaction::TransactionManager;
use domain::user::{UserFactory, UserRepository};
use usecase::auth::interactor::AuthInteractor;
use usecase::auth::service::AuthService;
use usecase::auth::token_interactor::TokenInteractor;
use usecase::auth::token_service::TokenService;
use usecase::relay::event_mapper::EventMapper;
use usecase::relay::interactor::RelayInteractor;
use usecase::relay::service::OutboxRelayService;
use usecase::shared::email_service::EmailService;
use usecase::user::interactor::UserInteractor;
use usecase::user::service::UserService;

pub struct RepoRegistry<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
    user_repository: Arc<dyn UserRepository>, // TODO: Remove this at #34
}

impl RepoRegistry<SeaOrmTransactionManager> {
    /// SeaORM 用の具体的な実装で構築
    pub fn new_seaorm(db: sea_orm::DatabaseConnection) -> Self {
        let transaction_manager = Arc::new(SeaOrmTransactionManager::new(db.clone()));
        Self {
            transaction_manager,
            user_repository: Arc::new(SeaOrmUserRepository::new(
                db,
                Arc::new(EntityTracker::new()),
            )), // TODO: Remove this at #34
        }
    }
}

/// アプリケーション全体の依存関係を保持する構造体
pub struct AppRegistry {
    pub auth_service: Arc<dyn AuthService>,
    pub user_service: Arc<dyn UserService>,
    pub token_service: Arc<dyn TokenService>,
    pub outbox_relay_service: Arc<dyn OutboxRelayService>,
}

impl AppRegistry {
    pub fn new<TM: TransactionManager + 'static>(
        repos: RepoRegistry<TM>,
        email_service: Arc<dyn EmailService>,
        jwt_secret: String,
    ) -> Self {
        let password_hasher = Arc::new(Argon2PasswordHasher);

        let token_service = Arc::new(TokenInteractor::new(jwt_secret));

        let user_id_generator = Arc::new(UuidUserIdGenerator);

        let user_factory = Arc::new(UserFactory::new(user_id_generator.clone()));

        let auth_service = Arc::new(AuthInteractor::new(
            repos.transaction_manager.clone(),
            password_hasher,
            token_service.clone(),
            user_factory.clone(),
        ));

        let user_service = Arc::new(UserInteractor::new(repos.transaction_manager.clone()));

        let outbox_relay_service = Arc::new(RelayInteractor::new(
            repos.transaction_manager.clone(),
            Arc::new(EventMapper::new(
                email_service,
                repos.user_repository.clone(), // TODO: Remove this at #34 (EventMapper should not depend on UserRepository)
            )),
        ));

        Self {
            auth_service,
            user_service,
            token_service,
            outbox_relay_service,
        }
    }
}
