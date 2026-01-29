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
#[strum(serialize_all = "snake_case")]
enum UserRoleKind {
    Admin,
    User,
}

impl FromStr for UserRole {
    type Err = UserReconstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = s
            .parse::<UserRoleKind>()
            .map_err(|_| UserReconstructionError::InvalidRole {
                invalid_role: s.to_string(),
            })?;

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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("admin", UserRole::Admin)]
    #[case("user", UserRole::User)]
    fn test_user_role_from_str_success(#[case] input: &str, #[case] expected: UserRole) {
        let result = UserRole::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("invalid_role", UserReconstructionError::InvalidRole { invalid_role:"invalid_role".to_string() })]
    #[case("", UserReconstructionError::InvalidRole { invalid_role:"".to_string() })]
    fn test_user_role_from_str_failure(
        #[case] input: &str,
        #[case] expected: UserReconstructionError,
    ) {
        let result = UserRole::from_str(input);
        assert_eq!(result, Err(expected));
    }

    #[rstest]
    #[case(UserRole::Admin, "admin")]
    #[case(UserRole::User, "user")]
    fn test_user_role_to_string(#[case] role: UserRole, #[case] expected: &str) {
        let role_str = role.to_string();
        assert_eq!(role_str, expected);
    }
}
