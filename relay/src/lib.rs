use infrastructure::{
    email_service::stub_email_service::email_service::StubEmailService,
    persistence::seaorm::{
        relay::SeaOrmOutboxRelay, repository::user_repository::SeaOrmUserRepository,
        transaction::EntityTracker,
    },
};
use sea_orm::DatabaseConnection;
use std::{sync::Arc, time::Duration};
use usecase::shared::relay::{EventMapper, OutboxRelay};

pub fn spawn_relay(db_conn: DatabaseConnection) {
    let relay_db = db_conn.clone();
    let email_service = Arc::new(StubEmailService::new());
    let relay_user_repo = Arc::new(SeaOrmUserRepository::new(
        relay_db.clone(),
        Arc::new(EntityTracker::new()),
    ));

    let event_mapper = Arc::new(EventMapper {
        email_service,
        user_repository: relay_user_repo,
    });

    let relay = SeaOrmOutboxRelay::new(relay_db, event_mapper);

    tokio::spawn(async move {
        // 5秒ごとにポーリングを実行する設定
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

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
    });
}
