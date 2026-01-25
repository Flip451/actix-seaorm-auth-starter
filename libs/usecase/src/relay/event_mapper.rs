use std::sync::Arc;

use domain::shared::{domain_event::DomainEvent, outbox_event::OutboxEvent};

use crate::shared::email_service::EmailService;

use super::event_handler::EventHandler;

use super::event_handler_impl::{
    SendEmailWhenUserCreatedHandler, SendEmailWhenUserDeactivatedHandler,
    SendEmailWhenUserEmailChanged, SendEmailWhenUserReactivated, SendEmailWhenUserSuspendedHandler,
    SendEmailWhenUserUnlockedHandler, SendEmailWhenUsernameChanged,
};

pub struct EventMapper {
    email_service: Arc<dyn EmailService>,
}

impl EventMapper {
    pub fn new(email_service: Arc<dyn EmailService>) -> Self {
        Self { email_service }
    }
}

impl EventMapper {
    pub fn map_event_to_handler(&self, outbox_event: &OutboxEvent) -> Vec<Box<dyn EventHandler>> {
        let id = outbox_event.id();
        let trace_id = outbox_event.trace_id();
        let event = outbox_event.domain_event();

        match event {
            DomainEvent::UserEvent(user_event) => match user_event {
                domain::user::UserEvent::Created(user_created_event) => {
                    vec![Box::new(SendEmailWhenUserCreatedHandler::new(
                        id,
                        trace_id,
                        user_created_event.clone(),
                        self.email_service.clone(),
                    ))]
                }
                domain::user::UserEvent::Suspended(user_suspended_event) => {
                    vec![Box::new(SendEmailWhenUserSuspendedHandler::new(
                        id,
                        trace_id,
                        user_suspended_event.clone(),
                        self.email_service.clone(),
                    ))]
                }
                domain::user::UserEvent::Unlocked(user_unlocked_event) => {
                    vec![Box::new(SendEmailWhenUserUnlockedHandler::new(
                        id,
                        trace_id,
                        user_unlocked_event.clone(),
                        self.email_service.clone(),
                    ))]
                }
                domain::user::UserEvent::Deactivated(user_deactivated_event) => {
                    vec![Box::new(SendEmailWhenUserDeactivatedHandler::new(
                        id,
                        trace_id,
                        user_deactivated_event.clone(),
                        self.email_service.clone(),
                    ))]
                }
                domain::user::UserEvent::Reactivated(user_reactivated_event) => {
                    vec![Box::new(SendEmailWhenUserReactivated::new(
                        id,
                        trace_id,
                        user_reactivated_event.clone(),
                        self.email_service.clone(),
                    ))]
                }
                domain::user::UserEvent::PromotedToAdmin(_user_promoted_to_admin_event) => {
                    vec![]
                }
                domain::user::UserEvent::UsernameChanged(username_changed_event) => {
                    vec![Box::new(SendEmailWhenUsernameChanged::new(
                        id,
                        trace_id,
                        username_changed_event.clone(),
                        self.email_service.clone(),
                    ))]
                }
                domain::user::UserEvent::EmailChanged(user_email_changed_event) => {
                    vec![Box::new(SendEmailWhenUserEmailChanged::new(
                        id,
                        trace_id,
                        user_email_changed_event.clone(),
                        self.email_service.clone(),
                    ))]
                }
                domain::user::UserEvent::EmailVerified(_user_email_verified_event) => {
                    vec![]
                }
            },
        }
    }
}
