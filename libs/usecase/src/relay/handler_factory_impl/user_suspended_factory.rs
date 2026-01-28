use std::sync::Arc;

use domain::{shared::domain_event::DomainEvent, user::UserEvent};

use crate::{
    relay::{
        event_handler::{EventHandler, HandlerContext},
        event_handler_impl::SendEmailWhenUserSuspendedHandler,
        handler_factory::HandlerFactory,
    },
    shared::email_service::EmailService,
};

pub struct UserSuspendedFactory {
    email_service: Arc<dyn EmailService>,
}

impl UserSuspendedFactory {
    pub fn new(email_service: Arc<dyn EmailService>) -> Self {
        Self { email_service }
    }
}

impl HandlerFactory for UserSuspendedFactory {
    fn create(&self, event: &DomainEvent, context: HandlerContext) -> Vec<Box<dyn EventHandler>> {
        if let DomainEvent::UserEvent(UserEvent::Suspended(user_suspended_event)) = event {
            vec![Box::new(SendEmailWhenUserSuspendedHandler::new(
                context,
                user_suspended_event.clone(),
                self.email_service.clone(),
            ))]
        } else {
            self.report_misconfiguration(event);
            vec![]
        }
    }
}
