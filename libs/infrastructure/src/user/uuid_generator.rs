use std::sync::Arc;

use crate::shared::uuid::{calculate_v7_timestamp_parts, generate_uuid_v7_with_parts};
use domain::{
    shared::service::clock::Clock,
    user::{IdGenerator, IdGeneratorFactory, UserId},
};
use uuid::ContextV7;

pub struct UuidUserIdGenerator {
    clock: Arc<dyn Clock>,
    context: ContextV7,
}

impl UuidUserIdGenerator {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            clock,
            context: ContextV7::new(),
        }
    }
}

impl IdGenerator for UuidUserIdGenerator {
    fn generate(&self) -> UserId {
        let (seconds, nanos) = calculate_v7_timestamp_parts(self.clock.now());
        generate_uuid_v7_with_parts(&self.context, seconds, nanos).into()
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

impl IdGeneratorFactory for UuidUserIdGeneratorFactory {
    fn create_user_id_generator(&self) -> Box<dyn IdGenerator> {
        Box::new(UuidUserIdGenerator::new(self.clock.clone()))
    }
}
