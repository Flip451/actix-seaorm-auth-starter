use domain::{
    auth::policy::Actor,
    user::{UserId, UserRole},
};
use uuid::Uuid;

pub trait Identity: std::fmt::Debug + Send {
    fn actor_id(&self) -> Uuid;
    fn actor_role(&self) -> UserRoleData;
}

#[derive(derive_more::Debug)]
pub(crate) struct IdentityWrapper {
    inner: Box<dyn Identity>,
}

impl Actor for IdentityWrapper {
    fn actor_id(&self) -> UserId {
        self.inner.actor_id().into()
    }

    fn actor_role(&self) -> UserRole {
        self.inner.actor_role().into()
    }
}

impl From<Box<dyn Identity>> for IdentityWrapper {
    fn from(identity: Box<dyn Identity>) -> Self {
        Self { inner: identity }
    }
}

#[derive(derive_more::Debug, Clone, Copy, strum::Display, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum UserRoleData {
    Admin,
    User,
}

impl From<UserRoleData> for UserRole {
    fn from(role: UserRoleData) -> Self {
        match role {
            UserRoleData::Admin => UserRole::Admin,
            UserRoleData::User => UserRole::User,
        }
    }
}

impl From<UserRole> for UserRoleData {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => UserRoleData::Admin,
            UserRole::User => UserRoleData::User,
        }
    }
}
