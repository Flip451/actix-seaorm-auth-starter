use std::sync::Arc;

use domain::{shared::domain_event::DomainEvent, user::UserEvent};

use crate::{
    relay::{
        event_handler::{EventHandler, HandlerContext},
        event_handler_impl::SendEmailWhenUsernameChangedHandler,
        handler_factory::HandlerFactory,
    },
    shared::email_service::EmailService,
};

pub struct UsernameChangedFactory {
    email_service: Arc<dyn EmailService>,
}

impl UsernameChangedFactory {
    pub fn new(email_service: Arc<dyn EmailService>) -> Self {
        Self { email_service }
    }
}

impl HandlerFactory for UsernameChangedFactory {
    fn create(&self, event: &DomainEvent, context: HandlerContext) -> Vec<Box<dyn EventHandler>> {
        if let DomainEvent::UserEvent(UserEvent::UsernameChanged(user_username_changed_event)) =
            event
        {
            vec![Box::new(SendEmailWhenUsernameChangedHandler::new(
                context,
                user_username_changed_event.clone(),
                self.email_service.clone(),
            ))]
        } else {
            self.report_misconfiguration(event);
            vec![]
        }
    }
}
