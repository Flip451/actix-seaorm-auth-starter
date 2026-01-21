use super::error::RelayError;
use async_trait::async_trait;

#[async_trait]
pub trait OutboxRelayService: Send + Sync {
    /// 1バッチ分のイベントを取得して処理する
    /// 成功した場合、処理を試みたイベントの数（失敗したものも含む）を返す
    async fn process_batch(&self, limit: u64) -> Result<usize, RelayError>;
}
