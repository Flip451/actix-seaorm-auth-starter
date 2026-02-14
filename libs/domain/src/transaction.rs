use crate::repository::RepositoryFactory;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use std::fmt::Debug;

// 1. DB等のシステムエラーを、そのドメインのエラー型に変換するためのトレイト
pub trait IntoTxError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self;
}

#[async_trait]
pub trait TransactionManager: Send + Sync {
    // E は IntoTxError を実装している必要がある
    async fn execute<T, E, F>(&self, f: F) -> Result<T, E>
    where
        T: Send,
        E: IntoTxError + Debug + Send + Sync,
        F: for<'a> FnOnce(&'a dyn RepositoryFactory) -> BoxFuture<'a, Result<T, E>> + Send;
}

#[macro_export]
macro_rules! tx {
    ($tm:expr, |$factory:ident| $body:expr) => {
        $tm.execute::<_, _, _>(move |$factory| std::boxed::Box::pin(async move { $body }))
    };
}
