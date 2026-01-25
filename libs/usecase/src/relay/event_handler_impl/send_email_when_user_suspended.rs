use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox_event::OutboxEventId,
    user::{EmailTrait, UserSuspendedEvent},
};
use opentelemetry::trace::TraceId;
use tracing::{Level, Span};

use crate::shared::email_service::{EmailMessage, EmailService};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserSuspendedHandler {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UserSuspendedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserSuspendedHandler {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UserSuspendedEvent,
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
impl EventHandler for SendEmailWhenUserSuspendedHandler {
    fn outbox_event_id(&self) -> OutboxEventId {
        self.outbox_event_id
    }

    fn trace_id(&self) -> Option<TraceId> {
        self.trace_id
    }

    fn construct_span(&self) -> Span {
        tracing::span!(Level::INFO, "SendEmailWhenUserSuspended")
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
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
