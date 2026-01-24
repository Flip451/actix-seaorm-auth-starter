pub mod config;
pub use config::RelayConfig;

use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use usecase::relay::service::OutboxRelayService;

/// ワーカーの現在の状態を表すステートマシン
enum RelayState {
    /// 待機状態: 定期実行のタイミング（interval）を待つ
    Idle,
    /// 処理中状態: イベントが残っているため、即座に次のバッチを処理する
    Busy,
}

pub fn spawn_relay(
    relay: Arc<dyn OutboxRelayService>,
    token: CancellationToken,
    config: RelayConfig,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        // interval_secs 秒ごとにポーリングを実行する設定
        let mut interval = config.interval_secs();
        let batch_size = config.batch_size();

        // 初期状態は Idle
        let mut state = RelayState::Idle;

        loop {
            match state {
                // [Idle 状態] interval 秒待機
                RelayState::Idle => {
                    tokio::select! {
                        _ = interval.tick() => {
                            // 時間が来たら Busy 状態に遷移
                            state = RelayState::Busy;
                        }
                        _ = token.cancelled() => {
                            // 待機中にシャットダウン信号が来たら終了
                            tracing::info!("Relay worker received stop signal during idle...");
                            break;
                        }
                    }
                }

                // [Busy 状態] 即座に次のバッチを処理
                RelayState::Busy => {
                    tokio::select! {
                        process_result = relay.process_batch(batch_size) => {
                            match process_result {
                                Ok(count) => {
                                    if count > 0 {
                                        tracing::info!("Processed {} events", count);
                                    }

                                    // 取得件数が上限未満なら「空になった」とみなして Idle へ戻る
                                    // 上限いっぱいなら、まだ残っているとみなして Busy を維持（連続実行）
                                    if count < batch_size as usize {
                                        state = RelayState::Idle;
                                    } else {
                                        tracing::debug!("Batch full, remaining busy");
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Failed to process batch: {:?}", e);
                                    // エラー発生時は Idle 状態に戻る
                                    // ※将来的にここで RelayState::Backoff などへ遷移させることも可能
                                    state = RelayState::Idle;
                                }
                            }
                        }
                        _ = token.cancelled() => {
                            // 処理中にシャットダウン信号が来たら終了
                            tracing::info!("Relay worker received stop signal during busy...");
                            break;
                        }
                    }
                }
            }
        }

        tracing::info!("Relay worker stopped gracefully.");
    })
}
