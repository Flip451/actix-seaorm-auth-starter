use serde::{Deserialize, Serialize};

use crate::user::UserEvent;

#[derive(Debug, Deserialize, Serialize, derive_more::Display)]
pub enum DomainEvent {
    #[display("UserEvent::{_0}")]
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

    use chrono::{DateTime, TimeZone as _, Utc};

    use crate::user::{self, EmailTrait, UnverifiedEmail, UserCreatedEvent};

    use super::*;

    fn fixed_time() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()
    }

    #[rstest]
    #[case(
        UserEvent::Created(UserCreatedEvent {
            email: UnverifiedEmail::new("user@example.com").unwrap(),
            username: "user123".to_string(),
            registered_at: fixed_time(),
        }),
        "UserEvent::Created"
    )]
    #[case(
        UserEvent::Suspended(user::UserSuspendedEvent {
            email: UnverifiedEmail::new("user@example.com").unwrap(),
            username: "user123".to_string(),
            reason: "Violation of terms".to_string(),
            suspended_at: fixed_time(),
        }),
        "UserEvent::Suspended"
    )]
    #[case(
        UserEvent::Unlocked(user::UserUnlockedEvent {
            username: "user123".to_string(),
            email: UnverifiedEmail::new("user@example.com").unwrap(),
            unlocked_at: fixed_time(),
        }),
        "UserEvent::Unlocked"
    )]
    #[case(
        UserEvent::Deactivated(user::UserDeactivatedEvent {
            username: "user123".to_string(),
            email: UnverifiedEmail::new("user@example.com").unwrap(),
            deactivated_at: fixed_time(),
        }),
        "UserEvent::Deactivated"
    )]
    fn test_domain_event_display(#[case] user_event: UserEvent, #[case] display_str: &str) {
        let domain_event = DomainEvent::from(user_event);
        assert_eq!(domain_event.to_string(), display_str);
    }
}
