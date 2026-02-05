use crate::shared::outbox_event::OutboxEventIdGenerator;

use super::OutboxEvent;

pub trait EntityWithEvents: Send {
    fn drain_events(&mut self, id_generator: &dyn OutboxEventIdGenerator) -> Vec<OutboxEvent>;

    fn tracking_id(&self) -> String;
}
