use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::user::{UnverifiedEmail, UserId};

#[derive(Deserialize, Serialize, Debug, Clone, strum::Display)]
#[strum(prefix = "UserEvent::")]
pub enum UserEvent {
    Created(UserCreatedEvent),
    Suspended(UserSuspendedEvent),
    Unlocked(UserUnlockedEvent),
    Deactivated(UserDeactivatedEvent),
    Reactivated(UserReactivatedEvent),
    PromotedToAdmin(UserPromotedToAdminEvent),
    UsernameChanged(UsernameChangedEvent),
    EmailChanged(UserEmailChangedEvent),
    EmailVerified(UserEmailVerifiedEvent),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserCreatedEvent {
    pub user_id: UserId,
    pub email: UnverifiedEmail,
    pub registered_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSuspendedEvent {
    pub user_id: UserId,
    pub reason: String,
    pub suspended_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserUnlockedEvent {
    pub user_id: UserId,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserDeactivatedEvent {
    pub user_id: UserId,
    pub deactivated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserReactivatedEvent {
    pub user_id: UserId,
    pub reactivated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserPromotedToAdminEvent {
    pub user_id: UserId,
    pub promoted_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UsernameChangedEvent {
    pub user_id: UserId,
    pub new_username: String,
    pub changed_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserEmailChangedEvent {
    pub user_id: UserId,
    pub new_email: UnverifiedEmail,
    pub changed_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserEmailVerifiedEvent {
    pub user_id: UserId,
    pub verified_at: DateTime<Utc>,
}
