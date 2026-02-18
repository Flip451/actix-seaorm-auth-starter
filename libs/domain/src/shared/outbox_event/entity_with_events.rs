use crate::shared::outbox_event::{OutboxEventIdGenerationError, OutboxEventIdGenerator};

use super::OutboxEvent;

pub trait EntityWithEvents: Send {
<<<<<<< HEAD
    fn drain_events(
        &mut self,
        id_generator: &dyn OutboxEventIdGenerator,
    ) -> Result<Vec<OutboxEvent>, OutboxEventIdGenerationError>;
=======
    fn pull_events(&mut self, id_generator: &dyn OutboxEventIdGenerator) -> Vec<OutboxEvent>;
>>>>>>> 895279a (Revert "refactor: #39 EntityTracker のリファクタリング")
}
