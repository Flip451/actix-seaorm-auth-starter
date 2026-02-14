use domain::{
    auth::policy::Actor,
    user::{UserId, UserRole},
};
use uuid::Uuid;

pub trait Identity: std::fmt::Debug + Send + Sync {
    fn actor_id(&self) -> Uuid;
    fn actor_role(&self) -> UserRoleData;
}

#[derive(derive_more::Debug)]
pub(crate) struct IdentityWrapper<T: Identity> {
    inner: T,
}

impl<T: Identity> Actor for IdentityWrapper<T> {
    fn actor_id(&self) -> UserId {
        self.inner.actor_id().into()
    }

    fn actor_role(&self) -> UserRole {
        self.inner.actor_role().into()
    }
}

impl Identity for &Box<dyn Identity> {
    fn actor_id(&self) -> Uuid {
        self.as_ref().actor_id()
    }

    fn actor_role(&self) -> UserRoleData {
        self.as_ref().actor_role()
    }
}

impl<'a> From<&'a Box<dyn Identity>> for IdentityWrapper<&'a Box<dyn Identity>> {
    fn from(identity: &'a Box<dyn Identity>) -> Self {
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
