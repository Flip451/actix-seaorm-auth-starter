use std::sync::{Arc, Mutex};

use crate::shared::uuid::{calculate_v7_timestamp_parts, generate_uuid_v7_with_parts};
use domain::{
    shared::service::clock::Clock,
    user::{UserId, UserIdGenerationError, UserIdGenerator, UserIdGeneratorFactory},
};
use uuid::ContextV7;

pub struct UuidUserIdGenerator {
    clock: Arc<dyn Clock>,
    context: Mutex<ContextV7>,
}

impl UuidUserIdGenerator {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            clock,
            context: Mutex::new(ContextV7::new()),
        }
    }
}

impl UserIdGenerator for UuidUserIdGenerator {
    fn generate(&self) -> Result<UserId, UserIdGenerationError> {
        let (seconds, nanos) = calculate_v7_timestamp_parts(self.clock.now())
            .map_err(|e| UserIdGenerationError::GenerationFailed(e.into()))?;
        Ok(generate_uuid_v7_with_parts(&self.context, seconds, nanos).into())
    }
}

pub struct UuidUserIdGeneratorFactory {
    clock: Arc<dyn Clock>,
}

impl UuidUserIdGeneratorFactory {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }
}

impl UserIdGeneratorFactory for UuidUserIdGeneratorFactory {
    fn create_user_id_generator(&self) -> Arc<dyn UserIdGenerator> {
        Arc::new(UuidUserIdGenerator::new(self.clock.clone()))
    }
}
