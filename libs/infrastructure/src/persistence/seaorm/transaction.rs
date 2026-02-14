use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use crate::persistence::seaorm::repository::outbox_repository::SeaOrmPostgresOutboxRepository;

use super::repository::user_repository::SeaOrmUserRepository;
use async_trait::async_trait;
use domain::repository::RepositoryFactory;
use domain::shared::outbox_event::{
    EntityWithEvents, OutboxEvent, OutboxEventIdGenerator, OutboxRepository,
};
use domain::transaction::{IntoTxError, TransactionManager};
use domain::user::UserRepository;
use futures_util::future::BoxFuture;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

pub struct EntityTracker {
    events: Mutex<Vec<OutboxEvent>>,
    outbox_event_id_generator: Arc<dyn OutboxEventIdGenerator>,
}

impl EntityTracker {
    pub fn new(outbox_event_id_generator: Arc<dyn OutboxEventIdGenerator>) -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            outbox_event_id_generator,
        }
    }

    pub fn track(&self, mut entity: Box<dyn EntityWithEvents>) {
        let new_events = entity.drain_events(self.outbox_event_id_generator.as_ref());

        if new_events.is_empty() {
            return;
        }

        let mut events = self.events.lock().unwrap();

        events.extend(new_events);
    }

    pub fn drain_all_events(&self) -> Vec<OutboxEvent> {
        let mut events = self.events.lock().unwrap();

        events.drain(..).collect()
    }
}

pub struct SeaOrmRepositoryFactory<'a> {
    txn: &'a DatabaseTransaction,
    tracker: Arc<EntityTracker>,
}

impl<'a> RepositoryFactory<'a> for SeaOrmRepositoryFactory<'a> {
    fn user_repository(&self) -> Arc<dyn UserRepository + 'a> {
        // ここで初めてインスタンス化される（遅延初期化）
        // SeaOrmUserRepositoryは軽量（接続参照を持つだけ）なので作成コストは低い
        Arc::new(SeaOrmUserRepository::new(self.txn, self.tracker.clone()))
    }

    fn outbox_repository(&self) -> Arc<dyn OutboxRepository + 'a> {
        Arc::new(SeaOrmPostgresOutboxRepository::new(self.txn))
    }
}

pub struct SeaOrmTransactionManager {
    db: DatabaseConnection,
    outbox_event_id_generator: Arc<dyn OutboxEventIdGenerator>,
}

impl SeaOrmTransactionManager {
    pub fn new(
        db: DatabaseConnection,
        outbox_event_id_generator: Arc<dyn OutboxEventIdGenerator>,
    ) -> Self {
        Self {
            db,
            outbox_event_id_generator,
        }
    }
}

#[async_trait]
impl TransactionManager for SeaOrmTransactionManager {
    async fn execute<T, E, F>(&self, f: F) -> Result<T, E>
    where
        T: Send,
        E: IntoTxError + Debug + Send + Sync,
        F: for<'a> FnOnce(&'a dyn RepositoryFactory) -> BoxFuture<'a, Result<T, E>> + Send,
    {
        // 1. 手動でトランザクションを開始 (戻り値は Result<DatabaseTransaction, DbErr>)
        let txn = self.db.begin().await.map_err(|e| E::into_tx_error(e))?;

        // 2. ファクトリを作成（ここではまだ各リポジトリはnewされない）
        let factory = SeaOrmRepositoryFactory {
            txn: &txn,
            tracker: Arc::new(EntityTracker::new(self.outbox_event_id_generator.clone())),
        };

        // 3. ユースケースにファクトリを渡す
        // factoryは &dyn RepositoryFactory として渡される
        let result = f(&factory).await;

        // 4. 結果に応じたコミット/ロールバック制御
        match result {
            Ok(value) => {
                // 5. トラッカーから全エンティティのイベントを回収
                let all_events = factory.tracker.drain_all_events();

                // 6. イベントがある場合、同一トランザクション内で Outbox 保存
                if !all_events.is_empty() {
                    let outbox_repo = factory.outbox_repository();
                    outbox_repo
                        .save_all(all_events)
                        .await
                        .map_err(|e| E::into_tx_error(e))?;
                }

                // 成功時はコミット
                txn.commit().await.map_err(|e| E::into_tx_error(e))?;
                Ok(value)
            }
            Err(e) => {
                // 失敗時はロールバック（失敗しても元のエラー E を優先する）
                if let Err(rollback_err) = txn.rollback().await {
                    tracing::error!(
                        error = ?rollback_err,
                        "Failed to rollback transaction. Original error: {:?}",
                        e
                    );
                }
                Err(e)
            }
        }
    }
}
