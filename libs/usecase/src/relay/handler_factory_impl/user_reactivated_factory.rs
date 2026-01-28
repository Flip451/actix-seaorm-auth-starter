use std::sync::Arc;

use domain::{shared::domain_event::DomainEvent, user::UserEvent};

use crate::{
    relay::{
        event_handler::{EventHandler, HandlerContext},
        event_handler_impl::SendEmailWhenUserReactivatedHandler,
        handler_factory::HandlerFactory,
    },
    shared::email_service::EmailService,
};

pub struct UserReactivatedFactory {
    email_service: Arc<dyn EmailService>,
}

impl UserReactivatedFactory {
    pub fn new(email_service: Arc<dyn EmailService>) -> Self {
        Self { email_service }
    }
}

impl HandlerFactory for UserReactivatedFactory {
    fn create(&self, event: &DomainEvent, context: HandlerContext) -> Vec<Box<dyn EventHandler>> {
        if let DomainEvent::UserEvent(UserEvent::Reactivated(user_reactivated_event)) = event {
            vec![Box::new(SendEmailWhenUserReactivatedHandler::new(
                context,
                user_reactivated_event.clone(),
                self.email_service.clone(),
            ))]
        } else {
            self.report_misconfiguration(event);
            vec![]
        }
    }
}
