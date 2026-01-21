use std::{sync::Arc, time::Duration};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use usecase::relay::service::OutboxRelayService;

/// Relayワーカー（バックグラウンド処理）の動作設定を保持する構造体。
///
/// Outboxイベントをポーリングして処理する際のパフォーマンス特性（スループットと負荷）を制御します。
pub struct RelayConfig {
    /// 1回のバッチ処理でデータベースから取得し、処理するイベントの最大数。
    ///
    /// # 役割
    /// 一度のトランザクションでロック（`FOR UPDATE SKIP LOCKED`）を取得する行数を決定します。
    ///
    /// # 推奨値とトレードオフ
    /// - **小さい値 (例: 10)**: トランザクションが短くなり、ロック競合が減りますが、大量のイベントに対するスループットは低下します。
    /// - **大きい値 (例: 100+)**: スループットは向上しますが、処理時間が長引くとDB接続を占有し続けたり、メモリ使用量が増加するリスクがあります。
    pub batch_size: u64,

    /// ポーリングを実行する間隔（秒単位）。
    ///
    /// # 役割
    /// 新しいイベントがないかデータベースを確認しに行く頻度を制御します。
    ///
    /// # 挙動
    /// 指定された秒数ごとにループが回り、`process_batch` が呼び出されます。
    ///
    /// # トレードオフ
    /// - **短い間隔 (例: 1-5秒)**: リアルタイム性が向上しますが、アイドル時でもDBへのクエリ負荷が発生します。
    /// - **長い間隔 (例: 60秒)**: DB負荷は下がりますが、イベント発生から処理開始までの遅延（レイテンシ）が増加します。
    pub interval_secs: u64,
}

pub fn spawn_relay(
    relay: Arc<dyn OutboxRelayService>,
    token: CancellationToken,
    config: RelayConfig,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        // interval_secs 秒ごとにポーリングを実行する設定
        let mut interval = tokio::time::interval(Duration::from_secs(config.interval_secs));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // PENDING状態のイベントをバッチ処理
                    match relay.process_batch(config.batch_size).await {
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
