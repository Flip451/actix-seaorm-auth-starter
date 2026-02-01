use chrono::{DateTime, Utc};
use derive_entity::Entity;
use opentelemetry::trace::{TraceContextExt, TraceId};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use crate::shared::{
    domain_event::DomainEvent,
    outbox_event::error::{OutboxEventDomainError, OutboxStatusTransitionError},
    service::clock::Clock,
};

use super::OutboxEventId;

#[derive(Entity)]
pub struct OutboxEvent {
    #[entity_id]
    id: OutboxEventId,
    event: DomainEvent,
    status: OutboxEventStatus,
    trace_id: Option<TraceId>,
    created_at: DateTime<Utc>,
    processed_at: Option<DateTime<Utc>>, // イベント処理が成功し完了した日時。将来的に issue #52 で processed_at と completed_at に分割予定
}

impl OutboxEvent {
    pub fn new(event: DomainEvent, created_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4().into(),
            event,
            status: OutboxEventStatus::Pending,
            trace_id: Self::get_current_trace_id(),
            created_at,
            processed_at: None,
        }
    }

    pub fn reconstruct(
        id: OutboxEventId,
        event: DomainEvent,
        status: OutboxEventStatus,
        trace_id: Option<TraceId>,
        created_at: DateTime<Utc>,
        processed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            event,
            status,
            trace_id,
            created_at,
            processed_at,
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

    pub fn processed_at(&self) -> Option<DateTime<Utc>> {
        self.processed_at
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum OutboxEventStatus {
    Pending,
    Failed, // TODO: #47 でリトライカウントを追加する
    Completed,
    PermanentlyFailed,
}

// TODO: #52 で completed_at と processed_at を分ける際に修正する
impl OutboxEvent {
    pub fn complete(&mut self, clock: &dyn Clock) -> Result<(), OutboxEventDomainError> {
        match &self.status {
            OutboxEventStatus::Pending | OutboxEventStatus::Failed => {
                self.status = OutboxEventStatus::Completed;
                self.processed_at = Some(clock.now());
            }
            OutboxEventStatus::Completed => Err(OutboxStatusTransitionError::AlreadyCompleted {
                to: OutboxEventStatus::Completed,
            })?,
            OutboxEventStatus::PermanentlyFailed => {
                Err(OutboxStatusTransitionError::AlreadyPermanentlyFailed {
                    to: OutboxEventStatus::Completed,
                })?
            }
        }

        Ok(())
    }

    pub fn fail(&mut self, _clock: &dyn Clock) -> Result<(), OutboxEventDomainError> {
        match &self.status {
            OutboxEventStatus::Pending => {
                self.status = OutboxEventStatus::Failed;
            }
            OutboxEventStatus::Completed => Err(OutboxStatusTransitionError::AlreadyCompleted {
                to: OutboxEventStatus::Failed,
            })?,
            OutboxEventStatus::Failed => {
                // TODO: #47 でリトライカウントを+1するロジックを追加する
                self.status = OutboxEventStatus::PermanentlyFailed;
            }
            OutboxEventStatus::PermanentlyFailed => {
                Err(OutboxStatusTransitionError::AlreadyPermanentlyFailed {
                    to: OutboxEventStatus::Failed,
                })?
            }
        }

        Ok(())
    }
}
