use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox::OutboxEventId,
    user::{EmailTrait, UserEmailChangedEvent, UserRepository},
};
use opentelemetry::trace::TraceId;

use crate::shared::{
    email_service::{EmailMessage, EmailService},
    relay::{EventHandler, RelayError},
};

pub struct SendEmailWhenUserEmailChanged {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UserEmailChangedEvent,
    email_service: Arc<dyn EmailService>,
    user_repository: Arc<dyn UserRepository>,
}

impl SendEmailWhenUserEmailChanged {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UserEmailChangedEvent,
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
            user_id,
            new_email,
            changed_at: _,
        } = &self.event;

        // ここでメール送信のロジックを実装します
        let user = self
            .user_repository
            .find_by_id(*user_id)
            .await
            .map_err(RelayError::UserRepositoryError)?
            .ok_or_else(|| RelayError::UserNotFound(*user_id))?;

        let username = user.username();

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
