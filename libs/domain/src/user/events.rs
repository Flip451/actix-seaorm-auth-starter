use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::user::{Email, UnverifiedEmail, VerifiedEmail};

#[derive(Deserialize, Serialize, Debug, Clone, strum::Display)]
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

impl UserEvent {
    /// イベントの発生日時を取得する
    pub(crate) fn created_at(&self) -> DateTime<Utc> {
        match self {
            UserEvent::Created(e) => e.registered_at,
            UserEvent::Suspended(e) => e.suspended_at,
            UserEvent::Unlocked(e) => e.unlocked_at,
            UserEvent::Deactivated(e) => e.deactivated_at,
            UserEvent::Reactivated(e) => e.reactivated_at,
            UserEvent::PromotedToAdmin(e) => e.promoted_at,
            UserEvent::UsernameChanged(e) => e.changed_at,
            UserEvent::EmailChanged(e) => e.changed_at,
            UserEvent::EmailVerified(e) => e.verified_at,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserCreatedEvent {
    pub email: UnverifiedEmail,
    pub username: String,
    pub registered_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSuspendedEvent {
    pub username: String,
    pub email: UnverifiedEmail,
    pub reason: String,
    pub suspended_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserUnlockedEvent {
    pub username: String,
    pub email: UnverifiedEmail,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserDeactivatedEvent {
    pub username: String,
    pub email: UnverifiedEmail,
    pub deactivated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserReactivatedEvent {
    pub username: String,
    pub email: UnverifiedEmail,
    pub reactivated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserPromotedToAdminEvent {
    pub username: String,
    pub email: VerifiedEmail,
    pub promoted_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UsernameChangedEvent {
    pub old_username: String,
    pub new_username: String,
    pub email: Email,
    pub changed_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserEmailChangedEvent {
    pub new_email: UnverifiedEmail,
    pub username: String,
    pub changed_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserEmailVerifiedEvent {
    pub email: VerifiedEmail,
    pub username: String,
    pub verified_at: DateTime<Utc>,
}
