use chrono::{DateTime, Utc};
use domain::shared::service::clock::Clock;

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
