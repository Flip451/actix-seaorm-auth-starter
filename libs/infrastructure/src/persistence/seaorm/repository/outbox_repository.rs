use async_trait::async_trait;
use domain::shared::{
    domain_event::DomainEvent,
    outbox_event::{
        OutboxEvent, OutboxEventStatus, OutboxReconstructionError, OutboxRepository,
        OutboxRepositoryError,
    },
};
use opentelemetry::trace::TraceId;
use sea_orm::{ActiveValue::Set, DbBackend, EntityTrait, Statement, Value, sea_query::OnConflict};

use crate::persistence::seaorm::connect::Connectable;

use super::super::entities::outbox as outbox_entity;

pub struct SeaOrmPostgresOutboxRepository<C, T>
where
    C: Connectable<T>,
    T: sea_orm::ConnectionTrait,
{
    conn: C,
    _marker: std::marker::PhantomData<T>,
}

fn get_active_model_from_event(
    event: OutboxEvent,
) -> Result<outbox_entity::ActiveModel, OutboxRepositoryError> {
    let payload = serde_json::to_value(event.domain_event())
        .map_err(|e| OutboxRepositoryError::Persistence(e.into()))?;

    let event_type = event.domain_event().to_string();

    Ok(outbox_entity::ActiveModel {
        id: Set(event.id().into()),
        event_type: Set(event_type),
        payload: Set(payload),
        status: Set(event.status().to_string()),
        trace_id: Set(event.trace_id().map(|tid| tid.to_string())),
        created_at: Set(event.created_at().into()),
        processed_at: Set(event.processed_at().map(|dt| dt.into())),
    })
}

impl<C: Connectable<T>, T: sea_orm::ConnectionTrait> SeaOrmPostgresOutboxRepository<C, T> {
    pub fn new(conn: C) -> Self {
        Self {
            conn,
            _marker: std::marker::PhantomData,
        }
    }

    /// DBモデルをドメインレベルの OutboxEvent に変換する内部ヘルパー
    fn map_to_outbox_event(
        &self,
        model: outbox_entity::Model,
    ) -> Result<OutboxEvent, OutboxReconstructionError> {
        let event: DomainEvent = serde_json::from_value(model.payload)
            .map_err(|e| OutboxReconstructionError::EventReconstructionError(e.into()))?;

        let trace_id = model
            .trace_id
            .map(|tid| {
                TraceId::from_hex(&tid).map_err(OutboxReconstructionError::ParseTraceIdError)
            })
            .transpose()?;

        Ok(OutboxEvent::reconstruct(
            model.id.into(),
            event,
            model.status.parse()?,
            trace_id,
            model.created_at.into(),
            model.processed_at.map(|dt| dt.into()),
        ))
    }
}

#[async_trait]
impl<C, T> OutboxRepository for SeaOrmPostgresOutboxRepository<C, T>
where
    C: Connectable<T> + Send + Sync,
    T: sea_orm::ConnectionTrait + Send + Sync,
{
    async fn save(&self, event: OutboxEvent) -> Result<(), OutboxRepositoryError> {
        let active_model = get_active_model_from_event(event)?;

        outbox_entity::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(outbox_entity::Column::Id)
                    .update_columns([
                        outbox_entity::Column::EventType,
                        outbox_entity::Column::Payload,
                        outbox_entity::Column::Status,
                        outbox_entity::Column::TraceId,
                        outbox_entity::Column::ProcessedAt,
                    ])
                    .to_owned(),
            )
            .exec(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::Persistence(e.into()))?;

        Ok(())
    }

    async fn save_all(&self, events: Vec<OutboxEvent>) -> Result<(), OutboxRepositoryError> {
        let active_models: Vec<outbox_entity::ActiveModel> = events
            .into_iter()
            .map(get_active_model_from_event)
            .collect::<Result<Vec<_>, _>>()?;

        outbox_entity::Entity::insert_many(active_models)
            .on_conflict(
                OnConflict::column(outbox_entity::Column::Id)
                    .update_columns([
                        outbox_entity::Column::EventType,
                        outbox_entity::Column::Payload,
                        outbox_entity::Column::Status,
                        outbox_entity::Column::TraceId,
                        outbox_entity::Column::ProcessedAt,
                    ])
                    .to_owned(),
            )
            .exec(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::Persistence(e.into()))?;

        Ok(())
    }

    async fn lock_pending_events(
        &self,
        limit: u64,
    ) -> Result<Vec<OutboxEvent>, OutboxReconstructionError> {
        let sql = r#"
            SELECT * FROM outbox
            WHERE status = $1
            ORDER BY created_at ASC
            LIMIT $2
            FOR UPDATE SKIP LOCKED
        "#;

        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            vec![
                OutboxEventStatus::Pending.to_string().into(),
                Value::BigUnsigned(Some(limit)),
            ],
        );

        let models = outbox_entity::Entity::find()
            .from_raw_sql(stmt)
            .all(self.conn.connect())
            .await
            .map_err(|e| OutboxReconstructionError::DataStoreError(e.into()))?;

        let events = models
            .into_iter()
            .map(|model| self.map_to_outbox_event(model))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(events)
    }
}
