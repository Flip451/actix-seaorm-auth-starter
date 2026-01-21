use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use app::telemetry;
use dotenvy::dotenv;
use relay::RelayConfig;
use sea_orm::Database;
use tokio_util::sync::CancellationToken;
use tracing_actix_web::TracingLogger;

use infrastructure::{
    AppRegistry, RepoRegistry, email_service::stub_email_service::email_service::StubEmailService,
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

    let relay_config = RelayConfig {
        batch_size: relay_batch_size,
        interval_secs: relay_interval_secs,
    };

    let db_conn = Database::connect(database_url)
        .await
        .expect("Failed to connect DB");

    let cancel_token = CancellationToken::new();

    // 2. 依存関係の構築 (DI Containerとしての役割)
    // リポジトリ群を一括生成
    let repos = RepoRegistry::new_seaorm(db_conn);

    let email_service = Arc::new(StubEmailService::new());

    // DIコンテナ（Registry）の初期化
    let registry = AppRegistry::new(repos, email_service, jwt_secret);

    // Actix-web 内で共有するために web::Data にラップ
    let auth_service = web::Data::from(registry.auth_service.clone());
    let user_service = web::Data::from(registry.user_service.clone());
    let token_service = web::Data::from(registry.token_service.clone());

    println!("Starting outbox relay worker... ");

    // Relayワーカーの起動
    let relay_handle = relay::spawn_relay(
        registry.outbox_relay_service.clone(),
        cancel_token.clone(),
        relay_config,
    );

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
