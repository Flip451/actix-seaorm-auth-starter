use std::sync::{Arc, Mutex};

use crate::shared::uuid::{calculate_v7_timestamp_parts, generate_uuid_v7_with_parts};
use domain::shared::{
    outbox_event::{
        OutboxEventId, OutboxEventIdGenerationError, OutboxEventIdGenerator,
        OutboxEventIdGeneratorFactory,
    },
    service::clock::Clock,
};
use uuid::ContextV7;

pub struct UuidOutboxIdGenerator {
    clock: Arc<dyn Clock>,
    context: Mutex<ContextV7>,
}

impl UuidOutboxIdGenerator {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            clock,
            context: Mutex::new(ContextV7::new()),
        }
    }
}

impl OutboxEventIdGenerator for UuidOutboxIdGenerator {
    fn generate(&self) -> Result<OutboxEventId, OutboxEventIdGenerationError> {
        let (seconds, nanos) = calculate_v7_timestamp_parts(self.clock.now())
            .map_err(|e| OutboxEventIdGenerationError::GenerationFailed(e.into()))?;
        Ok(generate_uuid_v7_with_parts(&self.context, seconds, nanos).into())
    }
}

pub struct UuidOutboxEventIdGeneratorFactory {
    clock: Arc<dyn Clock>,
}

impl UuidOutboxEventIdGeneratorFactory {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }
}

impl OutboxEventIdGeneratorFactory for UuidOutboxEventIdGeneratorFactory {
    fn create_outbox_event_id_generator(&self) -> Arc<dyn OutboxEventIdGenerator> {
        Arc::new(UuidOutboxIdGenerator::new(self.clock.clone()))
    }
}
