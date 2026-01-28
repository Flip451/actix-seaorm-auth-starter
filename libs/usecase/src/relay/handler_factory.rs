use domain::shared::domain_event::DomainEvent;

use crate::relay::event_handler::{EventHandler, HandlerContext};

pub trait HandlerFactory: Send + Sync {
    fn create(&self, event: &DomainEvent, context: HandlerContext) -> Vec<Box<dyn EventHandler>>;

    fn get_factory_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn report_misconfiguration(&self, event: &DomainEvent) {
        let factory_name = self.get_factory_name();

        tracing::error!(
            event_type = ?event,
            "Configuration Error: {factory_name} received a mismatched event type. Check EventMapper configuration."
        );
    }
}
