use crate::shared::outbox_event::OutboxEventIdGenerator;

use super::OutboxEvent;

pub trait EntityWithEvents: Send {
    fn pull_events(&mut self, id_generator: &dyn OutboxEventIdGenerator) -> Vec<OutboxEvent>;
}
