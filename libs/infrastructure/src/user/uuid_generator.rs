use std::sync::Arc;

use crate::shared::uuid::generate_uuid_v7;
use domain::{
    shared::service::clock::Clock,
    user::{IdGenerator, UserId},
};

pub struct UuidUserIdGenerator {
    clock: Arc<dyn Clock>,
}

impl UuidUserIdGenerator {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }
}

impl IdGenerator for UuidUserIdGenerator {
    fn generate(&self) -> UserId {
        generate_uuid_v7(self.clock.now()).into()
    }
}
