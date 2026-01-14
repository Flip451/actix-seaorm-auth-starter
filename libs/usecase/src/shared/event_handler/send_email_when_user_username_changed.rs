use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox::OutboxEventId,
    user::{UserRepository, UsernameChangedEvent},
};
use opentelemetry::trace::TraceId;

use crate::shared::{
    email_service::{EmailMessage, EmailService},
    relay::{EventHandler, RelayError},
};

pub struct SendEmailWhenUsernameChanged {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UsernameChangedEvent,
    email_service: Arc<dyn EmailService>,
    user_repository: Arc<dyn UserRepository>,
}

impl SendEmailWhenUsernameChanged {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UsernameChangedEvent,
        email_service: Arc<dyn EmailService>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            outbox_event_id,
            trace_id,
            event,
            email_service,
            user_repository,
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
            user_id,
            new_username,
            changed_at: _,
        } = &self.event;

        // ここでメール送信のロジックを実装します
        let user = self
            .user_repository
            .find_by_id(*user_id)
            .await
            .map_err(RelayError::UserRepositoryError)?
            .ok_or_else(|| RelayError::UserNotFound(*user_id))?;

        let to = user.email().as_str().to_string();
        let subject = "Your username has been changed".to_string();
        let body = format!(
            "Dear {new_username},\n\nYour username has been changed to: {new_username}\n\nBest regards,\nThe Team",
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
