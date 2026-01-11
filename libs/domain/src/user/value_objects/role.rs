use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, Default,
)]
#[strum(serialize_all = "lowercase")]
pub enum UserRole {
    Admin,
    #[default]
    User,
}
