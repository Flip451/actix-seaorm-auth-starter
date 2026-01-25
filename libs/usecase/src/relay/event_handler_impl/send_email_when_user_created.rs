use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox_event::OutboxEventId,
    user::{EmailTrait, UserCreatedEvent},
};
use opentelemetry::trace::TraceId;
use tracing::{Level, Span};

use crate::shared::email_service::{EmailMessage, EmailService};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserCreatedHandler {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UserCreatedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserCreatedHandler {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UserCreatedEvent,
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
impl EventHandler for SendEmailWhenUserCreatedHandler {
    fn outbox_event_id(&self) -> OutboxEventId {
        self.outbox_event_id
    }

    fn trace_id(&self) -> Option<TraceId> {
        self.trace_id
    }

    fn construct_span(&self) -> Span {
        tracing::span!(Level::INFO, "SendEmailWhenUserCreated")
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

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
