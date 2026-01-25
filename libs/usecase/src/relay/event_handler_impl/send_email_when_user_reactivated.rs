use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox_event::OutboxEventId,
    user::{EmailTrait, UserReactivatedEvent},
};
use opentelemetry::trace::TraceId;

use crate::shared::email_service::{EmailMessage, EmailService};

use super::super::{error::RelayError, event_handler::EventHandler};

pub struct SendEmailWhenUserReactivated {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UserReactivatedEvent,
    email_service: Arc<dyn EmailService>,
}

impl SendEmailWhenUserReactivated {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UserReactivatedEvent,
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
impl EventHandler for SendEmailWhenUserReactivated {
    fn outbox_event_id(&self) -> OutboxEventId {
        self.outbox_event_id
    }

    fn trace_id(&self) -> Option<TraceId> {
        self.trace_id
    }

    fn construct_span(&self) -> tracing::Span {
        tracing::span!(tracing::Level::INFO, "SendEmailWhenUserReactivated")
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
