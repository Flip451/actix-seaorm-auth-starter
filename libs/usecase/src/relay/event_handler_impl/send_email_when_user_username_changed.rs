use std::sync::Arc;

use async_trait::async_trait;
use domain::user::UsernameChangedEvent;

use crate::{
    relay::event_handler::HandlerContext,
    shared::email_service::{EmailMessage, EmailService},
};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUsernameChanged {
    context: HandlerContext,
    event: UsernameChangedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUsernameChanged {
    pub fn new(
        context: HandlerContext,
        event: UsernameChangedEvent,
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
impl EventHandler for SendEmailWhenUsernameChanged {
    fn context(&self) -> &HandlerContext {
        &self.context
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError> {
        let UsernameChangedEvent {
            old_username,
            new_username,
            email,
            changed_at: _,
        } = &self.event;

        let to = email.as_str().to_string();
        let subject = "Your username has been changed".to_string();
        let body = format!(
            "Dear {new_username},\n\nYour username has been changed from {old_username} to {new_username}\n\nBest regards,\nThe Team",
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
