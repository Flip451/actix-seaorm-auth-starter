use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::user::UnverifiedEmail;

#[derive(Serialize, Debug, Clone)]
pub enum UserEvent {
    UserCreated {
        user_id: Uuid,
        email: UnverifiedEmail,
        registered_at: DateTime<Utc>,
    },
    UserSuspended {
        user_id: Uuid,
        reason: String,
        suspended_at: DateTime<Utc>,
    },
    UserUnlocked {
        user_id: Uuid,
        unlocked_at: DateTime<Utc>,
    },
    UserDeactivated {
        user_id: Uuid,
        deactivated_at: DateTime<Utc>,
    },
    UserReactivated {
        user_id: Uuid,
        reactivated_at: DateTime<Utc>,
    },
    UserPromotedToAdmin {
        user_id: Uuid,
        promoted_at: DateTime<Utc>,
    },
    UsernameChanged {
        user_id: Uuid,
        new_username: String,
        changed_at: DateTime<Utc>,
    },
    UserEmailChanged {
        user_id: Uuid,
        new_email: UnverifiedEmail,
        changed_at: DateTime<Utc>,
    },
    EmailVerified {
        user_id: Uuid,
        verified_at: DateTime<Utc>,
    },
}
