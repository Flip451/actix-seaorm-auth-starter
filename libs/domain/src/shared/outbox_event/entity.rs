use chrono::{DateTime, Utc};
use derive_entity::Entity;
use opentelemetry::trace::{TraceContextExt, TraceId};
use serde_json::Value;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use crate::shared::{
    domain_event::DomainEvent,
    outbox_event::{
        NextAttemptCalculator,
        error::{
            OutboxEventDomainError, OutboxEventReconstructionError, OutboxStatusTransitionError,
        },
        service::NextAttemptStatus,
    },
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
}

impl OutboxEvent {
    pub fn new(event: DomainEvent, created_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4().into(),
            event,
            status: OutboxEventStatus::Pending,
            trace_id: Self::get_current_trace_id(),
            created_at,
        }
    }

    pub fn reconstruct(
        id: OutboxEventId,
        event: Value,
        status_source: OutboxEventStatusRaw,
        trace_id: Option<String>,
        created_at: DateTime<Utc>,
    ) -> Result<Self, OutboxEventReconstructionError> {
        let event: DomainEvent = serde_json::from_value(event).map_err(|e| {
            OutboxEventReconstructionError::DomainEventReconstructionError(e.into())
        })?;

        let status = status_source.try_into()?;

        let trace_id = trace_id
            .map(|tid| {
                TraceId::from_hex(&tid).map_err(OutboxEventReconstructionError::ParseTraceIdError)
            })
            .transpose()?;

        Ok(Self {
            id,
            event,
            status,
            trace_id,
            created_at,
        })
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

    pub fn next_attempt_at(&self) -> Option<DateTime<Utc>> {
        match &self.status {
            OutboxEventStatus::Pending => None,
            OutboxEventStatus::Failed {
                next_attempt_at, ..
            } => Some(*next_attempt_at),
            OutboxEventStatus::Completed { .. } => None,
            OutboxEventStatus::PermanentlyFailed { .. } => None,
        }
    }

    pub fn last_attempted_at(&self) -> Option<DateTime<Utc>> {
        match &self.status {
            OutboxEventStatus::Pending => None,
            OutboxEventStatus::Failed {
                last_attempted_at, ..
            } => Some(*last_attempted_at),
            OutboxEventStatus::Completed {
                last_attempted_at, ..
            } => Some(*last_attempted_at),
            OutboxEventStatus::PermanentlyFailed {
                last_attempted_at, ..
            } => Some(*last_attempted_at),
        }
    }

    pub fn processed_at(&self) -> Option<DateTime<Utc>> {
        match &self.status {
            OutboxEventStatus::Pending => None,
            OutboxEventStatus::Failed { failed_at, .. } => Some(*failed_at),
            OutboxEventStatus::Completed { completed_at, .. } => Some(*completed_at),
            OutboxEventStatus::PermanentlyFailed { failed_at, .. } => Some(*failed_at),
        }
    }

    pub fn retry_count(&self) -> u32 {
        match &self.status {
            OutboxEventStatus::Pending => 0,
            OutboxEventStatus::Failed { retry_count, .. } => *retry_count,
            OutboxEventStatus::Completed { retry_count, .. } => *retry_count,
            OutboxEventStatus::PermanentlyFailed { retry_count, .. } => *retry_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutboxEventStatus {
    Pending,
    Failed {
        retry_count: u32,
        next_attempt_at: DateTime<Utc>,
        last_attempted_at: DateTime<Utc>,
        failed_at: DateTime<Utc>,
    },
    Completed {
        retry_count: u32,
        last_attempted_at: DateTime<Utc>,
        completed_at: DateTime<Utc>,
    },
    PermanentlyFailed {
        retry_count: u32,
        last_attempted_at: DateTime<Utc>,
        failed_at: DateTime<Utc>,
    },
}

#[derive(Debug, PartialEq, Eq, strum::Display, strum::EnumString, strum::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum OutboxEventStatusKind {
    Pending,
    Failed,
    Completed,
    PermanentlyFailed,
}

impl OutboxEventStatus {
    pub fn kind(&self) -> &'static str {
        self.kind_raw().into()
    }

    fn kind_raw(&self) -> OutboxEventStatusKind {
        match self {
            OutboxEventStatus::Pending => OutboxEventStatusKind::Pending,
            OutboxEventStatus::Failed { .. } => OutboxEventStatusKind::Failed,
            OutboxEventStatus::Completed { .. } => OutboxEventStatusKind::Completed,
            OutboxEventStatus::PermanentlyFailed { .. } => OutboxEventStatusKind::PermanentlyFailed,
        }
    }
}

impl OutboxEvent {
    pub fn complete(
        &mut self,
        process_start_at: DateTime<Utc>,
        clock: &dyn Clock,
    ) -> Result<(), OutboxEventDomainError> {
        let (now, retry_count) = match &self.status {
            OutboxEventStatus::Pending => {
                let now = clock.now();
                let retry_count = 0;
                (now, retry_count)
            }
            OutboxEventStatus::Failed { retry_count, .. } => {
                let now = clock.now();
                let retry_count = *retry_count;
                (now, retry_count)
            }
            OutboxEventStatus::Completed { .. } => {
                Err(OutboxStatusTransitionError::AlreadyCompleted {
                    to: OutboxEventStatusKind::Completed,
                })?
            }
            OutboxEventStatus::PermanentlyFailed { .. } => {
                Err(OutboxStatusTransitionError::AlreadyPermanentlyFailed {
                    to: OutboxEventStatusKind::Completed,
                })?
            }
        };

        self.status = OutboxEventStatus::Completed {
            retry_count,
            last_attempted_at: process_start_at,
            completed_at: now,
        };

        Ok(())
    }

    pub fn handle_failure(
        &mut self,
        process_start_at: DateTime<Utc>,
        calculator: &dyn NextAttemptCalculator,
        clock: &dyn Clock,
        error: &impl std::error::Error,
    ) -> Result<(), OutboxEventDomainError> {
        let (now, current_retry_count) = match &self.status {
            OutboxEventStatus::Pending => {
                let now = clock.now();
                let current_retry_count = 0;
                (now, current_retry_count)
            }
            OutboxEventStatus::Completed { .. } => {
                Err(OutboxStatusTransitionError::AlreadyCompleted {
                    to: OutboxEventStatusKind::Failed,
                })?
            }
            OutboxEventStatus::Failed {
                retry_count: current_retry_count,
                ..
            } => {
                let now = clock.now();
                (now, *current_retry_count)
            }
            OutboxEventStatus::PermanentlyFailed { .. } => {
                Err(OutboxStatusTransitionError::AlreadyPermanentlyFailed {
                    to: OutboxEventStatusKind::Failed,
                })?
            }
        };

        match calculator.next_attempt_status(current_retry_count, now) {
            NextAttemptStatus::RetryAt(next_attempt_at) => {
                let retry_count = current_retry_count + 1;
                self.status = OutboxEventStatus::Failed {
                    retry_count,
                    next_attempt_at,
                    last_attempted_at: process_start_at,
                    failed_at: now,
                };
                tracing::warn!(
                    ?error,
                    retry_count = retry_count,
                    event_id = %self.id(),
                    "OutboxEvent failed processing, will retry at {}",
                    next_attempt_at
                );
            }
            NextAttemptStatus::PermanentlyFailed => {
                let retry_count = current_retry_count + 1;
                self.status = OutboxEventStatus::PermanentlyFailed {
                    retry_count,
                    last_attempted_at: process_start_at,
                    failed_at: now,
                };
                tracing::error!(
                    ?error,
                    retry_count = retry_count,
                    event_id = %self.id(),
                    "OutboxEvent permanently failed processing",
                );
            }
        }

        Ok(())
    }
}

pub struct OutboxEventStatusRaw {
    pub kind: String,
    pub retry_count: u32,
    pub next_attempt_at: Option<DateTime<Utc>>,
    pub last_attempted_at: Option<DateTime<Utc>>,
    pub processed_at: Option<DateTime<Utc>>,
}

impl TryFrom<OutboxEventStatusRaw> for OutboxEventStatus {
    type Error = OutboxEventReconstructionError;

    fn try_from(raw: OutboxEventStatusRaw) -> Result<Self, Self::Error> {
        let OutboxEventStatusRaw {
            kind,
            retry_count,
            next_attempt_at,
            last_attempted_at,
            processed_at,
        } = raw;

        let kind = kind.parse::<OutboxEventStatusKind>().map_err(|_e| {
            OutboxEventReconstructionError::InvalidStatus {
                invalid_status: kind,
            }
        })?;

        match kind {
            OutboxEventStatusKind::Pending => Ok(OutboxEventStatus::Pending),
            OutboxEventStatusKind::Failed => Ok(OutboxEventStatus::Failed {
                retry_count,
                next_attempt_at: next_attempt_at
                    .ok_or(OutboxEventReconstructionError::FailedButNoNextAttemptAt)?,
                last_attempted_at: last_attempted_at
                    .ok_or(OutboxEventReconstructionError::FailedButNoLastAttemptedAt)?,
                failed_at: processed_at
                    .ok_or(OutboxEventReconstructionError::FailedButNoProcessedAt)?,
            }),
            OutboxEventStatusKind::Completed => Ok(OutboxEventStatus::Completed {
                retry_count,
                last_attempted_at: last_attempted_at
                    .ok_or(OutboxEventReconstructionError::CompletedButNoLastAttemptedAt)?,
                completed_at: processed_at
                    .ok_or(OutboxEventReconstructionError::CompletedButNoProcessedAt)?,
            }),
            OutboxEventStatusKind::PermanentlyFailed => Ok(OutboxEventStatus::PermanentlyFailed {
                retry_count,
                last_attempted_at: last_attempted_at
                    .ok_or(OutboxEventReconstructionError::PermanentlyFailedButNoLastAttemptedAt)?,
                failed_at: processed_at
                    .ok_or(OutboxEventReconstructionError::PermanentlyFailedButNoProcessedAt)?,
            }),
        }
    }
}
