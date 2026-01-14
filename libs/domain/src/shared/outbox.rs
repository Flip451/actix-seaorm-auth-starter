use async_trait::async_trait;
use chrono::{DateTime, Utc};
use opentelemetry::trace::{TraceContextExt, TraceId};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use crate::user::UserEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OutboxEventId(pub(crate) Uuid);

pub struct OutboxEvent {
    pub id: OutboxEventId,
    pub event: DomainEvent,
    pub trace_id: Option<TraceId>,
    pub created_at: DateTime<Utc>,
}

impl From<OutboxEventId> for Uuid {
    fn from(outbox_event_id: OutboxEventId) -> Self {
        outbox_event_id.0
    }
}

impl From<Uuid> for OutboxEventId {
    fn from(uuid: Uuid) -> Self {
        OutboxEventId(uuid)
    }
}

impl OutboxEvent {
    pub fn new(event: DomainEvent) -> Self {
        Self {
            id: OutboxEventId(Uuid::new_v4()),
            event,
            trace_id: Self::get_current_trace_id(),
            created_at: Utc::now(),
        }
    }

    fn get_current_trace_id() -> Option<TraceId> {
        let span = Span::current();
        let context = span.context();
        let span_ref = context.span();
        let span_context = span_ref.span_context();

        if span_context.is_valid() {
            Some(span_context.trace_id())
        } else {
            None
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum DomainEvent {
    UserEvent(UserEvent),
    // 将来的に他のイベントタイプも追加可能
}

impl From<UserEvent> for DomainEvent {
    fn from(event: UserEvent) -> Self {
        DomainEvent::UserEvent(event)
    }
}

#[derive(Debug, Error)]
pub enum OutboxRepositoryError {
    #[error("イベントの保存に失敗しました: {0}")]
    Persistence(#[source] anyhow::Error),
}

#[async_trait]
pub trait OutboxRepository: Send + Sync {
    async fn save(&self, event: OutboxEvent) -> Result<(), OutboxRepositoryError>;
    async fn save_all(&self, events: Vec<OutboxEvent>) -> Result<(), OutboxRepositoryError>;
}

pub trait EntityWithEvents: Send {
    fn pull_events(&mut self) -> Vec<OutboxEvent>;
}
