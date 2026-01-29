use std::sync::Arc;

use async_trait::async_trait;
use domain::user::{EmailTrait, UserCreatedEvent};

use crate::{
    relay::event_handler::HandlerContext,
    shared::email_service::{EmailMessage, EmailService},
};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserCreatedHandler {
    context: HandlerContext,
    event: UserCreatedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserCreatedHandler {
    pub fn new(
        context: HandlerContext,
        event: UserCreatedEvent,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            context,
            event,
            email_service,
        }
    }
}

#[async_trait]
impl EventHandler for SendEmailWhenUserCreatedHandler {
    fn context(&self) -> &HandlerContext {
        &self.context
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError> {
        let UserCreatedEvent {
            email,
            username,
            registered_at: _,
        } = &self.event;

        let to = email.as_str().to_string();
        let subject = "Welcome to Our Service!".to_string();
        let body = format!(
            "Dear {username},\n\nThank you for registering with us.\n\nBest regards,\nThe Team",
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service.send_email(email_message).await?;

        Ok(())
    }
}
