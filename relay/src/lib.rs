use infrastructure::{
    email_service::stub_email_service::email_service::StubEmailService,
    persistence::seaorm::{
        relay::SeaOrmOutboxRelay, repository::user_repository::SeaOrmUserRepository,
        transaction::EntityTracker,
    },
};
use sea_orm::DatabaseConnection;
use std::{sync::Arc, time::Duration};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use usecase::shared::relay::{EventMapper, OutboxRelay};

pub fn spawn_relay(db_conn: DatabaseConnection, token: CancellationToken) -> JoinHandle<()> {
    let email_service = Arc::new(StubEmailService::new());
    let relay_user_repo = Arc::new(SeaOrmUserRepository::new(
        db_conn.clone(),
        Arc::new(EntityTracker::new()),
    ));

    let event_mapper = Arc::new(EventMapper::new(email_service, relay_user_repo));

    let relay = SeaOrmOutboxRelay::new(db_conn, event_mapper);

    tokio::spawn(async move {
        // 5秒ごとにポーリングを実行する設定
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // PENDING状態のイベントをバッチ処理
                    match relay.process_batch().await {
                        Ok(count) => {
                            if count > 0 {
                                tracing::info!("Processed {} events", count);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to process batch: {:?}", e);
                        }
                    }
                }
                _ = token.cancelled() => {
                    // 停止命令を受けたらループを抜ける
                    tracing::info!("Relay worker receiving stop signal...");
                    break;
                }
            }
        }

        tracing::info!("Relay worker stopped gracefully.");
    })
}
