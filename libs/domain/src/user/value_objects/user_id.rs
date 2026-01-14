use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub(crate) Uuid);

impl UserId {
    pub fn reconstruct(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
