use std::sync::Arc;

use crate::shared::uuid::generate_uuid_v7;
use domain::shared::{
    outbox_event::{OutboxEventId, OutboxEventIdGenerator},
    service::clock::Clock,
};

pub struct UuidOutboxIdGenerator {
    clock: Arc<dyn Clock>,
}

impl UuidOutboxIdGenerator {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }
}

impl OutboxEventIdGenerator for UuidOutboxIdGenerator {
    fn generate(&self) -> OutboxEventId {
        generate_uuid_v7(self.clock.now()).into()
    }
}
