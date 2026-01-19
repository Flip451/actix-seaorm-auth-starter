use chrono::{DateTime, Utc};
use opentelemetry::trace::{TraceContextExt, TraceId};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use crate::shared::domain_event::DomainEvent;

use super::OutboxEventId;

pub struct OutboxEvent {
    id: OutboxEventId,
    event: DomainEvent,
    status: OutboxEventStatus,
    trace_id: Option<TraceId>,
    created_at: DateTime<Utc>,
}

impl OutboxEvent {
    pub fn new(event: DomainEvent) -> Self {
        Self {
            id: Uuid::new_v4().into(),
            event,
            status: OutboxEventStatus::Pending,
            trace_id: Self::get_current_trace_id(),
            created_at: Utc::now(),
        }
    }

    pub fn reconstruct(
        id: OutboxEventId,
        event: DomainEvent,
        status: OutboxEventStatus,
        trace_id: Option<TraceId>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            event,
            status,
            trace_id,
            created_at,
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

    pub fn id(&self) -> OutboxEventId {
        self.id
    }

    pub fn domain_event(&self) -> &DomainEvent {
        &self.event
    }

    pub fn status(&self) -> OutboxEventStatus {
        self.status
    }

    pub fn trace_id(&self) -> Option<TraceId> {
        self.trace_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum OutboxEventStatus {
    Pending,
    Failed,
    Completed,
}

pub trait EntityWithEvents: Send {
    fn pull_events(&mut self) -> Vec<OutboxEvent>;
}
