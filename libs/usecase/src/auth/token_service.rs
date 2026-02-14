use chrono::{DateTime, Utc};
use domain::user::{UserId, UserRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{shared::identity::UserRoleData, usecase_error::UseCaseError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: UserId,
    role: UserRole,
    exp: i64,
    iat: i64,
}

impl Claims {
    pub(crate) fn new(sub: UserId, role: UserRole, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        Self {
            sub,
            role,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.sub.into()
    }

    pub fn user_role(&self) -> UserRoleData {
        self.role.into()
    }
}

pub trait TokenService: Send + Sync {
    fn issue_token(&self, user_id: UserId, role: UserRole) -> Result<String, UseCaseError>;
    fn verify_token(&self, token: &str) -> Result<Claims, UseCaseError>;
}
