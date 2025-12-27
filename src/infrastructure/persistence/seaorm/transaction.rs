use std::fmt::Debug;
use std::sync::Arc;

use super::repository::user_repository::SeaOrmUserRepository;
use crate::domain::repository::RepositoryFactory;
use crate::domain::transaction::{IntoTxError, TransactionManager};
use crate::domain::user::UserRepository;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

pub struct SeaOrmRepositoryFactory<'a> {
    txn: &'a DatabaseTransaction,
}

impl<'a> RepositoryFactory for SeaOrmRepositoryFactory<'a> {
    fn user_repository(&self) -> Box<dyn UserRepository + '_> {
        // ここで初めてインスタンス化される（遅延初期化）
        // SeaOrmUserRepositoryは軽量（接続参照を持つだけ）なので作成コストは低い
        Box::new(SeaOrmUserRepository::new(self.txn))
    }
}

pub struct SeaOrmTransactionManager {
    db: Arc<DatabaseConnection>,
}

impl SeaOrmTransactionManager {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TransactionManager for SeaOrmTransactionManager {
    async fn execute<T, E, F>(&self, f: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: IntoTxError + Debug + Send + Sync + 'static,
        F: for<'a> FnOnce(&'a dyn RepositoryFactory) -> BoxFuture<'a, Result<T, E>> + Send,
    {
        // 1. 手動でトランザクションを開始 (戻り値は Result<DatabaseTransaction, DbErr>)
        let txn = self.db.begin().await.map_err(|e| E::into_tx_error(e))?;

        // 2. ファクトリを作成（ここではまだ各リポジトリはnewされない）
        let factory = SeaOrmRepositoryFactory { txn: &txn };

        // 3. ユースケースにファクトリを渡す
        // factoryは &dyn RepositoryFactory として渡される
        let result = f(&factory).await;

        // 4. 結果に応じたコミット/ロールバック制御
        match result {
            Ok(value) => {
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
