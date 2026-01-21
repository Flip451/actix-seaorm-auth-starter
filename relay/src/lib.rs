pub mod config;
pub use config::RelayConfig;

use std::sync::Arc;
use tokio::{task::JoinHandle, time::Interval};
use tokio_util::sync::CancellationToken;
use usecase::relay::service::OutboxRelayService;

pub fn spawn_relay(
    relay: Arc<dyn OutboxRelayService>,
    token: CancellationToken,
    config: RelayConfig,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        // interval_secs 秒ごとにポーリングを実行する設定
        let mut interval: Interval = config.interval_secs.into();
        let batch_size = config.batch_size.into();

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // PENDING状態のイベントをバッチ処理
                    match relay.process_batch(batch_size).await {
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
