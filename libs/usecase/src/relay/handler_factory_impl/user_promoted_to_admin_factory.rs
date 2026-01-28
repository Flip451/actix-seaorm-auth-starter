use domain::{shared::domain_event::DomainEvent, user::UserEvent};

use crate::relay::{
    event_handler::{EventHandler, HandlerContext},
    handler_factory::HandlerFactory,
};

pub struct UserPromotedToAdminFactory {}

impl UserPromotedToAdminFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UserPromotedToAdminFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl HandlerFactory for UserPromotedToAdminFactory {
    fn create(&self, event: &DomainEvent, _context: HandlerContext) -> Vec<Box<dyn EventHandler>> {
        if let DomainEvent::UserEvent(UserEvent::PromotedToAdmin(_user_promoted_to_admin_event)) =
            event
        {
            vec![]
        } else {
            self.report_misconfiguration(event);
            vec![]
        }
    }
}
