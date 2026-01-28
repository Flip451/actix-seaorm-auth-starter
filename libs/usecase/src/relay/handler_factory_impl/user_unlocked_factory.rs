use std::sync::Arc;

use domain::{shared::domain_event::DomainEvent, user::UserEvent};

use crate::{
    relay::{
        event_handler::{EventHandler, HandlerContext},
        event_handler_impl::SendEmailWhenUserUnlockedHandler,
        handler_factory::HandlerFactory,
    },
    shared::email_service::EmailService,
};

pub struct UserUnlockedFactory {
    email_service: Arc<dyn EmailService>,
}

impl UserUnlockedFactory {
    pub fn new(email_service: Arc<dyn EmailService>) -> Self {
        Self { email_service }
    }
}

impl HandlerFactory for UserUnlockedFactory {
    fn create(&self, event: &DomainEvent, context: HandlerContext) -> Vec<Box<dyn EventHandler>> {
        if let DomainEvent::UserEvent(UserEvent::Unlocked(user_unlocked_event)) = event {
            vec![Box::new(SendEmailWhenUserUnlockedHandler::new(
                context,
                user_unlocked_event.clone(),
                self.email_service.clone(),
            ))]
        } else {
            self.report_misconfiguration(event);
            vec![]
        }
    }
}
