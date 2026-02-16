use std::sync::Arc;

use crate::shared::uuid::{calculate_v7_timestamp_parts, generate_uuid_v7_with_parts};
use domain::shared::{
    outbox_event::{IdGeneratorFactory, OutboxEventId, OutboxEventIdGenerator},
    service::clock::Clock,
};
use uuid::ContextV7;

pub struct UuidOutboxIdGenerator {
    clock: Arc<dyn Clock>,
    context: ContextV7,
}

impl UuidOutboxIdGenerator {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            clock,
            context: ContextV7::new(),
        }
    }
}

impl OutboxEventIdGenerator for UuidOutboxIdGenerator {
    fn generate(&self) -> OutboxEventId {
        let (seconds, nanos) = calculate_v7_timestamp_parts(self.clock.now());
        generate_uuid_v7_with_parts(&self.context, seconds, nanos).into()
    }
}

pub struct UuidOutboxIdGeneratorFactory {
    clock: Arc<dyn Clock>,
}

impl UuidOutboxIdGeneratorFactory {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }
}

impl IdGeneratorFactory for UuidOutboxIdGeneratorFactory {
    fn create_outbox_event_id_generator(&self) -> Box<dyn OutboxEventIdGenerator> {
        Box::new(UuidOutboxIdGenerator::new(self.clock.clone()))
    }
}
