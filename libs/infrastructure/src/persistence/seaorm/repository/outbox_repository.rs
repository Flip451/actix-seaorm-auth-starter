use std::fmt;

use async_trait::async_trait;
use domain::shared::outbox::{DomainEvent, OutboxEvent, OutboxRepository, OutboxRepositoryError};
use sea_orm::{ActiveValue::Set, EntityTrait};

use crate::persistence::seaorm::connect::Connectable;

use super::super::entities::outbox as outbox_entity;

pub struct SeaOrmOutboxRepository<C, T>
where
    C: Connectable<T>,
    T: sea_orm::ConnectionTrait,
{
    conn: C,
    _marker: std::marker::PhantomData<T>,
}

struct EventTypeFormatter<'a>(&'a DomainEvent);

impl fmt::Display for EventTypeFormatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let event_type = match self.0 {
            DomainEvent::UserEvent(user_event) => user_event.as_ref(),
        };
        write!(f, "{}", event_type)
    }
}

fn get_active_model_from_event(
    event: OutboxEvent,
) -> Result<outbox_entity::ActiveModel, OutboxRepositoryError> {
    let payload = serde_json::to_value(&event.event)
        .map_err(|e| OutboxRepositoryError::Persistence(e.into()))?;

    let event_type = EventTypeFormatter(&event.event).to_string();

    Ok(outbox_entity::ActiveModel {
        id: Set(event.id),
        event_type: Set(event_type),
        payload: Set(payload),
        status: Set("PENDING".to_string()),
        trace_id: Set(event.trace_id.map(|tid| tid.to_string())),
        created_at: Set(event.created_at.into()),
        processed_at: Set(None),
    })
}

impl<C: Connectable<T>, T: sea_orm::ConnectionTrait> SeaOrmOutboxRepository<C, T> {
    pub fn new(conn: C) -> Self {
        Self {
            conn,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<C, T> OutboxRepository for SeaOrmOutboxRepository<C, T>
where
    C: Connectable<T> + Send + Sync,
    T: sea_orm::ConnectionTrait + Send + Sync,
{
    async fn save(&self, event: OutboxEvent) -> Result<(), OutboxRepositoryError> {
        let active_model = get_active_model_from_event(event)?;

        outbox_entity::Entity::insert(active_model)
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
            .exec(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::Persistence(e.into()))?;

        Ok(())
    }
}
