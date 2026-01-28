use domain::shared::{domain_event::DomainEvent, outbox_event::OutboxEvent};
use domain::user::UserEvent;

use super::{
    event_handler::{EventHandler, HandlerContext},
    handler_factory::HandlerFactory,
};

pub struct EventMapper {
    user_created_factory: Box<dyn HandlerFactory>,
    user_suspended_factory: Box<dyn HandlerFactory>,
    user_unlocked_factory: Box<dyn HandlerFactory>,
    user_deactivated_factory: Box<dyn HandlerFactory>,
    user_reactivated_factory: Box<dyn HandlerFactory>,
    user_promoted_to_admin_factory: Box<dyn HandlerFactory>,
    username_changed_factory: Box<dyn HandlerFactory>,
    user_email_changed_factory: Box<dyn HandlerFactory>,
    user_email_verified_factory: Box<dyn HandlerFactory>,
}

pub struct EventFactories {
    pub user_created: Box<dyn HandlerFactory>,
    pub user_suspended: Box<dyn HandlerFactory>,
    pub user_unlocked: Box<dyn HandlerFactory>,
    pub user_deactivated: Box<dyn HandlerFactory>,
    pub user_reactivated: Box<dyn HandlerFactory>,
    pub user_promoted_to_admin: Box<dyn HandlerFactory>,
    pub user_username_changed: Box<dyn HandlerFactory>,
    pub user_email_changed: Box<dyn HandlerFactory>,
    pub user_email_verified: Box<dyn HandlerFactory>,
}

impl EventMapper {
    pub fn new(factories: EventFactories) -> Self {
        Self {
            user_created_factory: factories.user_created,
            user_suspended_factory: factories.user_suspended,
            user_unlocked_factory: factories.user_unlocked,
            user_deactivated_factory: factories.user_deactivated,
            user_reactivated_factory: factories.user_reactivated,
            user_promoted_to_admin_factory: factories.user_promoted_to_admin,
            username_changed_factory: factories.user_username_changed,
            user_email_changed_factory: factories.user_email_changed,
            user_email_verified_factory: factories.user_email_verified,
        }
    }
}

impl EventMapper {
    pub fn map_event_to_handler(&self, outbox_event: &OutboxEvent) -> Vec<Box<dyn EventHandler>> {
        let outbox_event_id = outbox_event.id();
        let trace_id = outbox_event.trace_id();

        let context = HandlerContext {
            outbox_event_id,
            trace_id,
        };
        let event = outbox_event.domain_event();

        match event {
            DomainEvent::UserEvent(user_event) => match user_event {
                UserEvent::Created(_) => self.user_created_factory.create(event, context),
                UserEvent::Suspended(_) => self.user_suspended_factory.create(event, context),
                UserEvent::Unlocked(_) => self.user_unlocked_factory.create(event, context),
                UserEvent::Deactivated(_) => self.user_deactivated_factory.create(event, context),
                UserEvent::Reactivated(_) => self.user_reactivated_factory.create(event, context),
                UserEvent::PromotedToAdmin(_) => {
                    self.user_promoted_to_admin_factory.create(event, context)
                }
                UserEvent::UsernameChanged(_) => {
                    self.username_changed_factory.create(event, context)
                }
                UserEvent::EmailChanged(_) => {
                    self.user_email_changed_factory.create(event, context)
                }
                UserEvent::EmailVerified(_) => {
                    self.user_email_verified_factory.create(event, context)
                }
            },
        }
    }
}
