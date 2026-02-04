use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use app::telemetry;
use dotenvy::dotenv;
use relay::{RelayConfig, RelayWorker};
use sea_orm::Database;
use tokio_util::sync::CancellationToken;
use tracing_actix_web::TracingLogger;

use infrastructure::{
    AppRegistry, RepoRegistry, email_service::stub_email_service::email_service::StubEmailService,
    relay::next_attempt_calculator::backoff_next_attempt_calculator::BackoffCalculatorConfig,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. 環境準備
    dotenv().ok();

    // ログの初期化
    telemetry::init_telemetry();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let relay_batch_size = std::env::var("RELAY_BATCH_SIZE")
        .expect("RELAY_BATCH_SIZE must be set")
        .parse()
        .expect("RELAY_BATCH_SIZE must be a valid number");
    let relay_interval_secs = std::env::var("RELAY_INTERVAL_SECS")
        .expect("RELAY_INTERVAL_SECS must be set")
        .parse()
        .expect("RELAY_INTERVAL_SECS must be a valid number");

    let relay_config = RelayConfig::new(relay_batch_size, relay_interval_secs)
        .unwrap_or_else(|e| panic!("Failed to create RelayConfig: {e}"));

    let max_retries = std::env::var("RELAY_MAX_RETRIES")
        .expect("RELAY_MAX_RETRIES must be set")
        .parse()
        .expect("RELAY_MAX_RETRIES must be a valid number");
    let backoff_max_factor = std::env::var("RELAY_BACKOFF_MAX_FACTOR")
        .expect("RELAY_BACKOFF_MAX_FACTOR must be set")
        .parse()
        .expect("RELAY_BACKOFF_MAX_FACTOR must be a valid number");
    let backoff_base_factor = std::env::var("RELAY_BACKOFF_BASE_FACTOR")
        .expect("RELAY_BACKOFF_BASE_FACTOR must be set")
        .parse()
        .expect("RELAY_BACKOFF_BASE_FACTOR must be a valid number");
    let backoff_base_delay_seconds = std::env::var("RELAY_BACKOFF_BASE_DELAY_SECONDS")
        .expect("RELAY_BACKOFF_BASE_DELAY_SECONDS must be set")
        .parse()
        .expect("RELAY_BACKOFF_BASE_DELAY_SECONDS must be a valid number");
    let backoff_jitter_max_millis = std::env::var("RELAY_BACKOFF_JITTER_MAX_MILLIS")
        .expect("RELAY_BACKOFF_JITTER_MAX_MILLIS must be set")
        .parse()
        .expect("RELAY_BACKOFF_JITTER_MAX_MILLIS must be a valid number");

    let backoff_calculator_config = BackoffCalculatorConfig::new(
        max_retries,
        backoff_max_factor,
        backoff_base_factor,
        backoff_base_delay_seconds,
        backoff_jitter_max_millis,
    )
    .unwrap_or_else(|e| panic!("Failed to create BackoffCalculatorConfig: {}", e));

    let db_conn = Database::connect(database_url)
        .await
        .expect("Failed to connect DB");

    let cancel_token = CancellationToken::new();

    // 2. 依存関係の構築 (DI Containerとしての役割)
    // リポジトリ群を一括生成
    let repos = RepoRegistry::new_seaorm(db_conn);

    let email_service = Arc::new(StubEmailService::new());

    // DIコンテナ（Registry）の初期化
    let registry = AppRegistry::new(repos, email_service, jwt_secret, backoff_calculator_config);

    // Actix-web 内で共有するために web::Data にラップ
    let auth_service = web::Data::from(registry.auth_service.clone());
    let user_service = web::Data::from(registry.user_service.clone());
    let token_service = web::Data::from(registry.token_service.clone());

    println!("Starting outbox relay worker... ");

    // Relayワーカーの起動
    let relay_worker = RelayWorker::new(
        relay_config,
        registry.outbox_relay_service.clone(),
        cancel_token.clone(),
    );
    let relay_handle = relay_worker.spawn();

    println!("Starting server at http://0.0.0.0:8080");

    // 3. サーバー起動
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default()) // ログ・追跡用ミドルウェア
            .app_data(auth_service.clone())
            .app_data(user_service.clone())
            .app_data(token_service.clone())
            .configure(api::auth::handler::auth_config)
            .configure(api::user::handler::user_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    let result = server.await;

    cancel_token.cancel();
    let _ = relay_handle.await;

    telemetry::shutdown();

    result
}
