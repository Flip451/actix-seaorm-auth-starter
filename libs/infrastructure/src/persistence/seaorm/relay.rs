use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use domain::shared::{
    domain_event::DomainEvent,
    outbox_event::{OutboxEvent, OutboxEventStatus, OutboxReconstructionError},
};
use opentelemetry::trace::TraceId;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use usecase::shared::relay::{EventMapper, OutboxRelay, RelayError};

use super::entities::outbox as outbox_entity;

pub struct SeaOrmOutboxRelay {
    db: DatabaseConnection,
    mapper: Arc<EventMapper>,
}

impl SeaOrmOutboxRelay {
    pub fn new(db: DatabaseConnection, mapper: Arc<EventMapper>) -> Self {
        Self { db, mapper }
    }

    /// DBモデルをドメインレベルの OutboxEvent に変換する内部ヘルパー
    fn map_to_outbox_event(&self, model: outbox_entity::Model) -> Result<OutboxEvent, RelayError> {
        let event: DomainEvent = serde_json::from_value(model.payload)
            .map_err(|e| OutboxReconstructionError::EventReconstructionError(e.into()))?;

        let trace_id = model
            .trace_id
            .map(|tid| TraceId::from_hex(&tid).map_err(|e| RelayError::ParseTraceIdError(e.into())))
            .transpose()?;

        Ok(OutboxEvent::reconstruct(
            model.id.into(),
            event,
            model
                .status
                .parse()
                .map_err(OutboxReconstructionError::InvalidOutboxEventStatus)?,
            trace_id,
            model.created_at.into(),
        ))
    }
}

#[async_trait]
impl OutboxRelay for SeaOrmOutboxRelay {
    #[tracing::instrument(skip(self))]
    async fn process_batch(&self) -> Result<usize, RelayError> {
        // 1. PENDING状態のイベントを一定数取得
        // TODO: 複数インスタンスでの競合を防ぐため、本来は SELECT FOR UPDATE SKIP LOCKED が望ましい
        let pending_events = outbox_entity::Entity::find()
            .filter(outbox_entity::Column::Status.eq(OutboxEventStatus::Pending.to_string()))
            .order_by_asc(outbox_entity::Column::CreatedAt)
            .limit(10) // 1回あたりのバッチサイズ
            .all(&self.db)
            .await
            .map_err(|e| RelayError::ProcessingError(e.into()))?;

        let count = pending_events.len();
        if count == 0 {
            return Ok(0);
        }

        for model in pending_events {
            let event_id = model.id;
            let outbox_event = self.map_to_outbox_event(model)?;

            // 2. イベントに対応するハンドラを取得
            let handlers = self.mapper.map_event_to_handler(outbox_event);

            // 3. 各ハンドラを実行
            let mut success = true;
            for handler in handlers {
                if let Err(e) = handler.handle_event().await {
                    tracing::error!(error = ?e, %event_id, "イベントハンドラの実行に失敗しました");
                    success = false;
                    break;
                }
            }

            // 4. ステータスの更新
            let mut active_model: outbox_entity::ActiveModel = outbox_entity::ActiveModel {
                id: Set(event_id),
                ..Default::default()
            };

            if success {
                active_model.status = Set(OutboxEventStatus::Completed.to_string());
                active_model.processed_at = Set(Some(Utc::now().into()));
            } else {
                active_model.status = Set(OutboxEventStatus::Failed.to_string());
                // 必要に応じてリトライカウントを増やすロジックをここに追加
            }

            active_model
                .update(&self.db)
                .await
                .map_err(|e| RelayError::ProcessingError(e.into()))?;
        }

        Ok(count)
    }
}
