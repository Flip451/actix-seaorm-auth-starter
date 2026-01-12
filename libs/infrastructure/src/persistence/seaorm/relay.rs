use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use domain::{
    shared::outbox::{DomainEvent, OutboxEvent},
    user::{UserCreatedEvent, UserEvent, UserSuspendedEvent},
};
use opentelemetry::trace::TraceId;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use usecase::shared::relay::{EventMapper, OutboxRelay, RelayError};

use super::entities::outbox as outbox_entity;

pub struct SeaOrmOutboxRelay {
    db: Arc<DatabaseConnection>,
    mapper: Arc<EventMapper>,
}

impl SeaOrmOutboxRelay {
    pub fn new(db: Arc<DatabaseConnection>, mapper: Arc<EventMapper>) -> Self {
        Self { db, mapper }
    }

    /// DBモデルをドメインレベルの OutboxEvent に変換する内部ヘルパー
    // TODO: リファクタリングの余地あり
    fn map_to_outbox_event(&self, model: outbox_entity::Model) -> Result<OutboxEvent, RelayError> {
        let event: DomainEvent = match model.event_type.as_str() {
            "UserEvent::Created" => {
                let user_created_event: UserCreatedEvent = serde_json::from_value(model.payload)
                    .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::Created(user_created_event))
            }
            "UserEvent::Suspended" => {
                let user_suspended_event: UserSuspendedEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::Suspended(user_suspended_event))
            }
            "UserEvent::Unlocked" => {
                let user_unlocked_event: domain::user::UserUnlockedEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::Unlocked(user_unlocked_event))
            }
            "UserEvent::Deactivated" => {
                let user_deactivated_event: domain::user::UserDeactivatedEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::Deactivated(user_deactivated_event))
            }
            "UserEvent::Reactivated" => {
                let user_reactivated_event: domain::user::UserReactivatedEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::Reactivated(user_reactivated_event))
            }
            "UserEvent::PromotedToAdmin" => {
                let user_promoted_to_admin_event: domain::user::UserPromotedToAdminEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::PromotedToAdmin(user_promoted_to_admin_event))
            }
            "UserEvent::UsernameChanged" => {
                let username_changed_event: domain::user::UsernameChangedEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::UsernameChanged(username_changed_event))
            }
            "UserEvent::EmailChanged" => {
                let user_email_changed_event: domain::user::UserEmailChangedEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::EmailChanged(user_email_changed_event))
            }
            "UserEvent::EmailVerified" => {
                let user_email_verified_event: domain::user::UserEmailVerifiedEvent =
                    serde_json::from_value(model.payload)
                        .map_err(|e| RelayError::ReconstructionError(e.into()))?;
                DomainEvent::UserEvent(UserEvent::EmailVerified(user_email_verified_event))
            }
            _other => Err(RelayError::UnknownEventType(model.event_type))?,
        };

        let trace_id = model
            .trace_id
            .map(|tid| TraceId::from_hex(&tid).map_err(|e| RelayError::ParseTraceIdError(e.into())))
            .transpose()?;

        Ok(OutboxEvent {
            id: model.id,
            event,
            trace_id,
            created_at: model.created_at.into(),
        })
    }
}

#[async_trait]
impl OutboxRelay for SeaOrmOutboxRelay {
    #[tracing::instrument(skip(self))]
    async fn process_batch(&self) -> Result<usize, RelayError> {
        // 1. PENDING状態のイベントを一定数取得
        // TODO: 複数インスタンスでの競合を防ぐため、本来は SELECT FOR UPDATE SKIP LOCKED が望ましい
        let pending_events = outbox_entity::Entity::find()
            .filter(outbox_entity::Column::Status.eq("PENDING"))
            .order_by_asc(outbox_entity::Column::CreatedAt)
            .limit(10) // 1回あたりのバッチサイズ
            .all(self.db.as_ref())
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
                active_model.status = Set("COMPLETED".to_string());
                active_model.processed_at = Set(Some(Utc::now().into()));
            } else {
                active_model.status = Set("FAILED".to_string());
                // 必要に応じてリトライカウントを増やすロジックをここに追加
            }

            active_model
                .update(self.db.as_ref())
                .await
                .map_err(|e| RelayError::ProcessingError(e.into()))?;
        }

        Ok(count)
    }
}
