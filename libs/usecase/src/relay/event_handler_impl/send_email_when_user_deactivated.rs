use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox_event::OutboxEventId,
    user::{EmailTrait, UserDeactivatedEvent},
};
use opentelemetry::trace::TraceId;

use crate::shared::email_service::{EmailMessage, EmailService};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserDeactivatedHandler {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UserDeactivatedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserDeactivatedHandler {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UserDeactivatedEvent,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            outbox_event_id,
            trace_id,
            event,
            email_service,
        }
    }
}

#[async_trait]
impl EventHandler for SendEmailWhenUserDeactivatedHandler {
    fn outbox_event_id(&self) -> OutboxEventId {
        self.outbox_event_id
    }

    fn trace_id(&self) -> Option<TraceId> {
        self.trace_id
    }

    fn construct_span(&self) -> tracing::Span {
        tracing::span!(tracing::Level::INFO, "SendEmailWhenUserDeactivated")
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
