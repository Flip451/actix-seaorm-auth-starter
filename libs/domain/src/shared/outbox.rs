use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use opentelemetry::trace::TraceContextExt;
use thiserror::Error;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use crate::user::UserEvent;

pub struct OutboxEvent {
    pub id: Uuid,
    pub event: DomainEvent,
    pub trace_id: Option<String>,
    pub created_at: DateTime<FixedOffset>,
}

impl OutboxEvent {
    pub fn new(event: DomainEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            event,
            trace_id: Self::get_current_trace_id(),
            created_at: DateTime::<FixedOffset>::from(chrono::offset::Utc::now()),
        }
    }

    fn get_current_trace_id() -> Option<String> {
        let span = Span::current();
        let context = span.context();
        let span_ref = context.span();
        let span_context = span_ref.span_context();

        if span_context.is_valid() {
            // Format the trace ID as a hex string
            Some(format!("{:x}", span_context.trace_id()))
        } else {
            None
        }
    }
}

pub enum DomainEvent {
    UserEvent(UserEvent),
    // 将来的に他のイベントタイプも追加可能
}

#[derive(Debug, Error)]
pub enum OutboxRepositoryError {
    #[error("イベントの保存に失敗しました: {0}")]
    Persistence(#[source] anyhow::Error),
}

#[async_trait]
pub trait OutboxRepository: Send + Sync {
    async fn save(&self, event: OutboxEvent) -> Result<(), OutboxRepositoryError>;
}
