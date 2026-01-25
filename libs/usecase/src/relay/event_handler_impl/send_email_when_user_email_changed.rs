use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox_event::OutboxEventId,
    user::{EmailTrait, UserEmailChangedEvent},
};
use opentelemetry::trace::TraceId;

use crate::shared::email_service::{EmailMessage, EmailService};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserEmailChanged {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UserEmailChangedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserEmailChanged {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UserEmailChangedEvent,
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
impl EventHandler for SendEmailWhenUserEmailChanged {
    fn outbox_event_id(&self) -> OutboxEventId {
        self.outbox_event_id
    }

    fn trace_id(&self) -> Option<TraceId> {
        self.trace_id
    }

    fn construct_span(&self) -> tracing::Span {
        tracing::span!(tracing::Level::INFO, "SendEmailWhenUserEmailChanged")
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
