use crate::domain::repository::TxRepositories;
use async_trait::async_trait;
use futures_util::future::BoxFuture;

// 1. DB等のシステムエラーを、そのドメインのエラー型に変換するためのトレイト
pub trait MapPersistenceError {
    fn from_persistence_error(msg: String) -> Self;
}

#[async_trait]
pub trait TransactionManager: Send + Sync {
    // E は MapPersistenceError を実装している必要がある
    async fn execute<T, E, F>(&self, f: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: MapPersistenceError + Send + Sync + 'static,
        F: for<'a> FnOnce(TxRepositories<'a>) -> BoxFuture<'a, Result<T, E>> + Send;
}
