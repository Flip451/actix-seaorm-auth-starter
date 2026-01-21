use std::sync::Arc;

use async_trait::async_trait;
use domain::{transaction::TransactionManager, tx};

use super::error::RelayError;
use super::event_mapper::EventMapper;
use super::service::OutboxRelayService;

pub struct RelayInteractor<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
    mapper: Arc<EventMapper>,
}

impl<TM: TransactionManager> RelayInteractor<TM> {
    pub fn new(transaction_manager: Arc<TM>, mapper: Arc<EventMapper>) -> Self {
        Self {
            transaction_manager,
            mapper,
        }
    }
}

// TODO: #51 で成功したハンドラーを追跡し、部分的な再試行ロジックを実装する
// TODO: #43 で冪等性の確保
#[async_trait]
impl<TM: TransactionManager> OutboxRelayService for RelayInteractor<TM> {
    #[tracing::instrument(skip(self))]
    async fn process_batch(&self, limit: u64) -> Result<usize, RelayError> {
        let mapper = self.mapper.clone();

        tx!(self.transaction_manager, |factory| {
            let outbox_repo = factory.outbox_repository();

            let mut events = outbox_repo.lock_pending_events(limit).await?;

            let count = events.len();

            // NOTE: ここで件数が0の場合、何もせずに正常終了(Ok)する
            // TransactionManagerはこれを「成功」とみなして空のトランザクションをコミットする
            if count == 0 {
                return Ok(0);
            }

            for event in events.iter_mut() {
                let handlers = mapper.map_event_to_handler(event);

                let mut success = true;
                for handler in handlers {
                    let event_id = handler.outbox_event_id();
                    if let Err(e) = handler.handle_event().await {
                        tracing::error!(error = ?e, %event_id, "イベントハンドラの実行に失敗しました");
                        success = false;
                        break;
                    }
                }
                if success {
                    event.complete()?;
                } else {
                    event.fail()?;
                }
            }

            outbox_repo.save_all(events).await?;

            Ok(count)
        })
        .await
    }
}
