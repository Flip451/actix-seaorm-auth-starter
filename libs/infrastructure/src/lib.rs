pub mod auth;
pub mod email_service;
pub mod outbox_event;
pub mod persistence;
pub mod relay;
pub mod shared;
pub mod user;

use std::sync::Arc;

use crate::auth::argon2::password_service::Argon2PasswordHasher;
use crate::outbox_event::outbox_event_id_generator::UuidOutboxEventIdGeneratorFactory;
use crate::persistence::seaorm::transaction::SeaOrmTransactionManager;
use crate::relay::next_attempt_calculator::backoff_next_attempt_calculator::{
    BackoffCalculatorConfig, BackoffNextAttemptCalculator,
};
use crate::shared::clock::SystemClock;
use crate::user::uuid_generator::UuidUserIdGeneratorFactory;
use domain::transaction::TransactionManager;
use domain::user::UserFactory;
use usecase::auth::interactor::AuthInteractor;
use usecase::auth::service::AuthService;
use usecase::auth::token_interactor::TokenInteractor;
use usecase::auth::token_service::TokenService;
use usecase::relay::event_mapper::{EventFactories, EventMapper};
use usecase::relay::handler_factory_impl::user_created_factory::UserCreatedFactory;
use usecase::relay::handler_factory_impl::user_deactivated_factory::UserDeactivatedFactory;
use usecase::relay::handler_factory_impl::user_email_changed_factory::UserEmailChangedFactory;
use usecase::relay::handler_factory_impl::user_email_verified_factory::UserEmailVerifiedFactory;
use usecase::relay::handler_factory_impl::user_promoted_to_admin_factory::UserPromotedToAdminFactory;
use usecase::relay::handler_factory_impl::user_reactivated_factory::UserReactivatedFactory;
use usecase::relay::handler_factory_impl::user_suspended_factory::UserSuspendedFactory;
use usecase::relay::handler_factory_impl::user_unlocked_factory::UserUnlockedFactory;
use usecase::relay::handler_factory_impl::username_changed_factory::UsernameChangedFactory;
use usecase::relay::interactor::RelayInteractor;
use usecase::relay::service::OutboxRelayService;
use usecase::shared::email_service::EmailService;
use usecase::user::interactor::UserInteractor;
use usecase::user::service::UserService;

pub struct RepoRegistry<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
}

impl RepoRegistry<SeaOrmTransactionManager> {
    /// SeaORM 用の具体的な実装で構築
    pub fn new_seaorm(db: sea_orm::DatabaseConnection) -> Self {
        let clock = Arc::new(SystemClock);
        let outbox_event_id_generator_factory =
            Arc::new(UuidOutboxEventIdGeneratorFactory::new(clock.clone()));
        let transaction_manager = Arc::new(SeaOrmTransactionManager::new(
            db.clone(),
            outbox_event_id_generator_factory,
        ));
        Self {
            transaction_manager,
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
        backoff_calculator_config: BackoffCalculatorConfig,
    ) -> Self {
        let clock = Arc::new(SystemClock);
        let next_attempt_calculator =
            Arc::new(BackoffNextAttemptCalculator::new(backoff_calculator_config));

        let password_hasher = Arc::new(Argon2PasswordHasher);

        let token_service = Arc::new(TokenInteractor::new(jwt_secret, clock.clone()));

        let user_id_generator_factory = Arc::new(UuidUserIdGeneratorFactory::new(clock.clone()));

        let user_factory = Arc::new(UserFactory::new(clock.clone()));

        let auth_service = Arc::new(AuthInteractor::new(
            repos.transaction_manager.clone(),
            password_hasher,
            token_service.clone(),
            user_factory.clone(),
            user_id_generator_factory.clone(),
        ));

        let user_service = Arc::new(UserInteractor::new(
            repos.transaction_manager.clone(),
            clock.clone(),
        ));

        let user_created_factory = UserCreatedFactory::new(email_service.clone());
        let user_suspended_factory = UserSuspendedFactory::new(email_service.clone());
        let user_unlocked_factory = UserUnlockedFactory::new(email_service.clone());
        let user_deactivated_factory = UserDeactivatedFactory::new(email_service.clone());
        let user_reactivated_factory = UserReactivatedFactory::new(email_service.clone());
        let user_promoted_to_admin_factory = UserPromotedToAdminFactory::new();
        let user_username_changed_factory = UsernameChangedFactory::new(email_service.clone());
        let user_email_changed_factory = UserEmailChangedFactory::new(email_service.clone());
        let user_email_verified_factory = UserEmailVerifiedFactory::new();

        let event_mapper = EventMapper::new(EventFactories {
            user_created: Box::new(user_created_factory),
            user_suspended: Box::new(user_suspended_factory),
            user_unlocked: Box::new(user_unlocked_factory),
            user_deactivated: Box::new(user_deactivated_factory),
            user_reactivated: Box::new(user_reactivated_factory),
            user_promoted_to_admin: Box::new(user_promoted_to_admin_factory),
            user_username_changed: Box::new(user_username_changed_factory),
            user_email_changed: Box::new(user_email_changed_factory),
            user_email_verified: Box::new(user_email_verified_factory),
        });

        let outbox_relay_service = Arc::new(RelayInteractor::new(
            repos.transaction_manager.clone(),
            Arc::new(event_mapper),
            next_attempt_calculator,
            clock.clone(),
        ));

        Self {
            auth_service,
            user_service,
            token_service,
            outbox_relay_service,
        }
    }
}
