use std::sync::Arc;

use async_trait::async_trait;
use domain::user::{EmailTrait, UserDeactivatedEvent};

use crate::{
    relay::event_handler::HandlerContext,
    shared::email_service::{EmailMessage, EmailService},
};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserDeactivatedHandler {
    context: HandlerContext,
    event: UserDeactivatedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserDeactivatedHandler {
    pub fn new(
        context: HandlerContext,
        event: UserDeactivatedEvent,
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
impl EventHandler for SendEmailWhenUserDeactivatedHandler {
    fn context(&self) -> &HandlerContext {
        &self.context
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError> {
        let UserDeactivatedEvent {
            username,
            email,
            deactivated_at: _,
        } = &self.event;

        let to = email.as_str().to_string();
        let subject = "Account Deactivation Notice".to_string();
        let body = format!(
            "Dear {username},\n\nYour account has been deactivated. If you have any questions, please contact support.\n\nBest regards,\nThe Team"
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
