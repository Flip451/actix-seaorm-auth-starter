use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

use crate::user::UserReconstructionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Default)]
#[strum(serialize_all = "snake_case")]
pub enum UserRole {
    Admin,
    #[default]
    User,
}

#[derive(Debug, PartialEq, Eq, Display, EnumString, IntoStaticStr)]
enum UserRoleKind {
    Admin,
    User,
}

impl FromStr for UserRole {
    type Err = UserReconstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = s
            .parse::<UserRoleKind>()
            .map_err(|_| UserReconstructionError::InvalidRole(s.to_string()))?;

        match kind {
            UserRoleKind::Admin => Ok(UserRole::Admin),
            UserRoleKind::User => Ok(UserRole::User),
        }
    }
}

impl TryFrom<&str> for UserRole {
    type Error = UserReconstructionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UserRole::from_str(value)
    }
}
