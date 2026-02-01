use chrono::{DateTime, Utc};

/// 時刻生成を抽象化するドメインサービス
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}
