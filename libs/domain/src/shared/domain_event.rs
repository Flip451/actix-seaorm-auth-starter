use serde::{Deserialize, Serialize};

use crate::user::UserEvent;

#[derive(Deserialize, Serialize)]
pub enum DomainEvent {
    UserEvent(UserEvent),
    // 将来的に他のイベントタイプも追加可能
}

impl From<UserEvent> for DomainEvent {
    fn from(event: UserEvent) -> Self {
        DomainEvent::UserEvent(event)
    }
}
