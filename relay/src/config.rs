use std::time::Duration;

use thiserror::Error;
use tokio::time::Interval;

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
    pub(crate) batch_size: BatchSize,

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
    pub(crate) interval_secs: IntervalSecs,
}

impl RelayConfig {
    pub fn new(batch_size: u64, interval_secs: u64) -> Result<Self, RelayConfigError> {
        Ok(Self {
            batch_size: BatchSize::new(batch_size)?,
            interval_secs: IntervalSecs::new(interval_secs)?,
        })
    }
}

#[derive(Debug, Error)]
pub enum RelayConfigError {
    #[error("Invalid batch size")]
    InvalidBatchSize,
    #[error("Invalid interval seconds")]
    InvalidIntervalSecs,
}

pub struct BatchSize(u64);

impl BatchSize {
    pub fn new(value: u64) -> Result<Self, RelayConfigError> {
        if value == 0 {
            return Err(RelayConfigError::InvalidBatchSize);
        }
        Ok(Self(value))
    }
}

impl From<BatchSize> for u64 {
    fn from(value: BatchSize) -> Self {
        value.0
    }
}

pub struct IntervalSecs(u64);

impl IntervalSecs {
    pub fn new(value: u64) -> Result<Self, RelayConfigError> {
        if value == 0 {
            return Err(RelayConfigError::InvalidIntervalSecs);
        }
        Ok(Self(value))
    }
}

impl From<IntervalSecs> for Interval {
    fn from(value: IntervalSecs) -> Self {
        tokio::time::interval(Duration::from_secs(value.0))
    }
}
