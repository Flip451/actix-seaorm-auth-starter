use std::sync::Arc;

use async_trait::async_trait;
use domain::user::{EmailTrait, UserReactivatedEvent};

use crate::{
    relay::event_handler::HandlerContext,
    shared::email_service::{EmailMessage, EmailService},
};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserReactivatedHandler {
    context: HandlerContext,
    event: UserReactivatedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserReactivatedHandler {
    pub fn new(
        context: HandlerContext,
        event: UserReactivatedEvent,
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
impl EventHandler for SendEmailWhenUserReactivatedHandler {
    fn context(&self) -> &HandlerContext {
        &self.context
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError> {
        let UserReactivatedEvent {
            username,
            email,
            reactivated_at: _,
        } = &self.event;

        let email = email.as_str().to_string();

        let to = email;
        let subject = "Your Account Has Been Reactivated".to_string();
        let body = format!(
            "Hello {username},\n\nYour account has been successfully reactivated.\n\nBest regards,\nThe Team",
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
