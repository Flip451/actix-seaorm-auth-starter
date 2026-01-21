use async_trait::async_trait;
use domain::shared::outbox_event::OutboxEventId;
use opentelemetry::trace::{TraceContextExt, TraceId};
use tracing::{Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::error::RelayError;

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
