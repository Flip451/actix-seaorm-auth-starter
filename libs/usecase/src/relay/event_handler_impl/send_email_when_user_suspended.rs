use std::sync::Arc;

use async_trait::async_trait;
use domain::user::{EmailTrait, UserSuspendedEvent};

use crate::{
    relay::event_handler::HandlerContext,
    shared::email_service::{EmailMessage, EmailService},
};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserSuspendedHandler {
    context: HandlerContext,
    event: UserSuspendedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserSuspendedHandler {
    pub fn new(
        context: HandlerContext,
        event: UserSuspendedEvent,
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
impl EventHandler for SendEmailWhenUserSuspendedHandler {
    fn context(&self) -> &HandlerContext {
        &self.context
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError> {
        let UserSuspendedEvent {
            username,
            suspended_at: _,
            reason,
            email,
        } = &self.event;

        let to = email.as_str().to_string();
        let subject = "Your Account Has Been Suspended".to_string();
        let body = format!(
            "Dear {username},\n\nYour account has been suspended for the following reason:\n{reason}\n\nIf you believe this is a mistake, please contact support.",
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(|e| RelayError::ProcessingError(e.into()))?;

        Ok(())
    }
}
