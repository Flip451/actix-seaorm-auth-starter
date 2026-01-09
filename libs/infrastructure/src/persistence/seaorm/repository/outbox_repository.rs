use async_trait::async_trait;
use domain::{
    shared::outbox::{DomainEvent, OutboxEvent, OutboxRepository, OutboxRepositoryError},
    user::UserEvent,
};
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

macro_rules! match_user_event {
    ($event:expr, $user_event:expr, $(UserEvent::$event_name:ident {$($item:ident),*}),*) => {
        match $user_event {
            $(
                UserEvent::$event_name {$($item,)* } => outbox_entity::ActiveModel {
                    id: Set($event.id),
                    event_type: Set(stringify!($event_name).to_string()),
                    payload: Set(serde_json::json!({
                        $( stringify!($item): $item, )*
                    })),
                    status: Set("PENDING".to_string()),
                    trace_id: Set($event.trace_id),
                    created_at: Set($event.created_at.into()),
                    processed_at: Set(None),
                },
            )*
        }
    };
}

impl<C: Connectable<T>, T: sea_orm::ConnectionTrait> SeaOrmOutboxRepository<C, T> {
    pub fn new(conn: C) -> Self {
        Self {
            conn,
            _marker: std::marker::PhantomData,
        }
    }

    fn get_active_model_from_event(&self, event: OutboxEvent) -> outbox_entity::ActiveModel {
        match event.event {
            DomainEvent::UserEvent(user_event) => match_user_event!(
                event,
                user_event,
                UserEvent::UserCreated {
                    user_id,
                    email,
                    registered_at
                },
                UserEvent::UserSuspended {
                    user_id,
                    reason,
                    suspended_at
                },
                UserEvent::UserUnlocked {
                    user_id,
                    unlocked_at
                },
                UserEvent::UserDeactivated {
                    user_id,
                    deactivated_at
                },
                UserEvent::UserReactivated {
                    user_id,
                    reactivated_at
                },
                UserEvent::UserPromotedToAdmin {
                    user_id,
                    promoted_at
                },
                UserEvent::UsernameChanged {
                    user_id,
                    new_username,
                    changed_at
                },
                UserEvent::UserEmailChanged {
                    user_id,
                    new_email,
                    changed_at
                },
                UserEvent::EmailVerified {
                    user_id,
                    verified_at
                }
            ),
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
        let active_model = self.get_active_model_from_event(event);

        outbox_entity::Entity::insert(active_model)
            .exec(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::Persistence(e.into()))?;

        Ok(())
    }

    async fn save_all(
        &self,
        events: Vec<OutboxEvent>,
    ) -> Result<(), OutboxRepositoryError> {
        let active_models: Vec<outbox_entity::ActiveModel> = events
            .into_iter()
            .map(|event| self.get_active_model_from_event(event))
            .collect();

        outbox_entity::Entity::insert_many(active_models)
            .exec(self.conn.connect())
            .await
            .map_err(|e| OutboxRepositoryError::Persistence(e.into()))?;

        Ok(())
    }
}
