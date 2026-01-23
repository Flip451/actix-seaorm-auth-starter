use serde::{Deserialize, Serialize};

use crate::user::UserEvent;

#[derive(Deserialize, Serialize, derive_more::Display)]
pub enum DomainEvent {
    UserEvent(UserEvent),
    // 将来的に他のイベントタイプも追加可能
}

impl From<UserEvent> for DomainEvent {
    fn from(event: UserEvent) -> Self {
        DomainEvent::UserEvent(event)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use chrono::Utc;
    use uuid::Uuid;

    use crate::user::{self, EmailTrait, UnverifiedEmail, UserCreatedEvent};

    use super::*;

    #[rstest]
    #[case(
        UserEvent::Created(UserCreatedEvent {
            user_id: Uuid::new_v4().into(),
            email: UnverifiedEmail::new("user@example.com").unwrap(),
            registered_at: Utc::now(),
        }),
        "UserEvent::Created"
    )]
    #[case(
        UserEvent::Suspended(user::UserSuspendedEvent {
            user_id: Uuid::new_v4().into(),
            reason: "Violation of terms".to_string(),
            suspended_at: Utc::now(),
        }),
        "UserEvent::Suspended"
    )]
    #[case(
        UserEvent::Unlocked(user::UserUnlockedEvent {
            user_id: Uuid::new_v4().into(),
            unlocked_at: Utc::now(),
        }),
        "UserEvent::Unlocked"
    )]
    #[case(
        UserEvent::Deactivated(user::UserDeactivatedEvent {
            user_id: Uuid::new_v4().into(),
            deactivated_at: Utc::now(),
        }),
        "UserEvent::Deactivated"
    )]
    fn test_domain_event_display(#[case] user_event: UserEvent, #[case] display_str: &str) {
        let domain_event = DomainEvent::from(user_event);
        assert_eq!(domain_event.to_string(), display_str);
    }
}
