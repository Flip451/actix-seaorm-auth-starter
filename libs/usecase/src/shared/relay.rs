use std::{sync::Arc, vec};

use async_trait::async_trait;
use domain::{
    shared::{
        domain_event::DomainEvent,
        outbox_event::{OutboxEvent, OutboxEventId, OutboxReconstructionError},
    },
    user::{UserId, UserRepository, UserRepositoryError},
};
use opentelemetry::trace::{TraceContextExt, TraceId};
use thiserror::Error;
use tracing::{Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::shared::{
    email_service::{EmailService, EmailServiceError},
    event_handler::{
        send_email_when_user_created::SendEmailWhenUserCreatedHandler,
        send_email_when_user_deactivated::SendEmailWhenUserDeactivatedHandler,
        send_email_when_user_email_changed::SendEmailWhenUserEmailChanged,
        send_email_when_user_reactivated::SendEmailWhenUserReactivated,
        send_email_when_user_suspended::SendEmailWhenUserSuspendedHandler,
        send_email_when_user_unlocked::SendEmailWhenUserUnlockedHandler,
        send_email_when_user_username_changed::SendEmailWhenUsernameChanged,
    },
};

#[derive(Debug, Error)]
pub enum RelayError {
    #[error("ユーザーが見つかりません: {0}")]
    UserNotFound(UserId),

    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),

    #[error(transparent)]
    EmailServiceError(#[from] EmailServiceError),

    #[error("トレースIDのパースに失敗しました: {0}")]
    ParseTraceIdError(#[source] anyhow::Error),

    #[error("未知のイベントタイプ: {0}")]
    UnknownEventType(String),

    #[error(transparent)]
    ReconstructionError(#[from] OutboxReconstructionError),

    #[error("イベントの処理に失敗しました: {0}")]
    ProcessingError(#[source] anyhow::Error),
}

#[async_trait]
pub trait OutboxRelay: Send + Sync {
    /// 1バッチ分のイベントを取得して処理する
    /// 成功した場合、処理したイベントの数を返す
    async fn process_batch(&self) -> Result<usize, RelayError>;
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    fn outbox_event_id(&self) -> OutboxEventId;

    fn trace_id(&self) -> Option<TraceId>;

    async fn handle_event_raw(&self) -> Result<(), RelayError>;

    /// tracing::span!(Level::INFO, "example_event_name") と実装する
    fn construct_span(&self) -> Span;

    /// 指定されたイベントを処理する
    async fn handle_event(&self) -> Result<(), RelayError> {
        let span = self.construct_span();

        if let Some(trace_id) = self.trace_id() {
            let parent_context = opentelemetry::Context::new().with_remote_span_context(
                opentelemetry::trace::SpanContext::new(
                    trace_id,
                    opentelemetry::trace::SpanId::INVALID,
                    opentelemetry::trace::TraceFlags::SAMPLED,
                    true,
                    opentelemetry::trace::TraceState::default(),
                ),
            );

            span.set_parent(parent_context);
        }

        async move { self.handle_event_raw().await }
            .instrument(span)
            .await
    }
}

pub struct EventMapper {
    email_service: Arc<dyn EmailService>,
    user_repository: Arc<dyn UserRepository>,
}

impl EventMapper {
    pub fn new(
        email_service: Arc<dyn EmailService>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            email_service,
            user_repository,
        }
    }
}

impl EventMapper {
    pub fn map_event_to_handler(&self, outbox_event: OutboxEvent) -> Vec<Box<dyn EventHandler>> {
        let id = outbox_event.id();
        let trace_id = outbox_event.trace_id();
        let event = outbox_event.domain_event();

        match event {
            DomainEvent::UserEvent(user_event) => match user_event {
                domain::user::UserEvent::Created(user_created_event) => {
                    vec![Box::new(SendEmailWhenUserCreatedHandler::new(
                        id,
                        trace_id,
                        user_created_event.clone(),
                        self.email_service.clone(),
                        self.user_repository.clone(),
                    ))]
                }
                domain::user::UserEvent::Suspended(user_suspended_event) => {
                    vec![Box::new(SendEmailWhenUserSuspendedHandler::new(
                        id,
                        trace_id,
                        user_suspended_event.clone(),
                        self.email_service.clone(),
                        self.user_repository.clone(),
                    ))]
                }
                domain::user::UserEvent::Unlocked(user_unlocked_event) => {
                    vec![Box::new(SendEmailWhenUserUnlockedHandler::new(
                        id,
                        trace_id,
                        user_unlocked_event.clone(),
                        self.email_service.clone(),
                        self.user_repository.clone(),
                    ))]
                }
                domain::user::UserEvent::Deactivated(user_deactivated_event) => {
                    vec![Box::new(SendEmailWhenUserDeactivatedHandler::new(
                        id,
                        trace_id,
                        user_deactivated_event.clone(),
                        self.email_service.clone(),
                        self.user_repository.clone(),
                    ))]
                }
                domain::user::UserEvent::Reactivated(user_reactivated_event) => {
                    vec![Box::new(SendEmailWhenUserReactivated::new(
                        id,
                        trace_id,
                        user_reactivated_event.clone(),
                        self.email_service.clone(),
                        self.user_repository.clone(),
                    ))]
                }
                domain::user::UserEvent::PromotedToAdmin(_user_promoted_to_admin_event) => {
                    vec![]
                }
                domain::user::UserEvent::UsernameChanged(username_changed_event) => {
                    vec![Box::new(SendEmailWhenUsernameChanged::new(
                        id,
                        trace_id,
                        username_changed_event.clone(),
                        self.email_service.clone(),
                        self.user_repository.clone(),
                    ))]
                }
                domain::user::UserEvent::EmailChanged(user_email_changed_event) => {
                    vec![Box::new(SendEmailWhenUserEmailChanged::new(
                        id,
                        trace_id,
                        user_email_changed_event.clone(),
                        self.email_service.clone(),
                        self.user_repository.clone(),
                    ))]
                }
                domain::user::UserEvent::EmailVerified(_user_email_verified_event) => {
                    vec![]
                }
            },
        }
    }
}
