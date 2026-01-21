use super::OutboxEvent;

pub trait EntityWithEvents: Send {
    fn pull_events(&mut self) -> Vec<OutboxEvent>;
}
