use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OutboxEventId(pub(crate) Uuid);

impl From<OutboxEventId> for Uuid {
    fn from(outbox_event_id: OutboxEventId) -> Self {
        outbox_event_id.0
    }
}

impl From<Uuid> for OutboxEventId {
    fn from(uuid: Uuid) -> Self {
        OutboxEventId(uuid)
    }
}
