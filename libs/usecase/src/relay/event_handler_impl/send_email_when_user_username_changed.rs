use std::sync::Arc;

use async_trait::async_trait;
use domain::{shared::outbox_event::OutboxEventId, user::UsernameChangedEvent};
use opentelemetry::trace::TraceId;

use crate::shared::email_service::{EmailMessage, EmailService};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUsernameChanged {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UsernameChangedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUsernameChanged {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UsernameChangedEvent,
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
impl EventHandler for SendEmailWhenUsernameChanged {
    fn outbox_event_id(&self) -> OutboxEventId {
        self.outbox_event_id
    }

    fn trace_id(&self) -> Option<TraceId> {
        self.trace_id
    }

    fn construct_span(&self) -> tracing::Span {
        tracing::span!(tracing::Level::INFO, "SendEmailWhenUsernameChanged")
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
