mod api;
mod domain;
mod infrastructure;
mod telemetry;
mod usecase;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sea_orm::Database;
use tracing_actix_web::TracingLogger;

use crate::infrastructure::{AppRegistry, RepoRegistry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. 環境準備
    dotenv().ok();

    // ログの初期化
    telemetry::init_telemetry();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let db_conn = Database::connect(database_url)
        .await
        .expect("Failed to connect DB");

    // 2. 依存関係の構築 (DI Containerとしての役割)
    // リポジトリ群を一括生成
    let repos = RepoRegistry::new_seaorm(db_conn);

    // DIコンテナ（Registry）の初期化
    let registry = AppRegistry::new(repos, jwt_secret);

    // Actix-web 内で共有するために web::Data にラップ
    let auth_service = web::Data::from(registry.auth_service.clone());
    let user_service = web::Data::from(registry.user_service.clone());

    println!("Starting server at http://0.0.0.0:8080");

    // 3. サーバー起動
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default()) // ログ・追跡用ミドルウェア
            .app_data(auth_service.clone())
            .app_data(user_service.clone())
            .configure(api::auth::handler::auth_config)
            .configure(api::user::handler::user_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    let result = server.await;

    telemetry::shutdown();

    result
}
