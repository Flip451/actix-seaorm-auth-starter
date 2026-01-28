use std::sync::Arc;

use async_trait::async_trait;
use domain::user::{EmailTrait, UserEmailChangedEvent};

use crate::{
    relay::event_handler::HandlerContext,
    shared::email_service::{EmailMessage, EmailService},
};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserEmailChangedHandler {
    context: HandlerContext,
    event: UserEmailChangedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserEmailChangedHandler {
    pub fn new(
        context: HandlerContext,
        event: UserEmailChangedEvent,
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
impl EventHandler for SendEmailWhenUserEmailChangedHandler {
    fn context(&self) -> &HandlerContext {
        &self.context
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError> {
        let UserEmailChangedEvent {
            new_email,
            username,
            changed_at: _,
        } = &self.event;

        let to = new_email.as_str().to_string();
        let subject = "Your email has been changed".to_string();
        let body = format!(
            "Hello {username},\n\nYour email has been changed to {new_email}.\n\nIf you did not make this change, please contact support immediately.",
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
