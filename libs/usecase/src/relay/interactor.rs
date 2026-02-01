use std::sync::Arc;

use async_trait::async_trait;
use domain::shared::service::clock::Clock;
use domain::{transaction::TransactionManager, tx};
use futures_util::future;

use super::error::RelayError;
use super::event_mapper::EventMapper;
use super::service::OutboxRelayService;

/// Outboxパターンにおけるリレー処理の中核を担うインタラクター。
///
/// この構造体は、永続化されたドメインイベント（Outbox）を非同期に処理し、
/// 外部システムへの副作用（メール送信など）を確実に実行する責務を持ちます。
///
/// # 主な役割
/// - **バッチ処理**: 未処理（PENDING）のイベントを指定された件数分（limit）取得します。
/// - **トランザクション管理**: `TransactionManager` と連携し、イベントの取得（ロック）、
///   ハンドラの実行、ステータス更新（COMPLETED/FAILED）を一連のトランザクションとして原子的に実行します。
/// - **イベントのディスパッチ**: `EventMapper` を使用して、Outboxイベントを具体的な処理を持つ
///   `EventHandler` に変換し、実行を委譲します。
///
/// # 並列実行について
/// 複数のRelayプロセスが起動している場合でも、リポジトリ層の `lock_pending_events`
/// (SELECT FOR UPDATE SKIP LOCKED相当) により、同一イベントの二重処理が防止されます。
pub struct RelayInteractor<TM: TransactionManager> {
    transaction_manager: Arc<TM>,
    mapper: Arc<EventMapper>,
    clock: Arc<dyn Clock>,
}

impl<TM: TransactionManager> RelayInteractor<TM> {
    pub fn new(
        transaction_manager: Arc<TM>,
        mapper: Arc<EventMapper>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            transaction_manager,
            mapper,
            clock,
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
        let clock = self.clock.clone();

        tx!(self.transaction_manager, |factory| {
            let outbox_repo = factory.outbox_repository();

            let mut events = outbox_repo.lock_pending_events(limit).await?;

            let count = events.len();

            // NOTE: ここで件数が0の場合、何もせずに正常終了(Ok)する
            // TransactionManagerはこれを「成功」とみなして空のトランザクションをコミットする
            if count == 0 {
                return Ok(0);
            }

            // 2. イベントごとにハンドラを生成し、非同期タスクリストを作成
            let mut tasks = Vec::with_capacity(count);

            for event in &events {
                // EventMapper はイベントデータをコピーして新しいハンドラを生成するため
                // ここで event への参照を持ったまま map しても安全です
                let handlers = mapper.map_event_to_handler(event);

                // 各イベントの処理を非同期タスクとして定義
                tasks.push(async move {
                    let mut success = true;
                    // 1つのイベントに対して複数のハンドラがある場合、それらは順次実行
                    // (1つでも失敗したらそのイベントは失敗とみなす)
                    for handler in handlers {
                        let event_id = handler.outbox_event_id();
                        if let Err(e) = handler.handle_event().await {
                            tracing::error!(error = ?e, %event_id, "イベントハンドラの実行に失敗しました");
                            success = false;
                            break;
                        }
                    }
                    success
                });
            }

            // 3. 全タスクを並列実行
            // ここですべてのメール送信等のIO待ちが並列に行われます
            let results = future::join_all(tasks).await;

            // 4. 結果に基づいてステータスを更新
            for (event, success) in events.iter_mut().zip(results) {
                if success {
                    event.complete(clock.as_ref())?;
                } else {
                    event.fail(clock.as_ref())?;
                }
            }

            // 5. 更新結果を一括保存
            outbox_repo.save_all(events).await?;

            Ok(count)
        })
        .await
    }
}
