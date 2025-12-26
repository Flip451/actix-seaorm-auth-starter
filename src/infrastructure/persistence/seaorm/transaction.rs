use std::sync::Arc;

use super::repository::user_repository::SeaOrmUserRepository;
use crate::domain::repository::TxRepositories;
use crate::domain::transaction::{MapPersistenceError, TransactionManager};
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use sea_orm::{DatabaseConnection, TransactionTrait};

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
        E: MapPersistenceError + Send + Sync + 'static,
        F: for<'a> FnOnce(TxRepositories<'a>) -> BoxFuture<'a, Result<T, E>> + Send,
    {
        // 1. 手動でトランザクションを開始 (戻り値は Result<DatabaseTransaction, DbErr>)
        let txn = self
            .db
            .begin()
            .await
            .map_err(|e| E::from_persistence_error(e.to_string()))?;

        // 2. トランザクション接続を持つリポジトリを作成
        let user_repo = SeaOrmUserRepository::new(&txn);
        // let post_repo = SeaOrmPostRepository::new(&txn);

        // 3. コンテナ構造体にまとめる
        let repos = TxRepositories {
            user: &user_repo,
            // post: &post_repo,
        };

        // 4. ユースケースを実行
        let result = f(repos).await;

        // 5. 結果に応じたコミット/ロールバック制御
        match result {
            Ok(value) => {
                // 成功時はコミット
                txn.commit()
                    .await
                    .map_err(|e| E::from_persistence_error(e.to_string()))?;
                Ok(value)
            }
            Err(e) => {
                // 失敗時はロールバック（失敗しても元のエラー E を優先する）
                let _ = txn.rollback().await;
                Err(e)
            }
        }
    }
}
