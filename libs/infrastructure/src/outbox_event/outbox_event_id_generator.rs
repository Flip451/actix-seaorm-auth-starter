use domain::shared::outbox_event::{OutboxEventId, OutboxEventIdGenerator};
use sea_orm::prelude::Uuid;

pub struct UuidOutboxIdGenerator;

impl OutboxEventIdGenerator for UuidOutboxIdGenerator {
    fn generate(&self) -> OutboxEventId {
        Uuid::new_v4().into()
    }
}
