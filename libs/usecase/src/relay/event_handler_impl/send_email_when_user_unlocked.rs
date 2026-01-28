use std::sync::Arc;

use async_trait::async_trait;
use domain::user::{EmailTrait, UserUnlockedEvent};

use crate::{
    relay::event_handler::HandlerContext,
    shared::email_service::{EmailMessage, EmailService},
};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserUnlockedHandler {
    context: HandlerContext,
    event: UserUnlockedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserUnlockedHandler {
    pub fn new(
        context: HandlerContext,
        event: UserUnlockedEvent,
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
impl EventHandler for SendEmailWhenUserUnlockedHandler {
    fn context(&self) -> &HandlerContext {
        &self.context
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError> {
        let UserUnlockedEvent {
            username,
            email,
            unlocked_at: _,
        } = &self.event;

        let to = email.as_str().to_string();
        let subject = "Your Account Has Been Unlocked".to_string();
        let body = format!(
            "Dear {username},\n\nYour account has been successfully unlocked. You can now log in and access our services.\n\nBest regards,\nThe Team"
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
