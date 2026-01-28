use async_trait::async_trait;
use domain::shared::outbox_event::OutboxEventId;
use opentelemetry::trace::{TraceContextExt, TraceId};
use tracing::{Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::error::RelayError;

// 共通のメタデータ保持用構造体
pub struct HandlerContext {
    pub outbox_event_id: OutboxEventId,
    pub trace_id: Option<TraceId>,
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    fn context(&self) -> &HandlerContext;

    fn outbox_event_id(&self) -> OutboxEventId {
        self.context().outbox_event_id
    }

    fn trace_id(&self) -> Option<TraceId> {
        self.context().trace_id
    }

    async fn handle_event_raw(&self) -> Result<(), RelayError>;

    /// トレーシング用のSpanを構築する（デフォルト実装）
    ///
    /// 実装構造体の型名（例: "SendEmailWhenUserCreatedHandler"）を自動的に取得して
    /// "event_handler" というSpan名の `handler` フィールドとして記録します。
    fn construct_span(&self) -> Span {
        // フルパス（libs::usecase::...::HandlerName）から最後の型名だけを取得
        let full_name = std::any::type_name::<Self>();
        let short_name = full_name.split("::").last().unwrap_or(full_name);

        tracing::span!(
            tracing::Level::INFO,
            "handle_event",       // Span名
            handler = short_name  // 属性: handler="SendEmailWhenUserCreatedHandler"
        )
    }

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
