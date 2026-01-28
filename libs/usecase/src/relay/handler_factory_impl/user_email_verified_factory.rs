use domain::{shared::domain_event::DomainEvent, user::UserEvent};

use crate::relay::{
    event_handler::{EventHandler, HandlerContext},
    handler_factory::HandlerFactory,
};

pub struct UserEmailVerifiedFactory {}

impl UserEmailVerifiedFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UserEmailVerifiedFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl HandlerFactory for UserEmailVerifiedFactory {
    fn create(&self, event: &DomainEvent, _context: HandlerContext) -> Vec<Box<dyn EventHandler>> {
        if let DomainEvent::UserEvent(UserEvent::EmailVerified(_user_email_verified_event)) = event
        {
            vec![]
        } else {
            self.report_misconfiguration(event);
            vec![]
        }
    }
}
