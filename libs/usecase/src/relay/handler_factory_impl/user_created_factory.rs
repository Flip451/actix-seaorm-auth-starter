use std::sync::Arc;

use domain::{shared::domain_event::DomainEvent, user::UserEvent};

use crate::{
    relay::{
        event_handler::{EventHandler, HandlerContext},
        event_handler_impl::SendEmailWhenUserCreatedHandler,
        handler_factory::HandlerFactory,
    },
    shared::email_service::EmailService,
};

pub struct UserCreatedFactory {
    email_service: Arc<dyn EmailService>,
}

impl UserCreatedFactory {
    pub fn new(email_service: Arc<dyn EmailService>) -> Self {
        Self { email_service }
    }
}

impl HandlerFactory for UserCreatedFactory {
    fn create(&self, event: &DomainEvent, context: HandlerContext) -> Vec<Box<dyn EventHandler>> {
        if let DomainEvent::UserEvent(UserEvent::Created(user_created_event)) = event {
            vec![Box::new(SendEmailWhenUserCreatedHandler::new(
                context,
                user_created_event.clone(),
                self.email_service.clone(),
            ))]
        } else {
            self.report_misconfiguration(event);
            vec![]
        }
    }
}
