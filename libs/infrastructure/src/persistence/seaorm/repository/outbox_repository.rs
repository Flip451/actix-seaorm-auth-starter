use async_trait::async_trait;
use domain::shared::{
    outbox_event::{
        OutboxEvent, OutboxRepository, OutboxRepositoryError,
        entity::{OutboxEventStatusKind, OutboxEventStatusRaw},
    },
    service::clock::Clock,
};
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
    let payload = serde_json::to_value(event.domain_event())?;

    let event_type = event.domain_event().to_string();

    Ok(outbox_entity::ActiveModel {
        id: Set(event.id().into()),
        event_type: Set(event_type),
        payload: Set(payload),
        status: Set(event.status().kind().to_string()),
        trace_id: Set(event.trace_id().map(|tid| tid.to_string())),
        created_at: Set(event.created_at().into()),
        processed_at: Set(event.processed_at().map(|dt| dt.into())),
        retry_count: Set(event.retry_count() as i32),
        next_attempt_at: Set(event.next_attempt_at().map(|dt| dt.into())),
        last_attempted_at: Set(event.last_attempted_at().map(|dt| dt.into())),
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
    ) -> Result<OutboxEvent, OutboxRepositoryError> {
        let outbox_entity::Model {
            id,
            event_type: _,
            payload,
            status,
            trace_id,
            created_at,
            processed_at,
            retry_count,
            next_attempt_at,
            last_attempted_at,
        } = model;

        let status_source = OutboxEventStatusRaw {
            kind: status,
            retry_count: retry_count as u32,
            next_attempt_at: next_attempt_at.map(|dt| dt.into()),
            last_attempted_at: last_attempted_at.map(|dt| dt.into()),
            processed_at: processed_at.map(|dt| dt.into()),
        };

        Ok(OutboxEvent::reconstruct(
            id.into(),
            payload,
            status_source,
            trace_id,
            created_at.into(),
        )?)
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
                        outbox_entity::Column::RetryCount,
                        outbox_entity::Column::NextAttemptAt,
                        outbox_entity::Column::LastAttemptedAt,
                    ])
                    .to_owned(),
            )
            .exec(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::DataStoreError(e.into()))?;

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
                        outbox_entity::Column::RetryCount,
                        outbox_entity::Column::NextAttemptAt,
                        outbox_entity::Column::LastAttemptedAt,
                    ])
                    .to_owned(),
            )
            .exec(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::DataStoreError(e.into()))?;

        Ok(())
    }

    async fn lock_pending_events(
        &self,
        limit: u64,
        clock: &dyn Clock,
    ) -> Result<Vec<OutboxEvent>, OutboxRepositoryError> {
        let sql = r#"
            SELECT * FROM outbox
            WHERE (status = $1)
                OR (status = $2 AND next_attempt_at <= $3)
            ORDER BY next_attempt_at ASC NULLS FIRST
            LIMIT $4
            FOR UPDATE SKIP LOCKED
        "#;

        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            vec![
                // $1:
                OutboxEventStatusKind::Pending.to_string().into(),
                // $2:
                OutboxEventStatusKind::Failed.to_string().into(),
                // $3: 現在時刻
                Value::from(clock.now()),
                // $4: Limit
                Value::BigUnsigned(Some(limit)),
            ],
        );

        let models = outbox_entity::Entity::find()
            .from_raw_sql(stmt)
            .all(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::DataStoreError(e.into()))?;

        let events = models
            .into_iter()
            .map(|model| self.map_to_outbox_event(model))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(events)
    }
}
