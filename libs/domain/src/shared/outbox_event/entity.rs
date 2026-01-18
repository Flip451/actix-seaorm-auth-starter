use chrono::{DateTime, Utc};
use opentelemetry::trace::{TraceContextExt, TraceId};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use crate::shared::domain_event::DomainEvent;

use super::OutboxEventId;

pub struct OutboxEvent {
    pub id: OutboxEventId,
    pub event: DomainEvent,
    pub trace_id: Option<TraceId>,
    pub created_at: DateTime<Utc>,
}

impl OutboxEvent {
    pub fn new(event: DomainEvent) -> Self {
        Self {
            id: Uuid::new_v4().into(),
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

pub trait EntityWithEvents: Send {
    fn pull_events(&mut self) -> Vec<OutboxEvent>;
}
