use crate::shared::outbox_event::{OutboxEventIdGenerationError, OutboxEventIdGenerator};

use super::OutboxEvent;

pub trait EntityWithEvents: Send {
    fn drain_events(
        &mut self,
        id_generator: &dyn OutboxEventIdGenerator,
    ) -> Result<Vec<OutboxEvent>, OutboxEventIdGenerationError>;
}
