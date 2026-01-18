use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    shared::outbox_event::OutboxEventId,
    user::{EmailTrait, UserCreatedEvent, UserRepository},
};
use opentelemetry::trace::TraceId;
use tracing::{Level, Span};

use crate::shared::{
    email_service::{EmailMessage, EmailService},
    relay::{EventHandler, RelayError},
};

pub struct SendEmailWhenUserCreatedHandler {
    outbox_event_id: OutboxEventId,
    trace_id: Option<TraceId>,
    event: UserCreatedEvent,
    email_service: Arc<dyn EmailService>,
    user_repository: Arc<dyn UserRepository>,
}

impl SendEmailWhenUserCreatedHandler {
    pub fn new(
        outbox_event_id: OutboxEventId,
        trace_id: Option<TraceId>,
        event: UserCreatedEvent,
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
            user_id,
            email,
            registered_at: _,
        } = &self.event;

        let user = self
            .user_repository
            .find_by_id(*user_id)
            .await
            .map_err(RelayError::UserRepositoryError)?
            .ok_or_else(|| RelayError::UserNotFound(*user_id))?;

        let username = user.username();

        let to = email.as_str().to_string();
        let subject = "Welcome to Our Service!".to_string();
        let body = format!(
            "Dear {username},\n\nThank you for registering with us. Your user ID is: {user_id}\n\nBest regards,\nThe Team",
        );

        let email_message = EmailMessage { to, subject, body };

        self.email_service
            .send_email(email_message)
            .await
            .map_err(RelayError::EmailServiceError)?;

        Ok(())
    }
}
