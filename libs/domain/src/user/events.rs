use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::user::UnverifiedEmail;

#[derive(Serialize, Debug, Clone)]
pub enum UserEvent {
    Created {
        user_id: Uuid,
        email: UnverifiedEmail,
        registered_at: DateTime<Utc>,
    },
    Suspended {
        user_id: Uuid,
        reason: String,
        suspended_at: DateTime<Utc>,
    },
    Unlocked {
        user_id: Uuid,
        unlocked_at: DateTime<Utc>,
    },
    Deactivated {
        user_id: Uuid,
        deactivated_at: DateTime<Utc>,
    },
    Reactivated {
        user_id: Uuid,
        reactivated_at: DateTime<Utc>,
    },
    PromotedToAdmin {
        user_id: Uuid,
        promoted_at: DateTime<Utc>,
    },
    UsernameChanged {
        user_id: Uuid,
        new_username: String,
        changed_at: DateTime<Utc>,
    },
    EmailChanged {
        user_id: Uuid,
        new_email: UnverifiedEmail,
        changed_at: DateTime<Utc>,
    },
    EmailVerified {
        user_id: Uuid,
        verified_at: DateTime<Utc>,
    },
}
